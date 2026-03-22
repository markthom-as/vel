import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
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
})
