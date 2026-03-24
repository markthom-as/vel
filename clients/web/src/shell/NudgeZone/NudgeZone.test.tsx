import { fireEvent, render, screen, waitFor } from '@testing-library/react'
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
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return { ok: true, data: buildNowData(), meta: { request_id: 'req_now' } } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })
    vi.mocked(api.apiPost).mockResolvedValue({ ok: true, data: { id: 'intv_review_1', state: 'acknowledged' }, meta: { request_id: 'req_post' } } as never)
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

    fireEvent.click(screen.getAllByRole('button', { name: /Open thread \(Standup check-in\) · check_in_bar/i }).at(-1) as HTMLElement)
    expect(onOpenThread).toHaveBeenCalledWith('conv_1')

    fireEvent.click(screen.getAllByRole('button', { name: /Sync & clients \(Vel Desktop needs attention\) · mesh_summary_warning/i }).at(-1) as HTMLElement)
    expect(onOpenSystem).toHaveBeenCalledWith({ section: 'integrations', subsection: 'accounts' })

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
      (node as HTMLElement).className.includes('max-w-[46%]'),
    )
    const standupSummaryRow = standupSummary.parentElement as HTMLElement

    expect(standupBody.className).toContain('items-start')
    expect(standupBody.className).toContain('gap-3')
    expect(standupBody.className).toContain('py-3')
    expect(standupSummary.className).toContain('leading-5')
    expect(standupSummary.className).toContain('w-full')
    expect(standupSummaryRow.className).toContain('w-full')
    expect(standupSummaryRow.className).toContain('flex-col')
    expect(standupActions.some((node) => (node as HTMLElement).className.includes('pt-0.5'))).toBe(true)
    expect(standupArticle.parentElement?.className).toContain('gap-3')
  })

  it('uses intervention mutations for intervention-backed nudge actions and defer', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Review morning plan').length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /Open thread \(Review morning plan\) · review_bar/i }).at(-1) as HTMLElement)

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

    fireEvent.click(screen.getByRole('button', { name: /Reschedule all to today \(2 overdue items are still unresolved\)/i }))
    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/v1/now/tasks/reschedule-today',
        { commitment_ids: ['commit_1', 'commit_2'] },
        expect.any(Function),
      )
    })

    fireEvent.click(screen.getByRole('button', { name: /Review backlog \(2 overdue items are still unresolved\)/i }))
    expect(scrollIntoView).toHaveBeenCalledWith({ behavior: 'smooth', block: 'start' })

    document.body.removeChild(backlogNode)
  })
})
