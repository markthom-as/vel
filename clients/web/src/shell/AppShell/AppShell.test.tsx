import { render } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { AppShell } from './AppShell'

describe('AppShell', () => {
  it('renders navigation and main content without a global side rail slot', () => {
    const { getByText, getByTestId, queryByText, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        actionBar={<div>Action bar</div>}
      />,
    )

    expect(getByText('Navigation')).toBeInTheDocument()
    expect(getByText('Main content')).toBeInTheDocument()
    expect(getByText('Nudges')).toBeInTheDocument()
    expect(getByText('Action bar')).toBeInTheDocument()
    expect(queryByText(/info/i)).not.toBeInTheDocument()
    expect(getByTestId('app-shell-nudges')).toBeInTheDocument()
    expect(getByTestId('app-shell-workspace-desktop')).toBeInTheDocument()
    expect(getByTestId('app-shell-workspace-desktop').closest('[data-surface]')).toHaveAttribute('data-surface', 'desktop')
    unmount()
  })

  it('keeps the main column scroll-owned while the nudge rail stays sticky', () => {
    const { getByTestId, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
      />,
    )

    expect(getByTestId('app-shell-main').className).toContain('overflow-visible')
    expect(getByTestId('app-shell-nudges').className).toContain('sticky')
    expect(getByTestId('app-shell-nudges').className).toContain('overflow-visible')
    expect(getByTestId('app-shell-nudges-scroll').className).not.toContain('max-h-[75vh]')
    expect(getByTestId('app-shell-nudges-scroll').className).not.toContain('overflow-y-auto')
    expect(getByTestId('app-shell-nudges-scroll').className).toContain('overflow-visible')
    unmount()
  })

  it('hides nudge rail on mobile surface', () => {
    const { queryByTestId, getByTestId, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="mobile"
      />,
    )

    expect(queryByTestId('app-shell-nudges')).not.toBeInTheDocument()
    expect(queryByTestId('app-shell-nudges-scroll')).not.toBeInTheDocument()
    expect(getByTestId('app-shell-workspace-mobile').className).toContain('safe-area-inset-bottom')
    expect(getByTestId('app-shell-workspace-mobile').closest('[data-surface]')).toHaveAttribute('data-layout', 'single')
    unmount()
  })

  it('keeps tablet in single-pane layout until split mode is active', () => {
    const { getByTestId, queryByTestId, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="tablet"
      />,
    )

    expect(queryByTestId('app-shell-nudges')).not.toBeInTheDocument()
    expect(getByTestId('app-shell-workspace-tablet').className).toContain('safe-area-inset-bottom')
    expect(getByTestId('app-shell-workspace-tablet').className).not.toContain('grid-cols')
    expect(getByTestId('app-shell-workspace-tablet').closest('[data-surface]')).toHaveAttribute('data-layout', 'single')
    unmount()
  })

  it('shows the nudge rail on tablet when split mode is active', () => {
    const { getByTestId, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="tablet"
        splitModeActive
      />,
    )

    expect(getByTestId('app-shell-nudges')).toBeInTheDocument()
    expect(getByTestId('app-shell-nudges-scroll')).toBeInTheDocument()
    expect(getByTestId('app-shell-workspace-tablet').className).toContain('md:grid-cols')
    expect(getByTestId('app-shell-workspace-tablet').closest('[data-surface]')).toHaveAttribute('data-layout', 'split')
    unmount()
  })

  it('treats explicit tablet split layout mode as split-capable', () => {
    const { getByTestId, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="tablet"
        layoutMode="split"
      />,
    )

    expect(getByTestId('app-shell-nudges')).toBeInTheDocument()
    expect(getByTestId('app-shell-workspace-tablet').closest('[data-surface]')).toHaveAttribute('data-layout', 'split')
    unmount()
  })

  it('ignores persisted split preference outside the tablet surface', () => {
    const mobile = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="mobile"
        layoutMode="split"
      />,
    )

    expect(mobile.getByTestId('app-shell-workspace-mobile').closest('[data-surface]')).toHaveAttribute('data-layout', 'single')
    expect(mobile.queryByTestId('app-shell-nudges')).not.toBeInTheDocument()
    mobile.unmount()

    const desktop = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="desktop"
        layoutMode="split"
      />,
    )

    expect(desktop.getByTestId('app-shell-workspace-desktop').closest('[data-surface]')).toHaveAttribute('data-layout', 'single')
    expect(desktop.getByTestId('app-shell-nudges')).toBeInTheDocument()
    desktop.unmount()
  })

  it('expands the loading workspace to the full frame below the navbar', () => {
    const { getByTestId, queryByTestId, unmount } = render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        fullFrameMain
      />,
    )

    expect(getByTestId('app-shell-workspace-full-frame')).toBeInTheDocument()
    expect(getByTestId('app-shell-main').className).toContain('flex-1')
    expect(getByTestId('app-shell-main').className).toContain('overflow-hidden')
    expect(queryByTestId('app-shell-nudges')).not.toBeInTheDocument()
    unmount()
  })
})
