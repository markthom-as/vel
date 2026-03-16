import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import { InboxView } from './InboxView'
import * as api from '../api/client'
import type { WsEnvelope } from '../types'

const subscribeWs = vi.fn()

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
}))

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

describe('InboxView realtime sync', () => {
  beforeEach(() => {
    subscribeWs.mockReset()
    vi.mocked(api.apiGet).mockReset()
  })

  it('refetches inbox items when interventions are updated', async () => {
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
            id: 'intv_1',
            message_id: 'msg_1',
            kind: 'reminder',
            state: 'active',
            surfaced_at: 0,
            snoozed_until: null,
            confidence: null,
          },
        ],
        meta: { request_id: 'req_1' },
      })
      .mockResolvedValueOnce({
        ok: true,
        data: [],
        meta: { request_id: 'req_2' },
      })

    render(<InboxView />)

    await waitFor(() => {
      expect(screen.getByText('reminder')).toBeInTheDocument()
    })

    wsListener?.({ type: 'interventions:updated', timestamp: '1', payload: { id: 'intv_1', state: 'resolved' } })

    await waitFor(() => {
      expect(screen.getByText('No active interventions.')).toBeInTheDocument()
    })
  })

  it('adds a new inbox item when interventions:new arrives', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: [],
      meta: { request_id: 'req_1' },
    })

    render(<InboxView />)

    await waitFor(() => {
      expect(screen.getByText('No active interventions.')).toBeInTheDocument()
    })

    wsListener?.({
      type: 'interventions:new',
      timestamp: '1',
      payload: {
        id: 'intv_2',
        message_id: 'msg_2',
        kind: 'risk',
        state: 'active',
        surfaced_at: 0,
        snoozed_until: null,
        confidence: null,
      },
    })

    await waitFor(() => {
      expect(screen.getByText('risk')).toBeInTheDocument()
    })
    expect(screen.getByText(/message: msg_2/)).toBeInTheDocument()
  })
})
