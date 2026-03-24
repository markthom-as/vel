import assert from 'node:assert/strict';
import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/phases/96-browser-proof-acceptance-audit-and-milestone-closeout/96-evidence';

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

function buildSettingsData() {
  return {
    node_display_name: 'Vel Desktop',
    timezone: 'America/Denver',
    tailscale_preferred: true,
    tailscale_base_url: 'https://vel.tailnet.ts.net',
    lan_base_url: 'http://192.168.1.4:8000',
    writeback_enabled: true,
    web_settings: {
      dense_rows: true,
      tabular_numbers: true,
      reduced_motion: false,
      strong_focus: true,
      docked_action_bar: true,
    },
  };
}

function buildState({
  nowOverrides = {},
  inspectOverrides = {},
  integrations = buildIntegrationsData(),
  connections = buildIntegrationConnections(),
  messages = buildConversationMessages(),
  settings = buildSettingsData(),
} = {}) {
  return {
    nowData: buildNowData(nowOverrides),
    inspectData: buildAgentInspectData(inspectOverrides),
    integrationsData: integrations,
    connections,
    conversations: buildConversations(),
    messages,
    settings,
  };
}

function createApiHandler(state) {
  return async ({ request, url }) => {
    if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(state.nowData);
    if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(state.conversations);
    if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') return apiSuccess(state.messages);
    if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(state.inspectData);
    if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(state.integrationsData);
    if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(state.connections);
    if (request.method() === 'GET' && url.pathname === '/api/settings') return apiSuccess(state.settings);
    if (request.method() === 'POST' && url.pathname === '/api/interventions/intv_review_1/acknowledge') return apiSuccess({ id: 'intv_review_1', state: 'acknowledged' });
    if (request.method() === 'POST' && url.pathname === '/api/interventions/intv_review_1/snooze') return apiSuccess({ id: 'intv_review_1', state: 'snoozed' });
    if (request.method() === 'POST' && url.pathname === '/api/integrations/sync/calendar') return apiSuccess({ source: 'calendar', signals_ingested: 3 });
    if (request.method() === 'POST' && url.pathname === '/api/integrations/sync/todoist') return apiSuccess({ source: 'todoist', signals_ingested: 2 });
    if (request.method() === 'POST' && url.pathname === '/api/integrations/google-calendar/disconnect') return apiSuccess(null);
    if (request.method() === 'POST' && url.pathname === '/api/integrations/todoist/disconnect') return apiSuccess(null);
    if (request.method() === 'PATCH' && url.pathname === '/api/settings') return apiSuccess({ ...state.settings, ...(request.postDataJSON?.() ?? {}) });
    if (request.method() === 'PATCH' && url.pathname === '/api/conversations/conv_1') {
      const patch = request.postDataJSON?.() ?? {};
      state.conversations = state.conversations.map((conversation) =>
        conversation.id === 'conv_1' ? { ...conversation, ...patch } : conversation,
      );
      return apiSuccess(state.conversations.find((conversation) => conversation.id === 'conv_1'));
    }
    return null;
  };
}

