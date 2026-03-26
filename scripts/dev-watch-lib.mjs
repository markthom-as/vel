#!/usr/bin/env node
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import fs from 'node:fs/promises';
import path from 'node:path';

export async function collectWatchSignature(watchPaths, { includeFile, relativeTo }) {
  const signature = [];
  const sortedPaths = [...watchPaths].sort((left, right) => left.localeCompare(right));

  for (const watchPath of sortedPaths) {
    await collectPath(watchPath);
  }

  return signature.join('\n');

  async function collectPath(targetPath) {
    let stats;
    try {
      stats = await fs.lstat(targetPath);
    } catch (error) {
      if (error?.code === 'ENOENT') {
        signature.push(`missing:${relativeLabel(targetPath)}`);
        return;
      }
      throw error;
    }

    if (stats.isDirectory()) {
      signature.push(`dir:${relativeLabel(targetPath)}`);
      const entries = await fs.readdir(targetPath, { withFileTypes: true });
      entries.sort((left, right) => left.name.localeCompare(right.name));
      for (const entry of entries) {
        await collectPath(path.join(targetPath, entry.name));
      }
      return;
    }

    if (!stats.isFile()) {
      return;
    }

    if (includeFile && !includeFile(targetPath)) {
      return;
    }

    signature.push(
      `file:${relativeLabel(targetPath)}:${stats.size}:${Math.trunc(stats.mtimeMs)}`,
    );
  }

  function relativeLabel(targetPath) {
    return path.relative(relativeTo, targetPath) || path.basename(targetPath);
  }
}

export function spawnDetached(command, args, options = {}) {
  return spawn(command, args, {
    ...options,
    detached: true,
    stdio: 'inherit',
  });
}

export async function stopDetachedProcess(child, { timeoutMs = 5000 } = {}) {
  if (!child || child.exitCode !== null || child.signalCode !== null) {
    return;
  }

  const exitPromise = once(child, 'exit').catch(() => {});

  try {
    process.kill(-child.pid, 'SIGTERM');
  } catch (error) {
    if (error?.code !== 'ESRCH') {
      throw error;
    }
    return;
  }

  const killTimer = setTimeout(() => {
    try {
      process.kill(-child.pid, 'SIGKILL');
    } catch (error) {
      if (error?.code !== 'ESRCH') {
        throw error;
      }
    }
  }, timeoutMs);
  killTimer.unref?.();

  await exitPromise;
  clearTimeout(killTimer);
}
