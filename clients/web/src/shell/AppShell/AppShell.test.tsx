import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { AppShell } from './AppShell'

describe('AppShell', () => {
  it('renders navigation and main content without a global side rail slot', () => {
    render(
      <AppShell
        navigation={<div>Navigation</div>}
        main={<div>Main content</div>}
      />,
    )

    expect(screen.getByText('Navigation')).toBeInTheDocument()
    expect(screen.getByText('Main content')).toBeInTheDocument()
    expect(screen.queryByText(/info/i)).not.toBeInTheDocument()
  })
})
