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
  function renderNavbar(
    activeView: 'now' | 'threads' | 'system' = 'now',
    surface: 'mobile' | 'tablet' | 'desktop' = 'desktop',
    activeAnchor: string | null = null,
    options: {
      layoutMode?: 'auto' | 'single' | 'split'
      layoutSurfaceSupportsToggle?: boolean
      splitModeActive?: boolean
      onLayoutMode?: (mode: 'auto' | 'single' | 'split') => void
    } = {},
  ) {
    const onSelectView = vi.fn()
    const onDeepLink = vi.fn()
    const onLayoutMode = options.onLayoutMode ?? vi.fn()

    render(
      <Navbar
        activeView={activeView}
        activeAnchor={activeAnchor}
        onSelectView={onSelectView}
        onDeepLink={onDeepLink}
        surface={surface}
        layoutMode={options.layoutMode}
        layoutSurfaceSupportsToggle={options.layoutSurfaceSupportsToggle}
        splitModeActive={options.splitModeActive}
        onLayoutMode={onLayoutMode}
      />, 
    )

    return { onSelectView, onDeepLink, onLayoutMode }
  }

  it('renders compact bottom navigation tabs on mobile', () => {
    renderNavbar('now', 'mobile')

    const nowTab = screen.getByRole('tab', { name: 'Now' })
    const threadsTab = screen.getByRole('tab', { name: 'Threads' })
    const nudgesTab = screen.getByRole('tab', { name: 'Nudges' })
    const settingsTab = screen.getByRole('tab', { name: 'Settings' })

    expect(nowTab).toBeInTheDocument()
    expect(threadsTab).toBeInTheDocument()
    expect(nudgesTab).toBeInTheDocument()
    expect(settingsTab).toBeInTheDocument()
    expect(nowTab).toHaveAttribute('aria-selected', 'true')
  })

  it('routes mobile primary tabs through their surface handlers', () => {
    const { onSelectView } = renderNavbar('now', 'mobile')

    fireEvent.click(screen.getByRole('tab', { name: 'Threads' }))
    fireEvent.click(screen.getByRole('tab', { name: 'Settings' }))
    fireEvent.click(screen.getByRole('tab', { name: 'Now' }))

    expect(onSelectView).toHaveBeenNthCalledWith(1, 'threads')
    expect(onSelectView).toHaveBeenNthCalledWith(2, 'system')
    expect(onSelectView).toHaveBeenNthCalledWith(3, 'now')
  })

  it('routes mobile nudges tab into now-nudges deep link', () => {
    const { onSelectView, onDeepLink } = renderNavbar('now', 'mobile')

    const nudgesTab = screen.getByRole('tab', { name: 'Nudges' })
    fireEvent.click(nudgesTab)

    expect(onDeepLink).toHaveBeenCalledWith({ view: 'now', anchor: 'nudges-section' })
    expect(onSelectView).not.toHaveBeenCalled()
  })

  it('marks the mobile nudges route active without also selecting Now', () => {
    renderNavbar('now', 'mobile', 'nudges-section')

    expect(screen.getByRole('tab', { name: 'Nudges' })).toHaveAttribute('aria-selected', 'true')
    expect(screen.getByRole('tab', { name: 'Now' })).toHaveAttribute('aria-selected', 'false')
  })

  it('keeps the compact mobile bar safe-area aware', () => {
    renderNavbar('now', 'mobile')

    const tablist = screen.getByRole('tablist', { name: 'Primary' })
    expect(tablist.parentElement?.parentElement?.className).toContain('gap-3')
    expect(tablist.parentElement?.parentElement?.parentElement?.className).toContain('min-h-[calc(4rem+env(safe-area-inset-bottom))]')
    expect(tablist.parentElement?.parentElement?.parentElement?.className).toContain('safe-area-inset-bottom')
  })

  it('keeps non-tab documentation affordance out of the tablist and mobile bar', () => {
    renderNavbar('now', 'mobile')

    const tablist = screen.getByRole('tablist', { name: 'Primary' })
    expect(within(tablist).queryByRole('button')).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /system documentation/i })).not.toBeInTheDocument()
  })

  it('shows adaptive layout controls only when the tablet surface supports them', () => {
    renderNavbar('now', 'tablet', null, {
      layoutMode: 'auto',
      layoutSurfaceSupportsToggle: true,
      splitModeActive: true,
    })

    expect(screen.getByRole('button', { name: 'Auto layout' })).toHaveAttribute('aria-pressed', 'true')
    expect(screen.getByRole('button', { name: 'Single-pane threads layout' })).toHaveAttribute('aria-pressed', 'false')
    expect(screen.getByRole('button', { name: 'Split-pane threads layout' })).toHaveAttribute('aria-pressed', 'true')
  })

  it('keeps tablet layout controls out of phone and unsupported desktop chrome', () => {
    const mobile = renderNavbar('now', 'mobile', null, {
      layoutSurfaceSupportsToggle: true,
    })

    expect(screen.queryByRole('button', { name: 'Auto layout' })).not.toBeInTheDocument()
    mobile.onSelectView.mockClear()
    cleanup()

    renderNavbar('now', 'desktop', null, {
      layoutSurfaceSupportsToggle: false,
    })

    expect(screen.queryByRole('button', { name: 'Auto layout' })).not.toBeInTheDocument()
  })

  it('routes tablet layout control clicks through the persisted layout handler', () => {
    const { onLayoutMode } = renderNavbar('now', 'tablet', null, {
      layoutMode: 'single',
      layoutSurfaceSupportsToggle: true,
    })

    fireEvent.click(screen.getByRole('button', { name: 'Auto layout' }))
    fireEvent.click(screen.getByRole('button', { name: 'Single-pane threads layout' }))
    fireEvent.click(screen.getByRole('button', { name: 'Split-pane threads layout' }))

    expect(onLayoutMode).toHaveBeenNthCalledWith(1, 'auto')
    expect(onLayoutMode).toHaveBeenNthCalledWith(2, 'single')
    expect(onLayoutMode).toHaveBeenNthCalledWith(3, 'split')
  })

  it('renders top nav items and routes clicks through handlers', () => {
    const { onSelectView } = renderNavbar()

    fireEvent.click(screen.getByRole('tab', { name: 'System' }))

    expect(onSelectView).toHaveBeenCalledWith('system')
  })

  it('keeps now, threads, and system in top-nav order', () => {
    renderNavbar()

    const labels = screen
      .getAllByRole('tab')
      .map((tab) => tab.textContent)
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
      systemTarget: { section: 'preferences', subsection: 'documentation', anchor: 'system-documentation' },
      anchor: 'system-documentation',
    })
  })

  it('keeps icon plus label visible for every primary surface control', () => {
    renderNavbar()
    for (const label of ['Now', 'Threads', 'System']) {
      const tab = screen.getByRole('tab', { name: label })
      expect(tab).toHaveTextContent(label)
      expect(within(tab).getByText(label)).toBeInTheDocument()
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
