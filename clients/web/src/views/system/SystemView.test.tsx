import '@testing-library/jest-dom/vitest'
import { fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { installEmbeddedBridgePacketRuntime } from '../../data/embeddedBridgePackets'
import { clearQueryCache } from '../../data/query'
import { SystemView } from './SystemView'

const loadAgentInspect = vi.fn()
const disconnectGoogleCalendar = vi.fn()
const disconnectTodoist = vi.fn()
const issuePairingToken = vi.fn()
const loadClusterBootstrap = vi.fn()
const loadClusterWorkers = vi.fn()
const loadIntegrationConnections = vi.fn()
const loadIntegrations = vi.fn()
const loadLinkingStatus = vi.fn()
const loadLlmProfileHealth = vi.fn()
const loadSettings = vi.fn()
const redeemPairingToken = vi.fn()
const revokeLinkedNode = vi.fn()
const runLlmProfileHandshake = vi.fn()
const startGoogleCalendarAuth = vi.fn()
const syncSource = vi.fn()
const updateGoogleCalendarIntegration = vi.fn()
const updateSettings = vi.fn()
const updateTodoistIntegration = vi.fn()
const updateWebSettings = vi.fn()

function normalizeTokenCode(value: string) {
  const normalized = value
    .toUpperCase()
    .replace(/[^A-Z0-9]/g, '')
    .slice(0, 6)
  if (normalized.length <= 3) {
    return normalized
  }
  return `${normalized.slice(0, 3)}-${normalized.slice(3)}`
}

function normalizeSemanticLabel(value: string | null | undefined) {
  return (value ?? '')
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, ' ')
    .trim()
}

function collectRemoteRoutes(
  syncBaseUrl?: string | null,
  tailscaleBaseUrl?: string | null,
  lanBaseUrl?: string | null,
  publicBaseUrl?: string | null,
) {
  const entries = [
    syncBaseUrl ? { label: 'sync', baseUrl: syncBaseUrl.trim() } : null,
    tailscaleBaseUrl ? { label: 'tailscale', baseUrl: tailscaleBaseUrl.trim() } : null,
    lanBaseUrl ? { label: 'lan', baseUrl: lanBaseUrl.trim() } : null,
    publicBaseUrl ? { label: 'public', baseUrl: publicBaseUrl.trim() } : null,
  ].filter((value): value is { label: string; baseUrl: string } => Boolean(value && value.baseUrl))

  return entries.filter(
    (entry, index) => entries.findIndex((candidate) => candidate.baseUrl === entry.baseUrl) === index,
  )
}

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

