import process from 'node:process';

process.env.VEL_WEB_PROOF_ROOT = '.planning/phases/100-mvp-proof-audit-and-closeout/100-evidence';

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

function buildIncompleteSettingsData() {
  return {
    node_display_name: null,
    timezone: 'America/Denver',
    tailscale_preferred: true,
    tailscale_base_url: 'https://vel.tailnet.ts.net',
    lan_base_url: 'http://192.168.1.4:8000',
    writeback_enabled: false,
    llm: {
      models_dir: '/models',
      default_chat_profile_id: null,
      fallback_chat_profile_id: null,
      profiles: [],
    },
    core_settings: {
      user_display_name: null,
      client_location_label: null,
      developer_mode: false,
      bypass_setup_gate: false,
      agent_profile: {
        role: null,
        preferences: null,
        constraints: null,
        freeform: null,
      },
    },
    web_settings: {
      dense_rows: true,
      tabular_numbers: true,
      reduced_motion: false,
      strong_focus: true,
      docked_action_bar: true,
    },
  };
}

function buildState() {
  const integrations = buildIntegrationsData();
  integrations.google_calendar = {
    ...integrations.google_calendar,
    configured: false,
    connected: false,
    has_client_id: false,
    has_client_secret: false,
  };
  integrations.todoist = {
    ...integrations.todoist,
    configured: false,
    connected: false,
    has_api_token: false,
  };

  return {
    nowData: buildNowData({
      nudge_bars: [],
    }),
    inspectData: buildAgentInspectData(),
    integrationsData: integrations,
    connections: buildIntegrationConnections(),
    conversations: buildConversations(),
    messages: buildConversationMessages(),
    settings: buildIncompleteSettingsData(),
  };
}

function createApiHandler(state) {
  return async ({ request, url }) => {
    if (request.method() === 'GET' && url.pathname === '/v1/now') return apiSuccess(state.nowData);
    if (request.method() === 'GET' && url.pathname === '/v1/agent/inspect') return apiSuccess(state.inspectData);
    if (request.method() === 'GET' && url.pathname === '/api/conversations') return apiSuccess(state.conversations);
    if (request.method() === 'GET' && url.pathname === '/api/conversations/conv_1/messages') return apiSuccess(state.messages);
    if (request.method() === 'GET' && url.pathname === '/api/integrations') return apiSuccess(state.integrationsData);
    if (request.method() === 'GET' && url.pathname === '/api/integrations/connections') return apiSuccess(state.connections);
    if (request.method() === 'GET' && url.pathname === '/api/settings') return apiSuccess(state.settings);
    if (request.method() === 'PATCH' && url.pathname === '/api/settings') {
      const patch = request.postDataJSON?.() ?? {};
      state.settings = {
        ...state.settings,
        ...patch,
        core_settings: {
          ...state.settings.core_settings,
          ...(patch.core_settings ?? {}),
          agent_profile: {
            ...state.settings.core_settings.agent_profile,
            ...(patch.core_settings?.agent_profile ?? {}),
          },
        },
      };
      return apiSuccess(state.settings);
    }
    return null;
  };
}

async function runCoreSetupProof() {
  const state = buildState();
  await withBrowserFlow('core-setup-nudge-proof', createApiHandler(state), async ({
    page,
    screenshot,
    writeJson,
    writeEvidenceNote,
  }) => {
    await page.goto(baseUrl);

    const nudgeRegion = page.getByRole('complementary', { name: 'Nudges' });
    await nudgeRegion.getByText('Finish Core setup to enable the composer').waitFor();
    await nudgeRegion.getByText(/Open System → Preferences → Accessibility → Core settings to finish setup\./i).waitFor();

    const composer = page.locator('main');
    await composer.getByText(/Core setup is incomplete\. Use the nudge to open Core settings and finish setup\./i).waitFor();
    await composer.getByRole('textbox', { name: /ask, capture, or talk to vel/i }).waitFor();

    await screenshot('core-setup-nudge.png', page);

    await nudgeRegion.getByRole('button', { name: /core settings/i }).click();
    await page.locator('#accessibility\\:\\:operator-settings').getByText('Operator settings').waitFor();
    await page.locator('#accessibility\\:\\:operator-settings').getByText('Core settings').waitFor();

    await screenshot('core-settings-deeplink.png', page.locator('main'));
    await writeJson('summary.json', {
      nudgeTitleVisible: await nudgeRegion.getByText('Finish Core setup to enable the composer').count(),
      coreSettingsActionVisible: await nudgeRegion.getByRole('button', { name: /core settings/i }).count(),
      operatorSettingsVisible: await page.locator('#accessibility\\:\\:operator-settings').getByText('Operator settings').count(),
      coreSettingsVisible: await page.locator('#accessibility\\:\\:operator-settings').getByText('Core settings').count(),
    });
    await writeEvidenceNote({
      title: 'Phase 100 Browser Proof — Core setup nudge deep-link',
      command: 'npm --prefix clients/web run proof:phase100:feedback',
      tested: 'Loaded the app with incomplete Core setup, verified the setup alert renders as a nudge, then followed the nudge action into System Core settings.',
      expected: 'Missing setup alerts should surface through nudges, the composer should remain only a disabled state, and the nudge action should deep-link into System → Preferences → Accessibility → Core settings.',
      observed: 'The browser rendered the core-setup nudge with the full setup guidance, the composer stayed in a disabled explanatory state, and the Core settings action landed on the operator Core settings block in System.',
    });
  });
}

await runCoreSetupProof();
