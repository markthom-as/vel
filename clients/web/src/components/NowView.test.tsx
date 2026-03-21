import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { NowView } from './NowView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
  apiPatch: vi.fn(),
}))

function buildNowData(overrides: Record<string, unknown> = {}) {
  return {
    computed_at: 1710000000,
    timezone: 'America/Denver',
    header: {
      title: "Jove's Now",
      buckets: [
        {
          kind: 'needs_input',
          count: 1,
          count_display: 'show_nonzero',
          urgent: true,
          route_target: {
            bucket: 'needs_input',
            thread_id: 'thr_check_in_1',
          },
        },
        {
          kind: 'new_nudges',
          count: 2,
          count_display: 'show_nonzero',
          urgent: false,
          route_target: {
            bucket: 'new_nudges',
            thread_id: null,
          },
        },
        {
          kind: 'snoozed',
          count: 0,
          count_display: 'show_nonzero',
          urgent: false,
          route_target: {
            bucket: 'snoozed',
            thread_id: null,
          },
        },
      ],
    },
    mesh_summary: {
      authority_node_id: 'vel-desktop',
      authority_label: 'Vel Desktop',
      sync_state: 'stale',
      linked_node_count: 2,
      queued_write_count: 1,
      last_sync_at: 1710000000,
      urgent: true,
      repair_route: {
        target: 'settings_recovery',
        summary: 'Sync or queued-write posture needs review before trusting all cross-client state.',
      },
    },
    status_row: {
      date_label: 'Mar 9',
      time_label: '4:00 PM',
      context_label: 'Write weekly review',
      elapsed_label: 'No active task',
    },
    context_line: {
      text: 'Standup is waiting on question 1 with 0 commitment draft(s) and 0 deferred item(s).',
      thread_id: null,
      fallback_used: false,
    },
    nudge_bars: [
      {
        id: 'check_in_bar',
        kind: 'needs_input',
        title: 'Standup check-in',
        summary: 'Vel needs one short answer before the standup can continue.',
        urgent: true,
        primary_thread_id: 'thr_check_in_1',
        actions: [
          { kind: 'expand', label: 'Continue in Threads' },
        ],
      },
      {
        id: 'mesh_summary_warning',
        kind: 'trust_warning',
        title: 'Vel Desktop needs attention',
        summary: 'Sync or queued-write posture needs review before trusting all cross-client state.',
        urgent: true,
        primary_thread_id: null,
        actions: [
          { kind: 'open_settings', label: 'Open settings' },
        ],
      },
    ],
    task_lane: {
      active: {
        id: 'commit_local_1',
        task_kind: 'commitment',
        text: 'Write weekly review',
        state: 'active',
        project: null,
        primary_thread_id: null,
      },
      pending: [
        {
          id: 'commit_todoist_1',
          task_kind: 'task',
          text: 'Reply to Dimitri',
          state: 'pending',
          project: 'Ops',
          primary_thread_id: null,
        },
      ],
      recent_completed: [
        {
          id: 'commit_done_1',
          task_kind: 'commitment',
          text: 'Confirm the design review agenda',
          state: 'completed',
          project: 'Ops',
          primary_thread_id: 'thr_exec_1',
        },
      ],
      overflow_count: 0,
    },
    docked_input: {
      supported_intents: ['task', 'question', 'note', 'command', 'continuation', 'reflection', 'scheduling'],
      day_thread_id: 'thr_day_1',
      raw_capture_thread_id: 'thr_capture_1',
    },
    overview: {
      dominant_action: {
        kind: 'check_in',
        title: 'Standup check-in',
        summary: 'Name the one to three commitments that matter most today.',
        reference_id: 'act_check_in_1',
      },
      today_timeline: [
        {
          kind: 'now',
          title: 'Current time',
          timestamp: 1710000000,
          detail: null,
        },
      ],
      visible_nudge: {
        kind: 'freshness',
        title: 'Review operator queue',
        summary: 'One supervised review is still pending.',
      },
      why_state: [
        { label: 'Mode', detail: 'Day' },
        { label: 'Attention', detail: 'On task' },
      ],
      suggestions: [],
      decision_options: ['accept', 'choose', 'thread', 'close'],
    },
    summary: {
      mode: { key: 'day_mode', label: 'Day' },
      phase: { key: 'engaged', label: 'Engaged' },
      meds: { key: 'pending', label: 'Pending' },
      risk: { level: 'medium', score: 0.72, label: 'medium · 72%' },
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
      confidence: 0.8,
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
    trust_readiness: {
      level: 'ok',
      headline: 'Ready',
      summary: 'No trust blockers are active.',
      backup: {
        level: 'ok',
        label: 'Backup',
        detail: 'Backup trust is healthy.',
      },
      freshness: {
        level: 'ok',
        label: 'Freshness',
        detail: 'Inputs are fresh enough to trust.',
      },
      review: {
        open_action_count: 0,
        pending_execution_reviews: 0,
        pending_writeback_count: 0,
        conflict_count: 0,
      },
      guidance: [],
      follow_through: [],
    },
    check_in: null,
    day_plan: null,
    reflow: null,
    reflow_status: null,
    action_items: [],
    review_snapshot: {
      open_action_count: 0,
      triage_count: 0,
      projects_needing_review: 0,
      pending_execution_reviews: 0,
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
    ...overrides,
  }
}

describe('NowView', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()
    vi.useRealTimers()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
    vi.mocked(api.apiPatch).mockReset()
    vi.mocked(api.apiPatch).mockImplementation(async () => ({
      ok: true,
      data: {
        id: 'commit_local_1',
        text: 'Write weekly review',
        source_type: 'capture',
        due_at: null,
        project: null,
        commitment_kind: 'writing',
      },
    }))
    let morningSession: Record<string, unknown> | null = null
    let standupSession: Record<string, unknown> | null = null
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            computed_at: 1710000000,
            timezone: 'America/Denver',
            overview: {
              dominant_action: {
                kind: 'check_in',
                title: 'Standup check-in',
                summary: 'Standup is waiting on question 1 with 0 commitment draft(s) and 0 deferred item(s).',
                reference_id: 'act_check_in_1',
              },
              today_timeline: [
                {
                  kind: 'now',
                  title: 'Current time',
                  timestamp: 1710000000,
                  detail: null,
                },
                {
                  kind: 'calendar_event',
                  title: 'Design review',
                  timestamp: 1710003600,
                  detail: 'Room 4B',
                },
              ],
              visible_nudge: {
                kind: 'freshness',
                title: 'Review execution handoff for runtime lane',
                summary: 'The supervised coding handoff is waiting for explicit approval.',
              },
              why_state: [
                { label: 'Mode', detail: 'Day' },
                { label: 'Phase', detail: 'Engaged' },
                { label: 'Attention', detail: 'On task' },
              ],
              suggestions: [],
              decision_options: ['accept', 'choose', 'thread', 'close'],
            },
            summary: {
              mode: { key: 'day_mode', label: 'Day' },
              phase: { key: 'engaged', label: 'Engaged' },
              meds: { key: 'pending', label: 'Pending' },
              risk: { level: 'medium', score: 0.72, label: 'medium · 72%' },
            },
            schedule: {
              empty_message: null,
              next_event: {
                title: 'Design review',
                start_ts: 1710003600,
                end_ts: 1710007200,
                location: 'Room 4B',
                prep_minutes: 15,
                travel_minutes: 0,
                leave_by_ts: 1710003600,
              },
              upcoming_events: [
                {
                  title: 'Design review',
                  start_ts: 1710003600,
                  end_ts: 1710007200,
                  location: 'Room 4B',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710003600,
                },
              ],
            },
            tasks: {
              todoist: [
                {
                  id: 'commit_todoist_1',
                  text: 'Reply to Dimitri',
                  source_type: 'todoist',
                  due_at: '2026-03-16T19:00:00Z',
                  project: 'Ops',
                  commitment_kind: 'todo',
                },
              ],
              other_open: [
                {
                  id: 'commit_local_1',
                  text: 'Write weekly review',
                  source_type: 'capture',
                  due_at: null,
                  project: null,
                  commitment_kind: 'writing',
                },
              ],
              next_commitment: {
                id: 'commit_local_1',
                text: 'Write weekly review',
                source_type: 'capture',
                due_at: null,
                project: null,
                commitment_kind: 'writing',
              },
            },
            attention: {
              state: { key: 'on_task', label: 'On task' },
              drift: { key: 'none', label: 'None' },
              severity: { key: 'none', label: 'None' },
              confidence: 0.8,
              reasons: ['recent git activity indicates active work'],
            },
            sources: {
              git_activity: {
                label: 'Git activity',
                timestamp: 1710000000,
                summary: {
                  repo: 'vel',
                  branch: 'main',
                  operation: 'commit',
                },
              },
              health: {
                label: 'Health',
                timestamp: 1710000030,
                summary: {
                  metric_type: 'resting_heart_rate',
                  value: 58,
                  unit: 'bpm',
                  source_app: 'Apple Health',
                  device: 'Apple Watch',
                },
              },
              mood: null,
              pain: null,
              note_document: {
                label: 'Recent note',
                timestamp: 1710000060,
                summary: {
                  title: 'Today',
                  path: 'daily/today.md',
                },
              },
              assistant_message: {
                label: 'Recent transcript',
                timestamp: 1710000120,
                summary: {
                  conversation_id: 'conv_external',
                  role: 'assistant',
                  source: 'chatgpt',
                },
              },
            },
            freshness: {
              overall_status: 'fresh',
              sources: [
                {
                  key: 'context',
                  label: 'Context',
                  status: 'fresh',
                  last_sync_at: 1710000000,
                  age_seconds: 10,
                  guidance: null,
                },
              ],
            },
            action_items: [
              {
                id: 'action_1',
                surface: 'now',
                kind: 'next_step',
                permission_mode: 'user_confirm',
                scope_affinity: 'project',
                title: 'Confirm the design review agenda',
                summary: 'Prep materials and confirm the current owner before the meeting starts.',
                project_id: 'proj_ops',
                project_label: 'Ops',
                project_family: 'areas',
                state: 'active',
                rank: 1,
                surfaced_at: '2026-03-16T18:30:00Z',
                snoozed_until: null,
                evidence: [
                  {
                    source_kind: 'calendar_event',
                    source_id: 'evt_1',
                    label: 'Design review at 10:00',
                    detail: null,
                  },
                  {
                    source_kind: 'person',
                    source_id: 'per_annie',
                    label: 'Annie Case',
                    detail: null,
                  },
                ],
                thread_route: {
                  target: 'filtered_threads',
                  label: 'Open related threads',
                  thread_id: null,
                  thread_type: 'project_review',
                  project_id: 'proj_ops',
                },
              },
              {
                id: 'action_2',
                surface: 'now',
                kind: 'review',
                permission_mode: 'user_confirm',
                scope_affinity: 'project',
                title: 'Review execution handoff for runtime lane',
                summary: 'The supervised coding handoff is waiting for explicit approval.',
                project_id: 'proj_exec',
                project_label: 'Execution',
                project_family: 'areas',
                state: 'active',
                rank: 2,
                surfaced_at: '2026-03-16T19:00:00Z',
                snoozed_until: null,
                evidence: [
                  {
                    source_kind: 'execution_handoff',
                    source_id: 'handoff_1',
                    label: 'implementation · quality · large',
                    detail: 'write_scope_requires_approval | write scopes: /tmp/vel',
                  },
                ],
                thread_route: {
                  target: 'existing_thread',
                  label: 'Open execution thread',
                  thread_id: 'thr_exec_1',
                  thread_type: 'execution_review',
                  project_id: 'proj_exec',
                },
              },
            ],
            review_snapshot: {
              open_action_count: 2,
              triage_count: 1,
              projects_needing_review: 1,
              pending_execution_reviews: 1,
            },
            trust_readiness: {
              level: 'warn',
              headline: 'Review is pending',
              summary: 'One execution review and one recovery follow-through still need attention.',
              backup: {
                level: 'warn',
                label: 'Backup',
                detail: 'Backup trust is degraded until the next verification run.',
              },
              freshness: {
                level: 'ok',
                label: 'Freshness',
                detail: 'Current context and integrations look fresh enough to trust.',
              },
              review: {
                open_action_count: 2,
                pending_execution_reviews: 1,
                pending_writeback_count: 1,
                conflict_count: 1,
              },
              guidance: [
                'Review the remaining conflicts or supervised execution handoffs before risky actions.',
              ],
              follow_through: [
                {
                  id: 'action_recovery_1',
                  surface: 'inbox',
                  kind: 'recovery',
                  permission_mode: 'user_confirm',
                  scope_affinity: 'global',
                  title: 'Backup is stale',
                  summary: 'Create or verify a fresh backup before risky maintenance.',
                  project_id: null,
                  project_label: null,
                  project_family: null,
                  state: 'active',
                  rank: 88,
                  surfaced_at: '2026-03-16T18:20:00Z',
                  snoozed_until: null,
                  evidence: [
                    {
                      source_kind: 'backup_trust',
                      source_id: 'warn',
                      label: 'Backup trust',
                      detail: 'Backup trust is degraded. Create or verify a fresh backup before risky maintenance.',
                    },
                  ],
                  thread_route: null,
                },
              ],
            },
            check_in: {
              id: 'act_check_in_1',
              source_kind: 'daily_loop',
              phase: 'standup',
              session_id: 'dls_1',
              title: 'Standup check-in',
              summary: 'Vel needs one short answer before the standup can continue.',
              prompt_id: 'standup_prompt_1',
              prompt_text: 'Name the one to three commitments that matter most today.',
              suggested_action_label: 'Continue standup',
              suggested_response: null,
              allow_skip: true,
              blocking: true,
              submit_target: {
                kind: 'daily_loop_turn',
                reference_id: 'dls_1',
              },
              escalation: {
                target: 'threads',
                label: 'Continue in Threads',
              },
              transitions: [
                {
                  kind: 'submit',
                  label: 'Continue standup',
                  target: 'daily_loop_turn',
                  reference_id: 'dls_1',
                  requires_response: true,
                  requires_note: false,
                },
                {
                  kind: 'bypass',
                  label: 'Skip for now',
                  target: 'daily_loop_turn',
                  reference_id: 'dls_1',
                  requires_response: false,
                  requires_note: true,
                },
                {
                  kind: 'escalate',
                  label: 'Continue in Threads',
                  target: 'threads',
                  reference_id: 'dls_1',
                  requires_response: false,
                  requires_note: false,
                },
              ],
            },
            day_plan: {
              headline: 'Today has a bounded plan',
              summary: 'Vel shaped a bounded same-day plan from current commitments, calendar anchors, and routine blocks.',
              scheduled_count: 2,
              deferred_count: 1,
              did_not_fit_count: 0,
              needs_judgment_count: 1,
              changes: [
                {
                  kind: 'scheduled',
                  title: 'Write weekly review',
                  detail: 'Write weekly review fits in the next bounded slot for today.',
                  project_label: 'Ops',
                  scheduled_start_ts: 1710001800,
                  rule_facets: [
                    { kind: 'time_window', label: 'time:prenoon', detail: 'Task prefers the prenoon window.' },
                  ],
                },
                {
                  kind: 'deferred',
                  title: 'Backlog cleanup',
                  detail: 'Backlog cleanup is marked for local defer and was left out of today\'s bounded plan.',
                  project_label: null,
                  scheduled_start_ts: null,
                  rule_facets: [
                    { kind: 'local_defer', label: 'defer', detail: 'Task is marked for local defer logic.' },
                  ],
                },
              ],
              routine_blocks: [
                {
                  id: 'routine_morning',
                  label: 'Morning routine',
                  source: 'inferred',
                  start_ts: 1710000000,
                  end_ts: 1710003600,
                  protected: true,
                },
              ],
            },
            reflow: {
              id: 'act_reflow_1',
              title: 'Day changed',
              summary: 'A scheduled event appears to have slipped past without the plan being updated.',
              trigger: 'missed_event',
              severity: 'critical',
              accept_mode: 'confirm_required',
              suggested_action_label: 'Accept',
              preview_lines: [
                'Next scheduled event started 20 minutes ago.',
                'Leave-by threshold passed 10 minutes ago.',
              ],
              edit_target: {
                target: 'threads',
                label: 'Edit',
              },
              proposal: {
                headline: 'Remaining day recomputed',
                summary: 'Vel recomputed the remaining day and found at least one item that still needs operator judgment.',
                moved_count: 1,
                unscheduled_count: 1,
                needs_judgment_count: 1,
                changes: [
                  {
                    kind: 'moved',
                    title: 'Deep work',
                    detail: 'Deep work can move to the next available slot in the remaining day.',
                    project_label: 'Project Atlas',
                    scheduled_start_ts: 1710003600,
                  },
                  {
                    kind: 'unscheduled',
                    title: 'Write proposal',
                    detail: 'Write proposal no longer fits in the remaining day without operator intervention.',
                    project_label: 'Project Atlas',
                    scheduled_start_ts: null,
                  },
                ],
                rule_facets: [
                  { kind: 'block_target', label: 'block:focus', detail: 'Task prefers a named block target.' },
                  { kind: 'calendar_free', label: 'cal:free', detail: 'Task prefers free calendar space.' },
                ],
              },
              transitions: [
                {
                  kind: 'accept',
                  label: 'Accept',
                  target: 'apply_suggestion',
                  confirm_required: true,
                },
                {
                  kind: 'edit',
                  label: 'Edit',
                  target: 'threads',
                  confirm_required: false,
                },
              ],
            },
            reflow_status: {
              kind: 'editing',
              trigger: 'missed_event',
              severity: 'critical',
              headline: 'Reflow moved to Threads',
              detail: 'Vel opened a thread-backed reflow follow-up so the day plan can be shaped before anything else changes.',
              recorded_at: 1710000150,
              preview_lines: ['Next scheduled event started 20 minutes ago.'],
              thread_id: 'thr_reflow_1',
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
                requested_at: '2026-03-16T18:30:00Z',
                applied_at: null,
                updated_at: '2026-03-16T18:30:00Z',
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
                opened_at: '2026-03-16T18:30:00Z',
                resolved_at: null,
                updated_at: '2026-03-16T18:30:00Z',
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
                aliases: [
                  {
                    platform: 'email',
                    handle: 'annie@example.com',
                    display: 'Annie Case',
                    source_ref: null,
                  },
                ],
                links: [],
              },
            ],
            reasons: ['Prep window active', 'recent git activity indicates active work'],
            debug: {
              raw_context: {},
              signals_used: ['sig_cal_1'],
              commitments_used: ['commit_1'],
              risk_used: ['risk_1'],
            },
          }),
          meta: { request_id: 'req_now' },
        } as never
      }
      if (path.startsWith('/v1/daily-loop/sessions/active?')) {
        const url = new URL(`http://localhost${path}`)
        const phase = url.searchParams.get('phase')
        return {
          ok: true,
          data: phase === 'standup' ? standupSession : morningSession,
          meta: { request_id: `req_daily_loop_active_${phase}` },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })
    vi.mocked(api.apiPost).mockImplementation(async (path: string, body?: unknown) => {
      if (path === '/v1/evaluate') {
        return {
          ok: true,
          data: {
            inferred_states: 4,
            nudges_created_or_updated: 1,
          },
          meta: { request_id: 'req_eval' },
        } as never
      }
      if (path === '/v1/sync/calendar') {
        return {
          ok: true,
          data: {
            source: 'calendar',
            signals_ingested: 3,
          },
          meta: { request_id: 'req_sync_calendar' },
        } as never
      }
      if (path === '/v1/sync/todoist') {
        return {
          ok: true,
          data: {
            source: 'todoist',
            signals_ingested: 5,
          },
          meta: { request_id: 'req_sync_todoist' },
        } as never
      }
      if (path === '/v1/sync/activity') {
        return {
          ok: true,
          data: {
            source: 'activity',
            signals_ingested: 2,
          },
          meta: { request_id: 'req_sync_activity' },
        } as never
      }
      if (path === '/v1/sync/messaging') {
        return {
          ok: true,
          data: {
            source: 'messaging',
            signals_ingested: 4,
          },
          meta: { request_id: 'req_sync_messaging' },
        } as never
      }
      if (path === '/v1/daily-loop/sessions') {
        const request = body as { phase: string; session_date: string }
        if (request.phase === 'morning_overview') {
          morningSession = {
            id: 'dls_morning_1',
            session_date: request.session_date,
            phase: 'morning_overview',
            status: 'waiting_for_input',
            start: {
              source: 'manual',
              surface: 'web',
            },
            turn_state: 'waiting_for_input',
            continuity_summary:
              'Morning overview is waiting on question 1 of 3 with 0 captured signal(s).',
            allowed_actions: ['accept', 'defer', 'choose', 'close'],
            current_prompt: {
              prompt_id: 'morning_prompt_1',
              kind: 'intent_question',
              text: 'What most needs to happen before noon?',
              ordinal: 1,
              allow_skip: true,
            },
            state: {
              phase: 'morning_overview',
              snapshot: 'Two meetings before noon. Reply to Dimitri is still open.',
              friction_callouts: [
                {
                  label: 'Packed morning',
                  detail: 'You have little slack before the design review.',
                },
              ],
              signals: [],
            },
            outcome: null,
          }
          return {
            ok: true,
            data: morningSession,
            meta: { request_id: 'req_daily_loop_start_morning' },
          } as never
        }

        standupSession = {
          id: 'dls_standup_1',
          session_date: request.session_date,
          phase: 'standup',
          status: 'waiting_for_input',
          start: {
            source: 'manual',
            surface: 'web',
          },
          turn_state: 'waiting_for_input',
          continuity_summary:
            'Standup is waiting on question 1 with 0 commitment draft(s) and 0 deferred item(s).',
          allowed_actions: ['accept', 'defer', 'choose', 'close'],
          current_prompt: {
            prompt_id: 'standup_prompt_1',
            kind: 'commitment_reduction',
            text: 'Name the one to three commitments that matter most today.',
            ordinal: 1,
            allow_skip: true,
          },
          state: {
            phase: 'standup',
            commitments: [],
            deferred_tasks: [],
            confirmed_calendar: ['Design review at 10:00'],
            focus_blocks: [],
          },
          outcome: null,
        }
        return {
          ok: true,
          data: standupSession,
          meta: { request_id: 'req_daily_loop_start_standup' },
        } as never
      }
      if (path === '/v1/daily-loop/sessions/dls_morning_1/turn') {
        const request = body as { action: string; response_text?: string | null }
        const currentSignals =
          ((morningSession?.state as { signals?: Array<{ kind: string; text: string }> } | undefined)
            ?.signals ?? [])
        if (request.action === 'submit') {
          currentSignals.push({
            kind: 'must_do_hint',
            text: request.response_text ?? '',
          })
        }
        if ((morningSession?.current_prompt as { ordinal?: number } | undefined)?.ordinal === 1) {
          morningSession = {
            ...morningSession,
            state: {
              ...(morningSession?.state as Record<string, unknown>),
              signals: currentSignals,
            },
            current_prompt: {
              prompt_id: 'morning_prompt_2',
              kind: 'intent_question',
              text: 'What could derail today if ignored?',
              ordinal: 2,
              allow_skip: true,
            },
            continuity_summary:
              'Morning overview is waiting on question 2 of 3 with 1 captured signal(s).',
            allowed_actions: ['accept', 'defer', 'choose', 'close'],
          }
        } else {
          const completedMorning = {
            ...morningSession,
            status: 'completed',
            turn_state: 'completed',
            current_prompt: null,
            continuity_summary: 'Morning overview continuity is available.',
            allowed_actions: ['accept', 'choose', 'close'],
            state: {
              ...(morningSession?.state as Record<string, unknown>),
              signals: currentSignals,
            },
            outcome: {
              phase: 'morning_overview',
              signals: currentSignals,
            },
          }
          morningSession = null
          return {
            ok: true,
            data: completedMorning,
            meta: { request_id: 'req_daily_loop_turn_morning_complete' },
          } as never
        }
        return {
          ok: true,
          data: morningSession,
          meta: { request_id: 'req_daily_loop_turn_morning' },
        } as never
      }
      if (path === '/v1/daily-loop/sessions/dls_standup_1/turn') {
        const request = body as { response_text?: string | null }
        const commitments = (request.response_text ?? '')
          .split(',')
          .map((item) => item.trim())
          .filter((item) => item.length > 0)
          .slice(0, 3)
          .map((title, index) => ({
            title,
            bucket: index === 0 ? 'must' : 'should',
            source_ref: null,
          }))
        const completedStandup = {
          ...standupSession,
          status: 'completed',
          turn_state: 'completed',
          current_prompt: null,
          continuity_summary: 'Standup continuity is available.',
          allowed_actions: ['accept', 'choose', 'close'],
          state: {
            phase: 'standup',
            commitments,
            deferred_tasks: [
              {
                title: 'Inbox cleanup',
                source_ref: null,
                reason: 'Not part of the top commitments.',
              },
            ],
            confirmed_calendar: ['Design review at 10:00'],
            focus_blocks: [],
          },
          outcome: {
            phase: 'standup',
            commitments,
            deferred_tasks: [
              {
                title: 'Inbox cleanup',
                source_ref: null,
                reason: 'Not part of the top commitments.',
              },
            ],
            confirmed_calendar: ['Design review at 10:00'],
            focus_blocks: [],
          },
        }
        standupSession = null
        return {
          ok: true,
          data: completedStandup,
          meta: { request_id: 'req_daily_loop_turn_standup_complete' },
        } as never
      }
      throw new Error(`unexpected apiPost path: ${path}`)
    })
  })

  it('renders the consolidated now snapshot', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText("Jove's Now")).toBeInTheDocument()
    })

    expect(screen.getByText('Needs input')).toBeInTheDocument()
    expect(screen.getByText('Nudges')).toBeInTheDocument()
    expect(screen.getByText('Vel Desktop')).toBeInTheDocument()
    expect(screen.getAllByText(/sync or queued-write posture needs review/i).length).toBeGreaterThan(0)
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Standup check-in').length).toBeGreaterThan(0)
    expect(screen.getByText('Tasks')).toBeInTheDocument()
    expect(screen.getAllByText('Design review').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Reply to Dimitri').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Review execution handoff for runtime lane').length).toBeGreaterThan(0)
    expect(screen.getByPlaceholderText(/ask, capture, or talk to vel/i)).toBeInTheDocument()

    fireEvent.click(screen.getByText(/more context and controls/i))

    expect(screen.getByText('Overview')).toBeInTheDocument()
    expect(screen.getByText('Today')).toBeInTheDocument()
    expect(screen.getByText('Visible nudge')).toBeInTheDocument()
    expect(screen.getByText('Why + state')).toBeInTheDocument()
    expect(screen.getByText(/today has a bounded plan/i)).toBeInTheDocument()
    expect(screen.getByText('Day changed')).toBeInTheDocument()
    expect(screen.getByText('1 moved')).toBeInTheDocument()
    expect(screen.getByText('1 unscheduled')).toBeInTheDocument()
    expect(screen.getByText('1 needs judgment')).toBeInTheDocument()
    expect(screen.getByText('Deep work')).toBeInTheDocument()
    expect(screen.getByText('block:focus')).toBeInTheDocument()
    expect(screen.getAllByText('Standup check-in').length).toBeGreaterThan(0)
    expect(screen.getByText('Trust and readiness')).toBeInTheDocument()
    expect(screen.getByText('Waiting elsewhere')).toBeInTheDocument()
    expect(screen.getByText('1 waiting for Inbox triage')).toBeInTheDocument()
    expect(screen.getByText('Backup is stale')).toBeInTheDocument()
    expect(screen.getAllByText('Confirm the design review agenda').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Review execution handoff for runtime lane').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Continue in Threads').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Reflow moved to Threads').length).toBeGreaterThan(0)
    expect(screen.getByText('Next event')).toBeInTheDocument()
    expect(screen.getAllByText('Design review').length).toBeGreaterThan(0)
    expect(screen.getByRole('button', { name: /start morning/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /start standup/i })).toBeInTheDocument()
  })

  it('routes compact header, nudge, and task actions through thread and settings handlers', async () => {
    const onOpenThread = vi.fn()
    const onOpenSettings = vi.fn()

    render(<NowView onOpenThread={onOpenThread} onOpenSettings={onOpenSettings} />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /needs input/i })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /needs input/i }))
    fireEvent.click(screen.getAllByRole('button', { name: /continue in threads/i })[0] as HTMLElement)
    fireEvent.click(screen.getByRole('button', { name: /open settings/i }))
    fireEvent.click(screen.getByRole('button', { name: /open thread/i }))

    expect(onOpenThread).toHaveBeenNthCalledWith(1, 'thr_check_in_1')
    expect(onOpenThread).toHaveBeenNthCalledWith(2, 'thr_check_in_1')
    expect(onOpenSettings).toHaveBeenCalledWith({ tab: 'runtime' })
    expect(onOpenThread).toHaveBeenLastCalledWith('thr_exec_1')
  })

  it('renders compact thread-backed reflow status without resurfacing the reflow card', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            reflow: null,
            reflow_status: {
              kind: 'editing',
              trigger: 'missed_event',
              severity: 'critical',
              headline: 'Reflow moved to Threads',
              detail: 'Vel opened a thread-backed reflow follow-up so the day plan can be shaped before anything else changes.',
              recorded_at: 1710000150,
              preview_lines: ['Next scheduled event started 20 minutes ago.'],
              thread_id: 'thr_reflow_1',
            },
          }),
          meta: { request_id: 'req_now_reflow_status' },
        } as never
      }
      if (path.startsWith('/v1/daily-loop/sessions/active?')) {
        return { ok: true, data: null, meta: { request_id: 'req_daily_loop_none' } } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText("Jove's Now")).toBeInTheDocument()
    })

    fireEvent.click(screen.getByText(/more context and controls/i))

    expect(screen.getByText('Reflow moved to Threads')).toBeInTheDocument()
    expect(screen.getAllByText('Continue in Threads').length).toBeGreaterThan(0)
    expect(screen.getByText('Thread thr_reflow_1')).toBeInTheDocument()
    expect(screen.getByText('Next scheduled event started 20 minutes ago.')).toBeInTheDocument()
    expect(screen.queryByText('1 moved')).not.toBeInTheDocument()
  })

  it('shows backend-owned suggestion fallback when no dominant action exists', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            overview: {
              dominant_action: null,
              today_timeline: [
                {
                  kind: 'now',
                  title: 'Current time',
                  timestamp: 1710000000,
                  detail: null,
                },
              ],
              visible_nudge: null,
              why_state: [{ label: 'Attention', detail: 'No strong pressure is active.' }],
              suggestions: [
                {
                  id: 'suggest_1',
                  kind: 'commitment',
                  title: 'Write weekly review',
                  summary: 'Continue the next open commitment.',
                },
                {
                  id: 'suggest_2',
                  kind: 'calendar_event',
                  title: 'Design review',
                  summary: 'Review the next calendar anchor before committing new work.',
                },
              ],
              decision_options: ['accept', 'choose', 'thread', 'close'],
            },
          }),
          meta: { request_id: 'req_now_suggestions' },
        } as never
      }
      if (path.startsWith('/v1/daily-loop/sessions/active?')) {
        return { ok: true, data: null, meta: { request_id: 'req_daily_loop_none' } } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText('Choose the next bounded move')).toBeInTheDocument()
    })

    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Continue the next open commitment.').length).toBeGreaterThan(0)
    expect(screen.getByText('accept')).toBeInTheDocument()
    expect(screen.getByText('thread')).toBeInTheDocument()
  })

  it('dedupes duplicate now action suggestions before rendering the compact context panel', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            action_items: [
              {
                id: 'act_1',
                title: 'Review execution handoff for runtime lane',
                summary: 'The runtime lane still needs an approval decision.',
                kind: 'execution_review',
                rank: 1,
                surface: 'now',
                project_label: 'Runtime',
                evidence: [],
                thread_route: {
                  thread_id: 'thr_exec_1',
                  label: 'Execution thread',
                },
              },
              {
                id: 'act_2',
                title: 'Review execution handoff for runtime lane',
                summary: 'The runtime lane still needs an approval decision.',
                kind: 'execution_review',
                rank: 2,
                surface: 'now',
                project_label: 'Runtime',
                evidence: [],
                thread_route: {
                  thread_id: 'thr_exec_1',
                  label: 'Execution thread',
                },
              },
            ],
          }),
          meta: { request_id: 'req_now_deduped_actions' },
        } as never
      }
      return {
        ok: true,
        data: null,
        meta: { request_id: `req_${path.replaceAll('/', '_')}` },
      } as never
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText(/more context and controls/i)).toBeInTheDocument()
    })

    fireEvent.click(screen.getByText(/more context and controls/i))

    const matchingRows = screen.getAllByText('Review execution handoff for runtime lane')
    expect(matchingRows).toHaveLength(1)
  })

  it('starts morning and resumes the next backend-owned prompt in place', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /start morning/i })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /start morning/i }))

    await waitFor(() => {
      expect(screen.getByText('What most needs to happen before noon?')).toBeInTheDocument()
    })

    fireEvent.change(screen.getByPlaceholderText(/type a brief response/i), {
      target: { value: 'Ship Phase 10 before lunch.' },
    })
    fireEvent.click(screen.getByRole('button', { name: /submit response/i }))

    await waitFor(() => {
      expect(screen.getByText('What could derail today if ignored?')).toBeInTheDocument()
    })

    expect(api.apiPost).toHaveBeenCalledWith(
      '/v1/daily-loop/sessions',
      expect.objectContaining({ phase: 'morning_overview' }),
      expect.any(Function),
    )
    expect(api.apiPost).toHaveBeenCalledWith(
      '/v1/daily-loop/sessions/dls_morning_1/turn',
      expect.objectContaining({
        action: 'submit',
        response_text: 'Ship Phase 10 before lunch.',
      }),
      expect.any(Function),
    )
  })

  it('completes standup and shows the saved outcome while refreshing now', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /start standup/i })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /start standup/i }))

    await waitFor(() => {
      expect(screen.getByText('Name the one to three commitments that matter most today.')).toBeInTheDocument()
    })

    fireEvent.change(screen.getByPlaceholderText(/type one concise answer/i), {
      target: { value: 'Ship Phase 10, Review runtime PR' },
    })
    fireEvent.click(screen.getByRole('button', { name: /submit response/i }))

    await waitFor(() => {
      expect(screen.getByText('Standup saved.')).toBeInTheDocument()
    })

    expect(screen.getAllByText('MUST · Ship Phase 10').length).toBeGreaterThan(0)
    expect(screen.getAllByText('SHOULD · Review runtime PR').length).toBeGreaterThan(0)
    expect(vi.mocked(api.apiGet).mock.calls.filter(([path]) => path === '/v1/now').length).toBeGreaterThan(1)
  })

  it('surfaces degraded freshness warnings while keeping stale data visible', async () => {
    vi.mocked(api.apiGet).mockImplementationOnce(async () => ({
      ok: true,
      data: buildNowData({
        computed_at: 1710000000,
        schedule: {
          empty_message: null,
          next_event: {
            title: 'Design review',
            start_ts: 1710003600,
            end_ts: 1710007200,
            location: 'Room 4B',
            prep_minutes: 15,
            travel_minutes: 0,
            leave_by_ts: 1710003600,
          },
          upcoming_events: [
            {
              title: 'Design review',
              start_ts: 1710003600,
              end_ts: 1710007200,
              location: 'Room 4B',
              prep_minutes: 15,
              travel_minutes: 0,
              leave_by_ts: 1710003600,
            },
          ],
        },
        tasks: {
          todoist: [
            {
              id: 'commit_todoist_1',
              text: 'Reply to Dimitri',
              source_type: 'todoist',
              due_at: '2026-03-16T19:00:00Z',
              project: 'Ops',
              commitment_kind: 'todo',
            },
          ],
          other_open: [],
          next_commitment: null,
        },
        attention: {
          state: { key: 'on_task', label: 'On task' },
          drift: { key: 'none', label: 'None' },
          severity: { key: 'none', label: 'None' },
          confidence: 0.8,
          reasons: [],
        },
        sources: {
          git_activity: null,
          note_document: null,
          assistant_message: null,
        },
        freshness: {
          overall_status: 'stale',
          sources: [
            {
              key: 'context',
              label: 'Context',
              status: 'aging',
              last_sync_at: 1709999400,
              age_seconds: 600,
              guidance: 'Re-run evaluate soon.',
            },
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'stale',
              last_sync_at: 1709990000,
              age_seconds: 10000,
              guidance: 'Calendar sync failed earlier. Inspect history and retry sync.',
            },
            {
              key: 'todoist',
              label: 'Todoist',
              status: 'error',
              last_sync_at: 1709995000,
              age_seconds: 5000,
              guidance: 'Todoist sync failed. Inspect history and retry sync.',
            },
          ],
        },
        reasons: ['Prep window active'],
      }),
      meta: { request_id: 'req_now_degraded' },
    }) as never)

    render(<NowView />)

    fireEvent.click(await screen.findByText(/more context and controls/i))

    await waitFor(() => {
      expect(screen.getByText(/Some inputs need a refresh/i)).toBeInTheDocument()
    })
    expect(screen.getByText(/Calendar: Stale/i)).toBeInTheDocument()
    expect(screen.getByText(/Todoist: Error/i)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /sync calendar/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /sync todoist/i })).toBeInTheDocument()
    expect(screen.getAllByText('Design review').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Reply to Dimitri').length).toBeGreaterThan(0)
  })

  it('runs evaluate directly from degraded context warnings', async () => {
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce({
        ok: true,
        data: buildNowData({
          computed_at: 1710000000,
          freshness: {
            overall_status: 'aging',
            sources: [
              {
                key: 'context',
                label: 'Context',
                status: 'aging',
                last_sync_at: 1709999400,
                age_seconds: 600,
                guidance: 'Re-run evaluate soon.',
              },
            ],
          },
        }),
        meta: { request_id: 'req_now_degraded_context' },
      } as never)
      .mockResolvedValueOnce({
        ok: true,
        data: buildNowData({
          computed_at: 1710000300,
          summary: {
            mode: { key: 'day_mode', label: 'Day' },
            phase: { key: 'engaged', label: 'Engaged' },
            meds: { key: 'pending', label: 'Pending' },
            risk: { level: 'low', score: 0.32, label: 'low · 32%' },
          },
          freshness: {
            overall_status: 'fresh',
            sources: [
              {
                key: 'context',
                label: 'Context',
                status: 'fresh',
                last_sync_at: 1710000300,
                age_seconds: 0,
                guidance: null,
              },
            ],
          },
        }),
        meta: { request_id: 'req_now_refreshed_context' },
      } as never)

    render(<NowView />)

    fireEvent.click(await screen.findByText(/more context and controls/i))

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /re-run evaluate/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /re-run evaluate/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith('/v1/evaluate', {}, expect.any(Function))
    })
    await waitFor(() => {
      expect(vi.mocked(api.apiGet).mock.calls.filter(([path]) => path === '/v1/now').length).toBeGreaterThan(1)
    })
    expect(screen.queryByText(/Some inputs need a refresh/i)).not.toBeInTheDocument()
  })

  it('retries calendar sync directly from degraded freshness warnings', async () => {
    const staleNow = {
      ok: true,
      data: buildNowData({
        computed_at: 1710000000,
        freshness: {
          overall_status: 'stale',
          sources: [
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'stale',
              last_sync_at: 1709990000,
              age_seconds: 10000,
              guidance: 'Calendar sync failed earlier. Inspect history and retry sync.',
            },
          ],
        },
      }),
      meta: { request_id: 'req_now_calendar_stale' },
    }
    const freshNow = {
      ok: true,
      data: {
        ...staleNow.data,
        computed_at: 1710000300,
        freshness: {
          overall_status: 'fresh',
          sources: [
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'fresh',
              last_sync_at: 1710000300,
              age_seconds: 0,
              guidance: null,
            },
          ],
        },
      },
      meta: { request_id: 'req_now_calendar_fresh' },
    }
    let nowCalls = 0
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        nowCalls += 1
        return (nowCalls === 1 ? staleNow : freshNow) as never
      }
      if (path.startsWith('/v1/daily-loop/sessions/active?')) {
        return {
          ok: true,
          data: null,
          meta: { request_id: 'req_daily_loop_active' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })

    render(<NowView />)

    fireEvent.click(await screen.findByText(/more context and controls/i))

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /sync calendar/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /sync calendar/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith('/v1/sync/calendar', {}, expect.any(Function))
    })
    await waitFor(() => {
      expect(vi.mocked(api.apiGet).mock.calls.filter(([path]) => path === '/v1/now').length).toBeGreaterThan(1)
    })
  })

  it('refetches on focus and reveals debug payload on demand', async () => {
    const initial = {
      ok: true,
      data: buildNowData({
        computed_at: 1710000000,
        debug: {
          raw_context: { mode: 'day_mode' },
          signals_used: ['sig_1'],
          commitments_used: ['commit_1'],
          risk_used: ['risk_1'],
        },
      }),
      meta: { request_id: 'req_now_1' },
    }
    const refreshed = {
      ...initial,
      data: {
        ...initial.data,
        computed_at: 1710000300,
        status_row: {
          ...initial.data.status_row,
          context_label: 'Meeting prep',
        },
      },
      meta: { request_id: 'req_now_2' },
    }
    let nowCalls = 0
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        nowCalls += 1
        return (nowCalls === 1 ? initial : refreshed) as never
      }
      if (path.startsWith('/v1/daily-loop/sessions/active?')) {
        return {
          ok: true,
          data: null,
          meta: { request_id: 'req_daily_loop_active' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText("Jove's Now")).toBeInTheDocument()
    })

    fireEvent(window, new Event('focus'))

    await waitFor(() => {
      expect(screen.getAllByText('Meeting prep').length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getByText(/more context and controls/i))
    fireEvent.click(screen.getByText(/show raw fields/i))
    expect(screen.getByText(/"signals_used": \[/i)).toBeInTheDocument()
    expect(screen.getByText(/"risk_used": \[/i)).toBeInTheDocument()
  })

  it('registers a background refresh interval', async () => {
    const setIntervalSpy = vi.spyOn(window, 'setInterval')
    const initial = {
      ok: true,
      data: buildNowData({
        computed_at: 1710000000,
      }),
      meta: { request_id: 'req_now_1' },
    }
    const refreshed = {
      ...initial,
      data: {
        ...initial.data,
        summary: {
          ...initial.data.summary,
          phase: { key: 'underway', label: 'Underway' },
        },
      },
    }
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce(initial as never)
      .mockResolvedValueOnce(refreshed as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText("Jove's Now")).toBeInTheDocument()
    })
    expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 60_000)
    setIntervalSpy.mockRestore()
  })

  it('opens integration settings for non-retryable degraded sources', async () => {
    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: buildNowData({
        computed_at: 1710000000,
        freshness: {
          overall_status: 'disconnected',
          sources: [
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'disconnected',
              last_sync_at: null,
              age_seconds: null,
              guidance: 'Connect Google before syncing calendar data.',
            },
            {
              key: 'activity',
              label: 'Computer activity',
              status: 'missing',
              last_sync_at: null,
              age_seconds: null,
              guidance: 'Configure a source path for this local adapter before syncing it.',
            },
          ],
        },
      }),
      meta: { request_id: 'req_now_settings' },
    } as never)

    const onOpenSettings = vi.fn()
    render(<NowView onOpenSettings={onOpenSettings} />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /open google settings/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /open google settings/i })[0] as HTMLElement)
    fireEvent.click(screen.getAllByRole('button', { name: /open source settings/i })[0] as HTMLElement)

    expect(onOpenSettings).toHaveBeenNthCalledWith(1, { tab: 'integrations', integrationId: 'google' })
    expect(onOpenSettings).toHaveBeenNthCalledWith(2, { tab: 'integrations', integrationId: 'activity' })
  })

  it('submits assistant entry from Now and follows backend inbox routing', async () => {
    vi.mocked(api.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        route_target: 'inbox',
        user_message: {
          id: 'msg_now_1',
          conversation_id: 'conv_now_1',
          role: 'user',
          kind: 'text',
          content: { text: 'Remember to send the update' },
          status: null,
          importance: null,
          created_at: 1,
          updated_at: null,
        },
        assistant_message: null,
        assistant_error: null,
        conversation: {
          id: 'conv_now_1',
          title: 'Capture',
          kind: 'general',
          pinned: false,
          archived: true,
          created_at: 1,
          updated_at: 1,
        },
      },
      meta: { request_id: 'req_now_entry_inbox' },
    } as never)

    const onOpenInbox = vi.fn()
    const onOpenThread = vi.fn()
    render(<NowView onOpenInbox={onOpenInbox} onOpenThread={onOpenThread} />)

    await waitFor(() => {
      expect(screen.getByPlaceholderText(/ask, capture, or talk to vel/i)).toBeInTheDocument()
    })
    expect(
      screen.getByText(/type or hold the mic to talk locally/i),
    ).toBeInTheDocument()

    fireEvent.change(screen.getByPlaceholderText(/ask, capture, or talk to vel/i), {
      target: { value: 'Remember to send the update' },
    })
    fireEvent.click(screen.getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/assistant/entry',
        { text: 'Remember to send the update' },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(onOpenInbox).toHaveBeenCalledTimes(1)
    })
    expect(onOpenThread).not.toHaveBeenCalled()
  })

  it('renders inline assistant replies in Now when the backend returns inline handling', async () => {
    vi.mocked(api.apiPost).mockResolvedValueOnce({
      ok: true,
      data: {
        route_target: 'inline',
        user_message: {
          id: 'msg_now_2',
          conversation_id: 'conv_now_2',
          role: 'user',
          kind: 'text',
          content: { text: 'Quick status check' },
          status: null,
          importance: null,
          created_at: 2,
          updated_at: null,
        },
        assistant_message: {
          id: 'msg_now_3',
          conversation_id: 'conv_now_2',
          role: 'assistant',
          kind: 'text',
          content: { text: 'You are clear until the next meeting.' },
          status: null,
          importance: null,
          created_at: 3,
          updated_at: null,
        },
        assistant_error: null,
        conversation: {
          id: 'conv_now_2',
          title: 'Inline',
          kind: 'general',
          pinned: false,
          archived: false,
          created_at: 2,
          updated_at: 3,
        },
      },
      meta: { request_id: 'req_now_entry_inline' },
    } as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByPlaceholderText(/ask, capture, or talk to vel/i)).toBeInTheDocument()
    })

    fireEvent.change(screen.getByPlaceholderText(/ask, capture, or talk to vel/i), {
      target: { value: 'Quick status check' },
    })
    fireEvent.click(screen.getByRole('button', { name: /send/i }))

    await waitFor(() => {
      expect(screen.getByText(/handled here in now/i)).toBeInTheDocument()
    })
    expect(screen.getByText(/you are clear until the next meeting\./i)).toBeInTheDocument()
  })

  it('renders the bounded day plan in Now when the backend includes planner output', async () => {
    render(<NowView />)

    fireEvent.click(await screen.findByText(/more context and controls/i))

    await waitFor(() => {
      expect(screen.getByText(/today has a bounded plan/i)).toBeInTheDocument()
    })

    expect(screen.getByText(/2 scheduled/i)).toBeInTheDocument()
    expect(screen.getByText(/1 deferred/i)).toBeInTheDocument()
    expect(screen.getByText(/inferred routine blocks/i)).toBeInTheDocument()
    expect(screen.getAllByText(/morning routine/i).length).toBeGreaterThan(0)
    expect(screen.getByText(/backend-inferred from current context/i)).toBeInTheDocument()
    expect(screen.getByText(/backlog cleanup is marked for local defer/i)).toBeInTheDocument()
  })

  it('surfaces planning-profile proposal continuity without turning Now into a planner', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            planning_profile_summary: {
              pending_count: 1,
              latest_pending: {
                thread_id: 'thr_planning_profile_edit_1',
                state: 'staged',
                title: 'Add shutdown block',
                summary: 'Add a protected shutdown block.',
                outcome_summary: null,
                updated_at: 1710000000,
              },
              latest_applied: {
                thread_id: 'thr_planning_profile_edit_0',
                state: 'applied',
                title: 'Save morning focus window',
                summary: 'Save a morning focus window.',
                outcome_summary:
                  'Planning-profile proposal applied through canonical mutation seam.',
                updated_at: 1709990000,
              },
              latest_failed: null,
            },
          }),
          meta: { request_id: 'req_now_planning_profile_summary' },
        } as never
      }
      return {
        ok: true,
        data: null,
        meta: { request_id: `req_${path.replaceAll('/', '_')}` },
      } as never
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText(/routine edits stay review-gated/i)).toBeInTheDocument()
    })

    expect(screen.getByText(/1 pending/i)).toBeInTheDocument()
    expect(screen.getByText(/Pending: Add shutdown block/i)).toBeInTheDocument()
    expect(screen.getByText(/Last applied: Save morning focus window/i)).toBeInTheDocument()
  })

  it('surfaces same-day scheduling continuity without turning Now into a planner', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            commitment_scheduling_summary: {
              pending_count: 1,
              latest_pending: {
                thread_id: 'thr_day_plan_apply_1',
                state: 'staged',
                title: 'Apply focus block shift',
                summary: 'Move the focus block after the calendar anchor.',
                outcome_summary: null,
                updated_at: 1710000000,
              },
              latest_applied: {
                thread_id: 'thr_reflow_edit_0',
                state: 'applied',
                title: 'Clear stale due time',
                summary: 'Remove the stale due time from one commitment.',
                outcome_summary:
                  'Commitment scheduling proposal applied through canonical mutation seam.',
                updated_at: 1709990000,
              },
              latest_failed: null,
            },
          }),
          meta: { request_id: 'req_now_commitment_scheduling_summary' },
        } as never
      }
      return {
        ok: true,
        data: null,
        meta: { request_id: `req_${path.replaceAll('/', '_')}` },
      } as never
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText(/schedule edits stay supervised/i)).toBeInTheDocument()
    })

    expect(screen.getByText(/1 pending/i)).toBeInTheDocument()
    expect(screen.getByText(/Pending: Apply focus block shift/i)).toBeInTheDocument()
    expect(screen.getByText(/Last applied: Clear stale due time/i)).toBeInTheDocument()
  })

  it('keeps ranked thread pressure in the compact context lane instead of resurfacing a separate panel', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText(/waiting elsewhere/i)).toBeInTheDocument()
    })

    expect(screen.queryByText(/resume thread/i)).not.toBeInTheDocument()
    expect(screen.getAllByText('Review execution handoff for runtime lane').length).toBeGreaterThan(0)
    expect(screen.getByText(/need continuity/i)).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /open execution thread/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /open related threads/i })).not.toBeInTheDocument()
  })
})
