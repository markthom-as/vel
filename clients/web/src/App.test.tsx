import { cleanup, fireEvent, render, screen } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import App from './App'

const useShellBootstrapMock = vi.fn()
const useViewportSurfaceMock = vi.fn()

vi.mock('./shell/useShellBootstrap', () => ({
  useShellBootstrap: () => useShellBootstrapMock(),
}))

vi.mock('./core/hooks/useViewportSurface', () => ({
  useViewportSurface: () => useViewportSurfaceMock(),
}))

vi.mock('./views/system', () => ({
  systemTargetForCoreSetting: () => ({ section: 'core' }),
}))

vi.mock('./shell/AppShell', () => ({
  AppShell: ({
    navigation,
    main,
    nudgeZone,
    surface,
    layoutMode,
    splitModeActive,
  }: {
    navigation: React.ReactNode
    main: React.ReactNode
    nudgeZone?: React.ReactNode
    surface?: string
    layoutMode?: string
    splitModeActive?: boolean
  }) => (
    <div
      data-testid="app-shell"
      data-surface={surface}
      data-layout-mode={layoutMode}
      data-split-mode={String(splitModeActive)}
    >
      <div>{navigation}</div>
      <div>{main}</div>
      {nudgeZone ? <div data-testid="app-shell-nudge-zone">{nudgeZone}</div> : null}
    </div>
  ),
}))

vi.mock('./shell/Navbar', () => ({
  Navbar: ({
    onDeepLink,
    onLayoutMode,
    layoutMode,
  }: {
    onDeepLink?: (target: { view: 'now'; anchor: string }) => void
    onLayoutMode?: (mode: 'auto' | 'single' | 'split') => void
    layoutMode?: string
  }) => (
    <div data-testid="navbar" data-layout-mode={layoutMode}>
      <button type="button" onClick={() => onDeepLink?.({ view: 'now', anchor: 'nudges-section' })}>
        Navbar nudges
      </button>
      <button type="button" onClick={() => onLayoutMode?.('auto')}>
        Mock auto layout
      </button>
      <button type="button" onClick={() => onLayoutMode?.('single')}>
        Mock single layout
      </button>
      <button type="button" onClick={() => onLayoutMode?.('split')}>
        Mock split layout
      </button>
    </div>
  ),
}))

vi.mock('./shell/MainPanel', () => ({
  MainPanel: ({
    shellBootLoading,
    shellOwnsNowNudges,
    mobileNudgeZone,
  }: {
    shellBootLoading?: boolean
    shellOwnsNowNudges?: boolean
    mobileNudgeZone?: React.ReactNode
  }) => (
    <div data-testid="main-panel" data-shell-owns-nudges={String(shellOwnsNowNudges)}>
      {shellBootLoading ? 'Boot loading' : 'Boot ready'}
      {mobileNudgeZone ? <div data-testid="main-panel-mobile-nudges">{mobileNudgeZone}</div> : null}
    </div>
  ),
}))

vi.mock('./shell/NudgeZone', () => ({
  NudgeZone: ({
    variant,
    compactInitiallyOpen,
    railCollapsible,
  }: {
    variant?: string
    compactInitiallyOpen?: boolean
    railCollapsible?: boolean
  }) => (
    <div
      data-testid={variant === 'compact' ? 'compact-nudge-zone' : 'rail-nudge-zone'}
      data-compact-open={String(Boolean(compactInitiallyOpen))}
      data-rail-collapsible={String(Boolean(railCollapsible))}
    >
      Nudge zone
    </div>
  ),
}))

