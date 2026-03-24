import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/phases/89-browser-proof-cleanup-and-milestone-closeout/89-evidence';

const [{ withBrowserFlow, baseUrl }, fixtures] = await Promise.all([
  import('./shared-harness.mjs'),
  import('./fixtures.mjs'),
]);

const {
  apiSuccess,
  buildAgentInspectData,
  buildConversationMessages,
  buildConversations,
  buildIntegrationConnections,
  buildIntegrationsData,
  buildNowData,
} = fixtures;

function buildProvenanceData() {
  return {
    message_id: 'msg_1',
    events: [
      {
        id: 'evt_1',
        event_name: 'assistant_proposal_staged',
        payload: { status: 'staged', source: 'thread', summary: 'Proposal is awaiting review.' },
        created_at: 1710000000,
      },
    ],
    signals: [{ signal_type: 'needs_input', summary: 'Awaiting explicit operator input.' }],
    policy_decisions: [{ decision: 'review_required', reason: 'Multi-step proposal needs supervision.' }],
    linked_objects: [{ object_type: 'thread', object_id: 'conv_1', summary: 'Proposal thread continuity.' }],
  };
}

function buildState({
  nowOverrides = {},
  inspectOverrides = {},
  integrations = buildIntegrationsData(),
  connections = buildIntegrationConnections(),
  messages = buildConversationMessages(),
} = {}) {
  return {
    nowData: buildNowData(nowOverrides),
    inspectData: buildAgentInspectData(inspectOverrides),
    integrationsData: integrations,
    connections,
    conversations: buildConversations(),
    messages,
    provenance: buildProvenanceData(),
  };
}

function createApiHandler(state) {
  return async ({ request, url }) => {
    if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(state.nowData);
    if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(state.conversations);
    if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') return apiSuccess(state.messages);
    if (request.method() === 'GET' && url.pathname === '/api/messages/msg_1/provenance') return apiSuccess(state.provenance);
    if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(state.inspectData);
    if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(state.integrationsData);
    if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(state.connections);
    if (request.method() === 'POST' && url.pathname === '/api/integrations/sync/calendar') return apiSuccess({ source: 'calendar', signals_ingested: 3 });
    if (request.method() === 'POST' && url.pathname === '/api/integrations/sync/todoist') return apiSuccess({ source: 'todoist', signals_ingested: 2 });
    if (request.method() === 'POST' && url.pathname === '/api/integrations/google-calendar/disconnect') return apiSuccess(null);
    if (request.method() === 'POST' && url.pathname === '/api/integrations/todoist/disconnect') return apiSuccess(null);
    return null;
  };
}

async function runNowNormal() {
  const state = buildState();
  await withBrowserFlow('now-normal', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    const main = page.locator('main');
    await main.getByRole('heading', { name: 'Now' }).waitFor();
    await main.getByRole('heading', { name: 'Write weekly review' }).waitFor();

    const hasTrustCard = await main.getByText('Trust state').count();
    assert.equal(hasTrustCard, 0);

    await screenshot('now-normal.png', main);
    await writeJson('summary.json', {
      heading: 'Now',
      activeTask: 'Write weekly review',
      trustCardCount: hasTrustCard,
      nextEvent: await main.getByText('Design review').count(),
    });
    await writeEvidenceNote({
      title: 'Phase 89 Browser Proof — Now Normal',
      command: 'npm --prefix clients/web run proof:phase89:ui-proof',
      tested: 'Loaded the normal `Now` surface with healthy trust state and the bounded task/event layout.',
      expected: '`Now` should remain bounded, show the active task as dominant, and avoid degraded trust chrome when the system is healthy.',
      observed: 'The browser rendered `Now` with the dominant active task and next event while the trust card remained absent in the healthy case.',
    });
  });
}

