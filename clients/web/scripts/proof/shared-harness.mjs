import { mkdir, rm, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';
import process from 'node:process';
import { chromium } from 'playwright-core';
import { createServer } from 'vite';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const webRoot = resolve(__dirname, '../..');
export const repoRoot = resolve(__dirname, '../../../..');
export const baseUrl = process.env.VEL_WEB_PROOF_URL ?? 'http://127.0.0.1:4173';
const chromePath = process.env.CHROME_PATH ?? '/etc/profiles/per-user/jove/bin/google-chrome';

export function resolveProofRoot() {
  return resolve(
    repoRoot,
    process.env.VEL_WEB_PROOF_ROOT ?? '.planning/milestones/v0.5.1-client-reconnection/71-evidence',
  );
}

export async function prepareFlowDir(flowName) {
  const flowDir = join(resolveProofRoot(), flowName);
  await rm(flowDir, { recursive: true, force: true });
  await mkdir(flowDir, { recursive: true });
  return flowDir;
}

export async function writeJson(flowDir, fileName, value) {
  await writeFile(join(flowDir, fileName), `${JSON.stringify(value, null, 2)}\n`);
}

export async function writeText(flowDir, fileName, value) {
  await writeFile(join(flowDir, fileName), value);
}

export async function writeEvidenceNote(flowDir, {
  title,
  tested,
  expected,
  observed,
  deviation = 'None.',
  command,
}) {
  const body = `# ${title}

## Command

\`${command}\`

## What Was Tested

${tested}

## Expected Canonical Behavior

${expected}

## Observed Result

${observed}

## Deviation

${deviation}
`;
  await writeText(flowDir, 'NOTE.md', body);
}

async function startDevServer() {
  const server = await createServer({
    root: webRoot,
    configFile: resolve(webRoot, 'vite.config.ts'),
    logLevel: 'silent',
    server: {
      host: '127.0.0.1',
      port: 4173,
      strictPort: true,
    },
  });
  await server.listen();
  return server;
}

export async function withBrowserFlow(flowName, handleApi, runFlow) {
  const flowDir = await prepareFlowDir(flowName);
  const server = await startDevServer();
  const networkLog = [];
  const browserLog = [];

  const browser = await chromium.launch({
    executablePath: chromePath,
    headless: true,
    args: ['--headless=new', '--disable-gpu', '--no-first-run', '--no-default-browser-check'],
  });

  const context = await browser.newContext();
  const page = await context.newPage();
  page.setDefaultTimeout(10000);
  page.on('console', (message) => {
    browserLog.push({ type: 'console', text: message.text() });
  });
  page.on('pageerror', (error) => {
    browserLog.push({ type: 'pageerror', text: error.message });
  });

  await page.addInitScript(() => {
    class NoopWebSocket {
      static CONNECTING = 0;
      static OPEN = 1;
      static CLOSING = 2;
      static CLOSED = 3;

      readyState = NoopWebSocket.OPEN;
      url;
      protocol = '';
      extensions = '';
      bufferedAmount = 0;
      binaryType = 'blob';
      onopen = null;
      onmessage = null;
      onerror = null;
      onclose = null;

      constructor(url) {
        this.url = url;
      }

      close() {
        this.readyState = NoopWebSocket.CLOSED;
      }

      send() {}

      addEventListener() {}

      removeEventListener() {}
    }

    window.WebSocket = NoopWebSocket;
  });

  await page.route('**/*', async (route) => {
    const request = route.request();
    const url = new URL(request.url());
    const isApi = url.origin === baseUrl && (url.pathname.startsWith('/api/') || url.pathname.startsWith('/v1/'));
    if (!isApi) {
      return route.continue();
    }

    const body = request.postDataJSON?.() ?? request.postData() ?? null;
    const result = await handleApi({ request, url, body, networkLog });
    if (!result) {
      networkLog.push({ method: request.method(), path: `${url.pathname}${url.search}`, body, status: 500, missing: true });
      return route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({
          ok: false,
          error: { message: `No fixture for ${request.method()} ${url.pathname}` },
          meta: { request_id: 'req_missing_fixture' },
        }),
      });
    }

    const status = result.status ?? 200;
    const json = 'json' in result ? result.json : result;
    networkLog.push({ method: request.method(), path: `${url.pathname}${url.search}`, body, status, json });
    return route.fulfill({
      status,
      contentType: 'application/json',
      body: JSON.stringify(json),
    });
  });

  try {
    await runFlow({
      page,
      flowDir,
      networkLog,
      writeJson: (name, value) => writeJson(flowDir, name, value),
      writeText: (name, value) => writeText(flowDir, name, value),
      writeEvidenceNote: (note) => writeEvidenceNote(flowDir, note),
      screenshot: async (name, locator = null) => {
        const target = locator ?? page;
        await target.screenshot({ path: join(flowDir, name) });
      },
    });
    await writeJson(flowDir, 'network-log.json', networkLog);
    await writeJson(flowDir, 'browser-log.json', browserLog);
  } catch (error) {
    await writeJson(flowDir, 'network-log.json', networkLog);
    await writeJson(flowDir, 'browser-log.json', browserLog);
    try {
      await writeText(flowDir, 'failure-dom.html', await page.content());
      await page.screenshot({ path: join(flowDir, 'failure.png') });
    } catch {
      // Best-effort diagnostics only.
    }
    throw error;
  } finally {
    await browser.close();
    await server.close();
  }
}
