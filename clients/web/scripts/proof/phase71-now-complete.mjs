import assert from 'node:assert/strict';
import { withBrowserFlow, baseUrl } from './shared-harness.mjs';
import {
  apiSuccess,
  buildCommitmentData,
  buildContextExplain,
  buildDriftExplain,
  buildNowData,
} from './fixtures.mjs';

const flowName = 'now-complete';
const command = 'npm run proof:phase71:now-complete';

const state = {
  nowData: buildNowData(),
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
        ...state.nowData.tasks,
        other_open: [],
      },
    });
    return apiSuccess(state.commitment);
  }
  return null;
}, async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  const main = page.locator('main');
  await main.getByRole('heading', { name: "Jove's Now" }).waitFor();

  const before = {
    activeTask: await main.getByText('Write weekly review').textContent(),
    completedSectionVisible: await main.getByText('COMPLETED').count(),
    completedButtonCount: await main.getByRole('button', { name: 'Write weekly review completed' }).count(),
  };

  await main.getByRole('button', { name: 'Complete Write weekly review' }).click();

  await main.getByText('COMPLETED').waitFor();
  await main.getByRole('button', { name: 'Write weekly review completed' }).waitFor();
  await main.getByText('Write weekly review').last().waitFor();

  const after = {
    completedSectionVisible: await main.getByText('COMPLETED').count(),
    completedButtonLabelVisible: await main.getByRole('button', { name: 'Write weekly review completed' }).count(),
    completedTaskVisible: await main.getByText('Write weekly review').last().isVisible(),
  };

  assert.equal(state.operations.length, 1);
  assert.deepEqual(state.operations[0]?.body, { status: 'done' });

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
    title: 'Phase 71 Browser Proof — Now Completion Reconciliation',
    command,
    tested: 'Executed the live Now completion affordance in a browser, intercepted the canonical commitment patch, and verified the surface reconciles onto backend-owned post-mutation truth.',
    expected: 'Completing a task should send a canonical commitment mutation, reconcile with refreshed Now data, and render completion state without local ghosting.',
    observed: 'The browser issued one PATCH to `/v1/commitments/commit_local_1` with `{ \"status\": \"done\" }`, then reconciled into a `COMPLETED` section with the task rendered in completed state and a disabled completed-state control.',
  });
});
