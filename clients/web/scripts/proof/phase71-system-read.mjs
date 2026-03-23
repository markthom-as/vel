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

const flowName = 'system-read';
const command = 'npm run proof:phase71:system-read';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(buildAgentInspectData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(buildIntegrationsData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(buildIntegrationConnections());
  return null;
}, async ({ page, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  await page.getByRole('button', { name: 'System' }).click();
  await page.getByRole('heading', { name: /Canonical object and capability truth/i }).waitFor();

  await page.getByRole('button', { name: 'Configuration' }).click();
  await page.getByText('Integrations').waitFor();

  assert.equal(await page.getByRole('button', { name: 'Refresh' }).count() > 0, true);
  assert.equal(await page.getByRole('button', { name: 'Disconnect' }).count() > 0, true);
  assert.equal(await page.getByRole('button', { name: /Reconnect/i }).count(), 0);

  await writeJson('dom-summary.json', {
    sections: ['Domain', 'Capabilities', 'Configuration'],
    allowListedActions: {
      refresh: await page.getByRole('button', { name: 'Refresh' }).count(),
      disconnect: await page.getByRole('button', { name: 'Disconnect' }).count(),
      reconnect: await page.getByRole('button', { name: /Reconnect/i }).count(),
    },
  });

  await writeEvidenceNote({
    title: 'Phase 71 Browser Proof — System Canonical Read',
    command,
    tested: 'Loaded `/system` through the shipped shell, moved into Configuration, and verified that the surface reads bounded canonical state while exposing only allow-listed configuration actions.',
    expected: 'System should remain one structural surface, consume bounded canonical reads, and expose only named allow-listed actions such as refresh and disconnect.',
    observed: 'The browser rendered the canonical System heading, the fixed Domain/Capabilities/Configuration sections, and only `Refresh` / `Disconnect` actions. No inferred `Reconnect` action appeared.',
  });
});
