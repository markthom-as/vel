import assert from 'node:assert/strict';
import { withBrowserFlow, baseUrl } from './shared-harness.mjs';
import {
  apiSuccess,
  buildAgentInspectData,
  buildContextExplain,
  buildDriftExplain,
  buildIntegrationConnections,
  buildIntegrationsData,
  buildNowData,
} from './fixtures.mjs';

const flowName = 'system-degraded';
const command = 'npm run proof:phase71:system-degraded';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') {
    return apiSuccess(buildAgentInspectData(), { degraded: true });
  }
  if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(buildIntegrationsData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(buildIntegrationConnections());
  return null;
}, async ({ page, writeJson, writeEvidenceNote, screenshot }) => {
  await page.goto(baseUrl);
  await page.getByRole('button', { name: 'System' }).click();

  const degradedMessage = page.getByText('Degraded canonical query response for /v1/agent/inspect');
  await degradedMessage.waitFor();
  await screenshot('degraded-system.png', page.locator('main'));

  assert.equal(await page.getByText('Avery').count(), 0);

  await writeJson('dom-summary.json', {
    degradedMessage: await degradedMessage.textContent(),
    peopleVisible: await page.getByText('Avery').count(),
  });

  await writeEvidenceNote({
    title: 'Phase 71 Browser Proof — Controlled Degraded State',
    command,
    tested: 'Injected a controlled degraded canonical inspect response in a real browser and verified that System renders an explicit degraded state rather than silently using stale structural data.',
    expected: 'A degraded canonical response should fail loudly in development-mode proof runs and render explicit degraded UI instead of stale canonical content.',
    observed: 'The browser rendered the explicit degraded response message for `/v1/agent/inspect`, and canonical detail content such as `Avery` did not render.',
  });
});