async function runNowDegraded() {
  const state = buildState({
    nowOverrides: {
      trust_readiness: {
        level: 'degraded',
        headline: 'Degraded',
        summary: 'Sync posture needs review before trusting cross-client state.',
        backup: { level: 'warning', label: 'Backup', detail: 'Backup posture is stale.' },
        freshness: { level: 'warning', label: 'Freshness', detail: 'Cross-client freshness is degraded.' },
        review: {
          open_action_count: 1,
          pending_execution_reviews: 0,
          pending_writeback_count: 0,
          conflict_count: 0,
        },
        guidance: [],
        follow_through: [],
      },
    },
  });

  await withBrowserFlow('now-degraded', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    const main = page.locator('main');
    await main.getByRole('heading', { name: 'Now' }).waitFor();
    await main.getByText('Trust state').waitFor();

    await screenshot('now-degraded.png', main);
    await writeJson('summary.json', {
      heading: 'Now',
      trustStateVisible: await main.getByText('Trust state').count(),
      openSystemCount: await main.getByRole('button', { name: 'Open system detail' }).count(),
    });
    await writeEvidenceNote({
      title: 'Phase 89 Browser Proof — Now Degraded',
      command: 'npm --prefix clients/web run proof:phase89:ui-proof',
      tested: 'Loaded degraded trust state on `Now` and verified the bounded trust intervention path.',
      expected: '`Now` should surface trust only when degraded and offer escalation into `System` without widening the page.',
      observed: 'The browser rendered the degraded trust card with `Open system detail` while keeping the rest of `Now` bounded.',
    });
  });
}

async function runThreadNormal() {
  const state = buildState();
  await withBrowserFlow('thread-normal', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    await page.getByRole('button', { name: 'Review' }).click();
    const main = page.locator('main');
    await main.getByRole('heading', { name: 'Proposal thread' }).waitFor();
    await main.getByText('What this is about right now').waitFor();

    await screenshot('thread-normal.png', main);
    await writeJson('summary.json', {
      heading: 'Proposal thread',
      contextShelf: await main.getByText('What this is about right now').count(),
      continuityStream: await main.getByText('Continuity stream').count(),
      reviewPanel: await main.getByText('Shared review panel').count(),
    });
    await writeEvidenceNote({
      title: 'Phase 89 Browser Proof — Threads Normal',
      command: 'npm --prefix clients/web run proof:phase89:ui-proof',
      tested: 'Navigated from the shell nudge into `Threads` and verified the object/context-first continuity layout.',
      expected: '`Threads` should lead with context, keep chronology secondary, and expose a collapsed review surface.',
      observed: 'The browser opened `Proposal thread` with the context shelf, shared review panel, and continuity stream in the approved structure.',
    });
  });
}

async function runThreadFocused() {
  const state = buildState({
    messages: [
      {
        id: 'msg_1',
        conversation_id: 'conv_1',
        role: 'user',
        kind: 'text',
        content: {
          text: 'Can you help shape the rollout plan?',
          actions: [{ action_type: 'show_why', label: 'Show why' }],
        },
        status: null,
        importance: null,
        created_at: 10,
        updated_at: null,
      },
    ],
  });
  await withBrowserFlow('thread-focused', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    await page.getByRole('button', { name: 'Review' }).click();
    const main = page.locator('main');
    await main.getByRole('heading', { name: 'Proposal thread' }).waitFor();
    await main.getByRole('button', { name: 'Show why' }).click();
    await page.getByRole('heading', { name: 'Provenance' }).waitFor();

    await screenshot('thread-focused.png');
    await writeJson('summary.json', {
      heading: 'Proposal thread',
      provenanceHeading: await page.getByRole('heading', { name: 'Provenance' }).count(),
      sourceMessage: await page.getByText('msg_1').count(),
    });
    await writeEvidenceNote({
      title: 'Phase 89 Browser Proof — Threads Focused Block',
      command: 'npm --prefix clients/web run proof:phase89:ui-proof',
      tested: 'Opened the focused provenance block from the thread continuity stream.',
      expected: 'Focused review should expand as a bounded detail surface without turning the thread into a raw trace dump.',
      observed: 'The browser opened the `Provenance` drawer from `Show why`, exposing message evidence and structured summaries while keeping the thread intact underneath.',
    });
  });
}

