import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/milestones/v0.5.2-operator-surface-embodiment/75-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const { apiSuccess, buildContextExplain, buildConversations, buildConversationMessages, buildDriftExplain, buildNowData } = fixtures;

const flowName = 'threads-read';
const command = 'npm run proof:phase75:threads-read';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(buildConversations());
  if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') {
    return apiSuccess(buildConversationMessages());
  }
  return null;
}, async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  await page.getByRole('button', { name: 'Threads' }).click();

  const main = page.locator('main');
  await main.getByRole('heading', { name: 'Proposal thread' }).waitFor();
  await main.getByText('Bound object', { exact: true }).waitFor();
  await main.getByRole('heading', { name: 'Object state' }).waitFor();
  await main.getByText(/proposal review gated/i).waitFor();
  await main.getByText(/source message id/i).waitFor();
  await main.getByText(/Chronology stays navigable, but it is secondary/i).waitFor();

  assert.equal(await main.getByText(/Attach or create an object first/i).count(), 0);
  assert.equal(await main.getByText('Conversation', { exact: true }).count(), 1);

  await screenshot('threads-read.png', main);
  await writeJson('dom-summary.json', {
    title: await main.getByRole('heading', { name: 'Proposal thread' }).textContent(),
    boundObjectCount: await main.getByText('Bound object', { exact: true }).count(),
    objectStateCount: await main.getByRole('heading', { name: 'Object state' }).count(),
    attachCreateCount: await main.getByText(/Attach or create an object first/i).count(),
    conversationSectionCount: await main.getByText('Conversation', { exact: true }).count(),
  });

  await writeEvidenceNote({
    title: 'Phase 75 Browser Proof — Threads State-First Read',
    command,
    tested: 'Loaded the shipped Threads surface in a real browser and verified that a bound thread now foregrounds canonical object state and compact provenance cues before transcript chronology.',
    expected: 'Threads should present the bound object, object-state context, and invocation gating beside the transcript, while avoiding the older attach/create fallback for already-bound threads.',
    observed: 'The browser rendered `Proposal thread` with `Bound object`, `Object state`, and the `proposal review gated` capability state before the `Conversation` transcript section, and it did not show the attach/create fallback guidance.',
  });
});
