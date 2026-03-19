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
    fireEvent.click(screen.getByRole('button', { name: /Projects/ }))
    fireEvent.click(screen.getByRole('button', { name: /Settings/ }))

    expect(onSelectView).toHaveBeenCalledWith('projects')
    expect(onSelectView).toHaveBeenCalledWith('settings')
  })

  it('keeps now and inbox first, with threads then projects demoted behind them', () => {
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
    const projectsIndex = topLevelButtons.findIndex((label) => label.includes('Projects'))

    expect(nowIndex).toBeLessThan(inboxIndex)
    expect(inboxIndex).toBeLessThan(threadsIndex)
    expect(threadsIndex).toBeLessThan(projectsIndex)
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
    expect(screen.getAllByText(/Conversation history is scoped to the Threads surface/i).length).toBeGreaterThan(0)

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

  it('groups daily-use, support, then advanced surfaces', () => {
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
    expect(screen.getAllByText('Advanced').length).toBeGreaterThan(0)
  })
})
