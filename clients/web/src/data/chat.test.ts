import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as client from '../api/client'
import {
  acknowledgeInboxItem,
  dismissInboxItem,
  getInboxThreadPath,
  snoozeInboxItem,
} from './chat'
import type { InboxItemData } from '../types'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

describe('chat data helpers', () => {
  beforeEach(() => {
    vi.mocked(client.apiPost).mockReset()
  })

  it('posts acknowledge inbox mutations to /api/interventions/:id/acknowledge', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_1', state: 'acknowledged' },
      meta: { request_id: 'req_ack' },
    } as never)

    const response = await acknowledgeInboxItem('intv_1')

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_1/acknowledge',
      {},
      expect.any(Function),
    )
    expect(response.data?.state).toBe('acknowledged')
  })

  it('posts snooze inbox mutations to /api/interventions/:id/snooze', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_2', state: 'snoozed' },
      meta: { request_id: 'req_snooze' },
    } as never)

    const response = await snoozeInboxItem('intv_2', 15)

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_2/snooze',
      { minutes: 15 },
      expect.any(Function),
    )
    expect(response.data?.state).toBe('snoozed')
  })

  it('posts dismiss inbox mutations to /api/interventions/:id/dismiss', async () => {
    vi.mocked(client.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_3', state: 'dismissed' },
      meta: { request_id: 'req_dismiss' },
    } as never)

    const response = await dismissInboxItem('intv_3')

    expect(client.apiPost).toHaveBeenCalledWith(
      '/api/interventions/intv_3/dismiss',
      {},
      expect.any(Function),
    )
    expect(response.data?.state).toBe('dismissed')
  })

  it('reuses /api/conversations/:id only when open_thread is available', () => {
    const threadable: InboxItemData = {
      id: 'intv_4',
      message_id: 'msg_4',
      kind: 'risk',
      state: 'active',
      surfaced_at: 1710000000,
      snoozed_until: null,
      confidence: null,
      conversation_id: 'conv_4',
      title: 'Open the thread',
      summary: 'Continue in the existing thread',
      project_id: null,
      project_label: null,
      available_actions: ['acknowledge', 'open_thread'],
      evidence: [],
    }
    const nonThreadable: InboxItemData = {
      ...threadable,
      id: 'intv_5',
      conversation_id: null,
      available_actions: ['acknowledge', 'dismiss'],
    }

    expect(getInboxThreadPath(threadable)).toBe('/api/conversations/conv_4')
    expect(getInboxThreadPath(nonThreadable)).toBeNull()
  })
})
