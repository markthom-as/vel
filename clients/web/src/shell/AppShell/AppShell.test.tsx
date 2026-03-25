import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { AppShell } from './AppShell'

describe('AppShell', () => {
  it('renders navigation and main content without a global side rail slot', () => {
    render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        actionBar={<div>Action bar</div>}
      />,
    )

    expect(screen.getByText('Navigation')).toBeInTheDocument()
    expect(screen.getByText('Main content')).toBeInTheDocument()
    expect(screen.getByText('Nudges')).toBeInTheDocument()
    expect(screen.getByText('Action bar')).toBeInTheDocument()
    expect(screen.queryByText(/info/i)).not.toBeInTheDocument()
  })

  it('keeps the main column scroll-owned while the nudge rail stays sticky', () => {
    render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
      />,
    )

    expect(screen.getAllByTestId('app-shell-main').at(-1)?.className).toContain('overflow-visible')
    expect(screen.getAllByTestId('app-shell-nudges').at(-1)?.className).toContain('sticky')
    expect(screen.getAllByTestId('app-shell-nudges').at(-1)?.className).toContain('overflow-visible')
    expect(screen.getAllByTestId('app-shell-nudges-scroll').at(-1)?.className).not.toContain('max-h-[75vh]')
    expect(screen.getAllByTestId('app-shell-nudges-scroll').at(-1)?.className).not.toContain('overflow-y-auto')
    expect(screen.getAllByTestId('app-shell-nudges-scroll').at(-1)?.className).toContain('overflow-visible')
  })

  it('hides nudge rail on mobile surface', () => {
    render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="mobile"
      />,
    )

    expect(screen.queryByTestId('app-shell-nudges')).not.toBeInTheDocument()
    expect(screen.queryByTestId('app-shell-nudges-scroll')).not.toBeInTheDocument()
    expect(screen.getByTestId('app-shell-workspace-mobile')).toBeInTheDocument()
  })

  it('shows nudge rail on tablet surface', () => {
    render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
        nudgeZone={<div>Nudges</div>}
        surface="tablet"
      />,
    )

    expect(screen.getByTestId('app-shell-nudges')).toBeInTheDocument()
    expect(screen.getByTestId('app-shell-nudges-scroll')).toBeInTheDocument()
    expect(screen.getByTestId('app-shell-workspace-tablet')).toBeInTheDocument()
  })
})