function buildIntegrations(overrides: Record<string, unknown> = {}) {
  return {
    google_calendar: {
      configured: true,
      connected: true,
      has_client_id: true,
      has_client_secret: true,
      calendars: [
        { id: 'cal_1', summary: 'Primary', primary: true, sync_enabled: true, display_enabled: true },
        { id: 'cal_2', summary: 'Team', primary: false, sync_enabled: true, display_enabled: false },
      ],
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
    ...overrides,
  }
}

function buildLinkingPrompt(overrides: Record<string, unknown> = {}) {
  return {
    target_node_id: 'node_local',
    target_node_display_name: 'Vel Desktop',
    issued_by_node_id: 'node_remote_prompt',
    issued_by_node_display_name: 'Road Mac',
    issued_at: '2026-03-26T17:00:00Z',
    expires_at: '2026-03-26T17:15:00Z',
    scopes: {
      read_context: true,
      write_safe_actions: false,
      execute_repo_tasks: false,
    },
    issuer_sync_base_url: 'http://road-mac.tailnet.ts.net:4130',
    issuer_sync_transport: 'tailscale',
    issuer_tailscale_base_url: 'http://road-mac.tailnet.ts.net:4130',
    issuer_lan_base_url: null,
    issuer_localhost_base_url: null,
    issuer_public_base_url: null,
    bootstrap_artifact: {
      artifact_id: 'artifact_prompt',
      trusted_node_id: 'node_remote_prompt',
      trusted_node_display_name: 'Road Mac',
      scopes: {
        read_context: true,
        write_safe_actions: false,
        execute_repo_tasks: false,
      },
      preferred_transport_hint: 'tailscale',
      endpoints: [
        {
          kind: 'tailscale',
          base_url: 'http://road-mac.tailnet.ts.net:4130',
          last_seen_at: null,
          advertised: true,
        },
      ],
      issued_at: '2026-03-26T17:00:00Z',
      expires_at: '2026-03-26T17:15:00Z',
    },
    ...overrides,
  }
}

function buildClusterBootstrap(overrides: Record<string, unknown> = {}) {
  return {
    node_id: 'node_local',
    node_display_name: 'Vel Desktop',
    active_authority_node_id: 'node_local',
    active_authority_epoch: 1,
    sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
    sync_transport: 'tailscale',
    tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
    lan_base_url: 'http://192.168.1.40:4130',
    localhost_base_url: 'http://127.0.0.1:4130',
    capabilities: ['read_context'],
    linked_nodes: [],
    projects: [],
    action_items: [],
    ...overrides,
  }
}

function buildWorker(overrides: Record<string, unknown> = {}) {
  return {
    worker_id: 'worker_local',
    node_id: 'node_local',
    node_display_name: 'Vel Desktop',
    client_kind: 'vel_macos',
    client_version: '0.5.2',
    protocol_version: '1',
    build_id: 'build_local',
    worker_classes: ['operator'],
    capabilities: ['read_context'],
    status: 'healthy',
    queue_depth: 0,
    reachability: 'direct',
    latency_class: 'low',
    compute_class: 'desktop',
    power_class: 'wall',
    recent_failure_rate: 0,
    tailscale_preferred: true,
    last_heartbeat_at: 1710000000,
    started_at: 1710000000,
    sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
    sync_transport: 'tailscale',
    tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
    preferred_tailnet_endpoint: 'vel-desktop.tailnet.ts.net',
    tailscale_reachable: true,
    lan_base_url: 'http://192.168.1.40:4130',
    localhost_base_url: 'http://127.0.0.1:4130',
    ping_ms: 18,
    sync_status: 'ok',
    last_upstream_sync_at: 1710000000,
    last_downstream_sync_at: 1710000000,
    last_sync_error: null,
    incoming_linking_prompt: null,
    capacity: {
      max_concurrency: 4,
      current_load: 1,
      available_concurrency: 3,
    },
    ...overrides,
  }
}

function buildClusterWorkers(overrides: Record<string, unknown> = {}) {
  return {
    active_authority_node_id: 'node_local',
    active_authority_epoch: 1,
    generated_at: 1710000000,
    workers: [
      buildWorker({ incoming_linking_prompt: buildLinkingPrompt() }),
      buildWorker({
        worker_id: 'worker_pocket',
        node_id: 'node_pocket',
        node_display_name: 'Pocket Mac',
        client_kind: 'vel_ios',
        build_id: 'build_pocket',
        sync_base_url: 'http://pocket-mac.tailnet.ts.net:4130',
        tailscale_base_url: 'http://pocket-mac.tailnet.ts.net:4130',
        lan_base_url: null,
        localhost_base_url: null,
        preferred_tailnet_endpoint: 'pocket-mac.tailnet.ts.net',
        incoming_linking_prompt: null,
      }),
      buildWorker({
        worker_id: 'worker_desk',
        node_id: 'node_desk',
        node_display_name: 'Desk Mac',
        client_kind: 'vel_macos',
        build_id: 'build_desk',
        sync_base_url: 'http://desk-mac.tailnet.ts.net:4130',
        tailscale_base_url: 'http://desk-mac.tailnet.ts.net:4130',
        lan_base_url: 'http://192.168.1.50:4130',
        localhost_base_url: null,
        preferred_tailnet_endpoint: 'desk-mac.tailnet.ts.net',
        incoming_linking_prompt: null,
      }),
    ],
    ...overrides,
  }
}

function buildLinkedNode(overrides: Record<string, unknown> = {}) {
  return {
    node_id: 'node_desk',
    node_display_name: 'Desk Mac',
    status: 'linked',
    scopes: {
      read_context: true,
      write_safe_actions: true,
      execute_repo_tasks: false,
    },
    linked_at: '2026-03-26T16:50:00Z',
    last_seen_at: '2026-03-26T16:58:00Z',
    transport_hint: 'tailscale',
    sync_base_url: 'http://desk-mac.tailnet.ts.net:4130',
    tailscale_base_url: 'http://desk-mac.tailnet.ts.net:4130',
    lan_base_url: 'http://192.168.1.50:4130',
    localhost_base_url: null,
    public_base_url: null,
    ...overrides,
  }
}

function buildPairingToken(overrides: Record<string, unknown> = {}) {
  return {
    token_id: 'pair_123',
    token_code: 'VEL-PAIR-123',
    issued_at: '2026-03-26T17:00:00Z',
    expires_at: '2026-03-26T17:15:00Z',
    issued_by_node_id: 'node_local',
    scopes: {
      read_context: true,
      write_safe_actions: false,
      execute_repo_tasks: false,
    },
    bootstrap_artifact: {
      artifact_id: 'artifact_token',
      trusted_node_id: 'node_local',
      trusted_node_display_name: 'Vel Desktop',
      scopes: {
        read_context: true,
        write_safe_actions: false,
        execute_repo_tasks: false,
      },
      preferred_transport_hint: 'tailscale',
      endpoints: [
        {
          kind: 'tailscale',
          base_url: 'http://vel-desktop.tailnet.ts.net:4130',
          last_seen_at: null,
          advertised: true,
        },
      ],
      issued_at: '2026-03-26T17:00:00Z',
      expires_at: '2026-03-26T17:15:00Z',
    },
    suggested_targets: [
      {
        label: 'Recommended',
        base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        transport_hint: 'tailscale',
        recommended: true,
        redeem_command_hint: 'vel --base-url http://vel-desktop.tailnet.ts.net:4130 node link redeem VEL-PAIR-123 --node-id <node_id> --node-display-name <name> --transport-hint tailscale',
      },
    ],
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
    issuePairingToken: (...args: unknown[]) => issuePairingToken(...args),
    loadClusterBootstrap: (...args: unknown[]) => loadClusterBootstrap(...args),
    loadClusterWorkers: (...args: unknown[]) => loadClusterWorkers(...args),
    loadIntegrationConnections: (...args: unknown[]) => loadIntegrationConnections(...args),
    loadIntegrations: (...args: unknown[]) => loadIntegrations(...args),
    loadLinkingStatus: (...args: unknown[]) => loadLinkingStatus(...args),
    loadLlmProfileHealth: (...args: unknown[]) => loadLlmProfileHealth(...args),
    loadSettings: (...args: unknown[]) => loadSettings(...args),
    redeemPairingToken: (...args: unknown[]) => redeemPairingToken(...args),
    revokeLinkedNode: (...args: unknown[]) => revokeLinkedNode(...args),
    runLlmProfileHandshake: (...args: unknown[]) => runLlmProfileHandshake(...args),
    operatorQueryKeys: {
      agentInspect: () => ['agent', 'inspect'],
      clusterBootstrap: () => ['cluster', 'bootstrap'],
      clusterWorkers: () => ['cluster', 'workers'],
      integrations: () => ['integrations'],
      integrationConnections: () => ['integrations', 'connections', 'all', 'all'],
      linkingStatus: () => ['linking', 'status'],
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
    Object.defineProperty(HTMLElement.prototype, 'scrollIntoView', {
      configurable: true,
      value: vi.fn(),
    })
    installEmbeddedBridgePacketRuntime({
      normalizePairingTokenPacket: (input: string) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({ tokenCode: normalizeTokenCode(input) }),
      }),
      normalizeDomainHintPacket: (input: string) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({ normalized: normalizeSemanticLabel(input) }),
      }),
      normalizeSemanticLabelPacket: (input: string) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({ normalized: normalizeSemanticLabel(input) }),
      }),
      normalizeTaskDisplayPacket: (tags?: string[] | null, project?: string | null) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({ tags: tags ?? [], project: project ?? null }),
      }),
      normalizeTaskDisplayBatchPacket: (entriesJson: string) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({
          items: (JSON.parse(entriesJson) as Array<{ tags?: string[] | null; project?: string | null }>)
            .map((entry) => ({ tags: entry.tags ?? [], project: entry.project ?? null })),
        }),
      }),
      shortClientKindLabelPacket: (clientKind?: string | null) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({ shortLabel: clientKind ?? null }),
      }),
      actionItemDedupeKeyPacket: (kind: string, title: string, summary: string) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({ key: `${kind}:${title}:${summary}` }),
      }),
      actionItemDedupeBatchPacket: (entriesJson: string) => ({
        kind: 'deterministic_domain_helpers',
        payloadJson: JSON.stringify({
          keys: (JSON.parse(entriesJson) as Array<{ kind: string; title: string; summary: string }>)
            .map((entry) => `${entry.kind}:${entry.title}:${entry.summary}`),
        }),
      }),
      queuedActionPacket: (kind: string, targetId?: string | null, text?: string | null, minutes?: number | null) => ({
        kind: 'queued_action_packaging',
        payloadJson: JSON.stringify({
          queueKind: kind,
          targetId: targetId ?? null,
          text: text ?? null,
          minutes: minutes ?? null,
          ready: Boolean(kind),
        }),
      }),
      voiceQuickActionPacket: (_intentStorageToken: string, primaryText: string, targetId?: string | null, minutes?: number | null) => ({
        kind: 'voice_quick_action_packaging',
        payloadJson: JSON.stringify({
          queueKind: primaryText,
          targetId: targetId ?? null,
          text: primaryText,
          minutes: minutes ?? null,
          ready: true,
        }),
      }),
      assistantEntryFallbackPacket: (text: string, requestedConversationId?: string | null) => ({
        kind: 'assistant_entry_fallback_packaging',
        payloadJson: JSON.stringify({
          payload: text,
          requestedConversationId: requestedConversationId ?? null,
        }),
      }),
      captureMetadataPacket: (text: string, captureType: string, sourceDevice: string) => ({
        kind: 'capture_metadata_packaging',
        payloadJson: JSON.stringify({ payload: `${captureType}:${sourceDevice}:${text}` }),
      }),
      threadDraftPacket: (text: string, requestedConversationId?: string | null) => ({
        kind: 'thread_draft_packaging',
        payloadJson: JSON.stringify({
          payload: text,
          requestedConversationId: requestedConversationId ?? null,
        }),
      }),
      voiceCapturePacket: (transcript: string, intentStorageToken: string) => ({
        kind: 'voice_capture_packaging',
        payloadJson: JSON.stringify({ payload: `${intentStorageToken}:${transcript}` }),
      }),
      linkingRequestPacket: (tokenCode?: string | null, targetBaseUrl?: string | null) => ({
        kind: 'linking_request_packaging',
        payloadJson: JSON.stringify({
          tokenCode: tokenCode ? normalizeTokenCode(tokenCode) : null,
          targetBaseUrl: targetBaseUrl?.trim() || null,
        }),
      }),
      linkingFeedbackPacket: (scenario: string, nodeDisplayName?: string | null) => ({
        kind: 'linking_feedback_packaging',
        payloadJson: JSON.stringify({ message: `${scenario}:${nodeDisplayName ?? ''}` }),
      }),
      appShellFeedbackPacket: (scenario: string, detail?: string | null) => ({
        kind: 'app_shell_feedback_packaging',
        payloadJson: JSON.stringify({ message: `${scenario}:${detail ?? ''}` }),
      }),
      collectRemoteRoutesPacket: (
        syncBaseUrl?: string | null,
        tailscaleBaseUrl?: string | null,
        lanBaseUrl?: string | null,
        publicBaseUrl?: string | null,
      ) => ({
        kind: 'linking_settings_normalization',
        payloadJson: JSON.stringify(
          collectRemoteRoutes(syncBaseUrl, tailscaleBaseUrl, lanBaseUrl, publicBaseUrl),
        ),
      }),
      voiceContinuitySummaryPacket: () => ({
        kind: 'voice_continuity_summary_packaging',
        payloadJson: JSON.stringify({ headline: null, detail: null, ready: false }),
      }),
      voiceOfflineResponsePacket: () => ({
        kind: 'voice_offline_response_packaging',
        payloadJson: JSON.stringify({
          summary: null,
          detail: null,
          historyStatus: 'unavailable',
          errorPrefix: '',
          ready: false,
        }),
      }),
      voiceCachedQueryResponsePacket: () => ({
        kind: 'voice_cached_query_packaging',
        payloadJson: JSON.stringify({ summary: null, detail: null, ready: false }),
      }),
    })
    loadAgentInspect.mockReset()
    disconnectGoogleCalendar.mockReset()
    disconnectTodoist.mockReset()
    issuePairingToken.mockReset()
    loadClusterBootstrap.mockReset()
    loadClusterWorkers.mockReset()
    loadIntegrationConnections.mockReset()
    loadIntegrations.mockReset()
    loadLinkingStatus.mockReset()
    loadLlmProfileHealth.mockReset()
    loadSettings.mockReset()
    redeemPairingToken.mockReset()
    revokeLinkedNode.mockReset()
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
      data: buildIntegrations(),
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
    loadClusterBootstrap.mockResolvedValue({
      ok: true,
      data: buildClusterBootstrap(),
      meta: { request_id: 'req_cluster_bootstrap' },
    })
    loadClusterWorkers.mockResolvedValue({
      ok: true,
      data: buildClusterWorkers(),
      meta: { request_id: 'req_cluster_workers' },
    })
    loadLinkingStatus.mockResolvedValue({
      ok: true,
      data: [buildLinkedNode()],
      meta: { request_id: 'req_linking_status' },
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
    issuePairingToken.mockResolvedValue({
      ok: true,
      data: buildPairingToken(),
      meta: { request_id: 'req_pair_issue' },
    })
    redeemPairingToken.mockResolvedValue({
      ok: true,
      data: buildLinkedNode({
        node_id: 'node_remote_prompt',
        node_display_name: 'Road Mac',
        scopes: {
          read_context: true,
          write_safe_actions: false,
          execute_repo_tasks: false,
        },
      }),
      meta: { request_id: 'req_pair_redeem' },
    })
    revokeLinkedNode.mockResolvedValue({
      ok: true,
      data: buildLinkedNode({ status: 'revoked', last_seen_at: null }),
      meta: { request_id: 'req_pair_revoke' },
    })
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

  it('adds node pairing controls to the domain group and runs issue redeem revoke flows', async () => {
    render(<SystemView target={{ section: 'core', subsection: 'pairing' }} />)

    await waitFor(() => {
      expect(loadClusterBootstrap).toHaveBeenCalled()
      expect(loadClusterWorkers).toHaveBeenCalled()
      expect(loadLinkingStatus).toHaveBeenCalled()
    })

    expect(screen.getAllByRole('button', { name: 'Node pairing. Issue, redeem, and inspect node trust links for companion devices.' }).length).toBeGreaterThan(0)
    expect(screen.getByRole('button', { name: 'Redeem a node token instead' })).toBeInTheDocument()
    expect(screen.getByText('Configure sources')).toBeInTheDocument()
    expect(screen.getByText(/This node already has a prompt waiting/i)).toBeInTheDocument()
    expect(screen.getByText('Desk Mac')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'Issue token for Pocket Mac' }))

    await waitFor(() => {
      expect(issuePairingToken).toHaveBeenCalledWith({
        issued_by_node_id: 'node_local',
        ttl_seconds: 900,
        scopes: {
          read_context: true,
          write_safe_actions: false,
          execute_repo_tasks: false,
        },
        target_node_id: 'node_pocket',
        target_node_display_name: 'Pocket Mac',
        target_base_url: 'http://pocket-mac.tailnet.ts.net:4130',
      })
    })

    expect(await screen.findByText('VEL-PAIR-123')).toBeInTheDocument()

    fireEvent.change(screen.getByLabelText('Pairing token'), {
      target: { value: 'vel123' },
    })
    fireEvent.click(screen.getAllByRole('button', { name: 'Redeem token' })[1])

    await waitFor(() => {
      expect(redeemPairingToken).toHaveBeenCalledWith({
        token_code: 'VEL-123',
        node_id: 'node_local',
        node_display_name: 'Vel Desktop',
        transport_hint: 'tailscale',
        sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        lan_base_url: 'http://192.168.1.40:4130',
        localhost_base_url: 'http://127.0.0.1:4130',
      })
    })

    fireEvent.click(screen.getByRole('button', { name: 'Revoke Desk Mac' }))

    await waitFor(() => {
      expect(revokeLinkedNode).toHaveBeenCalledWith('node_desk')
    })
  })

  it('starts a discovery refresh loop while pairing onboarding is still unresolved', async () => {
    loadLinkingStatus.mockResolvedValueOnce({
      ok: true,
      data: [],
      meta: { request_id: 'req_linking_status_empty' },
    })
    const setIntervalSpy = vi.spyOn(window, 'setInterval')
    const clearIntervalSpy = vi.spyOn(window, 'clearInterval')

    const { unmount } = render(<SystemView target={{ section: 'core', subsection: 'pairing' }} />)

    await waitFor(() => {
      expect(loadClusterWorkers).toHaveBeenCalledTimes(1)
      expect(loadLinkingStatus).toHaveBeenCalledTimes(1)
    })

    expect(setIntervalSpy).toHaveBeenCalled()
    expect(setIntervalSpy).toHaveBeenLastCalledWith(expect.any(Function), 4000)

    unmount()

    expect(clearIntervalSpy).toHaveBeenCalled()
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

  it('renders system documentation at the end of the system page', async () => {
    render(<SystemView target={{ section: 'overview', subsection: 'trust' }} />)

    const docsHeader = await screen.findByText('System documentation')
    const accessibilityHeader = await screen.findByText('Accessibility and operator ergonomics')
    expect(accessibilityHeader.compareDocumentPosition(docsHeader) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy()
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

    updateGoogleCalendarIntegration
      .mockResolvedValueOnce({
        ok: true,
        data: buildIntegrations({
          google_calendar: {
            ...buildIntegrations().google_calendar,
            calendars: [
              { id: 'cal_1', summary: 'Primary', primary: true, sync_enabled: true, display_enabled: true },
              { id: 'cal_2', summary: 'Team', primary: false, sync_enabled: false, display_enabled: false },
            ],
          },
        }),
        meta: { request_id: 'req_google_patch_sync' },
      })
      .mockResolvedValueOnce({
        ok: true,
        data: buildIntegrations({
          google_calendar: {
            ...buildIntegrations().google_calendar,
            calendars: [
              { id: 'cal_1', summary: 'Primary', primary: true, sync_enabled: true, display_enabled: false },
              { id: 'cal_2', summary: 'Team', primary: false, sync_enabled: false, display_enabled: false },
            ],
          },
        }),
        meta: { request_id: 'req_google_patch_visible' },
      })

    const teamSync = screen.getAllByRole('checkbox', { name: 'Team sync' }).at(-1) as HTMLInputElement
    expect(teamSync.checked).toBe(true)

    fireEvent.click(teamSync)
    await waitFor(() => {
      expect(updateGoogleCalendarIntegration).toHaveBeenCalledWith({
        calendar_settings: [
          {
            id: 'cal_2',
            sync_enabled: false,
            display_enabled: false,
          },
        ],
      })
    })
    await waitFor(() => {
      expect((screen.getAllByRole('checkbox', { name: 'Team sync' }).at(-1) as HTMLInputElement).checked).toBe(false)
    })

    const primaryVisible = screen.getAllByRole('checkbox', { name: 'Primary visible' }).at(-1) as HTMLInputElement
    expect(primaryVisible.checked).toBe(true)

    fireEvent.click(primaryVisible)
    await waitFor(() => {
      expect(updateGoogleCalendarIntegration).toHaveBeenCalledWith({
        calendar_settings: [
          {
            id: 'cal_1',
            display_enabled: false,
          },
        ],
      })
    })
    await waitFor(() => {
      expect((screen.getAllByRole('checkbox', { name: 'Primary visible' }).at(-1) as HTMLInputElement).checked).toBe(false)
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

  it('shows required setup copy in the top-level core section when setup is incomplete', async () => {
    const defaultSettings = buildSettings();
    loadSettings.mockResolvedValueOnce({
      ok: true,
      data: {
        ...defaultSettings,
        core_settings: {
          ...defaultSettings.core_settings,
          user_display_name: '',
        },
      },
      meta: { request_id: 'req_settings_incomplete' },
    });

    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText(/Vel will not be fully functional until required Core settings are submitted/i).length).toBeGreaterThan(0)
    })

    expect(screen.getAllByText('Open LLM routing').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Open integrations').length).toBeGreaterThan(0)
  })

  it('hides required setup copy in the top-level core section when setup is complete', async () => {
    render(<SystemView target={{ section: 'core', subsection: 'core_settings' }} />)

    await waitFor(() => {
      expect(screen.getAllByText('Core settings').length).toBeGreaterThan(0)
    })

    expect(screen.queryByText(/Vel will not be fully functional until required Core settings are submitted/i)).toBeNull()
    expect(screen.queryByText('Open LLM routing')).toBeNull()
    expect(screen.queryByText('Open integrations')).toBeNull()
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
