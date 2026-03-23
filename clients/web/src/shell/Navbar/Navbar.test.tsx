import { cleanup, fireEvent, render, screen, within } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Navbar } from '.'

afterEach(() => {
  cleanup()
})

describe('Navbar', () => {
  function renderNavbar(activeView: 'now' | 'threads' | 'system' = 'now') {
    const onSelectView = vi.fn()

    render(
      <Navbar
        activeView={activeView}
        onSelectView={onSelectView}
      />,
    )

    return { onSelectView }
  }

  it('renders top nav items and routes clicks through handlers', () => {
    const { onSelectView } = renderNavbar()

    fireEvent.click(screen.getByRole('button', { name: /System/i }))

    expect(onSelectView).toHaveBeenCalledWith('system')
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

  it('does not render a global info rail toggle', () => {
    renderNavbar()
    expect(screen.queryByRole('button', { name: /open info/i })).not.toBeInTheDocument()
  })

  it('keeps icon plus label visible for every primary surface control', () => {
    renderNavbar()
    for (const label of ['Now', 'Threads', 'System']) {
      const button = screen.getByRole('button', { name: label })
      expect(button).toHaveTextContent(label)
      expect(within(button).getByText(label)).toBeInTheDocument()
    }
  })
})
