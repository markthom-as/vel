import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { resetWsQuerySyncForTests } from '../../data/ws-sync'
import { ThreadView } from './ThreadView'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
  apiPatch: vi.fn(),
}))

vi.mock('../../realtime/ws', () => ({
  subscribeWs: () => () => {},
}))

describe('ThreadView', () => {
  beforeEach(() => {
    vi.spyOn(Date, 'now').mockReturnValue(20_000)
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
    vi.mocked(api.apiPatch).mockReset()
  })

  afterEach(() => {
    cleanup()
    vi.restoreAllMocks()
  })

  it('renders the compact thread layout without the rejected helper panels', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            {
              id: 'conv_1',
              title: 'Proposal thread',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 1,
              updated_at: 10,
              message_count: 1,
              last_message_at: 10,
              project_label: 'Vel',
              continuation: {
                thread_id: 'conv_1',
                thread_type: 'action_resolution',
                lifecycle_stage: 'staged',
                continuation: {
                  escalation_reason: 'This assistant proposal became multi-step and remains in Threads for explicit follow-through.',
                  continuation_context: {
                    source_message_id: 'msg_1',
                  },
                  review_requirements: [],
                  bounded_capability_state: 'proposal_review_gated',
                  continuation_category: 'needs_input',
                  open_target: 'thread_detail',
                },
              },
            },
          ],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      if (path === '/api/conversations/conv_1/messages') {
        return {
          ok: true,
          data: [
            {
              id: 'msg_1',
              conversation_id: 'conv_1',
              role: 'user',
              kind: 'text',
              content: { text: 'Can you help shape the rollout plan?' },
              status: null,
              importance: null,
              created_at: 10,
              updated_at: null,
            },
          ],
          meta: { request_id: 'req_messages' },
        } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    render(<ThreadView conversationId="conv_1" />)

    await waitFor(() => {
    expect(screen.getByRole('button', { name: 'Proposal thread' })).toBeInTheDocument()
    })
    expect(screen.getByText(/CURRENT THREAD \| 1 MESSAGE \| PARTICIPANTS/i)).toBeInTheDocument()
    expect(screen.getAllByText('Vel').length).toBeGreaterThan(0)
    expect(screen.getByPlaceholderText('Find thread')).toBeInTheDocument()
    expect(screen.getAllByText(/LATEST /i).length).toBeGreaterThan(0)
    expect(screen.getAllByText(/CREATED /i).length).toBeGreaterThan(0)
    expect(screen.getByRole('button', { name: 'Archive thread' })).toBeInTheDocument()
    expect(screen.getByText(/proposal review gated/i)).toBeInTheDocument()
    expect(screen.getByText(/source message id/i)).toBeInTheDocument()
    expect(screen.getAllByText('Can you help shape the rollout plan?').length).toBeGreaterThan(0)
    expect(screen.queryByText('Shared review panel')).not.toBeInTheDocument()
    expect(screen.queryByText('Continuity stream')).not.toBeInTheDocument()
    expect(screen.queryByText('Sticky per thread. Supporting, not page identity.')).not.toBeInTheDocument()
    expect(screen.queryByText(/Chronology follows context/i)).not.toBeInTheDocument()
  })

  it('switches threads from the left list', async () => {
    const onSelectConversation = vi.fn()

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            {
              id: 'conv_1',
              title: 'Current thread',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 1,
              updated_at: 20,
              continuation: null,
            },
            {
              id: 'conv_2',
              title: 'Review follow-up',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 2,
              updated_at: 10,
              continuation: null,
            },
          ],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      if (path === '/api/conversations/conv_1/messages' || path === '/api/conversations/conv_2/messages') {
        return { ok: true, data: [], meta: { request_id: `req_${path}` } } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    render(<ThreadView conversationId="conv_1" onSelectConversation={onSelectConversation} />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Open Current thread' })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: 'Open Review follow-up' }))
    expect(onSelectConversation).toHaveBeenCalledWith('conv_2')
  })

  it('keeps the left rail expanded without collapse controls', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            {
              id: 'conv_1',
              title: 'Current thread',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 1,
              updated_at: 20,
              continuation: null,
            },
          ],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_messages' } } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    render(<ThreadView conversationId="conv_1" onSelectConversation={vi.fn()} />)

    await screen.findByRole('button', { name: 'Open Current thread' })
    expect(screen.queryByRole('button', { name: 'Collapse thread rail' })).not.toBeInTheDocument()
    expect(screen.getByPlaceholderText('Find thread')).toBeInTheDocument()
  })

  it('marks the resolved thread active and exposes the full filter model', async () => {
    const onSelectConversation = vi.fn()

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            {
              id: 'conv_1',
              title: 'Needs review thread',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 1,
              updated_at: 20,
              continuation: {
                thread_id: 'conv_1',
                thread_type: 'action_resolution',
                lifecycle_stage: 'staged',
                continuation: {
                  escalation_reason: 'Needs explicit user judgment',
                  continuation_context: {},
                  review_requirements: ['confirm owner'],
                  bounded_capability_state: 'proposal_review_gated',
                  continuation_category: 'needs_input',
                  open_target: 'thread_detail',
                },
              },
            },
          ],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_messages' } } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    render(<ThreadView conversationId={null} onSelectConversation={onSelectConversation} />)

    await waitFor(() => {
      const activeRows = screen.getAllByRole('button', { name: /Open Needs review thread/i })
      expect(activeRows.some((button) => button.getAttribute('aria-current') === 'true')).toBe(true)
    })
    expect(onSelectConversation).toHaveBeenCalledWith('conv_1')
    expect(screen.getByRole('button', { name: 'Needs Review, 1 item' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Unread, 1 item' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Archived, 0 items' })).toBeInTheDocument()
  })

  it('archives the current thread and routes to the next active thread', async () => {
    const onSelectConversation = vi.fn()

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            {
              id: 'conv_1',
              title: 'Current thread',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 1,
              updated_at: 20,
              continuation: null,
            },
            {
              id: 'conv_2',
              title: 'Next thread',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 2,
              updated_at: 10,
              continuation: null,
            },
          ],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      if (path === '/api/conversations/conv_1/messages' || path === '/api/conversations/conv_2/messages') {
        return { ok: true, data: [], meta: { request_id: `req_${path}` } } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })
    vi.mocked(api.apiPatch).mockResolvedValue({
      ok: true,
      data: {
        id: 'conv_1',
        title: 'Current thread',
        kind: 'general',
        pinned: false,
        archived: true,
        created_at: 1,
        updated_at: 21,
        continuation: null,
      },
      meta: { request_id: 'req_patch' },
    } as never)

    render(<ThreadView conversationId="conv_1" onSelectConversation={onSelectConversation} />)

    await screen.findByRole('button', { name: 'Archive thread' })
    fireEvent.click(screen.getByRole('button', { name: 'Archive thread' }))

    await waitFor(() => {
      expect(api.apiPatch).toHaveBeenCalledWith('/api/conversations/conv_1', { archived: true }, expect.any(Function))
      expect(onSelectConversation).toHaveBeenCalledWith('conv_2')
    })
  })

  it('toggles thread call mode through the conversation patch seam', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            {
              id: 'conv_1',
              title: 'Call thread',
              kind: 'general',
              pinned: false,
              archived: false,
              call_mode_active: false,
              created_at: 1,
              updated_at: 20,
              continuation: null,
            },
          ],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_messages' } } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })
    vi.mocked(api.apiPatch).mockResolvedValue({
      ok: true,
      data: {
        id: 'conv_1',
        title: 'Call thread',
        kind: 'general',
        pinned: false,
        archived: false,
        call_mode_active: true,
        created_at: 1,
        updated_at: 21,
        continuation: null,
      },
      meta: { request_id: 'req_call_mode' },
    } as never)

    render(<ThreadView conversationId="conv_1" onSelectConversation={vi.fn()} />)

    await screen.findByRole('button', { name: 'Start call' })
    fireEvent.click(screen.getByRole('button', { name: 'Start call' }))

    await waitFor(() => {
      expect(api.apiPatch).toHaveBeenCalledWith(
        '/api/conversations/conv_1',
        { call_mode_active: true },
        expect.any(Function),
      )
    })
  })

  it('shows the intentional empty state when no threads exist yet', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [],
          meta: { request_id: 'req_conversations' },
        } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    render(<ThreadView conversationId={null} onSelectConversation={vi.fn()} />)

    await screen.findByText('No thread selected yet')
    expect(screen.getByText(/latest thread will land here/i)).toBeInTheDocument()
  })

  it('renders mobile compact rows as selectable stable thread buttons with cues', async () => {
    mockThreadViewData([
      buildConversation({
        id: 'conv_1',
        title: 'Pinned review thread',
        pinned: true,
        project_label: 'Vel',
        continuation: buildContinuation({ reviewRequirements: ['confirm owner'] }),
      }),
      buildConversation({
        id: 'conv_2',
        title: 'Quiet thread',
        updated_at: 30,
      }),
    ])

    render(<ThreadView conversationId="conv_1" onSelectConversation={vi.fn()} surface="mobile" />)

    const selectedRow = await screen.findByRole('button', {
      name: /pinned review thread, unread continuation, needs review/i,
    })
    expect(selectedRow).toHaveAttribute('aria-current', 'true')
    expect(selectedRow).toHaveAttribute('data-conversation-id', 'conv_1')
    expect(selectedRow.className).toContain('min-h-11')
    expect(within(selectedRow).getByText('Pinned')).toBeInTheDocument()
    expect(within(selectedRow).getByText('Review')).toBeInTheDocument()
    expect(within(selectedRow).getByText('Vel')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Quiet thread' })).not.toHaveAttribute('aria-current')
  })

  it('opens a thread from the mobile compact list', async () => {
    const onSelectConversation = vi.fn()
    mockThreadViewData([
      buildConversation({ id: 'conv_1', title: 'Current mobile thread' }),
      buildConversation({ id: 'conv_2', title: 'Open this mobile thread' }),
    ])

    render(<ThreadView conversationId="conv_1" onSelectConversation={onSelectConversation} surface="mobile" />)

    fireEvent.click(await screen.findByRole('button', { name: 'Open this mobile thread' }))

    expect(onSelectConversation).toHaveBeenCalledWith('conv_2')
  })

  it('preserves selected thread semantics through surface changes', async () => {
    const onSelectConversation = vi.fn()
    mockThreadViewData([
      buildConversation({ id: 'conv_1', title: 'First thread' }),
      buildConversation({ id: 'conv_2', title: 'Selected through resize' }),
    ])

    const view = render(
      <ThreadView conversationId="conv_2" onSelectConversation={onSelectConversation} surface="mobile" />,
    )

    expect(await screen.findByRole('button', { name: 'Selected through resize' })).toHaveAttribute('aria-current', 'true')

    view.rerender(<ThreadView conversationId="conv_2" onSelectConversation={onSelectConversation} surface="desktop" />)
    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: 'Selected through resize' }).length).toBeGreaterThan(0)
    })

    view.rerender(<ThreadView conversationId="conv_2" onSelectConversation={onSelectConversation} surface="mobile" />)
    expect(await screen.findByRole('button', { name: 'Selected through resize' })).toHaveAttribute('aria-current', 'true')
    expect(onSelectConversation).not.toHaveBeenCalled()
  })

  it('uses the full list and detail layout for tablet split mode', async () => {
    mockThreadViewData(
      [
        buildConversation({ id: 'conv_1', title: 'Tablet split thread' }),
        buildConversation({ id: 'conv_2', title: 'Side rail thread' }),
      ],
      [
        {
          id: 'msg_1',
          conversation_id: 'conv_1',
          role: 'user',
          kind: 'text',
          content: { text: 'Keep list and detail visible.' },
          status: null,
          importance: null,
          created_at: 10,
          updated_at: null,
        },
      ],
    )

    render(
      <ThreadView
        conversationId="conv_1"
        onSelectConversation={vi.fn()}
        surface="tablet"
        threadLayoutSplit
      />,
    )

    expect(await screen.findByTestId('thread-layout-shell')).toHaveAttribute('data-thread-layout', 'tablet-split')
    expect(screen.getByTestId('thread-list-rail')).toBeInTheDocument()
    expect(screen.getByTestId('thread-detail-region')).toBeInTheDocument()
    expect(screen.getByPlaceholderText('Find thread')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Archive thread' })).toBeInTheDocument()
    expect(screen.getAllByText('Keep list and detail visible.').length).toBeGreaterThan(0)
  })

  it('keeps pin and archive affordances reachable in the tablet split list rail', async () => {
    mockThreadViewData([
      buildConversation({ id: 'conv_1', title: 'Tablet active thread' }),
      buildConversation({ id: 'conv_2', title: 'Side rail action thread' }),
    ])
    vi.mocked(api.apiPatch).mockResolvedValue({
      ok: true,
      data: buildConversation({ id: 'conv_2', title: 'Side rail action thread', pinned: true }),
      meta: { request_id: 'req_pin_thread' },
    } as never)

    render(
      <ThreadView
        conversationId="conv_1"
        onSelectConversation={vi.fn()}
        surface="tablet"
        threadLayoutSplit
      />,
    )

    const pinButton = await screen.findByRole('button', { name: 'Pin Side rail action thread' })
    expect(pinButton).toHaveClass('min-h-8')
    expect(screen.getByRole('button', { name: 'Archive Side rail action thread' })).toHaveClass('min-h-8')
    expect(screen.getByRole('button', { name: 'Mute unavailable for Side rail action thread' })).toBeDisabled()

    fireEvent.click(pinButton)

    await waitFor(() => {
      expect(api.apiPatch).toHaveBeenCalledWith('/api/conversations/conv_2', { pinned: true }, expect.any(Function))
    })
  })

  it('uses compact thread layout for tablet single-pane mode', async () => {
    mockThreadViewData([
      buildConversation({ id: 'conv_1', title: 'Tablet single thread' }),
      buildConversation({ id: 'conv_2', title: 'Hidden detail rail thread' }),
    ])

    render(
      <ThreadView
        conversationId="conv_1"
        onSelectConversation={vi.fn()}
        surface="tablet"
        threadLayoutSplit={false}
      />,
    )

    expect(await screen.findByTestId('thread-layout-shell')).toHaveAttribute('data-thread-layout', 'compact')
    expect(screen.queryByTestId('thread-list-rail')).not.toBeInTheDocument()
    expect(screen.queryByTestId('thread-detail-region')).not.toBeInTheDocument()
    expect(screen.getByRole('button', { name: /threads \(2\)/i })).toBeInTheDocument()
  })
})

