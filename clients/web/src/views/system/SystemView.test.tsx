import { fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { clearQueryCache } from '../../data/query'
import { SystemView } from './SystemView'

const loadAgentInspect = vi.fn()
const disconnectGoogleCalendar = vi.fn()
const disconnectTodoist = vi.fn()
const loadIntegrationConnections = vi.fn()
const loadIntegrations = vi.fn()
const loadLlmProfileHealth = vi.fn()
const loadSettings = vi.fn()
const runLlmProfileHandshake = vi.fn()
const startGoogleCalendarAuth = vi.fn()
const syncSource = vi.fn()
const updateGoogleCalendarIntegration = vi.fn()
const updateSettings = vi.fn()
const updateTodoistIntegration = vi.fn()
const updateWebSettings = vi.fn()

function buildSettings(overrides: Record<string, unknown> = {}) {
  return {
    writeback_enabled: false,
    node_display_name: 'Vel Desktop',
    timezone: 'America/Denver',
    llm: {
      models_dir: 'configs/models',
      default_chat_profile_id: 'local-llama',
      fallback_chat_profile_id: 'oauth-openai',
      profiles: [
        {
          id: 'local-llama',
          provider: 'llama_cpp',
          base_url: 'http://127.0.0.1:8012/v1',
          model: 'qwen3',
          context_window: 32768,
          enabled: true,
          editable: false,
        },
        {
          id: 'oauth-openai',
          provider: 'openai_oauth',
          base_url: 'http://127.0.0.1:8014/v1',
          model: 'gpt-5.4',
          context_window: 32768,
          enabled: true,
          editable: true,
        },
      ],
    },
    core_settings: {
      user_display_name: 'Jove',
      client_location_label: 'Denver, CO',
      developer_mode: false,
      bypass_setup_gate: false,
      agent_profile: {
        role: 'Solo operator',
        preferences: 'Local-first, concise output',
        constraints: 'No broad automation by default',
        freeform: 'Prefers explainable actions.',
      },
    },
    backup: null,
    web_settings: {
      dense_rows: true,
      tabular_numbers: true,
      reduced_motion: false,
      strong_focus: true,
      docked_action_bar: true,
    },
    ...overrides,
  }
}

vi.mock('../../data/agent-grounding', () => ({
  loadAgentInspect: (...args: unknown[]) => loadAgentInspect(...args),
}))

vi.mock('../../data/operator', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../../data/operator')>()
  return {
    ...actual,
    disconnectGoogleCalendar: (...args: unknown[]) => disconnectGoogleCalendar(...args),
    disconnectTodoist: (...args: unknown[]) => disconnectTodoist(...args),
    loadIntegrationConnections: (...args: unknown[]) => loadIntegrationConnections(...args),
    loadIntegrations: (...args: unknown[]) => loadIntegrations(...args),
    loadLlmProfileHealth: (...args: unknown[]) => loadLlmProfileHealth(...args),
    loadSettings: (...args: unknown[]) => loadSettings(...args),
    runLlmProfileHandshake: (...args: unknown[]) => runLlmProfileHandshake(...args),
    operatorQueryKeys: {
      agentInspect: () => ['agent', 'inspect'],
      integrations: () => ['integrations'],
      integrationConnections: () => ['integrations', 'connections', 'all', 'all'],
      settings: () => ['settings'],
    },
    startGoogleCalendarAuth: (...args: unknown[]) => startGoogleCalendarAuth(...args),
    syncSource: (...args: unknown[]) => syncSource(...args),
    updateGoogleCalendarIntegration: (...args: unknown[]) => updateGoogleCalendarIntegration(...args),
    updateSettings: (...args: unknown[]) => updateSettings(...args),
    updateTodoistIntegration: (...args: unknown[]) => updateTodoistIntegration(...args),
    updateWebSettings: (...args: unknown[]) => updateWebSettings(...args),
  }
})