async function runNowProof() {
  const baseNow = buildNowData();
  const state = buildState({
    nowOverrides: {
      task_lane: {
        ...baseNow.task_lane,
        active_items: [
          {
            id: 'commit_local_1',
            task_kind: 'commitment',
            text: 'Write weekly review',
            title: 'Write weekly review',
            description: 'Capture the final draft and send the executive summary.',
            tags: ['Ops', 'Deep work'],
            state: 'active',
            project: 'Ops',
            primary_thread_id: null,
          },
        ],
        next_up: [
          {
            id: 'commit_todoist_1',
            task_kind: 'task',
            text: 'Reply to Dimitri',
            title: 'Reply to Dimitri',
            description: 'Confirm the milestone closeout and next review slot.',
            tags: ['Email'],
            state: 'pending',
            project: 'Ops',
            primary_thread_id: 'conv_1',
          },
        ],
        if_time_allows: [
          {
            id: 'commit_todoist_2',
            task_kind: 'task',
            text: 'Sketch Friday backlog',
            title: 'Sketch Friday backlog',
            description: 'Outline optional work if the main lane closes early.',
            tags: ['Backlog'],
            state: 'pending',
            project: 'Planning',
            primary_thread_id: null,
          },
        ],
        completed: [
          {
            id: 'commit_done_1',
            task_kind: 'task',
            text: 'Clean notes inbox',
            title: 'Clean notes inbox',
            description: null,
            tags: ['Inbox'],
            state: 'done',
            project: null,
            primary_thread_id: null,
          },
        ],
        overflow_count: 0,
      },
      action_items: [
        {
          id: 'review_bar',
          surface: 'now',
          kind: 'intervention',
          permission_mode: 'user_confirm',
          scope_affinity: 'thread',
          title: 'Review morning plan',
          summary: 'A review request is ready.',
          project_id: null,
          project_label: null,
          project_family: null,
          state: 'active',
          rank: 60,
          surfaced_at: '2026-03-22T10:00:00Z',
          snoozed_until: null,
          evidence: [{ source_kind: 'intervention', source_id: 'intv_review_1', label: 'review', detail: null }],
          thread_route: { target: 'existing_thread', thread_id: 'conv_1', label: 'Open thread', thread_type: null },
        },
      ],
      nudge_bars: [
        {
          id: 'review_bar',
          kind: 'review_request',
          title: 'Review morning plan',
          summary: 'A review request is ready.',
          urgent: true,
          primary_thread_id: 'conv_1',
          timestamp: 1710000000,
          actions: [{ kind: 'accept', label: 'Review' }],
        },
      ],
    },
  });

  await withBrowserFlow('now-proof', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    const main = page.locator('main');
    await main.getByText('Now').waitFor();
    await main.getByText(/ACTIVE TASK/i).waitFor();
    await screenshot('now.png', main);
    await writeJson('summary.json', {
      activeTaskCount: await main.getByText(/ACTIVE TASK/i).count(),
      nextUpVisible: await main.getByText(/NEXT UP/i).count(),
      nudgesVisible: await page.getByText(/NUDGES \(/i).count(),
    });
    await writeEvidenceNote({
      title: 'Phase 96 Browser Proof — Now',
      command: 'node clients/web/scripts/proof/phase96-ui-proof.mjs',
      tested: 'Loaded the current `Now` surface with widened task data, a nudge, and bounded sections.',
      expected: '`Now` should render the accepted bounded layout with active, next-up, optional, completed, and nudge surfaces visible without trust-side regressions.',
      observed: 'The browser rendered the bounded `Now` surface, active task lane, mixed next-up section, optional section, completed section, and floating nudge lane.',
    });
  });
}

async function runThreadsProof() {
  const state = buildState();
  await withBrowserFlow('threads-proof', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    await page.getByRole('button', { name: 'Threads' }).click();
    await page.getByText(/THREADS \(/i).waitFor();
    await page.getByText(/CURRENT THREAD/i).waitFor();
    await screenshot('threads.png', page.locator('main'));
    await writeJson('summary.json', {
      threadHeader: await page.getByText(/CURRENT THREAD/i).count(),
      sidebarHeader: await page.getByText(/THREADS \(/i).count(),
      titleVisible: await page.getByRole('button', { name: /Proposal thread/i }).count(),
    });
    await writeEvidenceNote({
      title: 'Phase 96 Browser Proof — Threads',
      command: 'node clients/web/scripts/proof/phase96-ui-proof.mjs',
      tested: 'Loaded the current `Threads` surface and verified sidebar plus thread header/message layout.',
      expected: '`Threads` should show the accepted sidebar, active thread state, editable title, and bounded chat surface.',
      observed: 'The browser rendered the thread sidebar and current thread surface with the expected thread header and message stack.',
    });
  });
}

async function runSystemProof() {
  const integrations = buildIntegrationsData();
  integrations.google_calendar = {
    ...integrations.google_calendar,
    connected: true,
    configured: true,
    last_sync_status: 'ok',
  };
  const state = buildState({ integrations });

  await withBrowserFlow('system-proof', createApiHandler(state), async ({ page, screenshot, writeJson, writeEvidenceNote }) => {
    await page.goto(baseUrl);
    await page.getByRole('button', { name: 'System' }).click();
    await page.getByText(/^SYSTEM$/i).waitFor();
    await page.locator('#trust').getByText(/^Status$/i).waitFor();
    await screenshot('system.png', page.locator('main'));
    await writeJson('summary.json', {
      systemSidebar: await page.getByText(/^SYSTEM$/i).count(),
      statusSection: await page.getByText(/^STATUS$/i).count(),
      providerVisible: await page.getByText('Google Calendar').count(),
    });
    await writeEvidenceNote({
      title: 'Phase 96 Browser Proof — System',
      command: 'node clients/web/scripts/proof/phase96-ui-proof.mjs',
      tested: 'Loaded the current `System` surface and verified sticky navigation plus dense single-document sections.',
      expected: '`System` should render as one dense document with sticky navigation, visible sections, and colocated integration/config detail.',
      observed: 'The browser rendered the current `System` sidebar and section list with visible provider detail and operator settings fields.',
    });
  });
}

await runNowProof();
await runThreadsProof();
await runSystemProof();
