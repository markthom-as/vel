import { describe, it, expect, vi, beforeEach, afterAll } from 'vitest'
import { render, screen, fireEvent, waitFor, within } from '@testing-library/react'
import { SettingsPage } from './SettingsPage'
import * as client from '../api/client'
import { clearQueryCache } from '../data/query'
import { resetWsQuerySyncForTests } from '../data/ws-sync'
import type { WsEnvelope } from '../types'

const subscribeWs = vi.fn()
const originalNavigatorPlatform = navigator.platform
const originalNavigatorUserAgent = navigator.userAgent

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
  apiPost: vi.fn(),
}))

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

function getSettingsRoot(container: HTMLElement) {
  return container.firstElementChild as HTMLElement
}

function createDeferred<T>() {
  let resolve!: (value: T | PromiseLike<T>) => void
  let reject!: (reason?: unknown) => void
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    resolve = resolvePromise
    reject = rejectPromise
  })
  return { promise, resolve, reject }
}

async function openRuntimeTab(container: HTMLElement) {
  const root = getSettingsRoot(container)
  await waitFor(() => {
    expect(within(root).getByRole('button', { name: /^runtime$/i })).toBeInTheDocument()
  })
  fireEvent.click(within(root).getByRole('button', { name: /^runtime$/i }))
  await waitFor(() => {
    expect(within(root).getByRole('heading', { name: /runtime controls/i })).toBeInTheDocument()
  })
  return root
}

async function openIntegrationsTab(container: HTMLElement) {
  const root = getSettingsRoot(container)
  await waitFor(() => {
    expect(within(root).getByRole('button', { name: /^integrations$/i })).toBeInTheDocument()
  })
  fireEvent.click(within(root).getByRole('button', { name: /^integrations$/i }))
  await waitFor(() => {
    expect(within(root).getByRole('heading', { name: /google calendar/i })).toBeInTheDocument()
  })
  return root
}

