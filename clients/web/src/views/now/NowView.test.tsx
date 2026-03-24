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

function buildClusterBootstrapFixture() {
  return {
    node_id: 'vel-desktop',
    node_display_name: 'Vel Desktop',
    active_authority_node_id: 'vel-desktop',
    active_authority_epoch: 1,
    sync_base_url: 'http://127.0.0.1:4130',
    sync_transport: 'localhost',
    tailscale_base_url: null,
    lan_base_url: null,
    localhost_base_url: 'http://127.0.0.1:4130',
    capabilities: [] as string[],
    linked_nodes: [],
    projects: [],
    action_items: [],
  }
}

function buildClusterWorkersFixture() {
  return {
    active_authority_node_id: 'vel-desktop',
    active_authority_epoch: 1,
    generated_at: 1710000000,
    workers: [
      {
        worker_id: 'vel-desktop',
        node_id: 'vel-desktop',
        node_display_name: 'Vel Desktop',
        client_kind: 'vel_web',
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
        tailscale_preferred: false,
        last_heartbeat_at: 1710000000,
        started_at: 1709999900,
        sync_base_url: 'http://127.0.0.1:4130',
        sync_transport: 'localhost',
        tailscale_base_url: null,
        preferred_tailnet_endpoint: null,
        tailscale_reachable: false,
        lan_base_url: null,
        localhost_base_url: 'http://127.0.0.1:4130',
        ping_ms: null,
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
  }
}

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
    schedule: {
      empty_message: null,
      next_event: {
        title: 'Design review',
        start_ts: 1710003600,
        end_ts: 1710007200,
        location: 'Studio',
        prep_minutes: null,
        travel_minutes: null,
        leave_by_ts: null,
      },
      upcoming_events: [
        {
          title: 'Design review',
          start_ts: 1710003600,
          end_ts: 1710007200,
          location: 'Studio',
          prep_minutes: null,
          travel_minutes: null,
          leave_by_ts: null,
        },
      ],
    },
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
  function setupApiMocks(nowPayload: ReturnType<typeof buildNowData>) {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return { ok: true, data: nowPayload, meta: { request_id: 'req_now' } } as never
      }
      if (path === '/v1/cluster/bootstrap') {
        return { ok: true, data: buildClusterBootstrapFixture(), meta: { request_id: 'req_boot' } } as never
      }
      if (path === '/v1/cluster/workers') {
        return { ok: true, data: buildClusterWorkersFixture(), meta: { request_id: 'req_workers' } } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })
  }

  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
    vi.mocked(api.apiPatch).mockReset()
    vi.mocked(api.apiPatch).mockResolvedValue({
      ok: true,
      data: {
        id: 'commit_local_1',
        title: 'Write weekly review',
        detail: null,
        status: 'done',
        source_kind: 'manual',
        source_ref: null,
        due_at: null,
        energy: null,
        urgency: null,
        confidence: null,
        created_at: '2026-03-09T15:00:00Z',
        updated_at: '2026-03-09T16:05:00Z',
        resolved_at: '2026-03-09T16:05:00Z',
        project_id: null,
        nudge_count: 0,
        tags: [],
      },
      meta: { request_id: 'req_patch' },
    } as never)
    setupApiMocks(buildNowData())
  })

  it('renders the approved focus-first now layout without reviving inbox-era affordances', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: 'Now' })).toBeInTheDocument()
    })
    expect(screen.getByText(/Saturday, March 9, 2024.*MST/i)).toBeInTheDocument()
    expect(screen.getByText('No current event | Write weekly review')).toBeInTheDocument()
    expect(screen.getByText('ACTIVE TASK (1)')).toBeInTheDocument()
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getByRole('button', { name: 'Complete Write weekly review' })).toBeInTheDocument()
    expect(screen.getByText('NEXT UP (2)')).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.queryByText('One subordinate slot')).not.toBeInTheDocument()
    expect(screen.queryByText('Current and next event')).not.toBeInTheDocument()
    expect(screen.queryByText('Trust state')).not.toBeInTheDocument()
    expect(screen.queryByText('Standup check-in')).not.toBeInTheDocument()
    expect(screen.queryByText('Vel Desktop needs attention')).not.toBeInTheDocument()
    expect(screen.queryByText('Commitments')).not.toBeInTheDocument()
    expect(screen.queryByText('Calendar')).not.toBeInTheDocument()
    expect(screen.queryByText('Triage')).not.toBeInTheDocument()
    expect(screen.queryByText('Tasks')).not.toBeInTheDocument()
    expect(screen.queryByText('NOW')).not.toBeInTheDocument()
    expect(screen.queryByText('TODAY')).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /Open inbox/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /Reschedule/i })).not.toBeInTheDocument()
    expect(screen.queryByText(/More Context and Controls/i)).not.toBeInTheDocument()
  })

  it('keeps trust remediation outside the main now surface even when degraded', async () => {
    setupApiMocks(buildNowData({
      trust_readiness: {
        level: 'degraded',
        headline: 'Degraded',
        summary: 'Google Calendar is stale and needs review.',
        backup: { level: 'ok', label: 'Backup', detail: 'Backup trust is healthy.' },
        freshness: { level: 'stale', label: 'Freshness', detail: 'Calendar is stale.' },
        review: { open_action_count: 1, triage_count: 1, projects_needing_review: 0, pending_execution_reviews: 0 },
        guidance: [],
        follow_through: [],
      },
    }))

    render(<NowView onOpenSystem={vi.fn()} />)

    await waitFor(() => {
      expect(screen.getAllByText('ACTIVE TASK (1)').length).toBeGreaterThan(0)
    })

    expect(screen.queryByText('Trust state')).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'Open system detail' })).not.toBeInTheDocument()
  })

  it('reconciles commitment completion into the focus-first layout', async () => {
    let currentNow = buildNowData({
      tasks: {
        todoist: [],
        other_open: [
          {
            id: 'commit_local_1',
            text: 'Write weekly review',
            source_type: 'local',
            source_id: null,
            status: 'active',
            due_at: '2026-03-09T17:00:00Z',
            project: null,
            commitment_kind: 'routine',
            created_at: '2026-03-09T15:00:00Z',
            resolved_at: null,
            scheduler_rules: {
              block_target: null,
              duration_minutes: null,
              calendar_free: false,
              fixed_start: false,
              time_window: null,
              local_urgency: false,
              local_defer: false,
            },
            metadata: {},
          },
        ],
      },
    })

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return { ok: true, data: currentNow, meta: { request_id: 'req_now' } } as never
      }
      if (path === '/v1/cluster/bootstrap') {
        return { ok: true, data: buildClusterBootstrapFixture(), meta: { request_id: 'req_boot' } } as never
      }
      if (path === '/v1/cluster/workers') {
        return { ok: true, data: buildClusterWorkersFixture(), meta: { request_id: 'req_workers' } } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    vi.mocked(api.apiPatch).mockImplementation(async () => {
      currentNow = buildNowData({
        task_lane: {
          active: null,
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
              id: 'commit_local_1',
              task_kind: 'commitment',
              text: 'Write weekly review',
              state: 'done',
              project: null,
              primary_thread_id: null,
            },
          ],
          overflow_count: 0,
        },
        tasks: { todoist: [], other_open: [], next_commitment: null },
      })

      return {
        ok: true,
        data: currentNow,
        meta: { request_id: 'req_patch' },
      } as never
    })

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: 'Complete Write weekly review' }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: 'Complete Write weekly review' }).at(0) as HTMLElement)

    await waitFor(() => {
      expect(vi.mocked(api.apiPatch)).toHaveBeenCalledWith(
        '/v1/now/task-lane',
        { commitment_id: 'commit_local_1', lane: 'completed', position: null },
        expect.any(Function),
      )
    })

    await waitFor(() => {
      expect(screen.getAllByText('COMPLETED (1)').length).toBeGreaterThan(0)
    })
    expect(screen.getAllByRole('button', { name: 'Reopen Write weekly review' }).length).toBeGreaterThan(0)
  })
})
