import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, waitFor, fireEvent, within } from '@testing-library/react'
import { ThreadView } from './ThreadView'
import * as api from '../api/client'
import type { WsEnvelope } from '../types'
import { clearQueryCache } from '../data/query'

const subscribeWs = vi.fn()

function requireWsListener(listener: ((event: WsEnvelope) => void) | null): (event: WsEnvelope) => void {
  expect(listener).not.toBeNull()
  return listener as (event: WsEnvelope) => void
}

function requireHtmlElement<T extends HTMLElement>(element: T | null): T {
  expect(element).not.toBeNull()
  return element as T
}

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

describe('ThreadView realtime sync', () => {
  beforeEach(() => {
    clearQueryCache()
    subscribeWs.mockReset()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  it('appends websocket messages for the active conversation and deduplicates repeats', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_1' } }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        return { ok: true, data: [], meta: { request_id: 'req_2' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    const { container } = render(<ThreadView conversationId="conv_1" />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByText('No messages yet.')).toBeInTheDocument()
    })

    const message = {
      id: 'msg_1',
      conversation_id: 'conv_1',
      role: 'assistant',
      kind: 'text',
      content: { text: 'streamed reply' },
      status: null,
      importance: null,
      created_at: 0,
      updated_at: null,
    }

    requireWsListener(wsListener)({ type: 'messages:new', timestamp: '1', payload: message })
    requireWsListener(wsListener)({ type: 'messages:new', timestamp: '2', payload: message })
    requireWsListener(wsListener)({
      type: 'messages:new',
      timestamp: '3',
      payload: { ...message, id: 'msg_2', conversation_id: 'conv_other', content: { text: 'ignore me' } },
    })

    await waitFor(() => {
      expect(within(thread).getByText('streamed reply')).toBeInTheDocument()
    })
    expect(within(thread).queryByText('ignore me')).not.toBeInTheDocument()
    expect(within(thread).getAllByText('streamed reply')).toHaveLength(1)
  })

  it('attaches intervention actions when interventions:new arrives for a message in the thread', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_1' } }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        return { ok: true, data: [], meta: { request_id: 'req_2' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    const { container } = render(<ThreadView conversationId="conv_1" />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByText('No messages yet.')).toBeInTheDocument()
    })

    requireWsListener(wsListener)({
      type: 'messages:new',
      timestamp: '1',
      payload: {
        id: 'msg_1',
        conversation_id: 'conv_1',
        role: 'assistant',
        kind: 'text',
        content: { text: 'needs action' },
        status: null,
        importance: null,
        created_at: 0,
        updated_at: null,
      },
    })

    await waitFor(() => {
      expect(within(thread).getByText('needs action')).toBeInTheDocument()
    })

    requireWsListener(wsListener)({
      type: 'interventions:new',
      timestamp: '2',
      payload: {
        id: 'intv_1',
        message_id: 'msg_1',
        kind: 'reminder',
        state: 'active',
        surfaced_at: 0,
        snoozed_until: null,
        confidence: null,
      },
    })

    await waitFor(() => {
      expect(within(thread).getByRole('button', { name: 'Snooze' })).toBeInTheDocument()
    })
  })

  it('replaces an optimistic user message when the confirmed websocket echo arrives', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_1' } }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        return { ok: true, data: [], meta: { request_id: 'req_2' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })
    vi.mocked(api.apiPost).mockImplementation(async (path: string) => {
      if (path === '/api/conversations/conv_1/messages') {
        requireWsListener(wsListener)({
          type: 'messages:new',
          timestamp: '1',
          payload: {
            id: 'msg_real',
            conversation_id: 'conv_1',
            role: 'user',
            kind: 'text',
            content: { text: 'Hi' },
            status: null,
            importance: null,
            created_at: 10,
            updated_at: null,
          },
        })
        return {
          ok: true,
          data: {
            user_message: {
              id: 'msg_real',
              conversation_id: 'conv_1',
              role: 'user',
              kind: 'text',
              content: { text: 'Hi' },
              status: null,
              importance: null,
              created_at: 10,
              updated_at: null,
            },
            assistant_message: null,
          },
          meta: { request_id: 'req_3' },
        }
      }
      throw new Error(`Unexpected POST ${path}`)
    })

    const { container } = render(<ThreadView conversationId="conv_1" />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByText('No messages yet.')).toBeInTheDocument()
    })

    fireEvent.change(within(thread).getByPlaceholderText(/message/i), { target: { value: 'Hi' } })
    fireEvent.click(within(thread).getByRole('button', { name: 'Send' }))

    await waitFor(() => {
      expect(within(thread).getAllByText('Hi')).toHaveLength(1)
    })
  })

  it('restores an intervention when an optimistic action request fails', async () => {
    subscribeWs.mockImplementation(() => () => {})

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations/conv_1/messages') {
        return {
          ok: true,
          data: [
            {
              id: 'msg_1',
              conversation_id: 'conv_1',
              role: 'assistant',
              kind: 'text',
              content: { text: 'needs action' },
              status: null,
              importance: null,
              created_at: 0,
              updated_at: null,
            },
          ],
          meta: { request_id: 'req_1' },
        }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        return {
          ok: true,
          data: [
            {
              id: 'intv_1',
              message_id: 'msg_1',
              kind: 'reminder',
              state: 'active',
              surfaced_at: 0,
              snoozed_until: null,
              confidence: null,
            },
          ],
          meta: { request_id: 'req_2' },
        }
      }
      throw new Error(`Unexpected GET ${path}`)
    })
    vi.mocked(api.apiPost).mockRejectedValue(new Error('nope'))

    const { container } = render(<ThreadView conversationId="conv_1" />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByRole('button', { name: 'Snooze' })).toBeInTheDocument()
    })

    fireEvent.click(within(thread).getByRole('button', { name: 'Snooze' }))

    await waitFor(() => {
      expect(within(thread).getByRole('button', { name: 'Snooze' })).toBeInTheDocument()
    })
  })
})
