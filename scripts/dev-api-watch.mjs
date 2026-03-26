#!/usr/bin/env node
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import {
  collectWatchSignature,
  spawnDetached,
  stopDetachedProcess,
} from './dev-watch-lib.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..');
const pollIntervalMs = parsePollInterval(process.env.VEL_DEV_RUST_POLL_INTERVAL_MS);
const requireInitialHealth = process.env.VEL_DEV_REQUIRE_INITIAL_HEALTH === '1';
const initialHealthUrl = process.env.VEL_DEV_HEALTHCHECK_URL ?? '';
const watchPaths = [
  path.join(repoRoot, 'Cargo.toml'),
  path.join(repoRoot, 'Cargo.lock'),
  path.join(repoRoot, 'crates'),
];

let currentSignature = await collectWatchSignature(watchPaths, {
  includeFile,
  relativeTo: repoRoot,
});
let veldProcess = null;
let polling = false;
let restarting = false;
let restartQueued = false;
let restartReason = 'Rust source change';
let shuttingDown = false;
let hasSeenHealthy = !requireInitialHealth || initialHealthUrl === '';

console.log(`Watching Rust workspace for veld restarts (${pollIntervalMs}ms poll)...`);
veldProcess = startVeldProcess('initial launch');

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
      await scheduleRestart('Rust source change');
    }
  } catch (error) {
    console.error(`dev-api watcher failed to scan Rust files: ${error.message}`);
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

function startVeldProcess(reason) {
  console.log(reason === 'initial launch' ? 'Starting veld in watch mode...' : `Restarting veld after ${reason}...`);
  const child = spawnDetached('bash', [path.join(repoRoot, 'scripts/dev-api.sh')], {
    cwd: repoRoot,
    env: process.env,
  });

  child.on('exit', (code, signal) => {
    if (child === veldProcess) {
      veldProcess = null;
    }
    if (shuttingDown || restarting) {
      return;
    }
    const detail = signal ? `signal ${signal}` : `code ${code ?? 0}`;
    if (!hasSeenHealthy) {
      console.error(`veld exited before passing the initial health check (${detail}).`);
      process.exit(typeof code === 'number' ? code : 1);
    }
    console.log(`veld exited (${detail}). Waiting for another Rust change before retrying.`);
  });

  return child;
}

async function scheduleRestart(reason) {
  restartReason = reason;
  if (restarting) {
    restartQueued = true;
    return;
  }

  restarting = true;
  try {
    do {
      restartQueued = false;
      await stopDetachedProcess(veldProcess);
      if (shuttingDown) {
        return;
      }
      veldProcess = startVeldProcess(restartReason);
    } while (restartQueued && !shuttingDown);
  } finally {
    restarting = false;
  }
}

async function shutdown(signal) {
  if (shuttingDown) {
    return;
  }

  shuttingDown = true;
  clearInterval(pollTimer);
  console.log(`Stopping dev-api watcher (${signal})...`);
  await stopDetachedProcess(veldProcess);
  process.exit(0);
}

if (!hasSeenHealthy) {
  const healthTimer = setInterval(async () => {
    if (shuttingDown || hasSeenHealthy) {
      clearInterval(healthTimer);
      return;
    }

    try {
      const response = await fetch(initialHealthUrl, { signal: AbortSignal.timeout(1000) });
      if (response.ok) {
        hasSeenHealthy = true;
        clearInterval(healthTimer);
      }
    } catch {
      // Keep polling until veld becomes healthy or exits.
    }
  }, 500);
}