function mockThreadViewData(conversations: Array<Record<string, unknown>>, messages: Array<Record<string, unknown>> = []) {
  vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
    if (path === '/api/conversations') {
      return {
        ok: true,
        data: conversations,
        meta: { request_id: 'req_conversations' },
      } as never
    }
    const conversationId = path.match(/^\/api\/conversations\/([^/]+)\/messages$/)?.[1]
    if (conversationId) {
      return {
        ok: true,
        data: messages.filter((message) => message.conversation_id === conversationId),
        meta: { request_id: `req_messages_${conversationId}` },
      } as never
    }
    throw new Error(`Unexpected GET ${path}`)
  })
}

function buildConversation(overrides: Record<string, unknown> = {}) {
  return {
    id: 'conv_1',
    title: 'Thread',
    kind: 'general',
    pinned: false,
    archived: false,
    call_mode_active: false,
    created_at: 1,
    updated_at: 20,
    message_count: 0,
    last_message_at: null,
    project_label: null,
    continuation: null,
    ...overrides,
  }
}

function buildContinuation({ reviewRequirements = [] }: { reviewRequirements?: string[] } = {}) {
  return {
    thread_id: 'thread_1',
    thread_type: 'action_resolution',
    lifecycle_stage: 'staged',
    continuation: {
      escalation_reason: 'Needs explicit user judgment',
      continuation_context: {},
      review_requirements: reviewRequirements,
      bounded_capability_state: 'proposal_review_gated',
      continuation_category: 'needs_input',
      open_target: 'thread_detail',
    },
  }
}