describe('SettingsPage', () => {
  beforeEach(() => {
    Object.defineProperty(window.navigator, 'platform', {
      configurable: true,
      value: 'MacIntel',
    })
    Object.defineProperty(window.navigator, 'userAgent', {
      configurable: true,
      value: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/537.36',
    })
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.useRealTimers()
    subscribeWs.mockReset()
    vi.mocked(client.apiGet).mockReset()
    vi.mocked(client.apiPatch).mockReset()
    vi.mocked(client.apiPost).mockReset()
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: {
            disable_proactive: false,
            toggle_risks: true,
            toggle_reminders: true,
            timezone: 'America/Denver',
            node_display_name: 'Vel Desktop',
            writeback_enabled: false,
            tailscale_preferred: true,
            tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            tailscale_base_url_auto_discovered: true,
            lan_base_url: 'http://192.168.1.50:4130',
            lan_base_url_auto_discovered: true,
            adaptive_policy_overrides: {
              commute_buffer_minutes: 30,
              default_prep_minutes: 45,
              commute_buffer_source_suggestion_id: 'sug_commute',
              commute_buffer_source_title: 'Increase commute buffer',
              commute_buffer_source_accepted_at: 1_710_000_100,
              default_prep_source_suggestion_id: 'sug_prep',
              default_prep_source_title: 'Increase prep window',
              default_prep_source_accepted_at: 1_710_000_200,
            },
            backup: {
              default_output_root: 'var/backups',
              trust: {
                level: 'warn',
                status: {
                  state: 'stale',
                  last_backup_id: 'bkp_123',
                  last_backup_at: '2026-03-18T18:20:00Z',
                  output_root: '/tmp/backups/bkp_123',
                  artifact_coverage: {
                    included: ['artifacts/captures', 'artifacts/exports'],
                    omitted: ['artifacts/cache'],
                    notes: ['Transient cache directories are excluded.'],
                  },
                  config_coverage: {
                    included: ['config/public-settings.json', 'config/runtime-config.json'],
                    omitted: ['integration_google_calendar_secrets'],
                    notes: ['Secret-bearing settings are omitted.'],
                  },
                  verification_summary: {
                    verified: true,
                    checksum_algorithm: 'sha256',
                    checksum: 'abc123',
                    checked_paths: ['/tmp/backups/bkp_123/manifest.json'],
                    notes: ['Verified from the snapshot and manifest.'],
                  },
                  warnings: ['last successful backup is stale'],
                },
                freshness: {
                  state: 'stale',
                  age_seconds: 60 * 60 * 50,
                  stale_after_seconds: 60 * 60 * 48,
                },
                guidance: ['Create or verify a fresh backup before risky maintenance.'],
              },
            },
          },
          meta: { request_id: 'req_1' },
        } as never
      }
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            summary: {
              mode: { key: 'focus', label: 'Focus' },
              phase: { key: 'engaged', label: 'Engaged' },
              meds: { key: 'ok', label: 'OK' },
              risk: { level: 'low', score: 0.2, label: 'low' },
            },
            schedule: {
              empty_message: null,
              next_event: null,
              upcoming_events: [],
            },
            tasks: {
              todoist: [],
              other_open: [],
              next_commitment: null,
            },
            attention: {
              state: { key: 'on_task', label: 'On task' },
              drift: { key: 'none', label: 'None' },
              severity: { key: 'none', label: 'None' },
              confidence: null,
              reasons: [],
            },
            sources: {
              git_activity: null,
              health: null,
              mood: null,
              pain: null,
              note_document: null,
              assistant_message: null,
            },
            freshness: {
              overall_status: 'fresh',
              sources: [],
            },
            action_items: [
              {
                id: 'act_person',
                surface: 'now',
                kind: 'next_step',
                title: 'Reply to Annie',
                summary: 'Draft review pending',
                project_id: null,
                state: 'active',
                rank: 72,
                surfaced_at: '2026-03-18T18:00:00Z',
                snoozed_until: null,
                evidence: [
                  {
                    source_kind: 'person',
                    source_id: 'per_annie',
                    label: 'Annie Case',
                    detail: null,
                  },
                ],
              },
            ],
            review_snapshot: {
              open_action_count: 1,
              triage_count: 0,
              projects_needing_review: 0,
            },
            pending_writebacks: [
              {
                id: 'wb_1',
                kind: 'email_create_draft_reply',
                risk: 'safe',
                status: 'queued',
                target: {
                  family: 'messaging',
                  provider_key: 'email',
                  project_id: null,
                  connection_id: 'icn_email',
                  external_id: 'thread_1',
                },
                requested_payload: {},
                result_payload: null,
                provenance: [],
                conflict_case_id: null,
                requested_by_node_id: 'vel-local',
                requested_at: '2026-03-18T18:00:00Z',
                applied_at: null,
                updated_at: '2026-03-18T18:00:00Z',
              },
            ],
            conflicts: [
              {
                id: 'conf_1',
                kind: 'upstream_vs_local',
                status: 'open',
                target: {
                  family: 'messaging',
                  provider_key: 'email',
                  project_id: null,
                  connection_id: 'icn_email',
                  external_id: 'thread_1',
                },
                summary: 'Conflict',
                local_payload: {},
                upstream_payload: null,
                resolution_payload: null,
                opened_at: '2026-03-18T18:00:00Z',
                resolved_at: null,
                updated_at: '2026-03-18T18:00:00Z',
              },
            ],
            people: [
              {
                id: 'per_annie',
                display_name: 'Annie Case',
                given_name: 'Annie',
                family_name: 'Case',
                relationship_context: null,
                birthday: null,
                last_contacted_at: null,
                aliases: [],
                links: [],
              },
            ],
            reasons: [],
            debug: {
              raw_context: {},
              signals_used: [],
              commitments_used: [],
              risk_used: [],
            },
          },
          meta: { request_id: 'req_now' },
        } as never
      }
      if (path === '/v1/execution/handoffs?state=pending_review') {
        return {
          ok: true,
          data: [
            {
              id: 'xho_1',
              project_id: 'proj_exec',
              origin_kind: 'human_to_agent',
              review_state: 'pending_review',
              handoff: {
                handoff: {
                  task_id: 'task_1',
                  trace_id: 'trace_1',
                  from_agent: 'operator',
                  to_agent: 'codex-local',
                  objective: 'Implement the runtime review queue',
                  inputs: {},
                  constraints: [],
                  read_scopes: ['/tmp/vel'],
                  write_scopes: ['/tmp/vel'],
                  project_id: 'proj_exec',
                  task_kind: 'implementation',
                  agent_profile: 'quality',
                  token_budget: 'large',
                  review_gate: 'operator_approval',
                  repo_root: {
                    path: '/tmp/vel',
                    label: 'vel',
                    branch: null,
                    head_rev: null,
                  },
                  allowed_tools: ['rg'],
                  capability_scope: {},
                  deadline: null,
                  expected_output_schema: { artifacts: ['patch'] },
                },
                project_id: 'proj_exec',
                task_kind: 'implementation',
                agent_profile: 'quality',
                token_budget: 'large',
                review_gate: 'operator_approval',
                repo: {
                  path: '/tmp/vel',
                  label: 'vel',
                  branch: null,
                  head_rev: null,
                },
                notes_root: {
                  path: '/tmp/vel/notes',
                  label: 'vel-notes',
                  kind: 'notes_root',
                },
                manifest_id: null,
              },
              routing: {
                task_kind: 'implementation',
                agent_profile: 'quality',
                token_budget: 'large',
                review_gate: 'operator_approval',
                read_scopes: ['/tmp/vel'],
                write_scopes: ['/tmp/vel'],
                allowed_tools: ['rg'],
                reasons: [
                  {
                    code: 'write_scope_requires_approval',
                    message: 'write scopes require explicit operator approval before launch',
                  },
                ],
              },
              manifest_id: null,
              requested_by: 'operator_shell',
              reviewed_by: null,
              decision_reason: null,
              reviewed_at: null,
              launched_at: null,
              created_at: '2026-03-18T18:00:00Z',
              updated_at: '2026-03-18T18:00:00Z',
            },
          ],
          meta: { request_id: 'req_handoffs' },
        } as never
      }
      if (path === '/v1/agent/inspect') {
        return {
          ok: true,
          data: {
            grounding: {
              generated_at: 1710000000,
              now: {
                computed_at: 1710000000,
                timezone: 'America/Denver',
                summary: {
                  mode: { key: 'focus', label: 'Focus' },
                  phase: { key: 'engaged', label: 'Engaged' },
                  meds: { key: 'ok', label: 'OK' },
                  risk: { level: 'low', score: 0.2, label: 'low' },
                },
                schedule: { empty_message: null, next_event: null, upcoming_events: [] },
                tasks: { todoist: [], other_open: [], next_commitment: null },
                attention: {
                  state: { key: 'on_task', label: 'On task' },
                  drift: { key: 'none', label: 'None' },
                  severity: { key: 'none', label: 'None' },
                  confidence: null,
                  reasons: [],
                },
                sources: {
                  git_activity: null,
                  health: null,
                  mood: null,
                  pain: null,
                  note_document: null,
                  assistant_message: null,
                },
                freshness: { overall_status: 'fresh', sources: [] },
                action_items: [],
                review_snapshot: { open_action_count: 1, triage_count: 0, projects_needing_review: 0 },
                pending_writebacks: [],
                conflicts: [],
                people: [],
                reasons: [],
                debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
              },
              current_context: {
                computed_at: 1710000000,
                mode: 'focus',
                morning_state: 'engaged',
                current_context_path: '/v1/context/current',
                explain_context_path: '/v1/explain/context',
                explain_drift_path: '/v1/explain/drift',
              },
              projects: [{ id: 'proj_exec', slug: 'vel', name: 'Vel', family: 'work', status: 'active', primary_repo: { path: '/tmp/vel', label: 'vel', kind: 'repo' }, primary_notes_root: { path: '/tmp/vel/notes', label: 'vel-notes', kind: 'notes_root' }, secondary_repos: [], secondary_notes_roots: [], upstream_ids: {}, pending_provision: { create_repo: false, create_notes_root: false }, created_at: '2026-03-18T18:00:00Z', updated_at: '2026-03-18T18:00:00Z', archived_at: null }],
              people: [{ id: 'per_annie', display_name: 'Annie Case', given_name: 'Annie', family_name: 'Case', relationship_context: null, birthday: null, last_contacted_at: null, aliases: [], links: [] }],
              commitments: [{ id: 'com_1', text: 'Ship agent grounding', source_type: 'todoist', source_id: 'todo_1', status: 'open', due_at: null, project: 'proj_exec', commitment_kind: 'must', created_at: '2026-03-18T18:00:00Z', resolved_at: null, metadata: {} }],
              review: {
                review_snapshot: { open_action_count: 1, triage_count: 0, projects_needing_review: 0 },
                pending_writebacks: [],
                conflicts: [],
                pending_execution_handoffs: [
                  {
                    id: 'xho_1',
                    project_id: 'proj_exec',
                    origin_kind: 'human_to_agent',
                    review_state: 'pending_review',
                    handoff: {
                      handoff: {
                        task_id: 'task_1',
                        trace_id: 'trace_1',
                        from_agent: 'operator',
                        to_agent: 'codex-local',
                        objective: 'Implement the runtime review queue',
                        inputs: {},
                        constraints: [],
                        read_scopes: ['/tmp/vel'],
                        write_scopes: ['/tmp/vel'],
                        project_id: 'proj_exec',
                        task_kind: 'implementation',
                        agent_profile: 'quality',
                        token_budget: 'large',
                        review_gate: 'operator_approval',
                        repo_root: { path: '/tmp/vel', label: 'vel', branch: null, head_rev: null },
                        allowed_tools: ['rg'],
                        capability_scope: {},
                        deadline: null,
                        expected_output_schema: { artifacts: ['patch'] },
                      },
                      project_id: 'proj_exec',
                      task_kind: 'implementation',
                      agent_profile: 'quality',
                      token_budget: 'large',
                      review_gate: 'operator_approval',
                      repo: { path: '/tmp/vel', label: 'vel', branch: null, head_rev: null },
                      notes_root: { path: '/tmp/vel/notes', label: 'vel-notes', kind: 'notes_root' },
                      manifest_id: null,
                    },
                    routing: {
                      task_kind: 'implementation',
                      agent_profile: 'quality',
                      token_budget: 'large',
                      review_gate: 'operator_approval',
                      read_scopes: ['/tmp/vel'],
                      write_scopes: ['/tmp/vel'],
                      allowed_tools: ['rg'],
                      reasons: [],
                    },
                    manifest_id: null,
                    requested_by: 'operator_shell',
                    reviewed_by: null,
                    decision_reason: null,
                    reviewed_at: null,
                    launched_at: null,
                    created_at: '2026-03-18T18:00:00Z',
                    updated_at: '2026-03-18T18:00:00Z',
                  },
                ],
              },
            },
            capabilities: {
              groups: [
                {
                  kind: 'read_context',
                  label: 'Read current Vel state',
                  entries: [
                    {
                      key: 'read_now',
                      label: 'Read Now and current context',
                      summary: 'The agent can inspect current Now state, typed context labels, and explain references.',
                      available: true,
                      blocked_reason: null,
                      requires_review_gate: null,
                      requires_writeback_enabled: false,
                    },
                  ],
                },
                {
                  kind: 'mutation_actions',
                  label: 'Bounded mutation affordances',
                  entries: [
                    {
                      key: 'integration_writeback',
                      label: 'Request integration writeback',
                      summary: 'Bounded upstream mutations remain subject to SAFE MODE and review gates.',
                      available: false,
                      blocked_reason: {
                        code: 'safe_mode_enabled',
                        message: 'SAFE MODE keeps writeback disabled.',
                        escalation_hint: 'Enable writeback in Settings before retrying.',
                      },
                      requires_review_gate: 'operator_preview',
                      requires_writeback_enabled: true,
                    },
                  ],
                },
              ],
            },
            blockers: [
              {
                code: 'writeback_disabled',
                message: 'Writeback-dependent mutation requests are unavailable while SAFE MODE is enabled.',
                escalation_hint: 'Enable writeback or stay within read/review lanes.',
              },
            ],
            explainability: {
              persisted_record_kinds: ['now'],
              supporting_paths: ['/v1/agent/inspect'],
              raw_context_json_supporting_only: true,
            },
          },
          meta: { request_id: 'req_agent_inspect' },
        } as never
      }
      if (path === '/v1/cluster/bootstrap') {
        return {
          ok: true,
          data: {
            node_id: 'vel-desktop',
            node_display_name: 'Vel Desktop',
            active_authority_node_id: 'vel-desktop',
            active_authority_epoch: 1,
            sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            sync_transport: 'tailscale',
            tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            lan_base_url: 'http://192.168.1.50:4130',
            localhost_base_url: 'http://127.0.0.1:4130',
            capabilities: ['read_context', 'write_safe_actions'],
            linked_nodes: [
              {
                node_id: 'vel-air',
                node_display_name: 'Vel Air',
                status: 'linked',
                scopes: {
                  read_context: true,
                  write_safe_actions: true,
                  execute_repo_tasks: false,
                },
                linked_at: '2026-03-16T18:00:00Z',
                last_seen_at: '2026-03-16T18:15:00Z',
                transport_hint: 'tailscale',
                sync_base_url: 'http://vel-air.tailnet.ts.net:4130',
                tailscale_base_url: 'http://vel-air.tailnet.ts.net:4130',
                lan_base_url: 'http://192.168.1.70:4130',
                localhost_base_url: null,
                public_base_url: null,
              },
            ],
            projects: [],
            action_items: [],
          },
          meta: { request_id: 'req_cluster_bootstrap' },
        } as never
      }
      if (path === '/v1/cluster/workers') {
        return {
          ok: true,
          data: {
            active_authority_node_id: 'vel-desktop',
            active_authority_epoch: 1,
            generated_at: 1_710_000_100,
            workers: [
              {
                worker_id: 'vel-desktop',
                node_id: 'vel-desktop',
                node_display_name: 'Vel Desktop',
                client_kind: 'veld',
                client_version: '0.1.0',
                protocol_version: '1',
                build_id: 'build_local',
                worker_classes: ['sync'],
                capabilities: ['sync_bootstrap'],
                status: 'ok',
                queue_depth: 0,
                reachability: 'reachable',
                latency_class: 'low',
                compute_class: 'standard',
                power_class: 'ac_or_unknown',
                recent_failure_rate: 0,
                tailscale_preferred: true,
                last_heartbeat_at: 1_710_000_095,
                started_at: 1_710_000_000,
                sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
                sync_transport: 'tailscale',
                tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
                preferred_tailnet_endpoint: null,
                tailscale_reachable: true,
                lan_base_url: 'http://192.168.1.50:4130',
                localhost_base_url: 'http://127.0.0.1:4130',
                ping_ms: 5,
                sync_status: 'ready',
                last_upstream_sync_at: null,
                last_downstream_sync_at: null,
                last_sync_error: null,
                incoming_linking_prompt: null,
                capacity: {
                  max_concurrency: 2,
                  current_load: 0,
                  available_concurrency: 2,
                },
              },
              {
                worker_id: 'worker_remote',
                node_id: 'node_remote',
                node_display_name: 'Remote Mac',
                client_kind: 'vel_macos',
                client_version: '0.1.0',
                protocol_version: '1',
                build_id: 'build_remote',
                worker_classes: ['sync'],
                capabilities: ['sync_bootstrap'],
                status: 'ok',
                queue_depth: 0,
                reachability: 'reachable',
                latency_class: 'low',
                compute_class: 'standard',
                power_class: 'ac_or_unknown',
                recent_failure_rate: 0,
                tailscale_preferred: true,
                last_heartbeat_at: 1_710_000_090,
                started_at: 1_710_000_000,
                sync_base_url: 'http://remote.tailnet.ts.net:4130',
                sync_transport: 'tailscale',
                tailscale_base_url: 'http://remote.tailnet.ts.net:4130',
                preferred_tailnet_endpoint: null,
                tailscale_reachable: true,
                lan_base_url: null,
                localhost_base_url: null,
                ping_ms: 14,
                sync_status: 'ready',
                last_upstream_sync_at: null,
                last_downstream_sync_at: null,
                last_sync_error: null,
                incoming_linking_prompt: null,
                capacity: {
                  max_concurrency: 2,
                  current_load: 0,
                  available_concurrency: 2,
                },
              },
            ],
          },
          meta: { request_id: 'req_cluster_workers' },
        } as never
      }
      if (path === '/v1/linking/status') {
        return {
          ok: true,
          data: [
            {
              node_id: 'vel-air',
              node_display_name: 'Vel Air',
              status: 'linked',
              scopes: {
                read_context: true,
                write_safe_actions: true,
                execute_repo_tasks: false,
              },
              linked_at: '2026-03-16T18:00:00Z',
              last_seen_at: '2026-03-16T18:15:00Z',
              transport_hint: 'tailscale',
              sync_base_url: 'http://vel-air.tailnet.ts.net:4130',
              tailscale_base_url: 'http://vel-air.tailnet.ts.net:4130',
              lan_base_url: 'http://192.168.1.70:4130',
              localhost_base_url: null,
              public_base_url: null,
            },
          ],
          meta: { request_id: 'req_linking_status' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: {
            google_calendar: {
              configured: true,
              connected: true,
              has_client_id: true,
              has_client_secret: true,
              calendars: [],
              all_calendars_selected: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: {
                title: 'Calendar has not synced yet',
                detail: 'Run a calendar sync to populate upcoming events and prep/commute context.',
                action: 'Sync now',
              },
            },
            todoist: {
              configured: true,
              connected: true,
              has_api_token: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            activity: {
              configured: true,
              source_path: '/tmp/activity.json',
              selected_paths: [],
              available_paths: ['/tmp/activity.json', '/home/test/.zsh_history'],
              internal_paths: ['var/integrations/activity/snapshot.json'],
              suggested_paths: ['/tmp/activity.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            health: {
              configured: true,
              source_path: '/tmp/health.json',
              selected_paths: [],
              suggested_paths: ['/tmp/health.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            git: {
              configured: true,
              source_path: '/tmp/git.json',
              selected_paths: ['/Users/test/code/vel'],
              available_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
              internal_paths: ['var/integrations/git/snapshot.json'],
              suggested_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            messaging: {
              configured: true,
              source_path: '/tmp/messaging.json',
              selected_paths: [],
              suggested_paths: ['/tmp/messaging.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            reminders: {
              configured: true,
              source_path: '/tmp/reminders.json',
              selected_paths: [],
              suggested_paths: ['/tmp/reminders.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            notes: {
              configured: true,
              source_path: '/tmp/notes',
              selected_paths: [],
              available_paths: ['/Users/test/Vault'],
              internal_paths: ['~/Library/Application Support/Vel/notes'],
              suggested_paths: ['/Users/test/Vault', '/tmp/notes'],
              source_kind: 'directory',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: {
                title: 'Local sync failed',
                detail: 'The notes source could not be read.',
                action: 'Fix the source and retry sync',
              },
            },
            transcripts: {
              configured: true,
              source_path: '/tmp/transcripts.json',
              selected_paths: [],
              suggested_paths: ['/tmp/transcripts.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
          },
          meta: { request_id: 'req_integrations' },
        } as never
      }
      if (path === '/v1/runs?limit=6') {
        return {
          ok: true,
          data: [
            {
              id: 'run_122',
              kind: 'search',
              status: 'failed',
              trace_id: 'trace_122',
              parent_run_id: null,
              automatic_retry_supported: false,
              automatic_retry_reason: 'search runs do not have an automatic retry executor',
              unsupported_retry_override: false,
              unsupported_retry_override_reason: null,
              created_at: '2026-03-16T21:55:00Z',
              started_at: null,
              finished_at: '2026-03-16T21:56:00Z',
              duration_ms: 60000,
              retry_scheduled_at: null,
              retry_reason: null,
              blocked_reason: null,
            },
            {
              id: 'run_123',
              kind: 'search',
              status: 'retry_scheduled',
              trace_id: 'trace_122',
              parent_run_id: 'run_122',
              automatic_retry_supported: false,
              automatic_retry_reason: 'search runs do not have an automatic retry executor',
              unsupported_retry_override: true,
              unsupported_retry_override_reason: 'manual operator override',
              created_at: '2026-03-16T22:00:00Z',
              started_at: null,
              finished_at: null,
              duration_ms: null,
              retry_scheduled_at: '2026-03-16T22:05:00Z',
              retry_reason: 'operator_override',
              blocked_reason: null,
          },
          ],
          meta: { request_id: 'req_runs' },
        } as never
      }
      if (path === '/api/components') {
        return {
          ok: true,
          data: [
            {
              id: 'google-calendar',
              name: 'Google Calendar',
              description: 'Calendar ingest',
              status: 'ok',
              last_restarted_at: 1_700_000_000,
              last_error: null,
              restart_count: 0,
            },
            {
              id: 'todoist',
              name: 'Todoist',
              description: 'Task ingest',
              status: 'ok',
              last_restarted_at: 1_700_000_000,
              last_error: null,
              restart_count: 2,
            },
            {
              id: 'evaluate',
              name: 'Evaluate',
              description: 'Evaluate all pipelines',
              status: 'idle',
              last_restarted_at: null,
              last_error: null,
              restart_count: 0,
            },
          ],
          meta: { request_id: 'req_components' },
        } as never
      }
      if (path === '/v1/loops') {
        return {
          ok: true,
          data: [
            {
              kind: 'evaluate_current_state',
              enabled: true,
              interval_seconds: 300,
              last_started_at: 1_710_000_000,
              last_finished_at: 1_710_000_030,
              last_status: 'success',
              last_error: null,
              next_due_at: 1_710_000_300,
            },
          ],
          meta: { request_id: 'req_loops' },
        } as never
      }
      if (path === '/v1/execution/handoffs?state=pending_review') {
        return {
          ok: true,
          data: [
            {
              id: 'handoff_1',
              project_id: 'proj_exec',
              origin_kind: 'human_to_agent',
              review_state: 'pending_review',
              handoff: {
                handoff: {
                  task_id: 'task_1',
                  trace_id: 'trace_1',
                  from_agent: 'operator',
                  to_agent: 'codex-local',
                  objective: 'Implement the next safe slice',
                  inputs: { project: 'vel' },
                  constraints: ['sidecar only'],
                  read_scopes: ['/tmp/vel', '/tmp/vel/notes'],
                  write_scopes: ['/tmp/vel'],
                  project_id: 'proj_exec',
                  task_kind: 'implementation',
                  agent_profile: 'quality',
                  token_budget: 'large',
                  review_gate: 'operator_approval',
                  repo_root: {
                    path: '/tmp/vel',
                    label: 'vel',
                    branch: 'main',
                    head_rev: 'abc123',
                  },
                  allowed_tools: ['rg', 'cargo test'],
                  capability_scope: {
                    read_scopes: ['/tmp/vel'],
                    write_scopes: ['/tmp/vel'],
                  },
                  deadline: null,
                  expected_output_schema: { artifacts: ['patch', 'summary'] },
                },
                project_id: 'proj_exec',
                task_kind: 'implementation',
                agent_profile: 'quality',
                token_budget: 'large',
                review_gate: 'operator_approval',
                repo: {
                  path: '/tmp/vel',
                  label: 'vel',
                  branch: 'main',
                  head_rev: 'abc123',
                },
                notes_root: {
                  path: '/tmp/vel/notes',
                  label: 'vel-notes',
                  kind: 'notes_root',
                },
                manifest_id: 'local-codex',
              },
              routing: {
                task_kind: 'implementation',
                agent_profile: 'quality',
                token_budget: 'large',
                review_gate: 'operator_approval',
                read_scopes: ['/tmp/vel', '/tmp/vel/notes'],
                write_scopes: ['/tmp/vel'],
                allowed_tools: ['rg', 'cargo test'],
                reasons: [
                  {
                    code: 'write_scope_requires_approval',
                    message: 'write scopes require explicit operator approval before launch',
                  },
                ],
              },
              manifest_id: 'local-codex',
              requested_by: 'operator_shell',
              reviewed_by: null,
              decision_reason: null,
              reviewed_at: null,
              launched_at: null,
              created_at: '2026-03-18T18:00:00Z',
              updated_at: '2026-03-18T18:00:00Z',
            },
          ],
          meta: { request_id: 'req_handoffs' },
        } as never
      }
      if (path === '/api/components/evaluate/logs?limit=50') {
        return {
          ok: true,
          data: [
            {
              id: 'log_eval_1',
              component_id: 'evaluate',
              event_name: 'component.restart.requested',
              status: 'running',
              message: 'component restart requested',
              payload: {},
              created_at: 1_700_000_100,
            },
            {
              id: 'log_eval_2',
              component_id: 'evaluate',
              event_name: 'component.restart.completed',
              status: 'success',
              message: 'Evaluate complete',
              payload: {},
              created_at: 1_700_000_200,
            },
          ],
          meta: { request_id: 'req_component_logs' },
        } as never
      }
      if (path === '/api/integrations/notes/logs?limit=10') {
        return {
          ok: true,
          data: [
            {
              id: 'int_notes_1',
              integration_id: 'notes',
              event_name: 'integration.sync.succeeded',
              status: 'ok',
              message: 'notes sync completed with 4 items.',
              payload: { item_count: 4 },
              created_at: 1_700_000_400,
            },
          ],
          meta: { request_id: 'req_notes_logs' },
        } as never
      }
      if (path === '/api/integrations/todoist/logs?limit=10') {
        return {
          ok: true,
          data: [
            {
              id: 'int_todoist_1',
              integration_id: 'todoist',
              event_name: 'integration.sync.failed',
              status: 'error',
              message: 'todoist sync failed: upstream 500',
              payload: { error: 'upstream 500' },
              created_at: 1_700_000_500,
            },
          ],
          meta: { request_id: 'req_todoist_logs' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })
    vi.mocked(client.apiPatch).mockResolvedValue({} as never)
    vi.mocked(client.apiPost).mockResolvedValue({ ok: true } as never)
  })

  afterAll(() => {
    Object.defineProperty(window.navigator, 'platform', {
      configurable: true,
      value: originalNavigatorPlatform,
    })
    Object.defineProperty(window.navigator, 'userAgent', {
      configurable: true,
      value: originalNavigatorUserAgent,
    })
  })

  it('shows Back button and Settings heading when loaded', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    expect(screen.getByText(/loading settings/i)).toBeInTheDocument()
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('button', { name: /back/i })).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    expect(within(root).getByRole('heading', { name: /settings/i })).toBeInTheDocument()
    expect(within(root).getByText(/advanced operator setup, trust summaries, and runtime detail live here/i)).toBeInTheDocument()
  })

  it('renders checkboxes for disable_proactive, toggle_risks, toggle_reminders', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByDisplayValue('America/Denver')).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    expect(within(root).getByRole('heading', { name: /advanced operator setup/i })).toBeInTheDocument()
    expect(within(root).getByText(/disable proactive/i)).toBeInTheDocument()
    expect(within(root).getByText(/show risks/i)).toBeInTheDocument()
    expect(within(root).getByText(/show reminders/i)).toBeInTheDocument()
    expect(within(root).getByDisplayValue('America/Denver')).toBeInTheDocument()
    expect(within(root).getByDisplayValue('Vel Desktop')).toBeInTheDocument()
  })

  it('calls onBack when Back is clicked', async () => {
    const onBack = vi.fn()
    const { container } = render(<SettingsPage onBack={onBack} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('button', { name: /back/i })).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    fireEvent.click(within(root).getByRole('button', { name: /back/i }))
    expect(onBack).toHaveBeenCalledTimes(1)
  })

  it('calls apiPatch when a checkbox is toggled', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/show risks/i)).toBeInTheDocument()
    })
    const root = getSettingsRoot(container)
    const risksCheckbox = within(root).getByRole('checkbox', { name: /show risks/i })
    fireEvent.click(risksCheckbox)
    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/settings',
        { toggle_risks: false },
        expect.any(Function),
      )
    })
  })

  it('saves timezone changes through settings api', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByDisplayValue('America/Denver')).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    const input = within(root).getByPlaceholderText('America/Denver')
    fireEvent.change(input, { target: { value: 'America/Los_Angeles' } })
    fireEvent.click(within(root).getByRole('button', { name: /save timezone/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/settings',
        { timezone: 'America/Los_Angeles' },
        expect.any(Function),
      )
    })
  })

  it('saves cross-client sync settings through settings api', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: /cross-client sync/i })).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    fireEvent.change(within(root).getByDisplayValue('Vel Desktop'), { target: { value: 'Vel NAS' } })
    expect(
      within(root).getByDisplayValue('http://vel-desktop.tailnet.ts.net:4130'),
    ).toBeDisabled()
    fireEvent.click(within(root).getByRole('button', { name: /save sync settings/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/settings',
        {
          node_display_name: 'Vel NAS',
          writeback_enabled: false,
          tailscale_preferred: true,
        },
        expect.any(Function),
      )
    })
  })

  it('explains when the tailscale url is auto-discovered and locked', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/auto-discovered from the local tailscale daemon/i)).toBeInTheDocument()
      expect(within(root).getByText(/auto-discovered from the local network interfaces/i)).toBeInTheDocument()
      expect(within(root).getByDisplayValue('http://192.168.1.50:4130')).toBeDisabled()
    })
  })

  it('renders active adaptive policy overrides in general settings', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByText(/adaptive policy overrides/i)).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    expect(within(root).getByText('30 min')).toBeInTheDocument()
    expect(within(root).getByText('45 min')).toBeInTheDocument()
    expect(within(root).getByText(/from increase commute buffer/i)).toBeInTheDocument()
    expect(within(root).getByText(/from increase prep window/i)).toBeInTheDocument()
  })

  it('renders the backend-owned backup trust card in general settings', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: /backup trust/i })).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    expect(within(root).getByText('Backup trust needs attention')).toBeInTheDocument()
    expect(within(root).getByText('stale (50h old)')).toBeInTheDocument()
    expect(within(root).getByText('/tmp/backups/bkp_123')).toBeInTheDocument()
    expect(within(root).getByText('Artifacts: 2 included, 1 omitted')).toBeInTheDocument()
    expect(within(root).getByText('Config: 2 included, 1 omitted')).toBeInTheDocument()
    expect(within(root).getAllByText('vel backup create').length).toBeGreaterThan(0)
    expect(within(root).getByText('vel backup inspect <backup_root>')).toBeInTheDocument()
    expect(within(root).getByText('vel backup verify <backup_root>')).toBeInTheDocument()
  })

  it('renders documentation entrypoints in general settings', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: 'Documentation' })).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    expect(within(root).getByText('Core documentation')).toBeInTheDocument()
    expect(within(root).getByText('Your Vel documentation')).toBeInTheDocument()
    expect(within(root).getByText('Open the user docs when you need operator steps. Open core docs when you need contract or architecture truth.')).toBeInTheDocument()
    expect(within(root).getByText('Contract and implementation authority')).toBeInTheDocument()
    expect(within(root).getByText('Day-to-day setup and recovery')).toBeInTheDocument()
    expect(within(root).getByText('docs/README.md')).toBeInTheDocument()
    expect(within(root).getByText('docs/MASTER_PLAN.md')).toBeInTheDocument()
    expect(within(root).queryByText('docs/status.md')).not.toBeInTheDocument()
    expect(within(root).queryByText('docs/architecture.md')).not.toBeInTheDocument()
    expect(within(root).getByText('docs/user/README.md')).toBeInTheDocument()
    expect(within(root).getByText('docs/user/quickstart.md')).toBeInTheDocument()
  })

  it('renders onboarding next steps from existing linking and integration payloads', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      const root = getSettingsRoot(container)
      expect(within(root).getByRole('heading', { name: 'Onboarding and recovery' })).toBeInTheDocument()
    })

    const root = getSettingsRoot(container)
    expect(within(root).getByText(/follow the next unfinished step instead of reverse-engineering diagnostics/i)).toBeInTheDocument()
    expect(within(root).getByText(/Next action/i)).toBeInTheDocument()
    expect(within(root).getByText('Reach the daemon')).toBeInTheDocument()
    expect(within(root).getByText('Link a companion device')).toBeInTheDocument()
    expect(within(root).getByText('Confirm local source paths')).toBeInTheDocument()
    expect(within(root).getByText('Validate Apple and macOS export paths')).toBeInTheDocument()
    expect(within(root).getAllByText('docs/user/setup.md').length).toBeGreaterThan(0)
    expect(within(root).getByText('docs/api/runtime.md')).toBeInTheDocument()
    expect(within(root).getByText('docs/user/integrations/apple-macos.md')).toBeInTheDocument()
  })

  it('keeps todoist sync active while google credential save is pending', async () => {
    const googleSave = createDeferred<unknown>()
    vi.mocked(client.apiPatch).mockImplementationOnce(() => googleSave.promise as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'todoist', signals_ingested: 0 },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    const googleClientIdInput = within(root).getByLabelText(/client id/i)
    const googleClientSecretInput = within(root).getByLabelText(/client secret/i)
    fireEvent.change(googleClientIdInput as HTMLElement, { target: { value: 'client-id' } })
    fireEvent.change(googleClientSecretInput as HTMLElement, { target: { value: 'client-secret' } })

    fireEvent.click(within(root).getByRole('button', { name: /save credentials/i }))

    await waitFor(() => {
      expect(within(root).getByRole('button', { name: /saving…/i })).toBeInTheDocument()
    })

    const todoistSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Todoist'),
    )
    expect(todoistSyncButton).toBeDefined()
    expect(todoistSyncButton).not.toBeDisabled()

    fireEvent.click(todoistSyncButton as HTMLElement)
    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/todoist', {}, expect.any(Function))
    })

    googleSave.resolve({ ok: true })
  })

  it('keeps unrelated integration feedback when actions finish out of order', async () => {
    const googleSave = createDeferred<unknown>()
    vi.mocked(client.apiPatch).mockImplementationOnce(() => googleSave.promise as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'todoist', signals_ingested: 0 },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    fireEvent.click(within(root).getByRole('button', { name: /save credentials/i }))
    await waitFor(() => {
      expect(within(root).getByRole('button', { name: /saving…/i })).toBeInTheDocument()
    })

    const todoistSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Todoist'),
    )
    expect(todoistSyncButton).toBeDefined()
    fireEvent.click(todoistSyncButton as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
    })

    googleSave.resolve({ ok: true })

    await waitFor(() => {
      expect(within(root).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    })

    const googleCard = within(root).getByRole('heading', { name: /google calendar/i }).closest('.rounded-lg')
    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(googleCard).not.toBeNull()
    expect(todoistCard).not.toBeNull()
    expect(within(googleCard as HTMLElement).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    expect(within(todoistCard as HTMLElement).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
  })

  it('renders integration feedback on the matching integration card', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({ ok: true } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    fireEvent.click(within(root).getByRole('button', { name: /save credentials/i }))

    await waitFor(() => {
      expect(within(root).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    })

    const googleCard = within(root).getByRole('heading', { name: /google calendar/i }).closest('.rounded-lg')
    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(googleCard).not.toBeNull()
    expect(todoistCard).not.toBeNull()
    expect(within(googleCard as HTMLElement).getByText('Google Calendar credentials saved.')).toBeInTheDocument()
    expect(within(todoistCard as HTMLElement).queryByText('Google Calendar credentials saved.')).toBeNull()
  })

  it('keeps multiple todoist action messages on the todoist card', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({ ok: true } as never)
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'todoist', signals_ingested: 0 },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    fireEvent.click(within(root).getByRole('button', { name: /save token/i }))
    await waitFor(() => {
      expect(within(root).getByText('Todoist token saved.')).toBeInTheDocument()
    })

    const todoistSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Todoist'),
    )
    expect(todoistSyncButton).toBeDefined()
    fireEvent.click(todoistSyncButton as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
    })

    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(todoistCard).not.toBeNull()
    expect(within(todoistCard as HTMLElement).getByText('Todoist token saved.')).toBeInTheDocument()
    expect(within(todoistCard as HTMLElement).getByText('Todoist synced (0 signals).')).toBeInTheDocument()
  })

  it('renders local integration cards and syncs the selected local adapter', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    expect(within(root).getByRole('heading', { name: /computer activity/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /git activity/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /^messaging$/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /apple reminders/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /obsidian vault/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /transcripts/i })).toBeInTheDocument()
    expect(within(root).getByText('Source: /tmp/activity.json')).toBeInTheDocument()
    expect(within(root).getByText('Selected repos')).toBeInTheDocument()
    expect(within(root).getAllByText('/Users/test/code/vel').length).toBeGreaterThan(0)
    expect(within(root).getByText('Obsidian vault')).toBeInTheDocument()
    expect(within(root).getByText('Zsh shell history')).toBeInTheDocument()
    expect(within(root).getByText('/Users/test/Vault')).toBeInTheDocument()
    expect(within(root).getByText('/home/test/.zsh_history')).toBeInTheDocument()
    expect(within(root).getAllByText('Operator path selection').length).toBeGreaterThan(0)
    expect(within(root).getAllByText('Internal/default paths (read only)').length).toBeGreaterThan(0)
    expect(within(root).getAllByText('docs/user/integrations/local-sources.md').length).toBeGreaterThan(0)
    expect(within(root).getAllByText('docs/user/integrations/apple-macos.md').length).toBeGreaterThan(0)
    expect(within(root).getAllByText('Vel notes path').length).toBeGreaterThan(0)
    expect(within(root).queryByText('Remote Mac')).not.toBeInTheDocument()

    const notesSyncButton = within(root).getAllByRole('button', { name: /sync now/i }).find((button) =>
      button.closest('.rounded-lg')?.textContent?.includes('Obsidian Vault'),
    )
    expect(notesSyncButton).toBeDefined()
    fireEvent.click(notesSyncButton as HTMLElement)

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/notes', {}, expect.any(Function))
    })
    expect(within(root).getByText('Obsidian Vault synced (0 signals).')).toBeInTheDocument()
  })

  it('shows integration sync history for the selected card', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    const notesCard = within(root).getByRole('heading', { name: /obsidian vault/i }).closest('.rounded-lg')
    expect(notesCard).not.toBeNull()
    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /^show history$/i }))

    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('Recent sync history')).toBeInTheDocument()
    })
    expect(within(notesCard as HTMLElement).getByText('notes sync completed with 4 items.')).toBeInTheDocument()
    expect(within(notesCard as HTMLElement).getByText('integration.sync.succeeded')).toBeInTheDocument()
    expect(within(notesCard as HTMLElement).getByText('Items: 4')).toBeInTheDocument()

    const todoistCard = within(root).getByRole('heading', { name: /todoist/i }).closest('.rounded-lg')
    expect(todoistCard).not.toBeNull()
    fireEvent.click(within(todoistCard as HTMLElement).getByRole('button', { name: /show history/i }))

    await waitFor(() => {
      expect(within(todoistCard as HTMLElement).getByText('todoist sync failed: upstream 500')).toBeInTheDocument()
    })
    expect(within(todoistCard as HTMLElement).getByText('Error: upstream 500')).toBeInTheDocument()

    fireEvent.click(within(notesCard as HTMLElement).getByRole('checkbox', { name: /failures only/i }))
    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('No failed syncs in recent history.')).toBeInTheDocument()
    })

    fireEvent.click(within(todoistCard as HTMLElement).getByRole('checkbox', { name: /failures only/i }))
    expect(within(todoistCard as HTMLElement).queryByText('No failed syncs in recent history.')).toBeNull()
    expect(within(todoistCard as HTMLElement).getByText('todoist sync failed: upstream 500')).toBeInTheDocument()
  })

  it('renders guidance callouts for degraded integrations', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    expect(within(root).getByText('Calendar has not synced yet')).toBeInTheDocument()
    expect(within(root).getByText(/Run a calendar sync to populate upcoming events/i)).toBeInTheDocument()
    expect(within(root).getByText(/Recommended action: Sync now/i)).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /run: sync now/i })).toBeInTheDocument()
    expect(within(root).getByText('Local sync failed')).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /retry sync/i })).toBeInTheDocument()
  })

  it('runs the recommended google guidance action from the guidance card', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'calendar', signals_ingested: 3 },
      meta: { request_id: 'req_sync_calendar' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const googleCard = within(root).getByRole('heading', { name: /google calendar/i }).closest('.rounded-lg')
    expect(googleCard).not.toBeNull()

    fireEvent.click(within(googleCard as HTMLElement).getByRole('button', { name: /run: sync now/i }))

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/calendar', {}, expect.any(Function))
    })
    expect(within(googleCard as HTMLElement).getByText('Calendar synced (3 signals).')).toBeInTheDocument()
  })

  it('opens history and retries local sync from guidance actions', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source: 'notes', signals_ingested: 4 },
      meta: { request_id: 'req_sync_notes' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const notesCard = within(root).getByRole('heading', { name: /obsidian vault/i }).closest('.rounded-lg')
    expect(notesCard).not.toBeNull()

    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /open history/i }))
    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('Recent sync history')).toBeInTheDocument()
    })

    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /retry sync/i }))

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith('/v1/sync/notes', {}, expect.any(Function))
    })
    expect(within(notesCard as HTMLElement).getByText('Obsidian Vault synced (4 signals).')).toBeInTheDocument()
  })

  it('chooses a local integration path from the native dialog action', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: { source_path: '/Users/test/Vault' },
      meta: { request_id: 'req_path_dialog' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const notesCard = within(root).getByRole('heading', { name: /obsidian vault/i }).closest('.rounded-lg')
    expect(notesCard).not.toBeNull()

    fireEvent.click(within(notesCard as HTMLElement).getByRole('button', { name: /choose vault/i }))

    await waitFor(() => {
      expect(client.apiPost).toHaveBeenCalledWith(
        '/api/integrations/notes/path-dialog',
        {},
        expect.any(Function),
      )
    })
    expect(
      within(notesCard as HTMLElement).getByDisplayValue('/Users/test/Vault'),
    ).toBeInTheDocument()
    expect(within(notesCard as HTMLElement).getByText('Path selected. Save to apply it.')).toBeInTheDocument()
  })

  it('shows linked macos client paths only after expanding the client host section', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const notesCard = within(root).getByRole('heading', { name: /obsidian vault/i }).closest('.rounded-lg')
    expect(notesCard).not.toBeNull()

    expect(within(notesCard as HTMLElement).queryByText('Remote Mac')).not.toBeInTheDocument()
    fireEvent.click(within(notesCard as HTMLElement).getByText(/other client hosts/i))

    await waitFor(() => {
      expect(within(notesCard as HTMLElement).getByText('Vel Air')).toBeInTheDocument()
    })
    expect(within(notesCard as HTMLElement).getAllByText('Linked macOS client').length).toBeGreaterThan(0)
    expect(within(notesCard as HTMLElement).getAllByText('Vel notes path').length).toBeGreaterThan(0)
  })

  it('allows selecting multiple discovered paths on the same integration card', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const activityCard = within(root).getByRole('heading', { name: /computer activity/i }).closest('.rounded-lg')
    expect(activityCard).not.toBeNull()

    const zshOption = within(activityCard as HTMLElement).getByRole('button', { name: /zsh shell history/i })
    const snapshotOption = within(activityCard as HTMLElement).getByText('/tmp/activity.json').closest('button')

    expect(snapshotOption).not.toBeNull()
    expect(snapshotOption as HTMLElement).toHaveAttribute('aria-pressed', 'true')
    fireEvent.click(zshOption)

    expect(zshOption).toHaveAttribute('aria-pressed', 'true')
    expect(snapshotOption as HTMLElement).toHaveAttribute('aria-pressed', 'true')
    expect(within(activityCard as HTMLElement).getByText('2 selected')).toBeInTheDocument()
  })

  it('saves selected local git repos through the integration settings route', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        google_calendar: {
          configured: true,
          connected: true,
          has_client_id: true,
          has_client_secret: true,
          calendars: [],
          all_calendars_selected: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: {
          configured: true,
          source_path: '/tmp/activity.json',
          selected_paths: [],
          available_paths: ['/tmp/activity.json', '/home/test/.zsh_history'],
          internal_paths: ['var/integrations/activity/snapshot.json'],
          suggested_paths: ['/tmp/activity.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        health: {
          configured: true,
          source_path: '/tmp/health.json',
          selected_paths: [],
          available_paths: ['/tmp/health.json'],
          internal_paths: [],
          suggested_paths: ['/tmp/health.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        git: {
          configured: true,
          source_path: '/tmp/git.json',
          selected_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
          available_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
          internal_paths: ['var/integrations/git/snapshot.json'],
          suggested_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        messaging: {
          configured: true,
          source_path: '/tmp/messaging.json',
          selected_paths: [],
          available_paths: ['/tmp/messaging.json'],
          internal_paths: [],
          suggested_paths: ['/tmp/messaging.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        reminders: {
          configured: true,
          source_path: '/tmp/reminders.json',
          selected_paths: [],
          available_paths: ['/tmp/reminders.json'],
          internal_paths: [],
          suggested_paths: ['/tmp/reminders.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        notes: {
          configured: true,
          source_path: '/tmp/notes',
          selected_paths: [],
          available_paths: ['/Users/test/Vault'],
          internal_paths: ['~/Library/Application Support/Vel/notes'],
          suggested_paths: ['/Users/test/Vault', '/tmp/notes'],
          source_kind: 'directory',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        transcripts: {
          configured: true,
          source_path: '/tmp/transcripts.json',
          selected_paths: [],
          available_paths: ['/tmp/transcripts.json'],
          internal_paths: [],
          suggested_paths: ['/tmp/transcripts.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
      },
      meta: { request_id: 'req_git_selection_save' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const gitCard = within(root).getByRole('heading', { name: /git activity/i }).closest('.rounded-lg')
    expect(gitCard).not.toBeNull()

    fireEvent.click(within(gitCard as HTMLElement).getByRole('button', { name: /git repo: other/i }))
    fireEvent.click(within(gitCard as HTMLElement).getByRole('button', { name: /save repo selection/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/integrations/git/source',
        {
          source_path: '/tmp/git.json',
          selected_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
        },
        expect.any(Function),
      )
    })
    expect(within(gitCard as HTMLElement).getByText('Repo selection saved.')).toBeInTheDocument()
  })

  it('hides apple-only integrations on non-apple hosts by default', async () => {
    Object.defineProperty(window.navigator, 'platform', {
      configurable: true,
      value: 'Linux x86_64',
    })
    Object.defineProperty(window.navigator, 'userAgent', {
      configurable: true,
      value: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36',
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)

    expect(within(root).queryByRole('heading', { name: /health/i })).not.toBeInTheDocument()
    expect(within(root).queryByRole('heading', { name: /apple reminders/i })).not.toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /computer activity/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /^messaging$/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /obsidian vault/i })).toBeInTheDocument()
  })

  it('saves a local integration source path from the settings card', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        google_calendar: {
          configured: true,
          connected: true,
          has_client_id: true,
          has_client_secret: true,
          calendars: [],
          all_calendars_selected: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: {
          configured: true,
          source_path: '/tmp/fresh-activity.json',
          suggested_paths: ['/tmp/fresh-activity.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        health: {
          configured: true,
          source_path: '/tmp/health.json',
          suggested_paths: ['/tmp/health.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        git: {
          configured: true,
          source_path: '/tmp/git.json',
          suggested_paths: ['/tmp/git.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        messaging: {
          configured: true,
          source_path: '/tmp/messaging.json',
          suggested_paths: ['/tmp/messaging.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        reminders: {
          configured: true,
          source_path: '/tmp/reminders.json',
          suggested_paths: ['/tmp/reminders.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        notes: {
          configured: true,
          source_path: '/tmp/notes',
          suggested_paths: ['/Users/test/Vault', '/tmp/notes'],
          source_kind: 'directory',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        transcripts: {
          configured: true,
          source_path: '/tmp/transcripts.json',
          suggested_paths: ['/tmp/transcripts.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
      },
      meta: { request_id: 'req_local_source_save' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const activityCard = within(root).getByRole('heading', { name: /computer activity/i }).closest('.rounded-lg')
    expect(activityCard).not.toBeNull()

    fireEvent.change(
      within(activityCard as HTMLElement).getByPlaceholderText(/path to local snapshot file or directory/i),
      { target: { value: '/tmp/fresh-activity.json' } },
    )
    fireEvent.click(within(activityCard as HTMLElement).getByRole('button', { name: /^save path$/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/integrations/activity/source',
        { source_path: '/tmp/fresh-activity.json' },
        expect.any(Function),
      )
    })
    expect(within(activityCard as HTMLElement).getByText('Source path saved.')).toBeInTheDocument()
    expect(within(activityCard as HTMLElement).getByDisplayValue('/tmp/fresh-activity.json')).toBeInTheDocument()
  })

  it('uses guidance actions to focus and save a missing local source path', async () => {
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/settings') {
        return {
          ok: true,
          data: {
            disable_proactive: false,
            toggle_risks: true,
            toggle_reminders: true,
            timezone: 'America/Denver',
            writeback_enabled: false,
          },
          meta: { request_id: 'req_1' },
        } as never
      }
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            summary: {
              mode: { key: 'focus', label: 'Focus' },
              phase: { key: 'engaged', label: 'Engaged' },
              meds: { key: 'ok', label: 'OK' },
              risk: { level: 'low', score: 0.2, label: 'low' },
            },
            schedule: {
              empty_message: null,
              next_event: null,
              upcoming_events: [],
            },
            tasks: {
              todoist: [],
              other_open: [],
              next_commitment: null,
            },
            attention: {
              state: { key: 'on_task', label: 'On task' },
              drift: { key: 'none', label: 'None' },
              severity: { key: 'none', label: 'None' },
              confidence: null,
              reasons: [],
            },
            sources: {
              git_activity: null,
              health: null,
              mood: null,
              pain: null,
              note_document: null,
              assistant_message: null,
            },
            freshness: {
              overall_status: 'fresh',
              sources: [],
            },
            action_items: [],
            review_snapshot: {
              open_action_count: 0,
              triage_count: 0,
              projects_needing_review: 0,
            },
            pending_writebacks: [],
            conflicts: [],
            people: [],
            reasons: [],
            debug: {
              raw_context: {},
              signals_used: [],
              commitments_used: [],
              risk_used: [],
            },
          },
          meta: { request_id: 'req_now' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: {
            google_calendar: {
              configured: true,
              connected: true,
              has_client_id: true,
              has_client_secret: true,
              calendars: [],
              all_calendars_selected: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            todoist: {
              configured: true,
              connected: true,
              has_api_token: true,
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            activity: {
              configured: false,
              source_path: null,
              suggested_paths: ['/tmp/activity.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: {
                title: 'Local source missing',
                detail: 'Configure a source path for this local adapter before syncing it.',
                action: 'Set source path',
              },
            },
            health: {
              configured: true,
              source_path: '/tmp/health.json',
              suggested_paths: ['/tmp/health.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            git: {
              configured: true,
              source_path: '/tmp/git.json',
              suggested_paths: ['/tmp/git.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            messaging: {
              configured: true,
              source_path: '/tmp/messaging.json',
              suggested_paths: ['/tmp/messaging.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            reminders: {
              configured: true,
              source_path: '/tmp/reminders.json',
              suggested_paths: ['/tmp/reminders.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            notes: {
              configured: true,
              source_path: '/tmp/notes',
              suggested_paths: ['/Users/test/Vault', '/tmp/notes'],
              source_kind: 'directory',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
            transcripts: {
              configured: true,
              source_path: '/tmp/transcripts.json',
              suggested_paths: ['/tmp/transcripts.json'],
              source_kind: 'file',
              last_sync_at: null,
              last_sync_status: null,
              last_error: null,
              last_item_count: null,
              guidance: null,
            },
          },
          meta: { request_id: 'req_integrations_local_missing' },
        } as never
      }
      if (path === '/v1/runs?limit=6') {
        return { ok: true, data: [], meta: { request_id: 'req_runs' } } as never
      }
      if (path === '/v1/agent/inspect') {
        return {
          ok: true,
          data: {
            grounding: {
              generated_at: 1710000000,
              now: {
                computed_at: 1710000000,
                timezone: 'America/Denver',
                summary: {
                  mode: { key: 'focus', label: 'Focus' },
                  phase: { key: 'engaged', label: 'Engaged' },
                  meds: { key: 'ok', label: 'OK' },
                  risk: { level: 'low', score: 0.2, label: 'low' },
                },
                schedule: { empty_message: null, next_event: null, upcoming_events: [] },
                tasks: { todoist: [], other_open: [], next_commitment: null },
                attention: {
                  state: { key: 'on_task', label: 'On task' },
                  drift: { key: 'none', label: 'None' },
                  severity: { key: 'none', label: 'None' },
                  confidence: null,
                  reasons: [],
                },
                sources: { git_activity: null, health: null, mood: null, pain: null, note_document: null, assistant_message: null },
                freshness: { overall_status: 'fresh', sources: [] },
                action_items: [],
                review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0 },
                pending_writebacks: [],
                conflicts: [],
                people: [],
                reasons: [],
                debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
              },
              current_context: null,
              projects: [],
              people: [],
              commitments: [],
              review: { review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0 }, pending_writebacks: [], conflicts: [], pending_execution_handoffs: [] },
            },
            capabilities: { groups: [] },
            blockers: [],
            explainability: { persisted_record_kinds: ['now'], supporting_paths: ['/v1/agent/inspect'], raw_context_json_supporting_only: true },
          },
          meta: { request_id: 'req_agent_inspect_local_missing' },
        } as never
      }
      return { ok: true, data: [], meta: { request_id: 'req_default' } } as never
    })
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        google_calendar: {
          configured: true,
          connected: true,
          has_client_id: true,
          has_client_secret: true,
          calendars: [],
          all_calendars_selected: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        todoist: {
          configured: true,
          connected: true,
          has_api_token: true,
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        activity: {
          configured: true,
          source_path: '/tmp/activity.json',
          suggested_paths: ['/tmp/activity.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        health: {
          configured: true,
          source_path: '/tmp/health.json',
          suggested_paths: ['/tmp/health.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        git: {
          configured: true,
          source_path: '/tmp/git.json',
          suggested_paths: ['/tmp/git.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        messaging: {
          configured: true,
          source_path: '/tmp/messaging.json',
          suggested_paths: ['/tmp/messaging.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        reminders: {
          configured: true,
          source_path: '/tmp/reminders.json',
          suggested_paths: ['/tmp/reminders.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        notes: {
          configured: true,
          source_path: '/tmp/notes',
          suggested_paths: ['/Users/test/Vault', '/tmp/notes'],
          source_kind: 'directory',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
        transcripts: {
          configured: true,
          source_path: '/tmp/transcripts.json',
          suggested_paths: ['/tmp/transcripts.json'],
          source_kind: 'file',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
      },
      meta: { request_id: 'req_local_source_guidance_save' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openIntegrationsTab(container)
    const activityCard = within(root).getByRole('heading', { name: /computer activity/i }).closest('.rounded-lg')
    expect(activityCard).not.toBeNull()

    fireEvent.click(within(activityCard as HTMLElement).getByRole('button', { name: /edit source path/i }))
    const input = within(activityCard as HTMLElement).getByPlaceholderText(/path to local snapshot file or directory/i)
    expect(input).toHaveFocus()

    fireEvent.change(input, { target: { value: '/tmp/activity.json' } })
    fireEvent.click(within(activityCard as HTMLElement).getByRole('button', { name: /^save path$/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/api/integrations/activity/source',
        { source_path: '/tmp/activity.json' },
        expect.any(Function),
      )
    })
    expect(within(activityCard as HTMLElement).getByText('Source path saved.')).toBeInTheDocument()
  })

  it('renders component cards in the components tab', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    expect(within(root).getByRole('heading', { name: /google calendar/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /todoist/i })).toBeInTheDocument()
    expect(within(root).getByRole('heading', { name: /evaluate/i })).toBeInTheDocument()
    expect(within(root).getByText('Calendar ingest')).toBeInTheDocument()
    expect(within(root).getAllByText(/Restarts: 0/)).toHaveLength(2)
  })

  it('renders pending execution handoff review and approves it from runtime controls', async () => {
    vi.mocked(client.apiPost).mockImplementation(async (path: string) => {
      if (path === '/v1/execution/handoffs/handoff_1/approve') {
        return {
          ok: true,
          data: {
            id: 'handoff_1',
            project_id: 'proj_exec',
            origin_kind: 'human_to_agent',
            review_state: 'approved',
            handoff: {
              handoff: {
                task_id: 'task_1',
                trace_id: 'trace_1',
                from_agent: 'operator',
                to_agent: 'codex-local',
                objective: 'Implement the next safe slice',
                inputs: { project: 'vel' },
                constraints: ['sidecar only'],
                read_scopes: ['/tmp/vel', '/tmp/vel/notes'],
                write_scopes: ['/tmp/vel'],
                project_id: 'proj_exec',
                task_kind: 'implementation',
                agent_profile: 'quality',
                token_budget: 'large',
                review_gate: 'operator_approval',
                repo_root: {
                  path: '/tmp/vel',
                  label: 'vel',
                  branch: 'main',
                  head_rev: 'abc123',
                },
                allowed_tools: ['rg', 'cargo test'],
                capability_scope: {
                  read_scopes: ['/tmp/vel'],
                  write_scopes: ['/tmp/vel'],
                },
                deadline: null,
                expected_output_schema: { artifacts: ['patch', 'summary'] },
              },
              project_id: 'proj_exec',
              task_kind: 'implementation',
              agent_profile: 'quality',
              token_budget: 'large',
              review_gate: 'operator_approval',
              repo: {
                path: '/tmp/vel',
                label: 'vel',
                branch: 'main',
                head_rev: 'abc123',
              },
              notes_root: {
                path: '/tmp/vel/notes',
                label: 'vel-notes',
                kind: 'notes_root',
              },
              manifest_id: 'local-codex',
            },
            routing: {
              task_kind: 'implementation',
              agent_profile: 'quality',
              token_budget: 'large',
              review_gate: 'operator_approval',
              read_scopes: ['/tmp/vel', '/tmp/vel/notes'],
              write_scopes: ['/tmp/vel'],
              allowed_tools: ['rg', 'cargo test'],
              reasons: [
                {
                  code: 'write_scope_requires_approval',
                  message: 'write scopes require explicit operator approval before launch',
                },
              ],
            },
            manifest_id: 'local-codex',
            requested_by: 'operator_shell',
            reviewed_by: 'operator_shell',
            decision_reason: 'Approved from runtime review queue.',
            reviewed_at: '2026-03-18T18:05:00Z',
            launched_at: null,
            created_at: '2026-03-18T18:00:00Z',
            updated_at: '2026-03-18T18:05:00Z',
          },
          meta: { request_id: 'req_handoff_approve' },
        } as never
      }
      return { ok: true } as never
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    await waitFor(() => {
      expect(within(root).getByText('Execution handoff review')).toBeInTheDocument()
    })
    expect(within(root).getByText('Pending execution review')).toBeInTheDocument()
    expect(within(root).getByText('Implement the runtime review queue')).toBeInTheDocument()
    expect(within(root).getByText('write_scope_requires_approval')).toBeInTheDocument()

    fireEvent.click(within(root).getByRole('button', { name: 'Approve' }))

    await waitFor(() => {
      expect(within(root).getByText('Execution handoff approved.')).toBeInTheDocument()
    })
    expect(client.apiPost).toHaveBeenCalledWith(
      '/v1/execution/handoffs/xho_1/approve',
      expect.objectContaining({ reviewed_by: 'operator_shell' }),
      expect.any(Function),
    )
  })

  it('renders agent grounding scope and blocker guidance from the backend inspect payload', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = getSettingsRoot(container)

    await waitFor(() => {
      expect(within(root).getByText('Agent grounding')).toBeInTheDocument()
    })
    expect(within(root).getByText('Projects in scope')).toBeInTheDocument()
    expect(within(root).getByText('Request integration writeback')).toBeInTheDocument()
    expect(within(root).getByText(/SAFE MODE keeps writeback disabled/i)).toBeInTheDocument()
    expect(within(root).getByText(/Enable writeback in Settings before retrying/i)).toBeInTheDocument()
  })

  it('expands component logs and shows restart event history', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    fireEvent.click(within(evaluateCard as HTMLElement).getByRole('button', { name: /show logs/i }))

    await waitFor(() => {
      expect(within(evaluateCard as HTMLElement).getByText('Recent logs')).toBeInTheDocument()
    })
    expect(within(evaluateCard as HTMLElement).getByText('component.restart.requested')).toBeInTheDocument()
    expect(within(evaluateCard as HTMLElement).getByText('component.restart.completed')).toBeInTheDocument()
  })

  it('restarts a component and shows success feedback', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'evaluate',
        name: 'Evaluate',
        description: 'Evaluate all pipelines',
        status: 'ok',
        last_restarted_at: 1_700_000_300,
        last_error: null,
        restart_count: 3,
      },
      meta: { request_id: 'req_restart_eval' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    fireEvent.click(within(evaluateCard as HTMLElement).getByRole('button', { name: /restart now/i }))

    await waitFor(() => {
      expect(within(evaluateCard as HTMLElement).getByText(/evaluate restarted \(ok\)\.?/i)).toBeInTheDocument()
    })
    expect(client.apiPost).toHaveBeenCalledWith('/api/components/evaluate/restart', {}, expect.any(Function))
  })

  it('shows an error message when component restart fails', async () => {
    vi.mocked(client.apiPost).mockRejectedValueOnce(new Error('component panic'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    fireEvent.click(within(evaluateCard as HTMLElement).getByRole('button', { name: /restart now/i }))

    await waitFor(() => {
      expect(within(evaluateCard as HTMLElement).getByText('component panic')).toBeInTheDocument()
    })
  })

  it('renders recent run policy and override metadata', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    expect(within(root).getByText('run_123')).toBeInTheDocument()
    expect(within(root).getAllByText('run_122').length).toBeGreaterThan(0)
    expect(within(root).getAllByText('trace_122')).toHaveLength(2)
    expect(within(root).getByText(/parent run:/i)).toBeInTheDocument()
    expect(within(root).getAllByText(/auto retry:/i)).toHaveLength(2)
    expect(within(root).getAllByText(/search runs do not have an automatic retry executor/i)).toHaveLength(2)
    expect(within(root).getByText(/manual override active: manual operator override/i)).toBeInTheDocument()
    expect(within(root).getAllByRole('button', { name: /schedule retry/i })).toHaveLength(2)
    expect(within(root).getAllByRole('button', { name: /block run/i })).toHaveLength(2)
    expect(within(root).getAllByLabelText(/retry reason/i)).toHaveLength(2)
    expect(within(root).getAllByLabelText(/delay seconds/i)).toHaveLength(2)
    expect(within(root).getAllByLabelText(/blocked reason/i)).toHaveLength(2)
    expect(within(root).getAllByText(/requires override/i)).toHaveLength(2)
  })

  it('schedules unsupported retry only after inline override confirmation', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'retry_scheduled',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: true,
        unsupported_retry_override_reason: 'manual operator override',
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: null,
        duration_ms: null,
        retry_scheduled_at: '2026-03-16T22:20:00Z',
        retry_reason: 'manual_backoff',
        blocked_reason: null,
      },
      meta: { request_id: 'req_patch' },
    } as never)
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    const reasonInputs = within(root).getAllByLabelText(/retry reason/i)
    const delayInputs = within(root).getAllByLabelText(/delay seconds/i)
    fireEvent.change(reasonInputs[0] as HTMLElement, { target: { value: 'manual_backoff' } })
    fireEvent.change(delayInputs[0] as HTMLElement, { target: { value: '90' } })
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)

    expect(client.apiPatch).not.toHaveBeenCalledWith('/v1/runs/run_122', expect.anything(), expect.any(Function))
    expect(within(root).getByText(/automatic retry is unsupported for search/i)).toBeInTheDocument()
    fireEvent.click(within(root).getByRole('button', { name: /confirm override retry/i }))

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/v1/runs/run_122',
        {
          status: 'retry_scheduled',
          reason: 'manual_backoff',
          retry_after_seconds: 90,
          allow_unsupported_retry: true,
        },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(within(root).getAllByText(/Retry at:/i)).toHaveLength(2)
    })
    expect(within(root).getAllByText('retry_scheduled')).toHaveLength(2)
    expect(within(root).getAllByText('Retry reason: manual_backoff')).toHaveLength(1)
  })

  it('cancels unsupported retry override without hitting the API', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)

    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()
    fireEvent.click(within(root).getByRole('button', { name: /cancel/i }))

    expect(within(root).queryByRole('button', { name: /confirm override retry/i })).toBeNull()
    expect(client.apiPatch).not.toHaveBeenCalledWith('/v1/runs/run_122', expect.anything(), expect.any(Function))
  })

  it('keeps pending override armed across eligible websocket updates', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)
    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:15:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'failed',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:14:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'still_retryable',
      },
    })
    await Promise.resolve()

    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()
  })

  it('clears pending override when websocket update makes the run ineligible', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)
    expect(within(root).getByRole('button', { name: /confirm override retry/i })).toBeInTheDocument()

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:16:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'retry_scheduled',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: true,
        unsupported_retry_override_reason: 'manual operator override',
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: null,
        duration_ms: null,
        retry_scheduled_at: '2026-03-16T22:16:00Z',
        retry_reason: 'operator_override',
        blocked_reason: null,
      },
    })
    await waitFor(() => {
      expect(within(root).queryByRole('button', { name: /confirm override retry/i })).toBeNull()
    })
  })

  it('clears retry success feedback when a later websocket update supersedes it', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'retry_scheduled',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: true,
        unsupported_retry_override_reason: 'manual operator override',
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: null,
        duration_ms: null,
        retry_scheduled_at: '2026-03-16T22:20:00Z',
        retry_reason: 'manual_backoff',
        blocked_reason: null,
      },
      meta: { request_id: 'req_patch' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /schedule retry/i })[0] as HTMLElement)
    fireEvent.click(within(root).getByRole('button', { name: /confirm override retry/i }))

    await waitFor(() => {
      expect(within(root).getByText('Retry scheduled.')).toBeInTheDocument()
    })

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:25:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:24:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'superseded_after_retry',
      },
    })

    await waitFor(() => {
      expect(within(root).queryByText('Retry scheduled.')).toBeNull()
    })
  })

  it('clears block error feedback when a later websocket update shows the run blocked anyway', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })
    vi.mocked(client.apiPatch).mockRejectedValueOnce(new Error('API 500: /v1/runs/run_122'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /block run/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('API 500: /v1/runs/run_122')).toBeInTheDocument()
    })

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }
    const runUpdateListener: (event: WsEnvelope) => void = wsListener
    runUpdateListener({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:30:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:29:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'blocked_elsewhere',
      },
    })

    await waitFor(() => {
      expect(within(root).queryByText('API 500: /v1/runs/run_122')).toBeNull()
    })
  })

  it('keeps other run controls active while one run action is pending', async () => {
    const firstPatch = createDeferred<unknown>()
    vi.mocked(client.apiPatch)
      .mockImplementationOnce(() => firstPatch.promise as never)
      .mockResolvedValueOnce({
        ok: true,
        data: {
          id: 'run_123',
          kind: 'search',
          status: 'blocked',
          automatic_retry_supported: false,
          automatic_retry_reason: 'search runs do not have an automatic retry executor',
          unsupported_retry_override: true,
          unsupported_retry_override_reason: 'manual operator override',
          created_at: '2026-03-16T22:00:00Z',
          started_at: null,
          finished_at: '2026-03-16T22:31:00Z',
          duration_ms: 60000,
          retry_scheduled_at: null,
          retry_reason: null,
          blocked_reason: 'parallel_block',
        },
        meta: { request_id: 'req_parallel' },
      } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    const blockButtons = within(root).getAllByRole('button', { name: /block run/i })
    fireEvent.click(blockButtons[0] as HTMLElement)

    await waitFor(() => {
      expect(within(root).getAllByRole('button', { name: /working…/i })).toHaveLength(2)
    })

    const updatedBlockButtons = within(root).getAllByRole('button', { name: /block run/i })
    expect(updatedBlockButtons[0]).not.toBeDisabled()
    fireEvent.click(updatedBlockButtons[0] as HTMLElement)

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/v1/runs/run_123',
        {
          status: 'blocked',
          blocked_reason: 'operator_ui_blocked',
        },
        expect.any(Function),
      )
    })

    firstPatch.resolve({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:32:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'first_block',
      },
      meta: { request_id: 'req_first_parallel' },
    })
  })

  it('blocks a run with an inline blocked reason', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T22:21:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'waiting_on_dependency',
      },
      meta: { request_id: 'req_patch_block' },
    } as never)
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    const blockedReasonInputs = within(root).getAllByLabelText(/blocked reason/i)
    fireEvent.change(blockedReasonInputs[0] as HTMLElement, { target: { value: 'waiting_on_dependency' } })
    fireEvent.click(within(root).getAllByRole('button', { name: /block run/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(client.apiPatch).toHaveBeenCalledWith(
        '/v1/runs/run_122',
        {
          status: 'blocked',
          blocked_reason: 'waiting_on_dependency',
        },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(within(root).getByText('Run blocked.')).toBeInTheDocument()
    })
    expect(within(root).getAllByText('Blocked reason: waiting_on_dependency')).toHaveLength(1)
  })

  it('renders run action errors inline on the relevant card', async () => {
    vi.mocked(client.apiPatch).mockRejectedValueOnce(new Error('API 500: /v1/runs/run_122'))

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    fireEvent.click(within(root).getAllByRole('button', { name: /block run/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(within(root).getByText('API 500: /v1/runs/run_122')).toBeInTheDocument()
    })
  })

  it('updates rendered runs from websocket payloads without refetching', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)
    const runsCallsBefore = vi.mocked(client.apiGet).mock.calls.filter(
      ([path]) => path === '/v1/runs?limit=6',
    ).length
    const runUpdateListener = wsListener as ((event: WsEnvelope) => void) | null
    expect(runUpdateListener).not.toBeNull()
    runUpdateListener?.({
      type: 'runs:updated',
      timestamp: '2026-03-16T22:10:00Z',
      payload: {
        id: 'run_122',
        kind: 'search',
        status: 'blocked',
        trace_id: 'trace_122b',
        parent_run_id: 'run_root',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T21:55:00Z',
        started_at: null,
        finished_at: '2026-03-16T21:56:00Z',
        duration_ms: 60000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'ws_blocked_reason',
      },
    })
    await Promise.resolve()

    const runsCallsAfter = vi.mocked(client.apiGet).mock.calls.filter(
      ([path]) => path === '/v1/runs?limit=6',
    ).length
    expect(runsCallsAfter).toBe(runsCallsBefore)
    expect(within(root).getByText('Blocked reason: ws_blocked_reason')).toBeInTheDocument()
    expect(within(root).getByText('trace_122b')).toBeInTheDocument()
    expect(within(root).getByText('run_root')).toBeInTheDocument()
  })

  it('updates components from websocket payloads without refetching', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    const evaluateCard = within(root).getByRole('heading', { name: /evaluate/i }).closest('.rounded-lg')
    expect(evaluateCard).not.toBeNull()
    expect(within(evaluateCard as HTMLElement).getByText(/Restarts: 0/)).toBeInTheDocument()

    expect(wsListener).not.toBeNull()
    if (!wsListener) {
      throw new Error('expected websocket listener')
    }

    const componentUpdateListener: (event: WsEnvelope) => void = wsListener
    componentUpdateListener({
      type: 'components:updated',
      timestamp: '2026-03-16T22:40:00Z',
      payload: {
        id: 'evaluate',
        name: 'Evaluate',
        description: 'Evaluate all pipelines',
        status: 'error',
        last_restarted_at: 1_700_000_400,
        last_error: 'restart failed',
        restart_count: 3,
      },
    })
    await Promise.resolve()

    expect(within(evaluateCard as HTMLElement).getByText('Restarts: 3')).toBeInTheDocument()
    expect(within(evaluateCard as HTMLElement).getByText('Last error: restart failed')).toBeInTheDocument()
    expect(within(evaluateCard as HTMLElement).getByText('error')).toBeInTheDocument()
  })

  it('updates runtime loops from the loops tab', async () => {
    vi.mocked(client.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: {
        kind: 'evaluate_current_state',
        enabled: false,
        interval_seconds: 600,
        last_started_at: 1_710_000_000,
        last_finished_at: 1_710_000_030,
        last_status: 'success',
        last_error: null,
        next_due_at: 1_710_000_600,
      },
      meta: { request_id: 'req_loop_patch' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = await openRuntimeTab(container)

    fireEvent.change(within(root).getByDisplayValue('300'), { target: { value: '600' } })
    fireEvent.click(within(root).getByLabelText(/enabled/i))

    await waitFor(() => {
      expect(vi.mocked(client.apiPatch)).toHaveBeenCalledWith(
        '/v1/loops/evaluate_current_state',
        { enabled: false, interval_seconds: 600 },
        expect.any(Function),
      )
    })

    expect(within(root).getByText('Loop updated.')).toBeInTheDocument()
  })

  it('subscribes to websocket updates for runs', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    await openRuntimeTab(container)

    expect(subscribeWs.mock.calls.length).toBeGreaterThanOrEqual(1)
  })

  it('opens directly to a targeted integration card', async () => {
    const scrollIntoView = vi.fn()
    Object.defineProperty(HTMLElement.prototype, 'scrollIntoView', {
      configurable: true,
      value: scrollIntoView,
    })

    const { container } = render(
      <SettingsPage
        onBack={() => {}}
        initialTab="integrations"
        initialIntegrationId="activity"
      />,
    )

    const root = getSettingsRoot(container)
    await waitFor(() => {
      expect(within(root).getByRole('heading', { name: /computer activity/i })).toBeInTheDocument()
    })

    expect(scrollIntoView).toHaveBeenCalled()
  })

  it('renders linked node status and issues a pairing token from the general tab', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        token_id: 'token_1',
        token_code: 'ABC-123',
        issued_at: '2026-03-16T18:20:00Z',
        expires_at: '2026-03-16T18:35:00Z',
        issued_by_node_id: 'vel-desktop',
        scopes: {
          read_context: true,
          write_safe_actions: true,
          execute_repo_tasks: false,
        },
        suggested_targets: [],
      },
      meta: { request_id: 'req_pairing' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = getSettingsRoot(container)

    await waitFor(() => {
      expect(within(root).getByText('linkedNodes')).toBeInTheDocument()
    })

    expect(within(root).getByText('Vel Air')).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /Remote Mac/i })).toBeInTheDocument()
    expect(within(root).getByText(/CLI fallback/i)).toBeInTheDocument()

    fireEvent.click(within(root).getByRole('button', { name: /Remote Mac/i }))
    fireEvent.click(within(root).getByRole('button', { name: /Pair nodes/i }))

    await waitFor(() => {
      expect(vi.mocked(client.apiPost)).toHaveBeenCalledWith(
        '/v1/linking/tokens',
        {
          issued_by_node_id: 'vel-desktop',
          scopes: {
            read_context: true,
            write_safe_actions: false,
            execute_repo_tasks: false,
          },
          target_node_id: 'node_remote',
          target_node_display_name: 'Remote Mac',
          target_base_url: 'http://remote.tailnet.ts.net:4130',
        },
        expect.any(Function),
      )
    })

    expect(within(root).getByText('Granted scopes')).toBeInTheDocument()
    expect(within(root).getByText('ABC-123')).toBeInTheDocument()
    expect(within(root).queryByText('Suggested link targets')).not.toBeInTheDocument()
    expect(within(root).getByText(/Pair nodes code created\. Remote Mac has been prompted to enter it on that client/i)).toBeInTheDocument()
    expect(within(root).getAllByText('Routes').length).toBeGreaterThan(0)
    expect(within(root).getByText('http://vel-air.tailnet.ts.net:4130')).toBeInTheDocument()
    expect(within(root).getByText('http://192.168.1.70:4130')).toBeInTheDocument()
    expect(within(root).queryByText('http://127.0.0.1:4130')).not.toBeInTheDocument()
  })

  it('renegotiates permissions for an already linked node', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        token_id: 'token_renegotiate',
        token_code: 'WXY-789',
        issued_at: '2026-03-16T18:20:00Z',
        expires_at: '2026-03-16T18:35:00Z',
        issued_by_node_id: 'vel-desktop',
        scopes: {
          read_context: true,
          write_safe_actions: false,
          execute_repo_tasks: true,
        },
        suggested_targets: [],
      },
      meta: { request_id: 'req_pairing_renegotiate' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = getSettingsRoot(container)

    await waitFor(() => {
      expect(within(root).getByText('Vel Air')).toBeInTheDocument()
    })

    fireEvent.click(within(root).getAllByLabelText(/Write safe actions/i)[1] as HTMLElement)
    fireEvent.click(within(root).getAllByLabelText(/Execute repo tasks/i)[1] as HTMLElement)
    fireEvent.click(within(root).getByRole('button', { name: /Request updated access/i }))

    await waitFor(() => {
      expect(vi.mocked(client.apiPost)).toHaveBeenCalledWith(
        '/v1/linking/tokens',
        {
          issued_by_node_id: 'vel-desktop',
          scopes: {
            read_context: true,
            write_safe_actions: false,
            execute_repo_tasks: true,
          },
          target_node_id: 'vel-air',
          target_node_display_name: 'Vel Air',
          target_base_url: 'http://vel-air.tailnet.ts.net:4130',
        },
        expect.any(Function),
      )
    })

    expect(within(root).getByText(/Pair nodes code created for Vel Air/i)).toBeInTheDocument()
    expect(within(root).getByText('WXY-789')).toBeInTheDocument()
  })

  it('unpairs a linked node with confirmation', async () => {
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        node_id: 'vel-air',
        node_display_name: 'Vel Air',
        status: 'revoked',
        scopes: {
          read_context: true,
          write_safe_actions: true,
          execute_repo_tasks: false,
        },
        linked_at: '2026-03-16T18:00:00Z',
        last_seen_at: '2026-03-16T18:20:00Z',
        transport_hint: 'tailscale',
        sync_base_url: 'http://vel-air.tailnet.ts.net:4130',
        tailscale_base_url: 'http://vel-air.tailnet.ts.net:4130',
        lan_base_url: 'http://192.168.1.70:4130',
        localhost_base_url: null,
        public_base_url: null,
      },
      meta: { request_id: 'req_revoke_link' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = getSettingsRoot(container)

    await waitFor(() => {
      expect(within(root).getByText('Vel Air')).toBeInTheDocument()
    })

    fireEvent.click(within(root).getByRole('button', { name: /^Unpair$/i }))
    expect(within(root).getByRole('button', { name: /Confirm unpair/i })).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /^Cancel$/i })).toBeInTheDocument()

    fireEvent.click(within(root).getByRole('button', { name: /Confirm unpair/i }))

    await waitFor(() => {
      expect(vi.mocked(client.apiPost)).toHaveBeenCalledWith(
        '/v1/linking/revoke/vel-air',
        {},
        expect.any(Function),
      )
    })

    expect(within(root).getByText(/Vel Air has been unpaired/i)).toBeInTheDocument()
  })

  it('shows local enter-token ui when this client receives a linking prompt', async () => {
    vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/cluster/workers') {
        return {
          ok: true,
          data: {
            active_authority_node_id: 'vel-desktop',
            active_authority_epoch: 1,
            generated_at: 1_710_000_100,
            workers: [
              {
                worker_id: 'vel-desktop',
                node_id: 'vel-desktop',
                node_display_name: 'Vel Desktop',
                client_kind: 'veld',
                client_version: '0.1.0',
                protocol_version: '1',
                build_id: 'build_local',
                worker_classes: ['sync'],
                capabilities: ['sync_bootstrap'],
                status: 'ok',
                queue_depth: 0,
                reachability: 'reachable',
                latency_class: 'low',
                compute_class: 'standard',
                power_class: 'ac_or_unknown',
                recent_failure_rate: 0,
                tailscale_preferred: true,
                last_heartbeat_at: 1_710_000_095,
                started_at: 1_710_000_000,
                sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
                sync_transport: 'tailscale',
                tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
                preferred_tailnet_endpoint: null,
                tailscale_reachable: true,
                lan_base_url: 'http://192.168.1.50:4130',
                localhost_base_url: 'http://127.0.0.1:4130',
                ping_ms: 5,
                sync_status: 'ready',
                last_upstream_sync_at: null,
                last_downstream_sync_at: null,
                last_sync_error: null,
                incoming_linking_prompt: {
                  target_node_id: 'vel-desktop',
                  target_node_display_name: 'Vel Desktop',
                  issued_by_node_id: 'node_remote',
                  issued_by_node_display_name: 'Remote Mac',
                  issued_at: '2026-03-16T18:20:00Z',
                  expires_at: '2026-03-16T18:35:00Z',
                  scopes: {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                  },
                  issuer_sync_base_url: 'http://remote.tailnet.ts.net:4130',
                  issuer_sync_transport: 'tailscale',
                  issuer_tailscale_base_url: 'http://remote.tailnet.ts.net:4130',
                  issuer_lan_base_url: 'http://192.168.1.60:4130',
                  issuer_localhost_base_url: null,
                  issuer_public_base_url: null,
                },
                capacity: {
                  max_concurrency: 2,
                  current_load: 0,
                  available_concurrency: 2,
                },
              },
              {
                worker_id: 'worker_remote',
                node_id: 'node_remote',
                node_display_name: 'Remote Mac',
                client_kind: 'vel_macos',
                client_version: '0.1.0',
                protocol_version: '1',
                build_id: 'build_remote',
                worker_classes: ['sync'],
                capabilities: ['sync_bootstrap'],
                status: 'ok',
                queue_depth: 0,
                reachability: 'reachable',
                latency_class: 'low',
                compute_class: 'standard',
                power_class: 'ac_or_unknown',
                recent_failure_rate: 0,
                tailscale_preferred: true,
                last_heartbeat_at: 1_710_000_090,
                started_at: 1_710_000_000,
                sync_base_url: 'http://remote.tailnet.ts.net:4130',
                sync_transport: 'tailscale',
                tailscale_base_url: 'http://remote.tailnet.ts.net:4130',
                preferred_tailnet_endpoint: null,
                tailscale_reachable: true,
                lan_base_url: null,
                localhost_base_url: null,
                ping_ms: 14,
                sync_status: 'ready',
                last_upstream_sync_at: null,
                last_downstream_sync_at: null,
                last_sync_error: null,
                incoming_linking_prompt: null,
                capacity: {
                  max_concurrency: 2,
                  current_load: 0,
                  available_concurrency: 2,
                },
              },
            ],
          },
          meta: { request_id: 'req_cluster_workers_prompt' },
        } as never
      }
      if (path === '/v1/cluster/bootstrap') {
        return {
          ok: true,
          data: {
            node_id: 'vel-desktop',
            node_display_name: 'Vel Desktop',
            active_authority_node_id: 'vel-desktop',
            active_authority_epoch: 1,
            sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            sync_transport: 'tailscale',
            tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            lan_base_url: 'http://192.168.1.50:4130',
            localhost_base_url: 'http://127.0.0.1:4130',
            capabilities: ['read_context'],
            linked_nodes: [],
            projects: [],
            action_items: [],
          },
          meta: { request_id: 'req_cluster_bootstrap_prompt' },
        } as never
      }
      if (path === '/v1/linking/status') {
        return {
          ok: true,
          data: [],
          meta: { request_id: 'req_linking_status_prompt' },
        } as never
      }
      if (path === '/api/settings') {
        return {
          ok: true,
          data: {
            disable_proactive: false,
            toggle_risks: true,
            toggle_reminders: true,
            timezone: 'America/Denver',
            node_display_name: 'Vel Desktop',
            tailscale_preferred: true,
            tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            tailscale_base_url_auto_discovered: true,
            lan_base_url: 'http://192.168.1.50:4130',
            lan_base_url_auto_discovered: true,
          },
          meta: { request_id: 'req_settings_prompt' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: {
            google_calendar: { configured: false, connected: false, has_client_id: false, has_client_secret: false, calendars: [], all_calendars_selected: true, last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            todoist: { configured: false, connected: false, has_api_token: false, last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            activity: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'file', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            health: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'file', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            git: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'file', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            messaging: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'file', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            reminders: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'file', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            notes: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
            transcripts: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'file', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
          },
          meta: { request_id: 'req_integrations_prompt' },
        } as never
      }
      if (path === '/v1/runs?limit=6' || path === '/api/components' || path === '/v1/loops') {
        return { ok: true, data: [], meta: { request_id: 'req_empty_prompt' } } as never
      }
      if (path === '/v1/execution/handoffs?state=pending_review') {
        return { ok: true, data: [], meta: { request_id: 'req_empty_handoffs_prompt' } } as never
      }
      if (path === '/v1/agent/inspect') {
        return {
          ok: true,
          data: {
            grounding: {
              generated_at: 1710000000,
              now: {
                computed_at: 1710000000,
                timezone: 'America/Denver',
                summary: {
                  mode: { key: 'focus', label: 'Focus' },
                  phase: { key: 'engaged', label: 'Engaged' },
                  meds: { key: 'ok', label: 'OK' },
                  risk: { level: 'low', score: 0.2, label: 'low' },
                },
                schedule: { empty_message: null, next_event: null, upcoming_events: [] },
                tasks: { todoist: [], other_open: [], next_commitment: null },
                attention: {
                  state: { key: 'on_task', label: 'On task' },
                  drift: { key: 'none', label: 'None' },
                  severity: { key: 'none', label: 'None' },
                  confidence: null,
                  reasons: [],
                },
                sources: { git_activity: null, health: null, mood: null, pain: null, note_document: null, assistant_message: null },
                freshness: { overall_status: 'fresh', sources: [] },
                action_items: [],
                review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0 },
                pending_writebacks: [],
                conflicts: [],
                people: [],
                reasons: [],
                debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
              },
              current_context: null,
              projects: [],
              people: [],
              commitments: [],
              review: { review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0 }, pending_writebacks: [], conflicts: [], pending_execution_handoffs: [] },
            },
            capabilities: { groups: [] },
            blockers: [],
            explainability: { persisted_record_kinds: ['now'], supporting_paths: ['/v1/agent/inspect'], raw_context_json_supporting_only: true },
          },
          meta: { request_id: 'req_agent_inspect_prompt' },
        } as never
      }
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            now: 1_710_000_100,
            timezone: 'America/Denver',
          },
          meta: { request_id: 'req_now_prompt' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })
    vi.mocked(client.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        node_id: 'vel-desktop',
        node_display_name: 'Vel Desktop',
        status: 'linked',
        scopes: {
          read_context: true,
          write_safe_actions: false,
          execute_repo_tasks: false,
        },
        linked_at: '2026-03-16T18:25:00Z',
        last_seen_at: '2026-03-16T18:25:00Z',
        transport_hint: 'tailscale',
      },
      meta: { request_id: 'req_redeem' },
    } as never)

    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = getSettingsRoot(container)

    await waitFor(() => {
      expect(within(root).getByText(/Enter pairing token/i)).toBeInTheDocument()
    })

    const tokenInput = within(root).getByPlaceholderText('ABC-123') as HTMLInputElement
    fireEvent.change(tokenInput, { target: { value: 'abc123' } })
    expect(tokenInput.value).toBe('ABC-123')
    fireEvent.click(within(root).getByRole('button', { name: /^Enter token$/i }))

    await waitFor(() => {
      expect(vi.mocked(client.apiPost)).toHaveBeenCalledWith(
        '/v1/linking/redeem',
        {
          token_code: 'ABC-123',
          sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
          tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
          lan_base_url: 'http://192.168.1.50:4130',
          localhost_base_url: 'http://127.0.0.1:4130',
          public_base_url: null,
          node_id: 'vel-desktop',
          node_display_name: 'Vel Desktop',
          transport_hint: 'tailscale',
          requested_scopes: {
            read_context: true,
            write_safe_actions: false,
            execute_repo_tasks: false,
          },
        },
        expect.any(Function),
      )
    })

    expect(within(root).getByText(/Linked as Vel Desktop/i)).toBeInTheDocument()
  })

  it('shows pending execution reviews in the runtime tab', async () => {
    const { container } = render(<SettingsPage onBack={() => {}} />)
    const root = getSettingsRoot(container)

  await waitFor(() => {
    expect(within(root).getAllByText('Pending execution review').length).toBeGreaterThan(0)
  })

    expect(within(root).getByText('Implement the runtime review queue')).toBeInTheDocument()
    expect(within(root).getByText('1 pending')).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /^Approve$/i })).toBeInTheDocument()
    expect(within(root).getByRole('button', { name: /^Reject$/i })).toBeInTheDocument()
  })

})
