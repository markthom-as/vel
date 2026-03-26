import { render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import App from './App'

const useShellBootstrapMock = vi.fn()

vi.mock('./shell/useShellBootstrap', () => ({
  useShellBootstrap: () => useShellBootstrapMock(),
}))

vi.mock('./core/hooks/useViewportSurface', () => ({
  useViewportSurface: () => ({ surface: 'desktop', isLandscape: false }),
}))

vi.mock('./views/system', () => ({
  systemTargetForCoreSetting: () => ({ section: 'core' }),
}))

vi.mock('./shell/AppShell', () => ({
  AppShell: ({
    navigation,
    main,
    nudgeZone,
  }: {
    navigation: React.ReactNode
    main: React.ReactNode
    nudgeZone?: React.ReactNode
  }) => (
    <div>
      <div>{navigation}</div>
      <div>{main}</div>
      {nudgeZone ? <div data-testid="app-shell-nudge-zone">{nudgeZone}</div> : null}
    </div>
  ),
}))

vi.mock('./shell/Navbar', () => ({
  Navbar: () => <div>Navbar</div>,
}))

vi.mock('./shell/MainPanel', () => ({
  MainPanel: ({ shellBootLoading }: { shellBootLoading?: boolean }) => (
    <div>{shellBootLoading ? 'Boot loading' : 'Boot ready'}</div>
  ),
}))

vi.mock('./shell/NudgeZone', () => ({
  NudgeZone: () => <div>Nudge zone</div>,
}))

describe('App', () => {
  afterEach(() => {
    useShellBootstrapMock.mockReset()
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
  })
})
