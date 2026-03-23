import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/milestones/v0.5.2-operator-surface-embodiment/77-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const {
  apiSuccess,
  buildAgentInspectData,
  buildCommitmentData,
  buildContextExplain,
  buildConversationMessages,
  buildConversations,
  buildDriftExplain,
  buildIntegrationConnections,
  buildIntegrationsData,
  buildNowData,
} = fixtures;

const flowName = 'operator-loop';
const command = 'npm run proof:phase77:operator-loop';

const state = {
  nowData: buildNowData({
    tasks: {
      todoist: [],
      other_open: [
        {
          id: 'commit_local_1',
          text: 'Write weekly review',
          source_type: 'local',
          due_at: '2026-03-09T17:00:00Z',
          project: null,
          commitment_kind: 'routine',
        },
      ],
      next_commitment: null,
    },
  }),
  commitment: buildCommitmentData(),
  operations: [],
};

await withBrowserFlow(flowName, async ({ request, url, body }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(state.nowData);
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(buildConversations());
  if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') return apiSuccess(buildConversationMessages());
  if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(buildAgentInspectData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(buildIntegrationsData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(buildIntegrationConnections());
  if (request.method() === 'PATCH' && url.pathname === '/v1/commitments/commit_local_1') {
    state.operations.push({ kind: 'patch', path: url.pathname, body });
    state.commitment = buildCommitmentData({
      status: 'done',
      resolved_at: '2026-03-09T16:05:00Z',
    });
    state.nowData = buildNowData({
      task_lane: {
        active: null,
        pending: state.nowData.task_lane.pending,
        recent_completed: [
          {
            id: 'commit_local_1',
            task_kind: 'commitment',
            text: 'Write weekly review',
            state: 'done',
            project: null,
            primary_thread_id: null,
          },
        ],
        overflow_count: 0,
      },
      tasks: {
        todoist: [],
        other_open: [],
        next_commitment: null,
      },
    });
    return apiSuccess(state.commitment);
  }
  return null;
}, async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  const main = page.locator('main');

  await main.getByRole('heading', { name: 'Now' }).waitFor();
  await main.getByRole('heading', { name: 'Write weekly review' }).waitFor();
  await main.getByRole('button', { name: 'Complete commitment' }).click();
  await main.getByText('Recently completed').waitFor();
  const completionVisible = await main.getByRole('button', { name: 'Write weekly review completed' }).count();

  await main.getByRole('button', { name: /Open thread \(Standup check-in\) · check_in_bar/i }).click();
  await main.getByRole('heading', { name: 'Proposal thread' }).waitFor();
  await main.getByText('Bound object', { exact: true }).waitFor();

  await page.getByRole('button', { name: 'System' }).click();
  await main.getByRole('heading', { name: /Canonical object and capability truth/i }).waitFor();
  await main.getByRole('button', { name: /Integrations/i }).click();
  await main.getByRole('heading', { name: 'Integrations' }).waitFor();

  assert.equal(state.operations.length, 1);
  assert.deepEqual(state.operations[0]?.body, { status: 'done' });
  assert.equal(await main.getByRole('button', { name: /Reconnect/i }).count(), 0);

  await screenshot('operator-loop.png', main);
  await writeJson('operator-loop.json', {
    mutation: state.operations[0],
    surfacesVisited: ['Now', 'Threads', 'System'],
    completionVisible,
    threadHeading: await page.getByRole('heading', { name: 'Proposal thread' }).count(),
    systemHeading: await main.getByRole('heading', { name: 'Integrations' }).count(),
    reconnectCount: await main.getByRole('button', { name: /Reconnect/i }).count(),
  });

  await writeEvidenceNote({
    title: 'Phase 77 Browser Proof — Cross-Surface Operator Loop',
    command,
    tested: 'Ran one shipped operator path through Now, Threads, and System: completed the active commitment on Now, followed the canonical nudge into Threads, then inspected bounded configuration state in System.',
    expected: 'The loop should keep one canonical mutation truthful, preserve local-first reconciliation on Now, carry the operator into a grounded thread view, and end in the grouped System surface without inventing extra actions.',
    observed: 'The browser completed `Write weekly review` with one canonical PATCH, reconciled it into `Recently completed`, moved through `Proposal thread` with bound-object context, and ended in `System > Integrations` with no inferred `Reconnect` action.',
  });
});
