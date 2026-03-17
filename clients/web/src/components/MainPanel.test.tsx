import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { MainPanel } from './MainPanel'

vi.mock('./NowView', () => ({
  NowView: () => <div>Now view</div>,
}))

vi.mock('./InboxView', () => ({
  InboxView: () => <div>Inbox view</div>,
}))

vi.mock('./ThreadView', () => ({
  ThreadView: ({ conversationId }: { conversationId: string | null }) => (
    <div>{conversationId ? `Thread ${conversationId}` : 'Thread empty'}</div>
  ),
}))

vi.mock('./SuggestionsView', () => ({
  SuggestionsView: () => <div>Suggestions view</div>,
}))

describe('MainPanel', () => {
  it('shows the Now view when mainView is now', () => {
    render(<MainPanel conversationId={null} mainView="now" onOpenSettings={() => {}} />)
    expect(screen.getByText('Now view')).toBeInTheDocument()
    expect(screen.queryByText('Inbox view')).toBeNull()
  })

  it('shows the Inbox view when mainView is inbox', () => {
    render(<MainPanel conversationId={null} mainView="inbox" onOpenSettings={() => {}} />)
    expect(screen.getByText('Inbox view')).toBeInTheDocument()
    expect(screen.queryByText('Thread empty')).toBeNull()
  })

  it('shows the thread view when mainView is threads', () => {
    render(<MainPanel conversationId="conv_1" mainView="threads" onOpenSettings={() => {}} />)
    expect(screen.getByText('Thread conv_1')).toBeInTheDocument()
  })

  it('shows the Suggestions view when mainView is suggestions', () => {
    render(<MainPanel conversationId={null} mainView="suggestions" onOpenSettings={() => {}} />)
    expect(screen.getByText('Suggestions view')).toBeInTheDocument()
  })
})
