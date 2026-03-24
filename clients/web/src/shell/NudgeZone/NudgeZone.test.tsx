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
    expect(screen.getByText('+1 more')).toBeInTheDocument()
    expect(screen.queryByText('Extra hidden nudge')).not.toBeInTheDocument()

    const standupNudge = screen.getByText('Standup check-in').closest('article') as HTMLElement
    fireEvent.click(standupNudge.querySelector('button') as HTMLElement)
    expect(onOpenThread).toHaveBeenCalledWith('conv_1')

    const settingsNudges = screen.getAllByText(/needs attention|lighter proposal/i)
    const settingsActionHost = settingsNudges[0].closest('article') as HTMLElement
    fireEvent.click(settingsActionHost.querySelector('button') as HTMLElement)
    expect(onOpenSystem).toHaveBeenCalledWith({ section: 'integrations', subsection: 'accounts' })

    fireEvent.click(screen.getByRole('button', { name: /show 1 more nudges/i }))
    expect(screen.getByText('Extra hidden nudge')).toBeInTheDocument()
  })

  it('uses intervention mutations for intervention-backed nudge actions and defer', async () => {
    render(<NudgeZone activeView="now" />)

    await waitFor(() => {
      expect(screen.getAllByText('Review morning plan').length).toBeGreaterThan(0)
    })

    const reviewNudge = screen.getAllByText('Review morning plan')[0].closest('article') as HTMLElement
    const buttons = reviewNudge.querySelectorAll('button')
    fireEvent.click(buttons[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/interventions/intv_review_1/acknowledge',
        {},
        expect.any(Function),
      )
    })

    fireEvent.click(buttons[1] as HTMLElement)
    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith(
        '/api/interventions/intv_review_1/snooze',
        { minutes: 10 },
        expect.any(Function),
      )
    })
  })
})