describe('App', () => {
  beforeEach(() => {
    useShellBootstrapMock.mockReturnValue({ shellBootLoading: false })
    useViewportSurfaceMock.mockReturnValue({ surface: 'desktop', isLandscape: false })
    window.localStorage.clear()
  })

  afterEach(() => {
    cleanup()
    useShellBootstrapMock.mockReset()
    useViewportSurfaceMock.mockReset()
    window.localStorage.clear()
  })

  it('keeps the nudge rail unmounted while shell bootstrap data is loading', () => {
    useShellBootstrapMock.mockReturnValue({ shellBootLoading: true })

    render(<App />)

    expect(screen.getByText('Boot loading')).toBeInTheDocument()
    expect(screen.queryByTestId('app-shell-nudge-zone')).toBeNull()
    expect(screen.queryByText('Nudge zone')).toBeNull()
  })

  it('renders the nudge rail after shell bootstrap data has loaded', () => {
    useShellBootstrapMock.mockReturnValue({ shellBootLoading: false })

    render(<App />)

    expect(screen.getByText('Boot ready')).toBeInTheDocument()
    expect(screen.getByTestId('app-shell-nudge-zone')).toBeInTheDocument()
    expect(screen.getByText('Nudge zone')).toBeInTheDocument()
    expect(screen.getByTestId('main-panel')).toHaveAttribute('data-shell-owns-nudges', 'true')
  })

  it('keeps mobile nudges inline by not mounting the shell rail', () => {
    useViewportSurfaceMock.mockReturnValue({ surface: 'mobile', isLandscape: false })

    render(<App />)

    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-surface', 'mobile')
    expect(screen.queryByTestId('app-shell-nudge-zone')).toBeNull()
    expect(screen.queryByTestId('main-panel-mobile-nudges')).toBeNull()
    expect(screen.getByTestId('main-panel')).toHaveAttribute('data-shell-owns-nudges', 'false')
  })

  it('mounts compact mobile nudges after the mobile nudges route is selected', () => {
    useViewportSurfaceMock.mockReturnValue({ surface: 'mobile', isLandscape: false })

    render(<App />)

    fireEvent.click(screen.getByRole('button', { name: /navbar nudges/i }))

    expect(screen.queryByTestId('app-shell-nudge-zone')).toBeNull()
    expect(screen.getByTestId('main-panel-mobile-nudges')).toBeInTheDocument()
    expect(screen.getByTestId('compact-nudge-zone')).toHaveAttribute('data-compact-open', 'true')
  })

  it('mounts the shell rail only for active tablet split mode', () => {
    useViewportSurfaceMock.mockReturnValue({ surface: 'tablet', isLandscape: false })

    const view = render(<App />)

    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-split-mode', 'false')
    expect(screen.queryByTestId('app-shell-nudge-zone')).toBeNull()
    expect(screen.getByTestId('main-panel')).toHaveAttribute('data-shell-owns-nudges', 'false')

    view.unmount()
    useViewportSurfaceMock.mockReturnValue({ surface: 'tablet', isLandscape: true })
    render(<App />)

    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-split-mode', 'true')
    expect(screen.getByTestId('app-shell-nudge-zone')).toBeInTheDocument()
    expect(screen.getByTestId('rail-nudge-zone')).toHaveAttribute('data-rail-collapsible', 'true')
    expect(screen.getByTestId('main-panel')).toHaveAttribute('data-shell-owns-nudges', 'true')
  })

  it('restores persisted tablet split layout before first render', () => {
    window.localStorage.setItem('vel-webui-tablet-layout', 'split')
    useViewportSurfaceMock.mockReturnValue({ surface: 'tablet', isLandscape: false })

    render(<App />)

    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-layout-mode', 'split')
    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-split-mode', 'true')
    expect(screen.getByTestId('app-shell-nudge-zone')).toBeInTheDocument()
    expect(screen.getByTestId('main-panel')).toHaveAttribute('data-shell-owns-nudges', 'true')
  })

  it('persists single-pane tablet preference and overrides landscape auto split', () => {
    useViewportSurfaceMock.mockReturnValue({ surface: 'tablet', isLandscape: true })

    render(<App />)
    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-layout-mode', 'auto')
    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-split-mode', 'true')

    fireEvent.click(screen.getByRole('button', { name: /mock single layout/i }))

    expect(window.localStorage.getItem('vel-webui-tablet-layout')).toBe('single')
    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-layout-mode', 'single')
    expect(screen.getByTestId('app-shell')).toHaveAttribute('data-split-mode', 'false')
    expect(screen.queryByTestId('app-shell-nudge-zone')).toBeNull()
    expect(screen.getByTestId('main-panel')).toHaveAttribute('data-shell-owns-nudges', 'false')
  })
})
