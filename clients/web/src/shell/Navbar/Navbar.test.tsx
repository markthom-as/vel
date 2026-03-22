import { cleanup, fireEvent, render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Navbar } from '.'

afterEach(() => {
  cleanup()
})

describe('Navbar', () => {
  function renderNavbar(
    activeView: 'now' | 'inbox' | 'threads' | 'settings' = 'now',
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

    fireEvent.click(screen.getByRole('button', { name: /Settings/i }))
    fireEvent.click(screen.getByRole('button', { name: 'Open info' }))

    expect(onSelectView).toHaveBeenCalledWith('settings')
    expect(onOpenDocumentation).toHaveBeenCalled()
  })

  it('keeps now, inbox, threads, and settings in top-nav order', () => {
    renderNavbar()

    const labels = screen
      .getAllByRole('button')
      .map((button) => button.textContent)
      .filter((label): label is string => Boolean(label))

    const nowIndex = labels.findIndex((label) => label.includes('Now'))
    const inboxIndex = labels.findIndex((label) => label.includes('Inbox'))
    const threadsIndex = labels.findIndex((label) => label.includes('Threads'))
    const settingsIndex = labels.findIndex((label) => label.includes('Settings'))

    expect(nowIndex).toBeLessThan(inboxIndex)
    expect(inboxIndex).toBeLessThan(threadsIndex)
    expect(threadsIndex).toBeLessThan(settingsIndex)
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
})
