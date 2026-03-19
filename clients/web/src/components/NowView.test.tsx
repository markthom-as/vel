import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { NowView } from './NowView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

describe('NowView', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()
    vi.useRealTimers()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
    let morningSession: Record<string, unknown> | null = null
    let standupSession: Record<string, unknown> | null = null
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
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
                title: 'Confirm the design review agenda',
                summary: 'Prep materials and confirm the current owner before the meeting starts.',
                project_id: 'proj_ops',
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
              },
              {
                id: 'action_2',
                surface: 'now',
                kind: 'review',
                title: 'Review execution handoff for runtime lane',
                summary: 'The supervised coding handoff is waiting for explicit approval.',
                project_id: 'proj_exec',
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
              },
            ],
            review_snapshot: {
              open_action_count: 2,
              triage_count: 1,
              projects_needing_review: 1,
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
          },
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
          }
        } else {
          const completedMorning = {
            ...morningSession,
            status: 'completed',
            turn_state: 'completed',
            current_prompt: null,
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
      expect(screen.getByText('What matters right now')).toBeInTheDocument()
    })

    expect(screen.getAllByText('Day').length).toBeGreaterThan(0)
    expect(screen.getByText('Engaged')).toBeInTheDocument()
    expect(screen.getByText('Pending')).toBeInTheDocument()
    expect(screen.getByText(/medium · 72%/i)).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Room 4B')).toBeInTheDocument()
    expect(screen.getByText(/prep 15m/i)).toBeInTheDocument()
    expect(screen.getByText(/travel 0m/i)).toBeInTheDocument()
    expect(screen.getAllByText('Reply to Dimitri').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getByText('Action stack')).toBeInTheDocument()
    expect(screen.getByText('Confirm the design review agenda')).toBeInTheDocument()
    expect(screen.getByText('Review execution handoff for runtime lane')).toBeInTheDocument()
    expect(screen.getByText('Design review at 10:00')).toBeInTheDocument()
    expect(screen.getByText('2 open actions')).toBeInTheDocument()
    expect(screen.getByText('1 execution reviews pending')).toBeInTheDocument()
    expect(
      screen
        .getByText('Confirm the design review agenda')
        .compareDocumentPosition(screen.getByText('Review execution handoff for runtime lane')) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).toBeTruthy()
    expect(screen.getByText('Execution review')).toBeInTheDocument()
    expect(screen.getByText('write_scope_requires_approval | write scopes: /tmp/vel')).toBeInTheDocument()
    expect(screen.getByText('Prep window active')).toBeInTheDocument()
    expect(screen.getByText('recent git activity indicates active work')).toBeInTheDocument()
    expect(screen.getByText('Recent source activity')).toBeInTheDocument()
    expect(screen.getByText('repo: vel')).toBeInTheDocument()
    expect(screen.getByText('metric type: resting_heart_rate')).toBeInTheDocument()
    expect(screen.getByText('value: 58')).toBeInTheDocument()
    expect(screen.getByText('source app: Apple Health')).toBeInTheDocument()
    expect(screen.getByText('path: daily/today.md')).toBeInTheDocument()
    expect(screen.getByText('conversation id: conv_external')).toBeInTheDocument()
    expect(screen.getByText('People status')).toBeInTheDocument()
    expect(screen.getAllByText('Annie Case').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Fresh').length).toBeGreaterThan(0)
    expect(screen.getByRole('button', { name: /start morning/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /start standup/i })).toBeInTheDocument()
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
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
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
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
      meta: { request_id: 'req_now_degraded' },
    }) as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText(/Some inputs are degraded/i)).toBeInTheDocument()
    })

    expect(screen.getByText(/Calendar: Stale/i)).toBeInTheDocument()
    expect(screen.getByText(/Todoist: Error/i)).toBeInTheDocument()
    expect(
      screen.getByText('Calendar is stale. Upcoming events may be out of date.'),
    ).toBeInTheDocument()
    expect(
      screen.getByText('Todoist sync last failed. Backlog state may be incomplete.'),
    ).toBeInTheDocument()
    expect(screen.getAllByText(/Calendar sync failed earlier\. Inspect history and retry sync\./i).length).toBeGreaterThan(0)
    expect(screen.getAllByText(/Todoist sync failed\. Inspect history and retry sync\./i).length).toBeGreaterThan(0)
    expect(
      screen.getByText('Current context is aging. Evaluate soon if you need fresher state.'),
    ).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
  })

  it('runs evaluate directly from degraded context warnings', async () => {
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce({
        ok: true,
        data: {
          computed_at: 1710000000,
          timezone: 'America/Denver',
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
            note_document: null,
            assistant_message: null,
          },
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
          reasons: [],
          debug: {
            raw_context: {},
            signals_used: [],
            commitments_used: [],
            risk_used: [],
          },
        },
        meta: { request_id: 'req_now_degraded_context' },
      } as never)
      .mockResolvedValueOnce({
        ok: true,
        data: {
          computed_at: 1710000300,
          timezone: 'America/Denver',
          summary: {
            mode: { key: 'day_mode', label: 'Day' },
            phase: { key: 'engaged', label: 'Engaged' },
            meds: { key: 'pending', label: 'Pending' },
            risk: { level: 'low', score: 0.32, label: 'low · 32%' },
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
            note_document: null,
            assistant_message: null,
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
          reasons: [],
          debug: {
            raw_context: {},
            signals_used: [],
            commitments_used: [],
            risk_used: [],
          },
        },
        meta: { request_id: 'req_now_refreshed_context' },
      } as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /re-run evaluate/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /re-run evaluate/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith('/v1/evaluate', {}, expect.any(Function))
    })
    await waitFor(() => {
      expect(screen.getByText('Context refreshed.')).toBeInTheDocument()
    })
    expect(screen.getAllByText('Fresh').length).toBeGreaterThan(0)
  })

  it('retries calendar sync directly from degraded freshness warnings', async () => {
    const staleNow = {
      ok: true,
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
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
          note_document: null,
          assistant_message: null,
        },
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
        reasons: [],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
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

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /sync calendar/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /sync calendar/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith('/v1/sync/calendar', {}, expect.any(Function))
    })
    await waitFor(() => {
      expect(screen.getByText('Calendar synced (3 signals).')).toBeInTheDocument()
    })
  })

  it('refetches on focus and reveals debug payload on demand', async () => {
    const initial = {
      ok: true,
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
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
          note_document: null,
          assistant_message: null,
        },
        freshness: {
          overall_status: 'fresh',
          sources: [],
        },
        reasons: [],
        debug: {
          raw_context: { mode: 'day_mode' },
          signals_used: ['sig_1'],
          commitments_used: ['commit_1'],
          risk_used: ['risk_1'],
        },
      },
      meta: { request_id: 'req_now_1' },
    }
    const refreshed = {
      ...initial,
      data: {
        ...initial.data,
        computed_at: 1710000300,
        summary: {
          ...initial.data.summary,
          mode: { key: 'meeting_mode', label: 'Meeting prep' },
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
      expect(screen.getByText('Day')).toBeInTheDocument()
    })

    fireEvent(window, new Event('focus'))

    await waitFor(() => {
      expect(screen.getByText('Meeting prep')).toBeInTheDocument()
    })

    fireEvent.click(screen.getByText(/show raw fields/i))
    expect(screen.getByText(/"signals_used": \[/i)).toBeInTheDocument()
    expect(screen.getByText(/"risk_used": \[/i)).toBeInTheDocument()
  })

  it('registers a background refresh interval', async () => {
    const setIntervalSpy = vi.spyOn(window, 'setInterval')
    const initial = {
      ok: true,
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
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
          note_document: null,
          assistant_message: null,
        },
        freshness: {
          overall_status: 'fresh',
          sources: [],
        },
        reasons: [],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
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
      expect(screen.getByText('Engaged')).toBeInTheDocument()
    })
    expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 60_000)
    setIntervalSpy.mockRestore()
  })

  it('opens integration settings for non-retryable degraded sources', async () => {
    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
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
          note_document: null,
          assistant_message: null,
        },
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
        reasons: [],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
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
})
