import { cleanup, fireEvent, render, screen, within } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Navbar } from '.'
import { clearQueryCache } from '../../data/query'
import { loadNow } from '../../data/context'

vi.mock('../../data/context', () => ({
  contextQueryKeys: {
    now: () => ['now'],
  },
  loadNow: vi.fn(async () => ({ ok: true, data: null, meta: { request_id: 'req_now' } })),
}))

afterEach(() => {
  cleanup()
  clearQueryCache()
})

describe('Navbar', () => {
  function renderNavbar(activeView: 'now' | 'threads' | 'system' = 'now', surface: 'mobile' | 'tablet' | 'desktop' = 'desktop') {
    const onSelectView = vi.fn()
    const onDeepLink = vi.fn()

    render(
      <Navbar
        activeView={activeView}
        onSelectView={onSelectView}
        onDeepLink={onDeepLink}
        surface={surface}
      />, 
    )

    return { onSelectView, onDeepLink }
  }

  it('renders compact bottom navigation tabs on mobile', () => {
    renderNavbar('now', 'mobile')

    const nowTab = screen.getByRole('tab', { name: 'Now' })
    const threadsTab = screen.getByRole('tab', { name: 'Threads' })
    const nudgesTab = screen.getByRole('tab', { name: 'Nudges' })
    const systemTab = screen.getByRole('tab', { name: 'System' })

    expect(nowTab).toBeInTheDocument()
    expect(threadsTab).toBeInTheDocument()
    expect(nudgesTab).toBeInTheDocument()
    expect(systemTab).toBeInTheDocument()
    expect(nowTab).toHaveAttribute('aria-selected', 'true')
  })

  it('routes mobile nudges tab into now-nudges deep link', () => {
    const { onSelectView, onDeepLink } = renderNavbar('now', 'mobile')

    const nudgesTab = screen.getByRole('tab', { name: 'Nudges' })
    fireEvent.click(nudgesTab)

    expect(onDeepLink).toHaveBeenCalledWith({ view: 'now', anchor: 'nudges-section' })
    expect(onSelectView).toHaveBeenCalledWith('now')
  })

  it('renders top nav items and routes clicks through handlers', () => {
    const { onSelectView } = renderNavbar()

    fireEvent.click(screen.getByRole('button', { name: 'System' }))

    expect(onSelectView).toHaveBeenCalledWith('system')
  })

  it('keeps now, threads, and system in top-nav order', () => {
    renderNavbar()

    const labels = screen
      .getAllByRole('button')
      .map((button) => button.textContent)
      .filter((label): label is string => Boolean(label))

    const nowIndex = labels.findIndex((label) => label.includes('Now'))
    const threadsIndex = labels.findIndex((label) => label.includes('Threads'))
    const systemIndex = labels.findIndex((label) => label.includes('System'))

    expect(nowIndex).toBeLessThan(threadsIndex)
    expect(threadsIndex).toBeLessThan(systemIndex)
  })

  it('does not render a global info rail toggle', () => {
    renderNavbar()
    expect(screen.queryByRole('button', { name: /open info/i })).not.toBeInTheDocument()
  })

  it('routes the docs affordance into the in-frame system documentation view', () => {
    const { onDeepLink } = renderNavbar()
    fireEvent.click(screen.getByRole('button', { name: /system documentation/i }))
    expect(onDeepLink).toHaveBeenCalledWith({
      view: 'system',
      systemTarget: { section: 'overview', subsection: 'trust' },
      anchor: 'system-docs',
    })
  })

  it('keeps icon plus label visible for every primary surface control', () => {
    renderNavbar()
    for (const label of ['Now', 'Threads', 'System']) {
      const button = screen.getByRole('button', { name: label })
      expect(button).toHaveTextContent(label)
      expect(within(button).getByText(label)).toBeInTheDocument()
    }
  })

  it('shows current event and active task context around the status group when not on now', async () => {
    vi.mocked(loadNow).mockResolvedValueOnce({
      ok: true,
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
        mesh_summary: { authority_label: 'Vel Desktop', sync_state: 'ready' },
        nudge_bars: [],
        task_lane: {
          active: { id: 'c1', task_kind: 'commitment', text: 'Write weekly review', state: 'active', project: null, primary_thread_id: null },
          pending: [],
          recent_completed: [],
          overflow_count: 0,
        },
        schedule: {
          upcoming_events: [
            { title: 'Design review', start_ts: 1709999900, end_ts: 1710003600, location: 'Studio', prep_minutes: null, travel_minutes: null, leave_by_ts: null },
          ],
        },
      } as never,
      meta: { request_id: 'req_now_context' },
    })

    renderNavbar('threads')

    expect(await screen.findByRole('button', { name: /current event design review/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /active task write weekly review/i })).toBeInTheDocument()
  })
})
