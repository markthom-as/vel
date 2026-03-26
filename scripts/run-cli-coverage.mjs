#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync, spawnSync } from 'node:child_process';

const repoRoot = process.cwd();
const outputDir = path.join(repoRoot, 'target/coverage/vel-cli');
const summaryPath = path.join(outputDir, 'llvm-cov-summary.json');
const lcovPath = path.join(outputDir, 'lcov.info');

function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function binaryExists(name) {
  const result = spawnSync('bash', ['-lc', `command -v ${name}`], {
    cwd: repoRoot,
    stdio: 'ignore',
  });
  return result.status === 0;
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    stdio: 'inherit',
    env: { ...process.env, ...(options.env ?? {}) },
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function rustcLlvmMajor() {
  const output = execFileSync('rustc', ['-vV'], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  const match = output.match(/^LLVM version:\s+(\d+)\./m);
  if (!match) {
    throw new Error('unable to determine rustc LLVM version');
  }
  return match[1];
}

function coverageCommand() {
  if (!binaryExists('cargo-llvm-cov')) {
    throw new Error(
      'cargo-llvm-cov is not installed. Install it with `cargo install cargo-llvm-cov --locked`.'
    );
  }

  if (binaryExists('llvm-cov') && binaryExists('llvm-profdata')) {
    return {
      command: 'cargo',
      args: [],
      steps: [
        [
          'llvm-cov',
          '-p',
          'vel-cli',
          '--summary-only',
          '--json',
          '--output-path',
          summaryPath,
        ],
        [
          'llvm-cov',
          'report',
          '-p',
          'vel-cli',
          '--lcov',
          '--output-path',
          lcovPath,
        ],
      ],
    };
  }

  if (!binaryExists('nix-shell')) {
    throw new Error(
      'llvm-cov/llvm-profdata are missing and nix-shell is unavailable. Set LLVM_COV and LLVM_PROFDATA, or run inside the repo nix shell.'
    );
  }

  const llvmMajor = rustcLlvmMajor();
  const shellCommand = [
    'export LLVM_COV="$(which llvm-cov)"',
    'export LLVM_PROFDATA="$(which llvm-profdata)"',
    'cargo llvm-cov -p vel-cli --summary-only --json --output-path "$1"',
    'cargo llvm-cov report -p vel-cli --lcov --output-path "$2"',
  ].join('; ');

  return {
    command: 'nix-shell',
    args: [
      '-p',
      `llvmPackages_${llvmMajor}.llvm`,
      '--run',
      `bash -lc '${shellCommand}' bash ${JSON.stringify(summaryPath)} ${JSON.stringify(lcovPath)}`,
    ],
  };
}

ensureDir(outputDir);
const { command, args, steps } = coverageCommand();
if (steps) {
  for (const stepArgs of steps) {
    run(command, stepArgs);
  }
} else {
  run(command, args);
}

if (!fs.existsSync(summaryPath)) {
  throw new Error(`expected coverage summary at ${summaryPath}`);
}
if (!fs.existsSync(lcovPath)) {
  throw new Error(`expected LCOV report at ${lcovPath}`);
}

console.log(`run-cli-coverage: wrote ${path.relative(repoRoot, summaryPath)}`);
console.log(`run-cli-coverage: wrote ${path.relative(repoRoot, lcovPath)}`);
