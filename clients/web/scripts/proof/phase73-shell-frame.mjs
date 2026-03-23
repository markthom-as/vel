import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/milestones/v0.5.2-operator-surface-embodiment/73-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const {
  apiSuccess,
  buildAgentInspectData,
  buildConversations,
  buildConversationMessages,
  buildIntegrationConnections,
  buildIntegrationsData,
  buildNowData,
} = fixtures;

const flowName = 'shell-frame';
const command = 'npm run proof:phase73:shell-frame';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(buildConversations());
  if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') {
    return apiSuccess(buildConversationMessages());
  }
  if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(buildAgentInspectData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(buildIntegrationsData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(buildIntegrationConnections());
  return null;
}, async ({ page, writeJson, writeEvidenceNote, screenshot }) => {
  await page.goto(baseUrl);

  const nav = page.getByRole('navigation', { name: 'Primary' });
  await nav.waitFor();

  for (const label of ['Now', 'Threads', 'System']) {
    await page.getByRole('button', { name: label }).waitFor();
  }

  assert.equal(await page.getByRole('button', { name: /Open info/i }).count(), 0);
  assert.equal(await page.getByRole('button', { name: /Close info/i }).count(), 0);

  await page.getByRole('button', { name: 'Threads' }).click();
  await page.getByRole('heading', { name: 'Proposal thread' }).waitFor();

  await page.getByRole('button', { name: 'System' }).click();
  await page.getByRole('heading', { name: 'Canonical object and capability truth' }).waitFor();

  await screenshot('shell-frame.png');

  await writeJson('dom-summary.json', {
    navButtons: await page.getByRole('navigation', { name: 'Primary' }).getByRole('button').evaluateAll(
      (buttons) => buttons.map((button) => button.textContent?.trim()),
    ),
    openInfoCount: await page.getByRole('button', { name: /Open info/i }).count(),
    closeInfoCount: await page.getByRole('button', { name: /Close info/i }).count(),
    activeHeading: await page.getByRole('heading', { level: 1 }).textContent(),
  });

  await writeEvidenceNote({
    title: 'Phase 73 Browser Proof — Shared Shell Frame',
    command,
    tested: 'Loaded the app in a real browser, verified the three-surface nav frame, confirmed the global info rail toggle is absent, and navigated across Threads and System using the shared shell.',
    expected: 'The shell should expose only Now, Threads, and System, keep icon-plus-label nav, and remove the old global info rail from the shared frame.',
    observed: 'The browser rendered the three-surface nav, exposed no global info toggle, and preserved the shared frame while navigating into Threads and System.',
  });
});
