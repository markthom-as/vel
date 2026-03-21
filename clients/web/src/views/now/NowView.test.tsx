import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { NowView } from './NowView'

vi.mock('../../api/client', () => ({
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
          route_target: { bucket: 'needs_input', thread_id: 'conv_1' },
        },
        {
          kind: 'new_nudges',
          count: 2,
          count_display: 'show_nonzero',
          urgent: false,
          route_target: { bucket: 'new_nudges', thread_id: null },
        },
        {
          kind: 'snoozed',
          count: 0,
          count_display: 'show_nonzero',
          urgent: false,
          route_target: { bucket: 'snoozed', thread_id: null },
        },
      ],
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
        primary_thread_id: 'conv_1',
        actions: [{ kind: 'expand', label: 'Continue in Threads' }],
      },
      {
        id: 'mesh_summary_warning',
        kind: 'trust_warning',
        title: 'Vel Desktop needs attention',
        summary: 'Sync or queued-write posture needs review before trusting all cross-client state.',
        urgent: true,
        primary_thread_id: null,
        actions: [{ kind: 'open_settings', label: 'Open settings' }],
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
      recent_completed: [],
      overflow_count: 0,
    },
    docked_input: {
      supported_intents: ['task', 'question'],
      day_thread_id: 'conv_day',
      raw_capture_thread_id: 'conv_capture',
    },
    summary: {
      mode: { key: 'day_mode', label: 'Day' },
      phase: { key: 'engaged', label: 'Engaged' },
      meds: { key: 'pending', label: 'Pending' },
      risk: { level: 'medium', score: 0.72, label: 'medium · 72%' },
    },
    schedule: { empty_message: null, next_event: null, upcoming_events: [] },
    tasks: { todoist: [], other_open: [], next_commitment: null },
    attention: { state: { key: 'on_task', label: 'On task' }, drift: { key: 'none', label: 'None' }, severity: { key: 'none', label: 'None' }, confidence: 0.8, reasons: [] },
    sources: { git_activity: null, health: null, mood: null, pain: null, note_document: null, assistant_message: null },
    freshness: { overall_status: 'fresh', sources: [] },
    trust_readiness: {
      level: 'ok',
      headline: 'Ready',
      summary: 'No trust blockers are active.',
      backup: { level: 'ok', label: 'Backup', detail: 'Backup trust is healthy.' },
      freshness: { level: 'ok', label: 'Freshness', detail: 'Inputs are fresh enough to trust.' },
      review: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 },
      guidance: [],
      follow_through: [],
    },
    check_in: null,
    day_plan: null,
    reflow: null,
    reflow_status: null,
    action_items: [],
    review_snapshot: { open_action_count: 0, triage_count: 0, projects_needing_review: 0, pending_execution_reviews: 0 },
    pending_writebacks: [],
    conflicts: [],
    people: [],
    reasons: [],
    debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
    ...overrides,
  }
}

describe('NowView', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
    vi.mocked(api.apiPatch).mockReset()
  })

  it('renders the compact containerless top area and grouped task container', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: buildNowData(),
      meta: { request_id: 'req_now' },
    } as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: "Jove's Now" })).toBeInTheDocument()
    })
    expect(screen.queryByRole('button', { name: /Needs input 1/i })).not.toBeInTheDocument()
    expect(screen.getByText('Standup check-in')).toBeInTheDocument()
    expect(screen.getByText('Vel Desktop needs attention')).toBeInTheDocument()
    expect(screen.getByText('NOW')).toBeInTheDocument()
    expect(screen.getByText('TODAY')).toBeInTheDocument()
    expect(screen.getByText('Tasks')).toBeInTheDocument()
    expect(screen.queryByText(/More Context and Controls/i)).not.toBeInTheDocument()
  })

  it('routes nudge actions through thread and settings handlers', async () => {
    const onOpenThread = vi.fn()
    const onOpenSettings = vi.fn()

    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: buildNowData(),
      meta: { request_id: 'req_now' },
    } as never)

    render(<NowView onOpenThread={onOpenThread} onOpenSettings={onOpenSettings} />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Continue in Threads/i })).toBeInTheDocument()
    })

    fireEvent.click(screen.getAllByRole('button', { name: /Continue in Threads/i }).at(-1) as HTMLElement)
    expect(onOpenThread).toHaveBeenCalledWith('conv_1')
    expect(screen.getAllByRole('button', { name: /Open settings/i }).length).toBeGreaterThan(0)
  })
})