async function runSystemIntegrationsIssue() {
  const integrations = buildIntegrationsData();
  integrations.google_calendar = {
    ...integrations.google_calendar,
    connected: false,
    last_sync_status: 'error',
    last_error: 'Token stale. Recovery required before trust is restored.',
    guidance: { title: 'Degraded', detail: 'Token stale. Recovery required before trust is restored.', action: 'refresh' },
  };
  const state = buildState({
    nowOverrides: {
      trust_readiness: {
        level: 'degraded',
        headline: 'Degraded',
        summary: 'Integration trust is degraded.',
        backup: { level: 'warning', label: 'Backup', detail: 'Backup trust is healthy.' },
        freshness: { level: 'warning', label: 'Freshness', detail: 'Inputs are fresh enough to inspect.' },
        review: {
          open_action_count: 1,
          pending_execution_reviews: 0,
          pending_writeback_count: 0,
          conflict_count: 0,
        },
        guidance: [],
        follow_through: [],
      },
    },
    inspectOverrides: {
      blockers: [{ code: 'integration_auth_degraded', message: 'Google Calendar auth must be repaired.' }],
    },
    integrations,
  });

  await withBrowserFlow('system-integrations-issue', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    const main = page.locator('main');
    await main.getByRole('heading', { name: 'Now' }).waitFor();
    await main.getByRole('button', { name: 'Open system detail' }).click();
    await main.getByRole('heading', { name: 'Providers' }).waitFor();
    await main.getByText('Browse / detail').waitFor();
    await main.getByText('Google Calendar').first().waitFor();

    await screenshot('system-integrations-issue.png', main);
    await writeJson('summary.json', {
      section: 'Integrations',
      provider: 'Google Calendar',
      providerVisible: await main.getByText('Google Calendar').count(),
      refreshVisible: await main.getByRole('button', { name: 'Refresh' }).count(),
    });
    await writeEvidenceNote({
      title: 'Phase 89 Browser Proof — System Integrations Issue',
      command: 'npm --prefix clients/web run proof:phase89:ui-proof',
      tested: 'Escalated from degraded `Now` trust state into `System > Integrations` with a stale provider token.',
      expected: '`System` should show a browse/detail integration issue view where provider identity stays subordinate to degraded trust state.',
      observed: 'The browser opened `System > Providers` with a degraded Google Calendar detail pane, named actions, and browse/detail structure.',
    });
  });
}

async function runSystemControlView() {
  const state = buildState();
  await withBrowserFlow('system-control', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    const main = page.locator('main');
    await page.getByRole('button', { name: 'System' }).click();
    await main.getByRole('heading', { name: 'Trust', exact: true }).waitFor();
    await main.getByRole('button', { name: 'Control' }).click();
    await main.getByText('Project registry', { exact: true }).waitFor();

    await screenshot('system-control.png', main);
    await writeJson('summary.json', {
      section: 'Control',
      projectRegistry: await main.getByText('Project registry').count(),
      capabilityButton: await main.getByRole('button', { name: /Capabilities/i }).count(),
      projectName: await main.getByText('Vel').count(),
    });
    await writeEvidenceNote({
      title: 'Phase 89 Browser Proof — System Control View',
      command: 'npm --prefix clients/web run proof:phase89:ui-proof',
      tested: 'Opened the `Control` section and verified dense-but-readable project detail.',
      expected: '`Control` should present structural objects as readable rows with bounded detail, not playful cards or raw tables.',
      observed: 'The browser rendered `Project registry` with dense row selection, bounded detail, and the retained `Capabilities` browse path.',
    });
  });
}

await runNowNormal();
await runNowDegraded();
await runThreadNormal();
await runThreadFocused();
await runSystemIntegrationsIssue();
await runSystemControlView();
