import assert from 'node:assert/strict';
import { withBrowserFlow, baseUrl } from './shared-harness.mjs';
import {
  apiError,
  apiSuccess,
  buildContextExplain,
  buildDriftExplain,
  buildIntegrationConnections,
  buildIntegrationsData,
  buildNowData,
} from './fixtures.mjs';

const flowName = 'system-no-fallback';
const command = 'npm run proof:phase71:system-no-fallback';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') {
    return apiError(500, 'canonical inspect failed');
  }
  if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(buildIntegrationsData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(buildIntegrationConnections());
  return null;
}, async ({ page, writeJson, writeEvidenceNote, screenshot }) => {
  await page.goto(baseUrl);
  await page.getByRole('button', { name: 'System' }).click();

  const errorMessage = page.getByText('API 500: canonical inspect failed');
  await errorMessage.waitFor();
  await screenshot('no-fallback-system.png', page.locator('main'));

  assert.equal(await page.getByRole('button', { name: 'Refresh' }).count(), 0);
  assert.equal(await page.getByText('Google Calendar').count(), 0);

  await writeJson('dom-summary.json', {
    errorMessage: await errorMessage.textContent(),
    refreshButtons: await page.getByRole('button', { name: 'Refresh' }).count(),
    googleCalendarCards: await page.getByText('Google Calendar').count(),
  });

  await writeEvidenceNote({
    title: 'Phase 71 Browser Proof — No Silent Fallback',
    command,
    tested: 'Forced the canonical inspect route to fail in a real browser and verified that System surfaces an explicit error state without falling back to guessed or stale structural content.',
    expected: 'Canonical route failure should block the affected surface, render explicit error UI, and avoid stale or inferred fallback controls.',
    observed: 'The browser rendered `API 500: canonical inspect failed`, and no System integration controls or structural cards silently rendered underneath that error state.',
  });
});
