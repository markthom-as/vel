import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
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
      expect(screen.getByRole('button', { name: 'Current thread' })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /Review follow-up/i }))
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

    await screen.findByRole('button', { name: 'Current thread' })
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
      const activeRows = screen.getAllByRole('button', { name: /Needs review thread/i })
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
})
