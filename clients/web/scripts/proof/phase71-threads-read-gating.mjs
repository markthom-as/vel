import assert from 'node:assert/strict';
import { withBrowserFlow, baseUrl } from './shared-harness.mjs';
import { apiSuccess, buildContextExplain, buildConversations, buildConversationMessages, buildDriftExplain, buildNowData } from './fixtures.mjs';

const flowName = 'threads-read-gating';
const command = 'npm run proof:phase71:threads-read-gating';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(buildConversations());
  if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') {
    return apiSuccess(buildConversationMessages());
  }
  return null;
}, async ({ page, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  await page.getByRole('button', { name: 'Threads' }).click();
  await page.getByRole('heading', { name: 'Proposal thread' }).waitFor();

  const gatingText = page.getByText(/Attach or create an object first/i);
  await gatingText.waitFor();

  assert.equal(await page.getByRole('button', { name: /Run workflow|Dry run|Execute workflow/i }).count(), 0);

  await writeJson('dom-summary.json', {
    heading: 'Proposal thread',
    gatingText: await gatingText.textContent(),
    workflowActionButtons: await page.getByRole('button', { name: /Run workflow|Dry run|Execute workflow/i }).count(),
  });

  await writeEvidenceNote({
    title: 'Phase 71 Browser Proof — Threads Canonical Read And Invocation Gating',
    command,
    tested: 'Loaded the shipped Threads surface in a real browser and verified that continuation context renders while workflow invocation stays gated when no bound canonical object is available.',
    expected: 'Threads should show canonical conversation truth, surface the attach-or-create guidance, and refuse to invent floating workflow execution controls.',
    observed: 'The browser rendered `Proposal thread`, displayed the explicit attach/create-object guidance, and exposed no workflow execution controls.',
  });
});
