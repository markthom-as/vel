import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { DocumentationPanel } from './DocumentationPanel'

describe('DocumentationPanel', () => {
  it('renders contextual current-view guidance for the active surface', () => {
    render(<DocumentationPanel currentView="Now" />)

    expect(screen.getByText('Current View')).toBeInTheDocument()
    expect(
      screen.getByText(/current-day truth, nudge meaning, and the operator rules behind queue ordering/i),
    ).toBeInTheDocument()
  })
})
