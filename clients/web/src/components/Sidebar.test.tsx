import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { Sidebar } from './Sidebar'

vi.mock('./ConversationList', () => ({
  ConversationList: ({ selectedId }: { selectedId: string | null }) => (
    <div>Conversation list {selectedId ?? 'none'}</div>
  ),
}))

describe('Sidebar', () => {
  it('renders all top-level surfaces and routes clicks through onSelectView', () => {
    const onSelectView = vi.fn()

    render(
      <Sidebar
        activeView="now"
        onSelectView={onSelectView}
        selectedConversationId={null}
        onSelectConversation={() => {}}
      />,
    )

    expect(screen.queryByRole('button', { name: /^Stats$/ })).toBeNull()
    expect(screen.queryByRole('button', { name: /^Suggestions$/ })).toBeNull()
    expect(screen.queryByRole('button', { name: /^Projects$/ })).toBeNull()
    fireEvent.click(screen.getByRole('button', { name: /Settings/ }))

    expect(onSelectView).toHaveBeenCalledWith('settings')
  })

  it('keeps now, inbox, and threads as the daily-use shell before support surfaces', () => {
    render(
      <Sidebar
        activeView="now"
        onSelectView={() => {}}
        selectedConversationId={null}
        onSelectConversation={() => {}}
      />,
    )

    const topLevelButtons = screen
      .getAllByRole('button')
      .map((button) => button.textContent)
      .filter((label): label is string => Boolean(label))

    const nowIndex = topLevelButtons.findIndex((label) => label.includes('Now'))
    const inboxIndex = topLevelButtons.findIndex((label) => label.includes('Inbox'))
    const threadsIndex = topLevelButtons.findIndex((label) => label.includes('Threads'))
    const settingsIndex = topLevelButtons.findIndex((label) => label.includes('Settings'))

    expect(nowIndex).toBeLessThan(inboxIndex)
    expect(inboxIndex).toBeLessThan(threadsIndex)
    expect(threadsIndex).toBeLessThan(settingsIndex)
  })

  it('shows conversation list only while on the Threads surface', () => {
    const { rerender } = render(
      <Sidebar
        activeView="now"
        onSelectView={() => {}}
        selectedConversationId="conv_1"
        onSelectConversation={() => {}}
      />,
    )
    expect(screen.queryByText('Conversation list conv_1')).toBeNull()
    expect(screen.getAllByText(/Conversation history is scoped to Threads/i).length).toBeGreaterThan(0)

    rerender(
      <Sidebar
        activeView="threads"
        onSelectView={() => {}}
        selectedConversationId="conv_1"
        onSelectConversation={() => {}}
      />,
    )
    expect(screen.getByText('Conversation list conv_1')).toBeInTheDocument()
  })

  it('groups daily-use and support surfaces only', () => {
    render(
      <Sidebar
        activeView="now"
        onSelectView={() => {}}
        selectedConversationId={null}
        onSelectConversation={() => {}}
      />,
    )

    expect(screen.getAllByText('Daily Use').length).toBeGreaterThan(0)
    expect(screen.getAllByText('Support').length).toBeGreaterThan(0)
    expect(screen.queryByText('Advanced')).toBeNull()
  })

  it('keeps thread controls contextual instead of surfacing them on every surface', () => {
    const { rerender } = render(
      <Sidebar
        activeView="now"
        onSelectView={() => {}}
        selectedConversationId={null}
        onSelectConversation={() => {}}
        onNewConversation={() => {}}
      />,
    )

    expect(screen.queryByRole('button', { name: /new thread/i })).toBeNull()
    expect(screen.getAllByText(/threads stay contextual/i).length).toBeGreaterThan(0)

    rerender(
      <Sidebar
        activeView="threads"
        onSelectView={() => {}}
        selectedConversationId="conv_1"
        onSelectConversation={() => {}}
        onNewConversation={() => {}}
      />,
    )

    expect(screen.getAllByRole('button', { name: /new thread/i }).length).toBeGreaterThan(0)
    expect(screen.getAllByRole('button', { name: /hide history/i }).length).toBeGreaterThan(0)
  })
})
