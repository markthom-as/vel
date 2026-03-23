import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/milestones/v0.5.2-operator-surface-embodiment/76-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const {
  apiSuccess,
  buildAgentInspectData,
  buildContextExplain,
  buildDriftExplain,
  buildIntegrationConnections,
  buildIntegrationsData,
  buildNowData,
} = fixtures;

const flowName = 'system-read';
const command = 'npm run proof:phase76:system-read';

await withBrowserFlow(flowName, async ({ request, url }) => {
  if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(buildNowData());
  if (request.method() === 'GET' && url.pathname === '/v1/explain/context') return buildContextExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/explain/drift') return buildDriftExplain();
  if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(buildAgentInspectData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(buildIntegrationsData());
  if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(buildIntegrationConnections());
  return null;
}, async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
  await page.goto(baseUrl);
  await page.getByRole('button', { name: 'System' }).click();

  const main = page.locator('main');
  await main.getByRole('heading', { name: /Canonical object and capability truth/i }).waitFor();
  await main.getByText('Structure', { exact: true }).waitFor();
  await main.getByRole('button', { name: /People/i }).waitFor();
  await main.getByRole('button', { name: /Integrations/i }).click();

  await main.getByRole('heading', { name: 'Integrations' }).waitFor();
  assert.equal(await main.getByText('Domain', { exact: true }).count() > 0, true);
  assert.equal(await main.getByText('Capabilities', { exact: true }).count() > 0, true);
  assert.equal(await main.getByText('Configuration', { exact: true }).count() > 0, true);

  assert.equal(await main.getByRole('button', { name: 'Refresh' }).count() > 0, true);
  assert.equal(await main.getByRole('button', { name: 'Disconnect' }).count() > 0, true);
  assert.equal(await main.getByRole('button', { name: /Reconnect/i }).count(), 0);
  assert.equal(await main.getByRole('button', { name: /Enable/i }).count(), 0);

  await screenshot('system-read.png', main);
  await writeJson('dom-summary.json', {
    groupLabels: ['Domain', 'Capabilities', 'Configuration'],
    subsectionHeading: await main.getByRole('heading', { name: 'Integrations' }).textContent(),
    allowListedActions: {
      refresh: await main.getByRole('button', { name: 'Refresh' }).count(),
      disconnect: await main.getByRole('button', { name: 'Disconnect' }).count(),
      reconnect: await main.getByRole('button', { name: /Reconnect/i }).count(),
      enable: await main.getByRole('button', { name: /Enable/i }).count(),
    },
  });

  await writeEvidenceNote({
    title: 'Phase 76 Browser Proof — System Structural Read',
    command,
    tested: 'Loaded `/system` through the shipped shell, verified the grouped sidebar-plus-detail posture, and confirmed that the `Integrations` detail still exposes only allow-listed canonical actions.',
    expected: 'System should read as one structural surface with visible `Domain` / `Capabilities` / `Configuration` grouping, one active detail pane, and only named canonical actions such as `Refresh` and `Disconnect`.',
    observed: 'The browser rendered the grouped sidebar, switched into the `Integrations` detail pane, and exposed only `Refresh` / `Disconnect` actions with no inferred `Reconnect` or scope-enablement controls.',
  });
});
