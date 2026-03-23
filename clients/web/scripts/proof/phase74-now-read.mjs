import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/milestones/v0.5.2-operator-surface-embodiment/74-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const { apiSuccess, buildContextExplain, buildDriftExplain, buildNowData } = fixtures;

const flowName = 'now-read';
const command = 'npm run proof:phase74:now-read';

const nowData = buildNowData({
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
});

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(nowData);
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  return null;
}, async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  const main = page.locator('main');

  await main.getByRole('heading', { name: 'Now' }).waitFor();
  await main.getByText('Execution locus').waitFor();
  await main.getByRole('heading', { name: 'Write weekly review' }).waitFor();
  await main.getByRole('heading', { name: 'Commitments' }).waitFor();
  await main.getByRole('heading', { name: 'Calendar' }).waitFor();
  await main.getByRole('heading', { name: 'Triage' }).waitFor();

  assert.equal(await main.getByText('Standup check-in').count() > 0, true);
  assert.equal(await main.getByText('Design review').count() > 0, true);
  assert.equal(await main.getByRole('heading', { name: 'Tasks' }).count(), 0);
  assert.equal(await main.getByRole('button', { name: /Open inbox/i }).count(), 0);
  assert.equal(await main.getByRole('button', { name: /Reschedule/i }).count(), 0);

  await screenshot('now-read.png', main);
  await writeJson('dom-summary.json', {
    headings: ['Focus', 'Commitments', 'Calendar', 'Triage'],
    focusHeading: await main.getByRole('heading', { name: 'Write weekly review' }).textContent(),
    tasksHeadingCount: await main.getByRole('heading', { name: 'Tasks' }).count(),
    inboxButtonCount: await main.getByRole('button', { name: /Open inbox/i }).count(),
    rescheduleButtonCount: await main.getByRole('button', { name: /Reschedule/i }).count(),
  });

  await writeEvidenceNote({
    title: 'Phase 74 Browser Proof — Now Focus-First Read',
    command,
    tested: 'Loaded the shipped app in a real browser, stayed on the default Now surface, and verified the approved focus-first structure over canonical task, calendar, and triage data.',
    expected: 'Now should render Focus, Commitments, Calendar, and Triage in the approved priority gradient, without reviving the older Tasks/TODAY grouping or Inbox-era controls.',
    observed: 'The browser rendered a dominant `Write weekly review` focus block above `Commitments`, `Calendar`, and `Triage`, kept `Standup check-in` in triage, and showed no `Tasks`, `TODAY`, Inbox, or reschedule affordances.',
  });
});
