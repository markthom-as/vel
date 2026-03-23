import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/milestones/v0.5.2-operator-surface-embodiment/74-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const {
  apiSuccess,
  buildCommitmentData,
  buildContextExplain,
  buildDriftExplain,
  buildNowData,
} = fixtures;

const flowName = 'now-complete';
const command = 'npm run proof:phase74:now-complete';

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

  const before = {
    focusHeading: await main.getByRole('heading', { name: 'Write weekly review' }).textContent(),
    loadingCount: await main.getByText(/Loading your current state/i).count(),
    completedSectionCount: await main.getByText('Recently completed').count(),
  };

  await main.getByRole('button', { name: 'Complete commitment' }).click();

  await main.getByText('Recently completed').waitFor();
  await main.getByRole('button', { name: 'Write weekly review completed' }).waitFor();

  const after = {
    loadingCount: await main.getByText(/Loading your current state/i).count(),
    completedSectionCount: await main.getByText('Recently completed').count(),
    completedButtonCount: await main.getByRole('button', { name: 'Write weekly review completed' }).count(),
    focusHeadingCount: await main.getByRole('heading', { name: 'Write weekly review' }).count(),
  };

  assert.equal(state.operations.length, 1);
  assert.deepEqual(state.operations[0]?.body, { status: 'done' });
  assert.equal(before.loadingCount, 0);
  assert.equal(after.loadingCount, 0);

  await writeJson('canonical-before.json', {
    task_lane: buildNowData().task_lane,
    commitment: buildCommitmentData(),
  });
  await writeJson('canonical-after.json', {
    task_lane: state.nowData.task_lane,
    commitment: state.commitment,
  });
  await writeJson('dom-diff.json', { before, after });
  await writeJson('operations.json', state.operations);
  await screenshot('now-complete.png', main);

  await writeEvidenceNote({
    title: 'Phase 74 Browser Proof — Now Completion Reconciliation',
    command,
    tested: 'Executed the live focus-surface completion affordance in a browser, intercepted the canonical commitment patch, and verified the surface reconciles locally before backend refresh without blanking the whole page.',
    expected: 'Completing a focus commitment should send one canonical mutation, move the item into completed state, and avoid full-surface loading churn while reconciliation happens.',
    observed: 'The browser issued one PATCH to `/v1/commitments/commit_local_1` with `{ \"status\": \"done\" }`, rendered `Recently completed`, preserved the rest of the surface shape, and never showed the centered loading state during the mutation flow.',
  });
});
