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
  const merged = {
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
        project: 'Inbox',
        tags: ['Inbox', 'Deep work'],
        primary_thread_id: null,
        due_at: null,
        deadline: '2024-03-12T00:00:00Z',
      },
      pending: [
        {
          id: 'commit_todoist_1',
          task_kind: 'task',
          text: 'Reply to Dimitri',
          state: 'pending',
          project: 'Ops',
          tags: ['Inbox', 'Urgent'],
          primary_thread_id: null,
          due_at: '2024-03-09T20:30:00Z',
          deadline: '2024-03-11T00:00:00Z',
        },
      ],
      if_time_allows: [
        {
          id: 'commit_todoist_2',
          task_kind: 'task',
          text: 'Sort reference notes',
          state: 'pending',
          project: null,
          tags: ['Inbox', 'Reading'],
          primary_thread_id: null,
          due_at: '2024-03-12T19:00:00Z',
          deadline: '2024-03-14T00:00:00Z',
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
  const computedAt = merged.computed_at as number
  const taskLane = (merged.task_lane ?? {}) as Record<string, unknown>
  const formatMonthDay = (timestamp: string) =>
    new Intl.DateTimeFormat('en-US', {
      timeZone: merged.timezone as string,
      month: 'short',
      day: 'numeric',
    }).format(new Date(timestamp))
  const formatSessionDay = (timestamp: string | number) =>
    new Intl.DateTimeFormat('en-CA', {
      timeZone: merged.timezone as string,
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    }).format(new Date(typeof timestamp === 'number' ? timestamp * 1000 : timestamp))
  const enrichLaneItem = (
    item: Record<string, unknown>,
    lane: 'active' | 'next_up' | 'inbox' | 'if_time_allows' | 'completed',
  ) => {
    const dueAt = typeof item.due_at === 'string' ? item.due_at : null
    const deadline = typeof item.deadline === 'string' ? item.deadline : null
    const dueTs = dueAt ? Date.parse(dueAt) : Number.NaN
    const deadlineTs = deadline ? Date.parse(deadline) : Number.NaN
    const dueLabel =
      item.due_label !== undefined
        ? item.due_label
        : dueAt && Number.isFinite(dueTs) && dueTs < computedAt * 1000
          ? 'Overdue'
          : dueAt && formatSessionDay(dueAt) === formatSessionDay(computedAt)
            ? 'Today'
          : lane === 'active' && !dueAt
            ? 'Committed'
            : lane === 'completed' && !dueAt
              ? 'Done'
              : lane === 'inbox' || lane === 'if_time_allows' || lane === 'completed'
                ? dueAt
                  ? `Due ${formatMonthDay(dueAt)}`
                  : null
                : null
    return {
      ...item,
      title: item.title ?? item.text,
      description: item.description ?? null,
      due_label: dueLabel,
      is_overdue:
        item.is_overdue !== undefined ? item.is_overdue : dueLabel === 'Overdue',
      deadline_label:
        item.deadline_label !== undefined
          ? item.deadline_label
          : deadline
            ? `Deadline ${formatMonthDay(deadline)}`
            : null,
      deadline_passed:
        item.deadline_passed !== undefined
          ? item.deadline_passed
          : Number.isFinite(deadlineTs) && deadlineTs < computedAt * 1000,
    }
  }
  const nextUp = ((taskLane.next_up ?? taskLane.pending ?? []) as Record<string, unknown>[])
    .map((item) => enrichLaneItem(item, 'next_up'))
  const activeItems = ((taskLane.active_items
    ?? (taskLane.active ? [taskLane.active] : [])) as Record<string, unknown>[])
    .map((item) => enrichLaneItem(item, 'active'))
  const inboxItems = ((taskLane.inbox ?? []) as Record<string, unknown>[])
    .map((item) => enrichLaneItem(item, 'inbox'))
  const backlogItems = ((taskLane.if_time_allows ?? []) as Record<string, unknown>[])
    .map((item) => enrichLaneItem(item, 'if_time_allows'))
  const completedItems = ((taskLane.completed ?? taskLane.recent_completed ?? []) as Record<string, unknown>[])
    .map((item) => enrichLaneItem(item, 'completed'))
  const existingBars = (merged.nudge_bars ?? []) as Record<string, unknown>[]
  const hasOverdueBar = existingBars.some((bar) => bar.id === 'todoist_overdue_backlog')
  const openTasks = [
    ...((merged.tasks?.todoist ?? []) as Record<string, unknown>[]),
    ...((merged.tasks?.other_open ?? []) as Record<string, unknown>[]),
    ...((merged.tasks?.next_commitment ? [merged.tasks.next_commitment] : []) as Record<string, unknown>[]),
  ]
  const overdueItems = [...nextUp, ...inboxItems, ...backlogItems, ...activeItems]
    .filter((item) => item.is_overdue)
  const overdueIds = overdueItems.length > 0
    ? overdueItems.map((item) => item.id)
    : openTasks
        .filter((item) => typeof item.due_at === 'string' && Date.parse(item.due_at) < computedAt * 1000)
        .map((item) => item.id as string)
  const overdueBar = !hasOverdueBar && overdueIds.length > 0
    ? [{
        id: 'todoist_overdue_backlog',
        kind: 'nudge',
        title: `${overdueIds.length} overdue ${overdueIds.length === 1 ? 'item is' : 'items are'} still unresolved`,
        summary: 'Overdue work stays visible until you commit it into the day, keep it in backlog, or reschedule it to today without committing it.',
        urgent: true,
        primary_thread_id: 'conv_day',
        actions: [
          { kind: `reschedule_today:${overdueIds.join(',')}`, label: 'Reschedule all to today' },
          { kind: 'jump_backlog:now-backlog', label: 'Review backlog' },
          { kind: 'open_thread', label: 'Open thread' },
        ],
      }]
    : []
  return {
    ...merged,
    nudge_bars: [...existingBars, ...overdueBar],
    task_lane: {
      ...taskLane,
      active: activeItems[0] ?? null,
      active_items: activeItems,
      next_up: nextUp,
      pending: taskLane.pending ?? nextUp,
      inbox: inboxItems,
      if_time_allows: backlogItems,
      completed: completedItems,
      recent_completed: taskLane.recent_completed ?? completedItems,
    },
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
    setupApiMocks(buildNowData({
      task_lane: {
        active: {
          id: 'commit_local_1',
          task_kind: 'commitment',
          text: 'Write weekly review',
          state: 'active',
          project: 'Inbox',
          tags: ['Inbox', 'Deep work'],
          primary_thread_id: null,
          due_at: null,
          deadline: '2024-03-12T00:00:00Z',
        },
        pending: [
          {
            id: 'commit_todoist_1',
            task_kind: 'task',
            text: 'Reply to Dimitri',
            state: 'pending',
            project: 'Ops',
            tags: ['Inbox', 'Urgent'],
            primary_thread_id: null,
            due_at: '2024-03-09T20:30:00Z',
            deadline: '2024-03-11T00:00:00Z',
          },
        ],
        active_items: [
          {
            id: 'commit_local_1',
            task_kind: 'commitment',
            text: 'Write weekly review',
            state: 'active',
            project: 'Inbox',
            tags: ['Inbox', 'Deep work'],
            primary_thread_id: null,
            due_at: null,
            deadline: '2024-03-12T00:00:00Z',
          },
        ],
        next_up: [
          {
            id: 'commit_todoist_1',
            task_kind: 'task',
            text: 'Reply to Dimitri',
            state: 'pending',
            project: 'Ops',
            tags: ['Inbox', 'Urgent'],
            primary_thread_id: null,
            due_at: '2024-03-09T20:30:00Z',
            deadline: '2024-03-11T00:00:00Z',
          },
        ],
        inbox: [
          {
            id: 'todoist_inbox_1',
            task_kind: 'task',
            text: 'Inbox item',
            title: 'Inbox item',
            description: null,
            state: 'pending',
            project: null,
            tags: [],
            primary_thread_id: null,
            due_at: null,
            deadline: null,
          },
        ],
        if_time_allows: [
          {
            id: 'commit_todoist_2',
            task_kind: 'task',
            text: 'Sort reference notes',
            state: 'pending',
            project: null,
            tags: ['Inbox', 'Reading'],
            primary_thread_id: null,
            due_at: '2024-03-12T19:00:00Z',
            deadline: '2024-03-14T00:00:00Z',
          },
        ],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'todoist_inbox_1',
            text: 'Inbox item',
            title: 'Inbox item',
            description: null,
            tags: [],
            source_type: 'todoist',
            due_at: null,
            deadline: null,
            project: null,
            commitment_kind: 'todo',
          },
        ],
        other_open: [],
      next_commitment: null,
      },
    }))
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: /Now/ })).toBeInTheDocument()
    })
    expect(screen.getByText(/Saturday, March 9, 2024.*MST/i)).toBeInTheDocument()
    expect(screen.getByText('No current event | Write weekly review')).toBeInTheDocument()
    expect(screen.getByText('ACTIVE TASK (1)')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'ACTIVE TASK (1)' })).toHaveAttribute('aria-expanded', 'true')
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getByRole('button', { name: 'Complete Write weekly review' })).toBeInTheDocument()
    expect(screen.getByText('NEXT UP (2)')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'NEXT UP (2)' })).toHaveAttribute('aria-expanded', 'true')
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('INBOX (1)')).toBeInTheDocument()
    expect(screen.getAllByRole('button', { name: 'INBOX (1)' }).at(0)).toHaveAttribute('aria-expanded', 'false')
    expect(screen.getByText('BACKLOG (1)')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'BACKLOG (1)' })).toHaveAttribute('aria-expanded', 'false')
    expect(screen.queryByText('Sort reference notes')).not.toBeInTheDocument()
    expect(screen.queryByText('Inbox item')).not.toBeInTheDocument()
    expect(screen.getByText('Deadline Mar 11')).toBeInTheDocument()
    expect(screen.getByText('Deadline Mar 10')).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'NEXT UP (2)' }))
    expect(screen.getByRole('button', { name: 'NEXT UP (2)' })).toHaveAttribute('aria-expanded', 'false')
    expect(screen.queryByText('Reply to Dimitri')).not.toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'NEXT UP (2)' }))
    expect(screen.getByRole('button', { name: 'NEXT UP (2)' })).toHaveAttribute('aria-expanded', 'true')
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    const nextUpSummary = screen.getByRole('button', { name: 'NEXT UP (2)' })
    const inboxSummary = screen.getByRole('button', { name: 'INBOX (1)' })
    expect(
      nextUpSummary.compareDocumentPosition(inboxSummary) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).toBeTruthy()
    fireEvent.click(inboxSummary)
    expect(screen.getByRole('button', { name: 'INBOX (1)' })).toHaveAttribute('aria-expanded', 'true')
    expect(screen.getByText('Inbox item')).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: 'BACKLOG (1)' }))
    expect(screen.getByRole('button', { name: 'BACKLOG (1)' })).toHaveAttribute('aria-expanded', 'true')
    expect(screen.getByText('Sort reference notes')).toBeInTheDocument()
    expect(screen.getByText('Due Mar 12')).toBeInTheDocument()
    expect(screen.getByText('Deadline Mar 13')).toBeInTheDocument()
    expect(screen.getAllByText('Inbox').length).toBeGreaterThan(0)
    expect(screen.getByText('Deep work')).toBeInTheDocument()
    expect(screen.getByText('Urgent')).toBeInTheDocument()
    expect(screen.getByText('Reading')).toBeInTheDocument()
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
    expect(screen.getAllByText('1 INBOX').length).toBeGreaterThan(0)
    expect(screen.getByText('1 BACKLOG')).toBeInTheDocument()
    expect(screen.queryByText('TODAY')).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /Open inbox/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /Reschedule/i })).not.toBeInTheDocument()
    expect(screen.queryByText(/More Context and Controls/i)).not.toBeInTheDocument()
  })

  it('shows task descriptions in compact lane rows when metadata carries them', async () => {
    setupApiMocks(buildNowData({
      task_lane: {
        active: {
          id: 'commit_todoist_1',
          task_kind: 'task',
          text: 'Reply to Dimitri',
          description: 'Confirm the rollout plan and next owner.',
          state: 'pending',
          project: 'Ops',
          tags: ['Urgent'],
          primary_thread_id: null,
          due_at: '2024-03-09T20:30:00Z',
          deadline: null,
        },
        pending: [],
        active_items: [
          {
            id: 'commit_todoist_1',
            task_kind: 'task',
            text: 'Reply to Dimitri',
            description: 'Confirm the rollout plan and next owner.',
            state: 'pending',
            project: 'Ops',
            tags: ['Urgent'],
            primary_thread_id: null,
            due_at: '2024-03-09T20:30:00Z',
            deadline: null,
          },
        ],
        next_up: [],
        inbox: [],
        if_time_allows: [],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'commit_todoist_1',
            text: 'Reply to Dimitri',
            title: 'Reply to Dimitri',
            description: 'Confirm the rollout plan and next owner.',
            tags: ['Urgent'],
            source_type: 'todoist',
            due_at: '2024-03-09T20:30:00Z',
            deadline: null,
            project: 'Ops',
            commitment_kind: 'todo',
          },
        ],
        other_open: [],
        next_commitment: null,
      },
    }))

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    })

    expect(screen.getByText('Confirm the rollout plan and next owner.')).toBeInTheDocument()
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

    expect(screen.getAllByText('0 INBOX').length).toBeGreaterThan(0)
    expect(screen.queryByText('Trust state')).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'Open system detail' })).not.toBeInTheDocument()
  })

  it('keeps each task in only one section at a time', async () => {
    setupApiMocks(buildNowData({
      task_lane: {
        active: null,
        active_items: [],
        next_up: [
          {
            id: 'todoist_inbox_due_today',
            task_kind: 'task',
            text: 'Inbox task due today',
            state: 'pending',
            project: null,
            tags: [],
            primary_thread_id: null,
            due_at: '2024-03-09T20:30:00Z',
            deadline: null,
          },
        ],
        inbox: [
          {
            id: 'todoist_inbox_unscheduled',
            task_kind: 'task',
            text: 'Inbox task unscheduled',
            title: 'Inbox task unscheduled',
            description: null,
            state: 'pending',
            project: null,
            tags: [],
            primary_thread_id: null,
            due_at: null,
            deadline: null,
          },
        ],
        if_time_allows: [],
        pending: [],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'todoist_inbox_due_today',
            text: 'Inbox task due today',
            title: 'Inbox task due today',
            description: null,
            tags: [],
            source_type: 'todoist',
            source_id: 'todo_today',
            status: 'open',
            due_at: '2024-03-09T20:30:00Z',
            deadline: null,
            project: null,
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:00:00Z',
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
          {
            id: 'todoist_inbox_unscheduled',
            text: 'Inbox task unscheduled',
            title: 'Inbox task unscheduled',
            description: null,
            tags: [],
            source_type: 'todoist',
            source_id: 'todo_inbox',
            status: 'open',
            due_at: null,
            deadline: null,
            project: null,
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:10:00Z',
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
        other_open: [],
        next_commitment: null,
      },
    }))

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByText('NEXT UP (2)').length).toBeGreaterThan(0)
    })

    expect(screen.getAllByText('1 INBOX').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Inbox task due today')).toHaveLength(1)
    expect(screen.queryByText('Inbox task unscheduled')).not.toBeInTheDocument()
  })

  it('surfaces overdue tasks together with today work using the overdue warning badge', async () => {
    setupApiMocks(buildNowData({
      computed_at: 1710000000,
      task_lane: {
        active: null,
        active_items: [],
        next_up: [
          {
            id: 'commit_todoist_overdue',
            task_kind: 'task',
            text: 'Reply to overdue thread',
            state: 'pending',
            project: 'Ops',
            tags: [],
            primary_thread_id: null,
            due_at: '2024-03-09T15:30:00Z',
            deadline: null,
          },
          {
            id: 'commit_local_overdue',
            task_kind: 'task',
            text: 'Finish delayed local task',
            state: 'pending',
            project: null,
            tags: [],
            primary_thread_id: null,
            due_at: '2024-03-09T15:45:00Z',
            deadline: null,
          },
        ],
        if_time_allows: [],
        pending: [],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'commit_todoist_overdue',
            text: 'Reply to overdue thread',
            source_type: 'todoist',
            source_id: 'todo_overdue',
            status: 'open',
            due_at: '2024-03-09T15:30:00Z',
            project: 'Ops',
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:00:00Z',
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
        other_open: [
          {
            id: 'commit_local_overdue',
            text: 'Finish delayed local task',
            source_type: 'local',
            source_id: null,
            status: 'open',
            due_at: '2024-03-09T15:45:00Z',
            project: null,
            commitment_kind: 'todo',
            created_at: '2024-03-09T13:00:00Z',
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
        next_commitment: null,
      },
      nudge_bars: [],
    }))

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByText('2 OVERDUE').length).toBeGreaterThan(0)
    })
    expect(screen.getAllByText('2 overdue items are still unresolved').length).toBeGreaterThan(0)
    expect(screen.getAllByText('NEXT UP (2)').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Reply to overdue thread').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Finish delayed local task').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Overdue').length).toBeGreaterThan(0)
  })

  it('keeps uncommitted lane items in next up without committed labeling', async () => {
    setupApiMocks(buildNowData({
      computed_at: 1710000000,
      task_lane: {
        active: null,
        active_items: [],
        next_up: [
          {
            id: 'commit_todoist_today',
            task_kind: 'task',
            text: 'Draft follow-up',
            state: 'pending',
            project: 'Ops',
            tags: ['Urgent'],
            primary_thread_id: null,
            due_at: '2024-03-09T20:30:00Z',
            deadline: null,
          },
        ],
        if_time_allows: [],
        pending: [],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'commit_todoist_today',
            text: 'Draft follow-up',
            source_type: 'todoist',
            source_id: 'todo_today',
            status: 'open',
            due_at: '2024-03-09T20:30:00Z',
            project: 'Ops',
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:00:00Z',
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
        other_open: [],
        next_commitment: null,
      },
    }))

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByText('ACTIVE TASK (0)').length).toBeGreaterThan(0)
    })

    expect(screen.getAllByText('NEXT UP (1)').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Draft follow-up').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Today').length).toBeGreaterThan(0)
    expect(screen.queryByText('Committed task')).not.toBeInTheDocument()
  })

  it('reschedules all overdue backlog items to today from the persistent nudge', async () => {
    const rescheduledNow = buildNowData({
      computed_at: 1710000000,
      task_lane: {
        active: null,
        active_items: [],
        next_up: [
          {
            id: 'commit_todoist_overdue',
            task_kind: 'task',
            text: 'Reply to overdue thread',
            state: 'pending',
            project: 'Ops',
            tags: [],
            primary_thread_id: null,
            due_at: '2024-03-10T06:59:59Z',
            deadline: null,
          },
        ],
        if_time_allows: [],
        pending: [],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'commit_todoist_overdue',
            text: 'Reply to overdue thread',
            source_type: 'todoist',
            source_id: 'todo_overdue',
            status: 'open',
            due_at: '2024-03-10T06:59:59Z',
            project: 'Ops',
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:00:00Z',
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
        other_open: [],
        next_commitment: null,
      },
    })

    setupApiMocks(buildNowData({
      computed_at: 1710000000,
      task_lane: {
        active: null,
        active_items: [],
        next_up: [
          {
            id: 'commit_todoist_overdue',
            task_kind: 'task',
            text: 'Reply to overdue thread',
            state: 'pending',
            project: 'Ops',
            tags: [],
            primary_thread_id: null,
            due_at: '2024-03-09T15:30:00Z',
            deadline: null,
          },
        ],
        if_time_allows: [],
        pending: [],
        completed: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'commit_todoist_overdue',
            text: 'Reply to overdue thread',
            source_type: 'todoist',
            source_id: 'todo_overdue',
            status: 'open',
            due_at: '2024-03-09T15:30:00Z',
            project: 'Ops',
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:00:00Z',
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
        other_open: [],
        next_commitment: null,
      },
    }))

    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: rescheduledNow,
      meta: { request_id: 'req_reschedule' },
    } as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /Reschedule all to today/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /Reschedule all to today/i }).at(0) as HTMLElement)

    await waitFor(() => {
      expect(vi.mocked(api.apiPost)).toHaveBeenCalled()
    })
    expect(vi.mocked(api.apiPost).mock.calls[0]?.[0]).toBe('/v1/now/tasks/reschedule-today')
    expect(vi.mocked(api.apiPost).mock.calls[0]?.[1]).toEqual({
      commitment_ids: ['commit_todoist_overdue', 'commit_local_overdue'],
    })
  })

  it('routes the overdue nudge into the shared sidebar rail when the shell owns nudges', async () => {
    const onRaiseNudge = vi.fn()
    const onClearNudge = vi.fn()

    setupApiMocks(buildNowData({
      computed_at: 1710000000,
      task_lane: {
        active: null,
        pending: [],
        recent_completed: [],
        overflow_count: 0,
      },
      tasks: {
        todoist: [
          {
            id: 'commit_todoist_overdue',
            text: 'Reply to overdue thread',
            source_type: 'todoist',
            source_id: 'todo_overdue',
            status: 'open',
            due_at: '2024-03-09T15:30:00Z',
            project: 'Ops',
            commitment_kind: 'todo',
            created_at: '2024-03-09T12:00:00Z',
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
        other_open: [],
        next_commitment: null,
      },
    }))

    render(
      <NowView
        hideNudgeLane
        onRaiseNudge={onRaiseNudge}
        onClearNudge={onClearNudge}
      />,
    )

    await waitFor(() => {
      expect(screen.queryByText('1 overdue item is still unresolved')).not.toBeInTheDocument()
    })
    expect(onRaiseNudge).not.toHaveBeenCalled()
    expect(onClearNudge).not.toHaveBeenCalled()
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
    const completedSummary = screen.getAllByRole('button', { name: 'COMPLETED (1)' }).at(0) as HTMLElement
    expect(completedSummary).toHaveAttribute('aria-expanded', 'false')

    fireEvent.click(completedSummary)

    await waitFor(() => {
      expect(completedSummary).toHaveAttribute('aria-expanded', 'true')
      expect(screen.getAllByRole('button', { name: 'Reopen Write weekly review' }).length).toBeGreaterThan(0)
    })
  })
})
