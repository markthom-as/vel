import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, waitFor, fireEvent, within } from '@testing-library/react'
import { ThreadView } from './ThreadView'
import * as api from '../api/client'
import type { WsEnvelope } from '../types'
import { clearQueryCache } from '../data/query'
import { resetWsQuerySyncForTests } from '../data/ws-sync'

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
    resetWsQuerySyncForTests()
    subscribeWs.mockReset()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  function conversationRecord(id: string, title: string, updatedAt = 10) {
    return {
      id,
      title,
      kind: 'general',
      pinned: false,
      archived: false,
      created_at: 1,
      updated_at: updatedAt,
    }
  }

  it('appends websocket messages for the active conversation and deduplicates repeats', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return { ok: true, data: [conversationRecord('conv_1', 'Current thread')], meta: { request_id: 'req_0' } }
      }
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
      expect(within(thread).getByText(/nothing needs longer follow-up yet/i)).toBeInTheDocument()
    })
    expect(within(thread).getByText('Resume longer follow-through')).toBeInTheDocument()
    expect(within(thread).getByText(/reflow edits, planning disagreements, and schedule shaping belong here/i)).toBeInTheDocument()
    expect(
      within(thread).getByPlaceholderText(/ask, capture, or talk to vel/i),
    ).toBeInTheDocument()

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

    requireWsListener(wsListener)({ type: 'messages:new', timestamp: '2026-03-16T12:00:00Z', payload: message })
    requireWsListener(wsListener)({ type: 'messages:new', timestamp: '2026-03-16T12:00:01Z', payload: message })
    requireWsListener(wsListener)({
      type: 'messages:new',
      timestamp: '2026-03-16T12:00:02Z',
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
      if (path === '/api/conversations') {
        return { ok: true, data: [conversationRecord('conv_1', 'Current thread')], meta: { request_id: 'req_0' } }
      }
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
      expect(within(thread).getByText(/nothing needs longer follow-up yet/i)).toBeInTheDocument()
    })
    expect(within(thread).getByText('Resume longer follow-through')).toBeInTheDocument()
    expect(
      within(thread).getByPlaceholderText(/ask, capture, or talk to vel/i),
    ).toBeInTheDocument()

    requireWsListener(wsListener)({
      type: 'messages:new',
      timestamp: '2026-03-16T12:01:00Z',
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
      timestamp: '2026-03-16T12:02:00Z',
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
      if (path === '/api/conversations') {
        return { ok: true, data: [conversationRecord('conv_1', 'Existing')], meta: { request_id: 'req_0' } }
      }
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_1' } }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        return { ok: true, data: [], meta: { request_id: 'req_2' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })
    vi.mocked(api.apiPost).mockImplementation(async (path: string) => {
      if (path === '/api/assistant/entry') {
        requireWsListener(wsListener)({
          type: 'messages:new',
          timestamp: '2026-03-16T12:03:00Z',
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
            route_target: 'threads',
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
            assistant_error: null,
            conversation: {
              id: 'conv_1',
              title: 'Existing',
              kind: 'general',
              pinned: false,
              archived: false,
              created_at: 0,
              updated_at: 10,
            },
          },
          meta: { request_id: 'req_3' },
        }
      }
      throw new Error(`Unexpected POST ${path}`)
    })

    const { container } = render(<ThreadView conversationId="conv_1" />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByText(/nothing needs longer follow-up yet/i)).toBeInTheDocument()
    })

    fireEvent.change(within(thread).getByPlaceholderText(/ask, capture, or talk to vel/i), { target: { value: 'Hi' } })
    fireEvent.click(within(thread).getByRole('button', { name: 'Send' }))

    await waitFor(() => {
      expect(within(thread).getAllByText('Hi')).toHaveLength(1)
    })
  })

  it('restores an intervention when an optimistic action request fails', async () => {
    subscribeWs.mockImplementation(() => () => {})

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return { ok: true, data: [conversationRecord('conv_1', 'Needs action')], meta: { request_id: 'req_0' } }
      }
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

  it('falls back to the latest updated conversation when no thread is explicitly selected', async () => {
    subscribeWs.mockImplementation(() => () => {})

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            conversationRecord('conv_old', 'Old thread', 10),
            conversationRecord('conv_latest', 'Latest thread', 20),
          ],
          meta: { request_id: 'req_conversations' },
        }
      }
      if (path === '/api/conversations/conv_latest/messages') {
        return {
          ok: true,
          data: [
            {
              id: 'msg_1',
              conversation_id: 'conv_latest',
              role: 'assistant',
              kind: 'text',
              content: { text: 'latest reply' },
              status: null,
              importance: null,
              created_at: 0,
              updated_at: null,
            },
          ],
          meta: { request_id: 'req_messages_latest' },
        }
      }
      if (path === '/api/conversations/conv_latest/interventions') {
        return { ok: true, data: [], meta: { request_id: 'req_interventions_latest' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    const { container } = render(<ThreadView conversationId={null} />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByText('latest reply')).toBeInTheDocument()
    })

    expect(vi.mocked(api.apiGet)).toHaveBeenCalledWith('/api/conversations', expect.any(Function))
    expect(vi.mocked(api.apiGet)).toHaveBeenCalledWith('/api/conversations/conv_latest/messages', expect.any(Function))
  })

  it('keeps an optimistically resolved intervention hidden when a stale active refetch arrives', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    let interventionsFetchCount = 0
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return { ok: true, data: [conversationRecord('conv_1', 'Needs action')], meta: { request_id: 'req_0' } }
      }
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
          meta: { request_id: 'req_msg' },
        }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        interventionsFetchCount += 1
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
          meta: { request_id: `req_intv_${interventionsFetchCount}` },
        }
      }
      if (path === '/api/inbox' || path === '/api/conversations') {
        return { ok: true, data: [], meta: { request_id: 'req_other' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })
    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_1', state: 'resolved' },
      meta: { request_id: 'req_post' },
    } as never)

    const { container } = render(<ThreadView conversationId="conv_1" />)
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByRole('button', { name: 'Resolve' })).toBeInTheDocument()
    })

    fireEvent.click(within(thread).getByRole('button', { name: 'Resolve' }))

    await waitFor(() => {
      expect(within(thread).queryByRole('button', { name: 'Resolve' })).not.toBeInTheDocument()
    })

    requireWsListener(wsListener)({
      type: 'interventions:updated',
      timestamp: '2026-03-16T12:04:00Z',
      payload: { id: 'intv_1', state: 'resolved' },
    })

    await waitFor(() => {
      expect(within(thread).queryByRole('button', { name: 'Resolve' })).not.toBeInTheDocument()
    })
  })

  it('filters recent threads and lets the operator switch continuity without reopening inbox triage', async () => {
    subscribeWs.mockImplementation(() => () => {})

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/conversations') {
        return {
          ok: true,
          data: [
            conversationRecord('conv_1', 'Design review'),
            conversationRecord('conv_2', 'Weekly planning', 20),
          ],
          meta: { request_id: 'req_conversations' },
        }
      }
      if (path === '/api/conversations/conv_1/messages') {
        return { ok: true, data: [], meta: { request_id: 'req_messages' } }
      }
      if (path === '/api/conversations/conv_1/interventions') {
        return { ok: true, data: [], meta: { request_id: 'req_interventions' } }
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    const onSelectConversation = vi.fn()
    const { container } = render(
      <ThreadView conversationId="conv_1" onSelectConversation={onSelectConversation} />,
    )
    const thread = requireHtmlElement(container as HTMLElement | null)

    await waitFor(() => {
      expect(within(thread).getByPlaceholderText(/find recent follow-up/i)).toBeInTheDocument()
    })

    expect(within(thread).getByText(/longer follow-through only\. triage stays in inbox\./i)).toBeInTheDocument()
    fireEvent.change(within(thread).getByPlaceholderText(/find recent follow-up/i), {
      target: { value: 'weekly' },
    })

    await waitFor(() => {
      expect(within(thread).getByRole('button', { name: 'Weekly planning' })).toBeInTheDocument()
    })

    fireEvent.click(within(thread).getByRole('button', { name: 'Weekly planning' }))
    expect(onSelectConversation).toHaveBeenCalledWith('conv_2')
  })
})
