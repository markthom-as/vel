import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { SurfaceDrawer } from './SurfaceDrawer'

describe('SurfaceDrawer', () => {
  it('renders title, content, and close action', () => {
    const onClose = vi.fn()

    render(
      <SurfaceDrawer title="Provenance" onClose={onClose}>
        <div>Drawer content</div>
      </SurfaceDrawer>,
    )

    expect(screen.getByText('Provenance')).toBeInTheDocument()
    expect(screen.getByText('Drawer content')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'Close Provenance' }))
    expect(onClose).toHaveBeenCalledTimes(1)
  })
})
