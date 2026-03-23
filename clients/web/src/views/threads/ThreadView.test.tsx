import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { resetWsQuerySyncForTests } from '../../data/ws-sync'
import { ThreadView } from './ThreadView'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

vi.mock('../../realtime/ws', () => ({
  subscribeWs: () => () => {},
}))

describe('ThreadView', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  it('renders the split thread layout with list metadata and message preview', async () => {
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
      expect(screen.getByRole('heading', { name: 'Proposal thread' })).toBeInTheDocument()
    })
    expect(screen.getByPlaceholderText('Find thread')).toBeInTheDocument()
    expect(screen.getAllByText('Can you help shape the rollout plan?').length).toBeGreaterThan(0)
    expect(screen.getAllByText(/needs input/i).length).toBeGreaterThan(0)
    expect(screen.getByText(/Attach or create an object first/i)).toBeInTheDocument()
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
      expect(screen.getByRole('heading', { name: 'Current thread' })).toBeInTheDocument()
    })

    fireEvent.click(screen.getByRole('button', { name: /Review follow-up/i }))
    expect(onSelectConversation).toHaveBeenCalledWith('conv_2')
  })
})
