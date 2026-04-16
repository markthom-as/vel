import { describe, it, expect, vi, beforeEach } from 'vitest'
import { fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { ConversationList } from './ConversationList'
import * as api from '../../../api/client'
import type { WsEnvelope } from '../../../types'
import { clearQueryCache } from '../../../data/query'
import { resetWsQuerySyncForTests } from '../../../data/ws-sync'

const subscribeWs = vi.fn()

function requireWsListener(listener: ((event: WsEnvelope) => void) | null): (event: WsEnvelope) => void {
  expect(listener).not.toBeNull()
  return listener as (event: WsEnvelope) => void
}

vi.mock('../../../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
}))

vi.mock('../../../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

describe('ConversationList realtime sync', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    subscribeWs.mockReset()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPatch).mockReset()
  })

  it('updates conversation list cache when a websocket message arrives', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet)
      .mockResolvedValueOnce({
        ok: true,
        data: [
          {
            id: 'conv_1',
            title: 'First',
            kind: 'general',
            pinned: false,
            archived: false,
            created_at: 0,
            updated_at: 0,
          },
        ],
        meta: { request_id: 'req_1' },
      })

    render(<ConversationList selectedId={null} onSelect={() => {}} />)

    await waitFor(() => {
      expect(screen.getByText('First')).toBeInTheDocument()
    })
    expect(api.apiGet).toHaveBeenCalledTimes(1)

    requireWsListener(wsListener)({
      type: 'messages:new',
      timestamp: '2026-03-16T12:00:00Z',
      payload: {
        id: 'msg_1',
        conversation_id: 'conv_1',
        role: 'assistant',
        kind: 'text',
        content: 'hello',
        status: null,
        importance: null,
        created_at: 1,
        updated_at: null,
      },
    })

    await waitFor(() => {
      expect(api.apiGet).toHaveBeenCalledTimes(1)
    })
    expect(screen.queryByText('Second')).not.toBeInTheDocument()
  })

  it('renders compact touch rows with selected, pinned, unread, and review cues', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: [
        buildConversation({
          id: 'conv_1',
          title: 'Pinned planning thread',
          pinned: true,
          project_label: 'Vel',
          continuation: buildContinuation({ reviewRequirements: ['confirm next action'] }),
        }),
        buildConversation({
          id: 'conv_2',
          title: 'Quiet follow-up',
          last_message_at: 20,
        }),
      ],
      meta: { request_id: 'req_threads' },
    })

    render(<ConversationList selectedId="conv_1" onSelect={() => {}} />)

    const selectedRow = await screen.findByRole('button', {
      name: /pinned planning thread, unread continuation, needs review/i,
    })
    expect(selectedRow).toHaveAttribute('aria-current', 'true')
    expect(selectedRow).toHaveAttribute('data-conversation-id', 'conv_1')
    expect(selectedRow.className).toContain('min-h-14')
    expect(within(selectedRow).getByText('Pinned')).toBeInTheDocument()
    expect(within(selectedRow).getByText('Review')).toBeInTheDocument()
    expect(within(selectedRow).getByText('Vel')).toBeInTheDocument()

    const quietRow = screen.getByRole('button', { name: 'Quiet follow-up' })
    expect(quietRow).not.toHaveAttribute('aria-current')
  })

  it('opens a conversation from the full row tap target', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: [
        buildConversation({
          id: 'conv_2',
          title: 'Open from mobile row',
        }),
      ],
      meta: { request_id: 'req_threads_open' },
    })
    const onSelect = vi.fn()

    render(<ConversationList selectedId={null} onSelect={onSelect} />)

    fireEvent.click(await screen.findByRole('button', { name: 'Open from mobile row' }))

    expect(onSelect).toHaveBeenCalledWith('conv_2')
  })

  it('keeps touch-friendly overflow actions reachable without selecting the row', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: [
        buildConversation({
          id: 'conv_3',
          title: 'Actionable thread',
        }),
      ],
      meta: { request_id: 'req_threads_actions' },
    })
    vi.mocked(api.apiPatch).mockResolvedValue({
      ok: true,
      data: buildConversation({ id: 'conv_3', title: 'Actionable thread', pinned: true }),
      meta: { request_id: 'req_patch_thread' },
    })
    const onSelect = vi.fn()

    render(<ConversationList selectedId={null} onSelect={onSelect} />)

    fireEvent.click(await screen.findByRole('button', { name: /more actions for actionable thread/i }))
    const menu = screen.getByRole('menu', { name: /actions for actionable thread/i })
    expect(within(menu).getByRole('menuitem', { name: 'Pin' })).toHaveClass('min-h-10')
    expect(within(menu).getByRole('menuitem', { name: 'Archive' })).toHaveClass('min-h-10')
    expect(within(menu).getByRole('menuitem', { name: 'Mute unavailable' })).toBeDisabled()

    fireEvent.click(within(menu).getByRole('menuitem', { name: 'Pin' }))

    await waitFor(() => {
      expect(api.apiPatch).toHaveBeenCalledWith('/api/conversations/conv_3', { pinned: true }, expect.any(Function))
    })
    expect(onSelect).not.toHaveBeenCalled()
  })
})

function buildConversation(overrides: Record<string, unknown> = {}) {
  return {
    id: 'conv_1',
    title: 'Thread',
    kind: 'general',
    pinned: false,
    archived: false,
    call_mode_active: false,
    created_at: 0,
    updated_at: 10,
    message_count: 1,
    last_message_at: null,
    project_label: null,
    continuation: null,
    ...overrides,
  }
}

function buildContinuation({ reviewRequirements = [] }: { reviewRequirements?: string[] } = {}) {
  return {
    thread_id: 'thread_1',
    thread_type: 'conversation',
    lifecycle_stage: 'active',
    continuation: {
      escalation_reason: 'Needs operator input',
      continuation_context: {},
      review_requirements: reviewRequirements,
      bounded_capability_state: 'needs_input',
      continuation_category: 'needs_input',
      open_target: 'thread',
    },
  }
}
