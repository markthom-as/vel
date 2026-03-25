import { fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { NudgeZone } from './NudgeZone'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
  apiPatch: vi.fn(),
}))

function buildNowData(overrides: Record<string, unknown> = {}) {
  return {
    computed_at: 1710000000,
    timezone: 'America/Denver',
    schedule: {
      empty_message: null,
      next_event: {
        event_id: 'evt_1',
        calendar_id: 'cal_1',
        calendar_name: 'Primary',
        title: 'Design review',
        start_ts: 1710003600,
        end_ts: 1710007200,
        event_url: 'https://calendar.google.com/calendar/event?eid=evt_1',
        location: 'Studio',
        notes: 'Review mocks and unblock the handoff.',
        attendees: ['alex@example.com', 'sam@example.com', 'pat@example.com'],
        video_url: 'https://meet.google.com/abc-defg-hij',
        video_provider: 'google_meet',
        prep_minutes: 15,
        travel_minutes: 0,
        leave_by_ts: 1710003600,
        rescheduled: false,
      },
      upcoming_events: [
        {
          event_id: 'evt_1',
          calendar_id: 'cal_1',
          calendar_name: 'Primary',
          title: 'Design review',
          start_ts: 1710003600,
          end_ts: 1710007200,
          event_url: 'https://calendar.google.com/calendar/event?eid=evt_1',
          location: 'Studio',
          notes: 'Review mocks and unblock the handoff.',
          attendees: ['alex@example.com', 'sam@example.com', 'pat@example.com'],
          video_url: 'https://meet.google.com/abc-defg-hij',
          video_provider: 'google_meet',
          prep_minutes: 15,
          travel_minutes: 0,
          leave_by_ts: 1710003600,
          rescheduled: false,
        },
      ],
      following_day_events: [],
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
        summary: 'Sync posture needs review.',
        urgent: true,
        primary_thread_id: null,
        actions: [{ kind: 'open_settings', label: 'Open settings' }],
      },
      {
        id: 'review_bar',
        kind: 'review_request',
        title: 'Review morning plan',
        summary: 'A review request is ready.',
        urgent: false,
        primary_thread_id: 'conv_2',
        actions: [{ kind: 'accept', label: 'Review' }],
      },
      {
        id: 'reflow_bar',
        kind: 'reflow_proposal',
        title: 'Reflow afternoon',
        summary: 'A lighter proposal is available.',
        urgent: false,
        primary_thread_id: null,
        actions: [{ kind: 'open_settings', label: 'Open settings' }],
      },
      {
        id: 'extra_bar',
        kind: 'needs_input',
        title: 'Extra hidden nudge',
        summary: 'This should start collapsed.',
        urgent: false,
        primary_thread_id: null,
        actions: [{ kind: 'expand', label: 'Continue in Threads' }],
      },
    ],
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
        thread_route: { target: 'existing_thread', thread_id: 'conv_2', label: 'Open thread', thread_type: null },
      },
    ],
    trust_readiness: {
      level: 'ok',
      summary: 'No trust blockers are active.',
    },
    ...overrides,
  }
}

function buildIntegrationsData(overrides: Record<string, unknown> = {}) {
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
      last_item_count: 2,
      guidance: null,
    },
    todoist: {
      configured: false,
      connected: false,
      has_api_token: false,
      last_sync_at: null,
      last_sync_status: null,
      last_error: null,
      last_item_count: null,
      guidance: null,
      write_capabilities: {
        completion_status: false,
        due_date: false,
        tags: false,
      },
    },
    activity: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    health: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    git: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    messaging: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    reminders: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    notes: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    transcripts: { configured: false, source_path: null, selected_paths: [], available_paths: [], internal_paths: [], suggested_paths: [], source_kind: 'directory', last_sync_at: null, last_sync_status: null, last_error: null, last_item_count: null, guidance: null },
    ...overrides,
  }
}

function findNudgeArticle(text: string): HTMLElement {
  return screen.getAllByText(text)
    .map((node) => node.closest('article'))
    .filter((node): node is HTMLElement => node instanceof HTMLElement)
    .at(-1) as HTMLElement
}

