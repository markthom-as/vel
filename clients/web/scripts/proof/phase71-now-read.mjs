import assert from 'node:assert/strict';
import { withBrowserFlow, baseUrl } from './shared-harness.mjs';
import { apiSuccess, buildContextExplain, buildDriftExplain, buildNowData } from './fixtures.mjs';

const flowName = 'now-read';
const command = 'npm run proof:phase71:now-read';

const nowData = buildNowData();

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(nowData);
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  return null;
}, async ({ page, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  const main = page.locator('main');
  await main.getByRole('heading', { name: "Jove's Now" }).waitFor();

  const tasksHeading = main.getByRole('heading', { name: 'Tasks' });
  const calendarHeading = main.getByRole('heading', { name: 'Calendar' });
  await tasksHeading.waitFor();
  await calendarHeading.waitFor();

  assert.equal(await main.getByText('Write weekly review').isVisible(), true);
  assert.equal(await main.getByText('Design review').isVisible(), true);
  assert.equal(await main.getByRole('button', { name: /Open inbox/i }).count(), 0);
  assert.equal(await main.getByRole('button', { name: /Reschedule/i }).count(), 0);

  await writeJson('dom-summary.json', {
    headings: ['Tasks', 'Calendar'],
    taskVisible: true,
    eventVisible: true,
    inboxButtonCount: await main.getByRole('button', { name: /Open inbox/i }).count(),
    rescheduleButtonCount: await main.getByRole('button', { name: /Reschedule/i }).count(),
  });

  await writeEvidenceNote({
    title: 'Phase 71 Browser Proof — Now Canonical Read',
    command,
    tested: 'Loaded the shipped app in a real browser, stayed on the default Now surface, and verified the canonical task and calendar sections render without reviving Inbox-era affordances.',
    expected: 'Now should render adjacent canonical Tasks and Calendar sections, show current task/event truth, and avoid synthetic Inbox or local reschedule controls.',
    observed: "The browser rendered `Jove's Now`, separate `Tasks` and `Calendar` headings, the active task `Write weekly review`, and the upcoming event `Design review`. No Inbox or reschedule affordances rendered.",
  });
});
