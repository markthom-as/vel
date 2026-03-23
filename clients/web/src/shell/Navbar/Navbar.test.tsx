import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { Navbar } from '.'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
}))

afterEach(() => {
  cleanup()
})

describe('Navbar', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            task_lane: {
              active: { id: 'task_1', text: 'Write weekly review' },
              pending: [],
              recent_completed: [],
            },
            context_line: { text: 'Fallback context line' },
            nudge_bars: [{ id: 'nudge_1' }, { id: 'nudge_2' }],
            action_items: [{ thread_route: { thread_id: 'conv_1' } }],
            reflow_status: { thread_id: 'conv_2' },
            mesh_summary: { queued_write_count: 3 },
          },
          meta: { request_id: 'req_nav' },
        } as never
      }
      throw new Error(`Unmocked apiGet path: ${path}`)
    })
  })

  function renderNavbar(
    activeView: 'now' | 'threads' | 'system' = 'now',
    infoPanelOpen = false,
  ) {
    const onSelectView = vi.fn()
    const onOpenDocumentation = vi.fn()

    render(
      <Navbar
        activeView={activeView}
        onSelectView={onSelectView}
        onOpenDocumentation={onOpenDocumentation}
        infoPanelOpen={infoPanelOpen}
      />,
    )

    return { onSelectView, onOpenDocumentation }
  }

  it('renders top nav items and routes clicks through handlers', () => {
    const { onSelectView, onOpenDocumentation } = renderNavbar()

    fireEvent.click(screen.getByRole('button', { name: /System/i }))
    fireEvent.click(screen.getByRole('button', { name: 'Open info' }))

    expect(onSelectView).toHaveBeenCalledWith('system')
    expect(onOpenDocumentation).toHaveBeenCalled()
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

  it('uses the top-level info button as the documentation affordance', () => {
    renderNavbar()
    expect(screen.getByRole('button', { name: 'Open info' })).toBeInTheDocument()
  })

  it('labels the info button Close when the panel is open', () => {
    renderNavbar('now', true)
    expect(screen.getByRole('button', { name: 'Close info' })).toBeInTheDocument()
  })

  it('shows the top-level info affordance by default', () => {
    renderNavbar()
    expect(screen.getByRole('button', { name: /open info/i })).toBeInTheDocument()
  })

  it('renders navbar context and surface status from current now data', async () => {
    renderNavbar()

    await waitFor(() => {
      expect(screen.getByText('Write weekly review')).toBeInTheDocument()
    })

    expect(screen.getByText(/Sat, Mar 9, 9:00 AM/i)).toBeInTheDocument()
    const statusGroup = screen.getByLabelText('Surface status')
    expect(within(statusGroup).getAllByText('2')).toHaveLength(2)
    expect(within(statusGroup).getByText('3')).toBeInTheDocument()
  })
})
