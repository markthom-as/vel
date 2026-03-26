#!/usr/bin/env node
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import {
  collectWatchSignature,
  spawnDetached,
  stopDetachedProcess,
} from '../../../scripts/dev-watch-lib.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const webRoot = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(webRoot, '..', '..');
const pollIntervalMs = parsePollInterval(process.env.VEL_DEV_RUST_POLL_INTERVAL_MS);
const buildScript = path.join(webRoot, 'scripts/build-embedded-bridge-wasm.sh');
const bridgeOutDir = path.join(webRoot, 'public', 'embedded-bridge');
const sourceSignatureFile = path.join(bridgeOutDir, '.source-signature');
const outputJsFile = path.join(bridgeOutDir, 'vel-embedded-bridge.js');
const outputWasmFile = path.join(bridgeOutDir, 'vel-embedded-bridge_bg.wasm');
const watchPaths = [
  path.join(repoRoot, 'Cargo.toml'),
  path.join(repoRoot, 'Cargo.lock'),
  path.join(repoRoot, 'crates/vel-embedded-bridge'),
];

let currentSignature = await collectWatchSignature(watchPaths, {
  includeFile,
  relativeTo: repoRoot,
});
let viteProcess = null;
let buildQueued = false;
let buildRunning = false;
let buildReason = 'Rust bridge change';
let polling = false;
let shuttingDown = false;

console.log(`Watching vel-embedded-bridge for browser rebuilds (${pollIntervalMs}ms poll)...`);
await ensureBridgeBuild('initial launch');
viteProcess = startVite();

const pollTimer = setInterval(async () => {
  if (polling || shuttingDown) {
    return;
  }

  polling = true;
  try {
    const nextSignature = await collectWatchSignature(watchPaths, {
      includeFile,
      relativeTo: repoRoot,
    });
    if (nextSignature !== currentSignature) {
      currentSignature = nextSignature;
      await scheduleBridgeBuild('Rust bridge change');
    }
  } catch (error) {
    console.error(`web bridge watcher failed to scan Rust files: ${error.message}`);
  } finally {
    polling = false;
  }
}, pollIntervalMs);

process.on('SIGINT', () => {
  void shutdown('SIGINT');
});

process.on('SIGTERM', () => {
  void shutdown('SIGTERM');
});

function includeFile(filePath) {
  return (
    filePath.endsWith('.rs')
    || path.basename(filePath) === 'Cargo.toml'
    || path.basename(filePath) === 'Cargo.lock'
  );
}

function parsePollInterval(rawValue) {
  const parsed = Number.parseInt(rawValue ?? '1000', 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 1000;
}

function startVite() {
  console.log('Starting Vite dev server...');
  const child = spawnDetached('npm', ['run', 'dev:vite'], {
    cwd: webRoot,
    env: process.env,
  });

  child.on('exit', (code, signal) => {
    if (child === viteProcess) {
      viteProcess = null;
    }
    if (shuttingDown) {
      return;
    }
    clearInterval(pollTimer);
    const detail = signal ? `signal ${signal}` : `code ${code ?? 0}`;
    console.error(`Vite dev server exited unexpectedly (${detail}).`);
    process.exit(typeof code === 'number' ? code : 1);
  });

  return child;
}

async function runBridgeBuild(reason) {
  const label = reason === 'initial launch'
    ? 'Building embedded Rust bridge before Vite starts...'
    : `Rebuilding embedded Rust bridge after ${reason}...`;
  console.log(label);

  const build = spawn('bash', [buildScript], {
    cwd: webRoot,
    env: process.env,
    stdio: 'inherit',
  });

  const [code, signal] = await once(build, 'exit');
  if (signal) {
    throw new Error(`build terminated by signal ${signal}`);
  }
  if (code !== 0) {
    throw new Error(`build exited with code ${code}`);
  }

  await fs.mkdir(bridgeOutDir, { recursive: true });
  await fs.writeFile(sourceSignatureFile, currentSignature, 'utf8');
}

async function scheduleBridgeBuild(reason) {
  buildReason = reason;
  if (buildRunning) {
    buildQueued = true;
    return;
  }

  buildRunning = true;
  try {
    do {
      buildQueued = false;
      try {
        await runBridgeBuild(buildReason);
      } catch (error) {
        console.error(`Embedded bridge rebuild failed: ${error.message}`);
      }
    } while (buildQueued && !shuttingDown);
  } finally {
    buildRunning = false;
  }
}

async function ensureBridgeBuild(reason) {
  if (await bridgeBuildIsFresh()) {
    console.log('Embedded Rust bridge artifact is current; skipping startup rebuild.');
    return;
  }
  await runBridgeBuild(reason);
}

async function bridgeBuildIsFresh() {
  try {
    const [recordedSignature, jsStats, wasmStats] = await Promise.all([
      fs.readFile(sourceSignatureFile, 'utf8'),
      fs.stat(outputJsFile),
      fs.stat(outputWasmFile),
    ]);

    return (
      recordedSignature === currentSignature
      && jsStats.isFile()
      && wasmStats.isFile()
    );
  } catch {
    return false;
  }
}

async function shutdown(signal) {
  if (shuttingDown) {
    return;
  }

  shuttingDown = true;
  clearInterval(pollTimer);
  console.log(`Stopping web dev watcher (${signal})...`);
  await stopDetachedProcess(viteProcess);
  process.exit(0);
}
