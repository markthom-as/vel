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

describe('MainPanel', () => {
  it('shows the Now view when showNow is enabled', () => {
    render(<MainPanel conversationId={null} showInbox={false} showNow />)
    expect(screen.getByText('Now view')).toBeInTheDocument()
    expect(screen.queryByText('Inbox view')).toBeNull()
  })

  it('shows the Inbox view when inbox is active', () => {
    render(<MainPanel conversationId={null} showInbox showNow={false} />)
    expect(screen.getByText('Inbox view')).toBeInTheDocument()
    expect(screen.queryByText('Thread empty')).toBeNull()
  })

  it('shows the thread view and thread header for a selected conversation', () => {
    render(<MainPanel conversationId="conv_1" showInbox={false} showNow={false} />)
    expect(screen.getByText('Thread')).toBeInTheDocument()
    expect(screen.getByText('Thread conv_1')).toBeInTheDocument()
  })
})