describe('SystemView', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.unstubAllGlobals()
    loadAgentInspect.mockReset()
    disconnectGoogleCalendar.mockReset()
    disconnectTodoist.mockReset()
    loadIntegrationConnections.mockReset()
    loadIntegrations.mockReset()
    loadLlmProfileHealth.mockReset()
    loadSettings.mockReset()
    runLlmProfileHandshake.mockReset()
    startGoogleCalendarAuth.mockReset()
    syncSource.mockReset()
    updateGoogleCalendarIntegration.mockReset()
    updateSettings.mockReset()
    updateTodoistIntegration.mockReset()
    updateWebSettings.mockReset()

    loadAgentInspect.mockResolvedValue({
      ok: true,
      data: {
        grounding: {
          generated_at: 1710000000,
          now: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            schedule: {
              upcoming_events: [
                { id: 'event_1', title: 'Design review', kind: 'event', start_ts: 1710003600, end_ts: 1710007200 },
              ],
            },
          },
          current_context: {
            computed_at: 1710000000,
            mode: 'work',
            morning_state: 'ready',
            current_context_path: 'var/context/current.json',
            explain_context_path: 'var/context/explain.json',
            explain_drift_path: 'var/context/drift.json',
          },
          projects: [{ id: 'project_1', slug: 'vel', name: 'Vel', family: 'work', status: 'active', primary_repo: { path: '/repo', label: 'repo', kind: 'repo' }, primary_notes_root: { path: '/notes', label: 'notes', kind: 'notes_root' }, secondary_repos: [], secondary_notes_roots: [], upstream_ids: {}, pending_provision: { create_repo: false, create_notes_root: false }, created_at: '2026-03-22T00:00:00Z', updated_at: '2026-03-22T00:00:00Z', archived_at: null }],
          people: [{ id: 'person_1', display_name: 'Avery', given_name: null, family_name: null, relationship_context: 'teammate', birthday: null, last_contacted_at: null, aliases: [], links: [] }],
          commitments: [],
          review: { review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 }, pending_writebacks: [], conflicts: [], pending_execution_handoffs: [] },
        },
        capabilities: {
          groups: [
            {
              kind: 'read_context',
              label: 'Read context',
              entries: [
                {
                  key: 'object.read',
                  label: 'Read objects',
                  summary: 'Read canonical objects safely.',
                  available: true,
                  blocked_reason: null,
                  requires_review_gate: null,
                  requires_writeback_enabled: false,
                },
              ],
            },
          ],
        },
        blockers: [],
        explainability: {
          persisted_record_kinds: ['task', 'event'],
          supporting_paths: ['docs/MASTER_PLAN.md'],
          raw_context_json_supporting_only: true,
        },
      },
      meta: { request_id: 'req_inspect' },
    })
    loadIntegrations.mockResolvedValue({
      ok: true,
      data: {
        google_calendar: {
          configured: true,
          connected: true,
          has_client_id: true,
          has_client_secret: true,
          calendars: [{ id: 'cal_1', summary: 'Primary', primary: true, selected: true }],
          all_calendars_selected: false,
          last_sync_at: 1710000000,
          last_sync_status: 'ok',
          last_error: null,
          last_item_count: 12,
          guidance: { title: 'Healthy', detail: 'Calendar connection is healthy.', action: 'refresh' },
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: 1710000000,
          last_sync_status: 'ok',
          last_error: null,
          last_item_count: 8,
          guidance: { title: 'Healthy', detail: 'Todoist connection is healthy.', action: 'refresh' },
          write_capabilities: {
            completion_status: true,
            due_date: true,
            tags: false,
          },
        },
        activity: buildLocalIntegration(),
        health: buildLocalIntegration(),
        git: buildLocalIntegration(),
        messaging: buildLocalIntegration(),
        reminders: buildLocalIntegration(),
        notes: buildLocalIntegration(),
        transcripts: buildLocalIntegration(),
      },
      meta: { request_id: 'req_integrations' },
    })
    loadIntegrationConnections.mockResolvedValue({
      ok: true,
      data: [
        {
          id: 'conn_google',
          family: 'calendar',
          provider_key: 'google_calendar',
          status: 'connected',
          display_name: 'Google Calendar',
          account_ref: 'acct_google',
          metadata: {},
          created_at: 1710000000,
          updated_at: 1710000000,
          setting_refs: [],
        },
      ],
      meta: { request_id: 'req_connections' },
    })
    loadSettings.mockResolvedValue({
      ok: true,
      data: buildSettings(),
      meta: { request_id: 'req_settings' },
    })
    loadLlmProfileHealth.mockResolvedValue({
      ok: true,
      data: {
        profile_id: 'oauth-openai',
        healthy: true,
        message: 'Provider handshake succeeded.',
      },
      meta: { request_id: 'req_llm_health' },
    })
    runLlmProfileHandshake.mockResolvedValue({
      ok: true,
      data: {
        profile_id: 'oauth-openai',
        healthy: true,
        message: 'Provider handshake succeeded.',
      },
      meta: { request_id: 'req_llm_handshake' },
    })
    syncSource.mockResolvedValue({ ok: true, data: { source: 'calendar', signals_ingested: 3 }, meta: { request_id: 'req_sync' } })
    disconnectGoogleCalendar.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_disconnect' } })
    disconnectTodoist.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_disconnect_todoist' } })
    startGoogleCalendarAuth.mockResolvedValue({ ok: true, data: { auth_url: 'https://accounts.google.com/o/oauth2/v2/auth?state=test' }, meta: { request_id: 'req_google_auth' } })
    updateGoogleCalendarIntegration.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_google_patch' } })
    updateSettings.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_settings_patch' } })
    updateTodoistIntegration.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_todoist_patch' } })
    updateWebSettings.mockResolvedValue({ ok: true, data: null, meta: { request_id: 'req_web_settings' } })
  })

  it('renders the compact system rail and scroll document without the rejected helper panels', async () => {
    render(<SystemView target={{ section: 'overview', subsection: 'trust' }} />)

    await waitFor(() => {
      expect(screen.getByText('Status and activity')).toBeInTheDocument()
    })

    expect(screen.getByRole('button', { name: /Overview/i })).toHaveAttribute('aria-pressed', 'true')
    expect(screen.getByRole('button', { name: /Operations/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Integrations' })).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /Control/i })).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: /Preferences/i })).toBeInTheDocument()
    expect(screen.getByText('Status and activity')).toBeInTheDocument()
    expect(screen.queryByText('One surface, five sections. Trust stays legible. Operations stay bounded.')).not.toBeInTheDocument()
    expect(screen.queryByRole('heading', { name: /Structural truth, trust, and repair/i })).not.toBeInTheDocument()

    expect(screen.getByText('Avery')).toBeInTheDocument()
    expect(screen.queryByText('Design review')).not.toBeInTheDocument()
    expect(screen.queryByText(/No grounded upcoming events are available right now\./i)).not.toBeInTheDocument()
  })

  it('hides writeback-specific blockers outside developer mode and keeps recovery scoped to real issues', async () => {
    loadAgentInspect.mockResolvedValueOnce({
      ok: true,
      data: {
        grounding: {
          generated_at: 1710000000,
          now: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            schedule: { upcoming_events: [] },
          },
          current_context: {
            computed_at: 1710000000,
            mode: 'work',
            morning_state: 'ready',
            current_context_path: 'var/context/current.json',
            explain_context_path: 'var/context/explain.json',
            explain_drift_path: 'var/context/drift.json',
          },
          projects: [],
          people: [],
          commitments: [],
          review: { review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 }, pending_writebacks: [], conflicts: [], pending_execution_handoffs: [] },
        },
        capabilities: { groups: [] },
        blockers: [
          {
            code: 'writeback_disabled',
            message: 'Writeback-dependent mutation requests are unavailable while SAFE MODE is enabled.',
            escalation_hint: 'Enable writeback or stay within read/review lanes.',
          },
          {
            code: 'no_matching_write_grant',
            message: 'No approved repo-local write grant is currently available.',
            escalation_hint: 'Open developer controls to review write grants.',
          },
        ],
        explainability: {
          persisted_record_kinds: ['task', 'event'],
          supporting_paths: ['docs/MASTER_PLAN.md'],
          raw_context_json_supporting_only: true,
        },
      },
      meta: { request_id: 'req_inspect_blockers' },
    })

    render(<SystemView target={{ section: 'overview', subsection: 'trust' }} />)

    await waitFor(() => {
      expect(screen.getByText('Status and activity')).toBeInTheDocument()
    })

    expect(screen.queryByText('WRITEBACK_DISABLED')).not.toBeInTheDocument()
    expect(screen.queryByText('NO_MATCHING_WRITE_GRANT')).not.toBeInTheDocument()

    fireEvent.click(screen.getAllByRole('button', { name: /Operations/i })[0])
    fireEvent.click(await screen.findByRole('button', { name: 'Backup & Recovery' }))
    expect((await screen.findAllByText('No blocker records are active.')).length).toBeGreaterThan(0)
    expect(screen.queryByText('Writeback-dependent mutation requests are unavailable while SAFE MODE is enabled.')).not.toBeInTheDocument()
    expect(screen.queryByText('No approved repo-local write grant is currently available.')).not.toBeInTheDocument()
  })

  it('reveals the control section when developer mode is enabled', async () => {
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        writeback_enabled: false,
        node_display_name: 'Vel Desktop',
        timezone: 'America/Denver',
        llm: {
          models_dir: 'configs/models',
          default_chat_profile_id: 'local-llama',
          fallback_chat_profile_id: 'oauth-openai',
          profiles: [
            {
              id: 'local-llama',
              provider: 'llama_cpp',
              base_url: 'http://127.0.0.1:8012/v1',
              model: 'qwen3',
              context_window: 32768,
              enabled: true,
              editable: false,
            },
          ],
        },
        core_settings: {
          user_display_name: 'Jove',
          client_location_label: 'Denver, CO',
          developer_mode: true,
          bypass_setup_gate: false,
          agent_profile: {
            role: 'Solo operator',
            preferences: 'Local-first, concise output',
            constraints: 'No broad automation by default',
            freeform: 'Prefers explainable actions.',
          },
        },
        backup: null,
        web_settings: {
          dense_rows: true,
          tabular_numbers: true,
          reduced_motion: false,
          strong_focus: true,
          docked_action_bar: true,
        },
      },
      meta: { request_id: 'req_settings_developer' },
    })

    render(<SystemView target={{ section: 'overview', subsection: 'trust' }} />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Control' })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: 'Control' }))
    expect((await screen.findAllByText('Vel')).length).toBeGreaterThan(0)
    expect(screen.getByDisplayValue('/repo')).toBeInTheDocument()

    fireEvent.click(screen.getAllByRole('button', { name: /Capabilities/i })[0])
    expect(await screen.findByText('Read objects')).toBeInTheDocument()
  })

  it('renders system documentation markdown inside the system surface', async () => {
    render(<SystemView target={{ section: 'overview', subsection: 'trust' }} />)

    await screen.findAllByText('System documentation')
    expect(screen.getAllByText('System Surface').length).toBeGreaterThan(0)
    expect(screen.getAllByText(/This page explains the web/i).length).toBeGreaterThan(0)
    expect(screen.getAllByText(/What belongs here/i).length).toBeGreaterThan(0)
  })

  it('keeps integration actions available without the old browse-detail shell', async () => {
    const openSpy = vi.spyOn(window, 'open').mockImplementation(() => null)
    render(<SystemView target={{ section: 'integrations', subsection: 'providers' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Providers').length).toBeGreaterThan(0)
    })

    expect(screen.getAllByText('Google Calendar').length).toBeGreaterThan(0)
    expect(screen.queryByText('Browse / detail')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Google Calendar' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Todoist' })).toBeInTheDocument()
    expect(screen.getAllByText('Unavailable on this system').length).toBeGreaterThan(0)

    const refreshButton = screen.getAllByRole('button', { name: 'Refresh' })[0]
    fireEvent.click(refreshButton)
    await waitFor(() => expect(syncSource).toHaveBeenCalledWith('calendar'))

    const disconnectButton = screen.getAllByRole('button', { name: 'Disconnect' })[0]
    fireEvent.click(disconnectButton)
    await waitFor(() => expect(disconnectGoogleCalendar).toHaveBeenCalled())

    expect(screen.getAllByText('LLM routing').length).toBeGreaterThan(0)
    expect(screen.getAllByText('local-llama').length).toBeGreaterThan(0)
    expect(screen.getAllByText('oauth-openai').length).toBeGreaterThan(0)

    fireEvent.click(screen.getAllByRole('button', { name: 'Set default' })[0])
    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({
        llm: {
          default_chat_profile_id: 'oauth-openai',
          fallback_chat_profile_id: null,
          openai_compat_profiles: [
            {
              id: 'oauth-openai',
              base_url: 'http://127.0.0.1:8014/v1',
              model: 'gpt-5.4',
              context_window: 32768,
              enabled: true,
            },
          ],
          openai_api_profiles: [],
        },
      })
    })

    const oauthProfileCard = document.getElementById('providers-llm-oauth-openai')
    expect(oauthProfileCard).not.toBeNull()
    const oauthProfile = within(oauthProfileCard as HTMLElement)

    const openAiModelField = oauthProfile.getByLabelText('Model')
    fireEvent.change(openAiModelField, { target: { value: 'gpt-5.5' } })
    fireEvent.click(oauthProfile.getByRole('button', { name: 'Save oauth-openai proxy' }))
    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({
        llm: {
          default_chat_profile_id: 'local-llama',
          fallback_chat_profile_id: 'oauth-openai',
          openai_compat_profiles: [
            {
              id: 'oauth-openai',
              base_url: 'http://127.0.0.1:8014/v1',
              model: 'gpt-5.5',
              context_window: 32768,
              enabled: true,
            },
          ],
          openai_api_profiles: [],
        },
      })
    })

    fireEvent.click(oauthProfile.getByRole('button', { name: 'Handshake oauth-openai' }))
    await waitFor(() => {
      expect(runLlmProfileHandshake).toHaveBeenCalledWith({
        profile_id: 'oauth-openai',
        provider: 'openai_oauth',
        base_url: 'http://127.0.0.1:8014/v1',
        model: 'gpt-5.5',
        context_window: 32768,
      })
    })

    const googleClientIdField = screen.getAllByLabelText('Replace Google client ID')[0]
    fireEvent.change(googleClientIdField, { target: { value: 'google-client-id' } })
    fireEvent.blur(googleClientIdField)
    await waitFor(() => {
      expect(updateGoogleCalendarIntegration).toHaveBeenCalledWith({ client_id: 'google-client-id' })
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Reconnect Google' })[0])
    await waitFor(() => {
      expect(startGoogleCalendarAuth).toHaveBeenCalled()
      expect(openSpy).toHaveBeenCalled()
    })

    const todoistTokenField = screen.getAllByLabelText('Replace Todoist API token')[0]
    fireEvent.change(todoistTokenField, { target: { value: 'todoist-token' } })
    fireEvent.blur(todoistTokenField)
    await waitFor(() => {
      expect(updateTodoistIntegration).toHaveBeenCalledWith({ api_token: 'todoist-token' })
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Tags' })[0])
    await waitFor(() => {
      expect(updateTodoistIntegration).toHaveBeenCalledWith({
        write_capabilities: {
          tags: true,
        },
      })
    })

    fireEvent.click(screen.getAllByRole('button', { name: /Accounts/i })[0])
    expect((await screen.findAllByDisplayValue('acct_google')).length).toBeGreaterThan(0)
    expect(screen.getAllByText('Google Calendar').length).toBeGreaterThan(0)
    openSpy.mockRestore()
  })

  it('allows manual OpenAI API credential entry and server-side handshake', async () => {
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        writeback_enabled: false,
        node_display_name: 'Vel Desktop',
        timezone: 'America/Denver',
        llm: {
          models_dir: 'configs/models',
          default_chat_profile_id: 'openai-api',
          fallback_chat_profile_id: null,
          profiles: [
            {
              id: 'openai-api',
              provider: 'openai_api',
              base_url: 'https://api.openai.com/v1',
              model: 'gpt-5.4',
              context_window: 32768,
              enabled: true,
              editable: true,
              has_api_key: false,
            },
          ],
        },
        core_settings: {
          user_display_name: 'Jove',
          client_location_label: 'Denver, CO',
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: 'Solo operator',
            preferences: 'Local-first, concise output',
            constraints: 'No broad automation by default',
            freeform: 'Prefers explainable actions.',
          },
        },
        backup: null,
        web_settings: {
          dense_rows: true,
          tabular_numbers: true,
          reduced_motion: false,
          strong_focus: true,
          docked_action_bar: true,
        },
      },
      meta: { request_id: 'req_settings_openai_api' },
    })
    runLlmProfileHandshake.mockResolvedValueOnce({
      ok: true,
      data: {
        profile_id: 'openai-api',
        healthy: true,
        message: 'Provider handshake succeeded.',
      },
      meta: { request_id: 'req_llm_health_openai_api' },
    })

    render(<SystemView target={{ section: 'integrations', subsection: 'providers' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('LLM routing').length).toBeGreaterThan(0)
    })

    const openAiApiProfileCard = document.getElementById('providers-llm-openai-api')
    expect(openAiApiProfileCard).not.toBeNull()
    const openAiApiProfile = within(openAiApiProfileCard as HTMLElement)

    const apiKeyField = openAiApiProfile.getByLabelText('Replace OpenAI API key')
    fireEvent.change(apiKeyField, { target: { value: 'sk-test' } })
    await waitFor(() => {
      expect((openAiApiProfile.getByLabelText('Replace OpenAI API key') as HTMLInputElement).value).toBe('sk-test')
    })
    fireEvent.click(openAiApiProfile.getByRole('button', { name: 'Handshake openai-api' }))

    await waitFor(() => {
      expect(runLlmProfileHandshake).toHaveBeenCalledWith({
        profile_id: 'openai-api',
        provider: 'openai_api',
        base_url: 'https://api.openai.com/v1',
        model: 'gpt-5.4',
        context_window: 32768,
        api_key: 'sk-test',
      })
    })
    expect(screen.getAllByText('Provider handshake succeeded.').length).toBeGreaterThan(0)

    fireEvent.click(openAiApiProfile.getByRole('button', { name: 'Save openai-api API' }))

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({
        llm: {
          default_chat_profile_id: 'openai-api',
          fallback_chat_profile_id: null,
          openai_compat_profiles: [],
          openai_api_profiles: [
            {
              id: 'openai-api',
              base_url: 'https://api.openai.com/v1',
              model: 'gpt-5.4',
              context_window: 32768,
              enabled: true,
              api_key: 'sk-test',
            },
          ],
        },
      })
    })
  })

  it('allows handshaking a new unsaved OpenAI API profile before saving it', async () => {
    render(<SystemView target={{ section: 'integrations', subsection: 'providers' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('LLM routing').length).toBeGreaterThan(0)
    })

    runLlmProfileHandshake.mockResolvedValueOnce({
      ok: true,
      data: {
        profile_id: 'openai-api-1',
        healthy: true,
        message: 'Provider handshake succeeded.',
      },
      meta: { request_id: 'req_llm_health_new_openai_api' },
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Add OpenAI API' }).at(-1) as HTMLElement)

    await screen.findByRole('heading', { name: 'openai-api-1' })

    const apiKeyFields = screen.getAllByLabelText('Replace OpenAI API key')
    const draftApiKeyField = apiKeyFields.at(-1) as HTMLElement

    fireEvent.change(draftApiKeyField, {
      target: { value: 'sk-draft-test' },
    })
    fireEvent.click(screen.getAllByRole('button', { name: 'Handshake openai-api-1' }).at(-1) as HTMLElement)

    await waitFor(() => {
      expect(runLlmProfileHandshake).toHaveBeenCalledWith({
        profile_id: 'openai-api-1',
        provider: 'openai_api',
        base_url: 'https://api.openai.com/v1',
        model: 'gpt-5.4',
        context_window: 32768,
        api_key: 'sk-draft-test',
      })
    })
    expect(screen.getAllByText('Provider handshake succeeded.').length).toBeGreaterThan(0)
  })

  it('exposes truthful persisted operator settings fields', async () => {
    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Core settings').length).toBeGreaterThan(0)
    })

    const userField = screen.getAllByLabelText('Your name *')[0]
    fireEvent.change(userField, { target: { value: 'Jove Operator' } })
    fireEvent.blur(userField)

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ core_settings: { user_display_name: 'Jove Operator' } })
    })

    const locationField = screen.getAllByLabelText('Client location')[0]
    fireEvent.change(locationField, { target: { value: 'Boulder, CO' } })
    fireEvent.blur(locationField)

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ core_settings: { client_location_label: 'Boulder, CO' } })
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Developer mode' })[0])
    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ core_settings: { developer_mode: true } })
    })

    const timezoneField = screen.getAllByLabelText('Timezone')[0]
    fireEvent.change(timezoneField, { target: { value: 'America/Chicago' } })
    fireEvent.blur(timezoneField)

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ timezone: 'America/Chicago' })
    })
  }, 10000)

  it('auto-saves core settings without waiting for blur', async () => {
    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Core settings').length).toBeGreaterThan(0)
    })

    const userField = screen.getAllByLabelText('Your name *')[0]
    fireEvent.change(userField, { target: { value: 'Jove Autosave' } })

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ core_settings: { user_display_name: 'Jove Autosave' } })
    }, { timeout: 1500 })
  })

  it('persists returned core settings into the shared settings cache immediately', async () => {
    updateSettings.mockResolvedValueOnce({
      ok: true,
      data: buildSettings({
        core_settings: {
          user_display_name: 'Jove Operator',
          client_location_label: 'Denver, CO',
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: 'Solo operator',
            preferences: 'Local-first, concise output',
            constraints: 'No broad automation by default',
            freeform: 'Prefers explainable actions.',
          },
        },
      }),
      meta: { request_id: 'req_settings_patch_updated' },
    })

    const firstRender = render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Core settings').length).toBeGreaterThan(0)
    })

    const userField = screen.getAllByLabelText('Your name *')[0]
    fireEvent.change(userField, { target: { value: 'Jove Operator' } })
    fireEvent.blur(userField)

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ core_settings: { user_display_name: 'Jove Operator' } })
    })

    firstRender.unmount()

    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByLabelText('Your name *')[0]).toHaveValue('Jove Operator')
    })
    expect(loadSettings).toHaveBeenCalledTimes(1)
  })

  it('shows required setup copy in the top-level core section', async () => {
    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText(/Vel will not be fully functional until required Core settings are submitted/i).length).toBeGreaterThan(0)
    })

    expect(screen.getAllByText('Open LLM routing').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Open integrations').length).toBeGreaterThan(0)
  })

  it('auto-infers host node name and timezone when they are missing', async () => {
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        writeback_enabled: false,
        node_display_name: null,
        timezone: null,
        llm: {
          models_dir: 'configs/models',
          default_chat_profile_id: 'local-llama',
          fallback_chat_profile_id: 'oauth-openai',
          profiles: [
            {
              id: 'local-llama',
              provider: 'llama_cpp',
              base_url: 'http://127.0.0.1:8012/v1',
              model: 'qwen3',
              context_window: 32768,
              enabled: true,
              editable: false,
            },
          ],
        },
        core_settings: {
          user_display_name: 'Jove',
          client_location_label: 'Denver, CO',
          developer_mode: false,
          bypass_setup_gate: false,
          agent_profile: {
            role: 'Solo operator',
            preferences: null,
            constraints: null,
            freeform: null,
          },
        },
        backup: null,
        web_settings: {
          dense_rows: true,
          tabular_numbers: true,
          reduced_motion: false,
          strong_focus: true,
          docked_action_bar: true,
        },
      },
      meta: { request_id: 'req_settings_infer' },
    })

    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ node_display_name: 'Local node' })
    })
    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ timezone: 'America/Denver' })
    })
  })

  it('can auto-set client location from browser geolocation and persist it', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        address: {
          city: 'Denver',
          state: 'Colorado',
          country: 'United States',
        },
        display_name: 'Denver, Colorado, United States',
      }),
    })
    vi.stubGlobal('fetch', fetchMock)
    Object.defineProperty(window.navigator, 'geolocation', {
      configurable: true,
      value: {
        getCurrentPosition: (success: PositionCallback) => {
          success({
            coords: {
              latitude: 39.7392,
              longitude: -104.9903,
            },
          } as GeolocationPosition)
        },
      } satisfies Partial<Geolocation>,
    })

    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Core settings').length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Auto-set client location' })[0])

    await waitFor(() => {
      expect(updateSettings).toHaveBeenCalledWith({ core_settings: { client_location_label: 'Denver, Colorado' } })
    })
    expect(fetchMock).toHaveBeenCalledTimes(1)
    expect(screen.getByText(/Updated from browser location: Denver, Colorado/i)).toBeInTheDocument()
  })

  it('surfaces a clear error when browser geolocation is unavailable', async () => {
    Object.defineProperty(window.navigator, 'geolocation', {
      configurable: true,
      value: undefined,
    })

    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Core settings').length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Auto-set client location' })[0])

    expect(await screen.findByText(/Browser geolocation is unavailable on this device\./i)).toBeInTheDocument()
    expect(updateSettings).not.toHaveBeenCalledWith({ core_settings: { client_location_label: expect.any(String) } })
  })
})

function buildLocalIntegration() {
  return {
    configured: false,
    source_path: null,
    selected_paths: [],
    available_paths: [],
    internal_paths: [],
    suggested_paths: [],
    source_kind: 'directory',
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  }
}