describe('NudgeZone', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
    vi.mocked(api.apiPatch).mockReset()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return { ok: true, data: buildNowData(), meta: { request_id: 'req_now' } } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })
    vi.mocked(api.apiPost).mockResolvedValue({ ok: true, data: { id: 'intv_review_1', state: 'acknowledged' }, meta: { request_id: 'req_post' } } as never)
    vi.mocked(api.apiPatch).mockResolvedValue({
      ok: true,
      data: buildIntegrationsData({
        google_calendar: {
          ...buildIntegrationsData().google_calendar,
          calendars: [
            { id: 'cal_1', summary: 'Primary', primary: true, sync_enabled: true, display_enabled: true },
            { id: 'cal_2', summary: 'Team', primary: false, sync_enabled: true, display_enabled: true },
          ],
        },
      }),
      meta: { request_id: 'req_patch' },
    } as never)
  })

  it('routes nudge actions through thread and system handlers', async () => {
    const onOpenThread = vi.fn()
    const onOpenSystem = vi.fn()

    render(
      <NudgeZone
        activeView="now"
        onOpenThread={onOpenThread}
        onOpenSystem={onOpenSystem}
      />,
    )

    await waitFor(() => {
      expect(screen.getByText('Standup check-in')).toBeInTheDocument()
    })
    expect(screen.getByText('NUDGES (5)')).toBeInTheDocument()
    expect(screen.getByText('Extra hidden nudge')).toBeInTheDocument()

    fireEvent.click(screen.getAllByRole('button', { name: /Open \(Standup check-in\) · check_in_bar/i }).at(-1) as HTMLElement)
    expect(onOpenThread).toHaveBeenCalledWith('conv_1')

    fireEvent.click(screen.getAllByRole('button', { name: /Sync & clients \(Vel Desktop needs attention\) · mesh_summary_warning/i }).at(-1) as HTMLElement)
    expect(onOpenSystem).toHaveBeenCalledWith({ section: 'integrations', subsection: 'accounts' })

  })

  it('falls back to local nudges when live context fails', async () => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        throw new Error('now offline')
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(
      <NudgeZone
        activeView="now"
        extraNudges={[
          {
            id: 'local_only_nudge',
            kind: 'needs_input',
            title: 'Local fallback nudge',
            summary: 'This should still render without live context.',
            urgent: true,
            primary_thread_id: null,
            actions: [{ kind: 'expand', label: 'Continue in Threads' }],
          },
        ]}
      />,
    )

    await waitFor(() => {
      expect(screen.getByText('Local fallback nudge')).toBeInTheDocument()
    })
    expect(screen.getByText('Live context is unavailable. Showing local nudges only.')).toBeInTheDocument()
  })

  it('deep-links the core setup nudge to the core settings block', async () => {
    const onOpenSystem = vi.fn()
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            nudge_bars: [
              {
                id: 'core_setup_required',
                kind: 'needs_input',
                title: 'Finish Core setup to enable the composer',
                summary: 'Finish the checklist below to enable Vel.',
                urgent: true,
                primary_thread_id: null,
                actions: [
                  { kind: 'open_settings:core_settings:user_display_name:ready:Jove%20Operator', label: 'Your name' },
                  { kind: 'open_settings:core_settings:agent_profile:missing', label: 'Agent profile' },
                  { kind: 'open_settings:core_settings:llm_provider:missing', label: 'LLM integration' },
                ],
              },
            ],
          }),
          meta: { request_id: 'req_now_core_setup' },
        } as never
      }
      if (path === '/api/integrations') {
        return { ok: true, data: buildIntegrationsData(), meta: { request_id: 'req_integrations' } } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(<NudgeZone activeView="now" onOpenSystem={onOpenSystem} />)

    await waitFor(() => {
      expect(screen.getByText('Finish Core setup to enable the composer')).toBeInTheDocument()
    })

    const coreSetupNudge = screen.getByText('Finish Core setup to enable the composer').closest('article') as HTMLElement
    fireEvent.click(coreSetupNudge)
    expect(screen.getByTestId('core-setup-open-icon-user_display_name')).toBeInTheDocument()
    expect(screen.getByText('Jove Operator')).toBeInTheDocument()
    fireEvent.click(screen.getByRole('button', { name: /Your name \(Finish Core setup to enable the composer\) · core_setup_required/i }))
    expect(onOpenSystem).toHaveBeenCalledWith({
      section: 'core',
      subsection: 'core_settings',
      anchor: 'core-settings-user-display-name',
    })
  })

  it('re-expands and highlights a spotlighted nudge', async () => {
    render(
      <NudgeZone
        activeView="now"
        highlightedNudgeId="extra_bar"
        highlightedNudgeNonce={1}
      />,
    )

    await waitFor(() => {
      expect(screen.getByText('Extra hidden nudge')).toBeInTheDocument()
    })

    const extraNudge = screen.getAllByText('Extra hidden nudge').at(-1)?.closest('article') as HTMLElement
    expect(extraNudge.querySelector('[class*="ring-2"], [class*="animate-["]')).not.toBeNull()
  })

  it('treats nudges as a single-open accordion when switching between them', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getByText('Standup check-in')).toBeInTheDocument()
    })

    const standupBody = findNudgeArticle('Standup check-in').querySelector('div[class*="rounded-[20px]"]') as HTMLElement
    const warningBody = findNudgeArticle('Vel Desktop needs attention').querySelector('div[class*="rounded-[20px]"]') as HTMLElement

    fireEvent.click(standupBody)
    await waitFor(() => {
      expect(standupBody.className).toContain('ring-1')
      expect((findNudgeArticle('Standup check-in').querySelector('p.text-sm') as HTMLElement).className).toContain('whitespace-normal')
      expect((findNudgeArticle('Vel Desktop needs attention').querySelector('p.text-sm') as HTMLElement).className).toContain('truncate')
    })

    fireEvent.click(warningBody)
    await waitFor(() => {
      expect(warningBody.className).toContain('ring-1')
      expect((findNudgeArticle('Vel Desktop needs attention').querySelector('p.text-sm') as HTMLElement).className).toContain('whitespace-normal')
      expect((findNudgeArticle('Standup check-in').querySelector('p.text-sm') as HTMLElement).className).toContain('truncate')
    })
  })

  it('collapses an expanded nudge when its non-interactive body is clicked again', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Standup check-in').length).toBeGreaterThan(0)
    })

    const standupBody = findNudgeArticle('Standup check-in').querySelector('div[class*="rounded-[20px]"]') as HTMLElement

    fireEvent.click(standupBody)
    await waitFor(() => {
      expect((findNudgeArticle('Standup check-in').querySelector('p.text-sm') as HTMLElement).className).toContain('whitespace-normal')
    })

    fireEvent.click(standupBody)
    await waitFor(() => {
      expect((findNudgeArticle('Standup check-in').querySelector('p.text-sm') as HTMLElement).className).toContain('truncate')
    })
  })

  it('switches the accordion when clicking nudge titles directly', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Standup check-in').length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByTestId('nudge-toggle-check_in_bar').at(-1) as HTMLElement)
    await waitFor(() => {
      expect((findNudgeArticle('Standup check-in').querySelector('p.text-sm') as HTMLElement).className).toContain('whitespace-normal')
    })

    fireEvent.click(screen.getAllByTestId('nudge-toggle-mesh_summary_warning').at(-1) as HTMLElement)
    await waitFor(() => {
      expect((findNudgeArticle('Vel Desktop needs attention').querySelector('p.text-sm') as HTMLElement).className).toContain('whitespace-normal')
      expect((findNudgeArticle('Standup check-in').querySelector('p.text-sm') as HTMLElement).className).toContain('truncate')
    })
  })

  it('keeps expanded nudges on the roomier padded layout', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Standup check-in').length).toBeGreaterThan(0)
    })

    const standupArticle = findNudgeArticle('Standup check-in')
    const standupBody = standupArticle.querySelector('div[class*="rounded-[20px]"]') as HTMLElement
    fireEvent.click(standupBody)

    const standupSummary = screen.getAllByText('Vel needs one short answer before the standup can continue.').at(-1) as HTMLElement
    const standupActions = Array.from(standupArticle.querySelectorAll('div')).filter((node) =>
      (node as HTMLElement).className.includes('w-[5.5rem]'),
    )
    const standupSummaryRow = standupSummary.parentElement as HTMLElement

    expect(standupBody.className).toContain('items-stretch')
    expect(standupBody.className).toContain('gap-3')
    expect(standupBody.className).toContain('py-3')
    expect(standupSummary.className).toContain('text-xs')
    expect(standupSummaryRow.className).toContain('flex')
    expect(standupSummaryRow.className).toContain('flex-col')
    expect(standupActions.length).toBeGreaterThan(0)
    expect(standupArticle.parentElement?.className).toContain('gap-3')
  })

  it('uses intervention mutations for intervention-backed nudge actions and defer', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Review morning plan').length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /Open \(Review morning plan\) · review_bar/i }).at(-1) as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/interventions/intv_review_1/acknowledge',
        {},
        expect.any(Function),
      )
    })

    fireEvent.click(screen.getAllByRole('button', { name: /Defer \(Review morning plan\) · review_bar/i }).at(-1) as HTMLElement)
    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/interventions/intv_review_1/snooze',
        { minutes: 10 },
        expect.any(Function),
      )
    })
  })

  it('executes local overdue nudge actions from the shared rail', async () => {
    const scrollIntoView = vi.fn()
    const backlogNode = document.createElement('section')
    backlogNode.id = 'now-backlog'
    backlogNode.scrollIntoView = scrollIntoView
    document.body.appendChild(backlogNode)

    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            nudge_bars: [],
          }),
          meta: { request_id: 'req_now_local_overdue' },
        } as never
      }
      if (path === '/api/integrations') {
        return { ok: true, data: buildIntegrationsData(), meta: { request_id: 'req_integrations' } } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: buildNowData({ nudge_bars: [] }),
      meta: { request_id: 'req_reschedule_today' },
    } as never)

    render(
      <NudgeZone
        activeView="now"
        extraNudges={[
          {
            id: 'todoist_overdue_backlog',
            kind: 'nudge',
            title: '2 overdue items are still unresolved',
            summary: 'Overdue work stays visible until you commit it into the day, keep it in backlog, or reschedule it to today without committing it.',
            timestamp: 1710000000,
            urgent: true,
            primary_thread_id: 'conv_9',
            actions: [
              { kind: 'reschedule_today:commit_1,commit_2', label: 'Reschedule all to today' },
              { kind: 'jump_backlog:now-backlog', label: 'Review backlog' },
              { kind: 'open_thread', label: 'Open thread' },
            ],
          },
        ]}
      />,
    )

    await waitFor(() => {
      expect(screen.getByText('2 overdue items are still unresolved')).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /To Today \(2 overdue items are still unresolved\)/i }))
    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/v1/now/tasks/reschedule-today',
        { commitment_ids: ['commit_1', 'commit_2'] },
        expect.any(Function),
      )
    })

    fireEvent.click(screen.getByRole('button', { name: /Backlog \(2 overdue items are still unresolved\)/i }))
    expect(scrollIntoView).toHaveBeenCalledWith({ behavior: 'smooth', block: 'start' })

    document.body.removeChild(backlogNode)
  })

  it('hides non-visible calendars and their events from the sidebar while persisting visibility changes', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            schedule: {
              empty_message: null,
              next_event: {
                event_id: 'evt_1',
                calendar_id: 'cal_1',
                calendar_name: 'Primary',
                title: 'Design review',
                start_ts: 1710003600,
                end_ts: 1710007200,
                event_url: 'https://calendar.google.com/calendar/event?eid=evt_1',
                location: 'Studio',
                notes: 'Review mocks and unblock the handoff.',
                attendees: ['alex@example.com', 'sam@example.com', 'pat@example.com'],
                video_url: 'https://meet.google.com/abc-defg-hij',
                video_provider: 'google_meet',
                prep_minutes: 15,
                travel_minutes: 0,
                leave_by_ts: 1710003600,
                rescheduled: false,
              },
              upcoming_events: [
                {
                  event_id: 'evt_1',
                  calendar_id: 'cal_1',
                  calendar_name: 'Primary',
                  title: 'Design review',
                  start_ts: 1710003600,
                  end_ts: 1710007200,
                  event_url: 'https://calendar.google.com/calendar/event?eid=evt_1',
                  location: 'Studio',
                  notes: 'Review mocks and unblock the handoff.',
                  attendees: ['alex@example.com', 'sam@example.com', 'pat@example.com'],
                  video_url: 'https://meet.google.com/abc-defg-hij',
                  video_provider: 'google_meet',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710003600,
                  rescheduled: false,
                },
                {
                  event_id: 'evt_2',
                  calendar_id: 'cal_2',
                  calendar_name: 'Team',
                  title: 'Hidden team sync',
                  start_ts: 1710010800,
                  end_ts: 1710012600,
                  event_url: 'https://calendar.google.com/calendar/event?eid=evt_2',
                  location: 'War room',
                  notes: null,
                  attendees: [],
                  video_url: null,
                  video_provider: null,
                  prep_minutes: 0,
                  travel_minutes: 0,
                  leave_by_ts: 1710010800,
                  rescheduled: false,
                },
              ],
              following_day_events: [],
            },
          }),
          meta: { request_id: 'req_now_hidden_calendar' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations_hidden_calendar' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(<NudgeZone activeView="now" />)

    expect((await screen.findAllByRole('table')).length).toBeGreaterThan(0)
    expect(screen.getAllByText('Design review').length).toBeGreaterThan(0)
    expect(screen.queryByText('Hidden team sync')).not.toBeInTheDocument()
    expect(screen.queryAllByRole('button', { name: /Team/i })).toHaveLength(0)

    vi.mocked(api.apiPatch).mockResolvedValueOnce({
      ok: true,
      data: buildIntegrationsData({
        google_calendar: {
          ...buildIntegrationsData().google_calendar,
          calendars: [
            { id: 'cal_1', summary: 'Primary', primary: true, sync_enabled: true, display_enabled: false },
            { id: 'cal_2', summary: 'Team', primary: false, sync_enabled: true, display_enabled: false },
          ],
        },
      }),
      meta: { request_id: 'req_patch_hide_calendar' },
    } as never)

    fireEvent.click(screen.getAllByRole('button', { name: /Primary/i }).at(-1) as HTMLElement)
    await waitFor(() => {
      expect(api.apiPatch).toHaveBeenCalledWith(
        '/api/integrations/google-calendar',
        {
          calendar_settings: [
            {
              id: 'cal_1',
              display_enabled: false,
            },
          ],
        },
        expect.any(Function),
      )
    })
    expect(screen.getAllByRole('button', { name: /Primary/i }).length).toBeGreaterThan(0)
    expect(screen.queryAllByRole('button', { name: /Team/i })).toHaveLength(0)
  })

  it('uses contextual day copy when the selected calendar stream is empty', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            schedule: {
              empty_message: null,
              next_event: {
                event_id: 'evt_2',
                calendar_id: 'cal_2',
                calendar_name: 'Team',
                title: 'Hidden team sync',
                start_ts: 1710010800,
                end_ts: 1710012600,
                event_url: 'https://calendar.google.com/calendar/event?eid=evt_2',
                location: 'War room',
                notes: null,
                attendees: [],
                video_url: null,
                video_provider: null,
                prep_minutes: 0,
                travel_minutes: 0,
                leave_by_ts: 1710010800,
                rescheduled: false,
              },
              upcoming_events: [
                {
                  event_id: 'evt_2',
                  calendar_id: 'cal_2',
                  calendar_name: 'Team',
                  title: 'Hidden team sync',
                  start_ts: 1710010800,
                  end_ts: 1710012600,
                  event_url: 'https://calendar.google.com/calendar/event?eid=evt_2',
                  location: 'War room',
                  notes: null,
                  attendees: [],
                  video_url: null,
                  video_provider: null,
                  prep_minutes: 0,
                  travel_minutes: 0,
                  leave_by_ts: 1710010800,
                  rescheduled: false,
                },
              ],
              following_day_events: [],
            },
          }),
          meta: { request_id: 'req_now_empty_selected_stream' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations_empty_selected_stream' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(<NudgeZone activeView="now" />)

    expect(await screen.findByText(/No calendar events for /i)).toBeInTheDocument()
  })

  it('posts drag-and-drop calendar reschedules through the now api', async () => {
    vi.mocked(api.apiPost).mockImplementation(async (path: string) => {
      if (path === '/v1/now/calendar-events/reschedule') {
        return {
          ok: true,
          data: buildNowData({
            schedule: {
              empty_message: null,
              next_event: {
                event_id: 'evt_1',
                calendar_id: 'cal_1',
                calendar_name: 'Primary',
                title: 'Design review',
                start_ts: 1710005400,
                end_ts: 1710009000,
                location: 'Studio',
                prep_minutes: 15,
                travel_minutes: 0,
                leave_by_ts: 1710005400,
                rescheduled: true,
              },
              upcoming_events: [
                {
                  event_id: 'evt_1',
                  calendar_id: 'cal_1',
                  calendar_name: 'Primary',
                  title: 'Design review',
                  start_ts: 1710005400,
                  end_ts: 1710009000,
                  location: 'Studio',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710005400,
                  rescheduled: true,
                },
              ],
            },
          }),
          meta: { request_id: 'req_calendar_reschedule' },
        } as never
      }
      return { ok: true, data: { id: 'intv_review_1', state: 'acknowledged' }, meta: { request_id: 'req_post' } } as never
    })

    render(<NudgeZone activeView="now" />)

    const dragCard = (await screen.findAllByText('Design review')).at(-1)?.closest('div[draggable="true"]') as HTMLElement
    const dropTarget = screen.getAllByText('Design review')[0]?.closest('tr') as HTMLElement
    const dataTransfer = {
      effectAllowed: 'move',
      setData: vi.fn(),
      getData: vi.fn((type: string) => (type === 'application/x-vel-calendar-event' || type === 'text/plain' ? 'evt_1' : '')),
    }

    fireEvent.dragStart(dragCard, { dataTransfer })
    fireEvent.dragOver(dropTarget, { dataTransfer })
    fireEvent.drop(dropTarget, { dataTransfer })

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/v1/now/calendar-events/reschedule',
        {
          event_id: 'evt_1',
          calendar_id: 'cal_1',
          start_ts: 1710003600,
          end_ts: 1710007200,
        },
        expect.any(Function),
      )
    })
    await waitFor(() => {
      expect(screen.getAllByText('Moved in Vel').length).toBeGreaterThan(0)
    })
  })

  it('does not render events for synced calendars that are hidden in settings', async () => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            schedule: {
              empty_message: null,
              next_event: {
                event_id: 'evt_2',
                calendar_id: 'cal_2',
                calendar_name: 'Team',
                title: 'Team offsite',
                start_ts: 1710003600,
                end_ts: 1710007200,
                location: 'HQ',
                prep_minutes: 15,
                travel_minutes: 0,
                leave_by_ts: 1710003600,
                rescheduled: false,
              },
              upcoming_events: [
                {
                  event_id: 'evt_2',
                  calendar_id: 'cal_2',
                  calendar_name: 'Team',
                  title: 'Team offsite',
                  start_ts: 1710003600,
                  end_ts: 1710007200,
                  location: 'HQ',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710003600,
                  rescheduled: false,
                },
              ],
            },
          }),
          meta: { request_id: 'req_now_hidden_calendar' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations_hidden_calendar' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(<NudgeZone activeView="now" />)

    const calendarSections = screen.getAllByLabelText('Calendar')
    await waitFor(() => {
      expect(
        calendarSections.some((section) => within(section as HTMLElement).queryAllByText(/No calendar events for/i).length > 0),
      ).toBe(true)
    })

    expect(screen.queryByText('Team offsite')).not.toBeInTheDocument()
  })

  it('reveals the following day schedule when toggled on', async () => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            schedule: {
              empty_message: null,
              next_event: {
                event_id: 'evt_1',
                calendar_id: 'cal_1',
                calendar_name: 'Primary',
                title: 'Design review',
                start_ts: 1710003600,
                end_ts: 1710007200,
                location: 'Studio',
                prep_minutes: 15,
                travel_minutes: 0,
                leave_by_ts: 1710003600,
                rescheduled: false,
              },
              upcoming_events: [
                {
                  event_id: 'evt_1',
                  calendar_id: 'cal_1',
                  calendar_name: 'Primary',
                  title: 'Design review',
                  start_ts: 1710003600,
                  end_ts: 1710007200,
                  location: 'Studio',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710003600,
                  rescheduled: false,
                },
              ],
              following_day_events: [
                {
                  event_id: 'evt_3',
                  calendar_id: 'cal_1',
                  calendar_name: 'Primary',
                  title: 'Tomorrow planning',
                  start_ts: 1710090000,
                  end_ts: 1710093600,
                  event_url: 'https://calendar.google.com/calendar/event?eid=evt_3',
                  location: 'Desk',
                  notes: 'Align on priorities for tomorrow.',
                  attendees: ['morgan@example.com', 'riley@example.com'],
                  video_url: 'https://zoom.us/j/123456789',
                  video_provider: 'zoom',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710090000,
                  rescheduled: false,
                },
              ],
            },
          }),
          meta: { request_id: 'req_now_following_day' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations_following_day' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(<NudgeZone activeView="now" />)

    expect(screen.queryByText('Tomorrow planning')).not.toBeInTheDocument()

    const nextDayCheckbox = (await screen.findAllByRole('checkbox', { name: /Next day/i })).at(0) as HTMLInputElement
    fireEvent.click(nextDayCheckbox as HTMLInputElement)

    await waitFor(() => {
      expect(screen.getByText('Tomorrow planning')).toBeInTheDocument()
    })
    expect(screen.getAllByText('morgan@example.com, riley@example.com').length).toBeGreaterThan(0)
  })

  it('shows attendees, notes, and the video conference link for calendar events', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Design review').length).toBeGreaterThan(0)
    })

    expect(screen.getAllByText('alex@example.com, sam@example.com +1').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Review mocks and unblock the handoff.').length).toBeGreaterThan(0)
    expect(
      screen
        .getAllByRole('link', { name: /Google Meet/i })
        .some((node) => node.getAttribute('href') === 'https://meet.google.com/abc-defg-hij'),
    ).toBe(true)
    expect(
      screen
        .getAllByRole('link', { name: 'Design review' })
        .some((node) => node.getAttribute('href') === 'https://calendar.google.com/calendar/event?eid=evt_1'),
    ).toBe(true)
  })

  it('reveals the following day schedule even when today is empty', async () => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: buildNowData({
            schedule: {
              empty_message: null,
              next_event: null,
              upcoming_events: [],
              following_day_events: [
                {
                  event_id: 'evt_4',
                  calendar_id: 'cal_1',
                  calendar_name: 'Primary',
                  title: 'Tomorrow retro',
                  start_ts: 1710097200,
                  end_ts: 1710100800,
                  location: 'Desk',
                  prep_minutes: 0,
                  travel_minutes: 0,
                  leave_by_ts: 1710097200,
                  rescheduled: false,
                },
              ],
            },
          }),
          meta: { request_id: 'req_now_following_day_only' },
        } as never
      }
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: buildIntegrationsData(),
          meta: { request_id: 'req_integrations_following_day_only' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })

    render(<NudgeZone activeView="now" />)

    const calendarSections = screen.getAllByLabelText('Calendar')
    let nextDayCheckbox: HTMLInputElement | null = null
    await waitFor(() => {
      const sectionWithNoEvents = calendarSections.find((section) =>
        within(section as HTMLElement).queryAllByText(/No calendar events for/i).length > 0,
      )
      expect(sectionWithNoEvents).not.toBeUndefined()
      nextDayCheckbox = within(sectionWithNoEvents as HTMLElement).getByRole('checkbox', { name: /Next day/i }) as HTMLInputElement
      expect(nextDayCheckbox).toBeTruthy()
    })

    fireEvent.click(nextDayCheckbox)

    await waitFor(() => {
      expect(screen.getByText(/Tomorrow retro/i)).toBeInTheDocument()
    })
  })
})
