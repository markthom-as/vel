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
    expect(screen.getAllByTestId('app-shell-nudges-scroll').at(-1)?.className).toContain('max-h-[75vh]')
    expect(screen.getAllByTestId('app-shell-nudges-scroll').at(-1)?.className).toContain('overflow-y-auto')
  })
})
