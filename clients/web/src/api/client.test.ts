import { afterEach, describe, expect, it, vi } from 'vitest'
import { apiGet, apiPost } from './client'
import { decodeApiResponse, decodeConversationData, type ApiResponse, type ConversationData } from '../types'

describe('api client decoders', () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('applies the provided decoder to GET responses', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: true,
      json: async () => ({
        ok: true,
        data: {
          id: 'conv_1',
          title: 'Thread',
          kind: 'general',
          pinned: false,
          archived: false,
          created_at: 1,
          updated_at: 2,
        },
        meta: { request_id: 'req_1' },
      }),
    } as Response)

    const response = await apiGet<ApiResponse<ConversationData>>(
      '/api/conversations/conv_1',
      (value) => decodeApiResponse(value, decodeConversationData),
    )

    expect(response.data?.id).toBe('conv_1')
  })

  it('surfaces decoder failures for malformed POST responses', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: true,
      json: async () => ({
        ok: true,
        data: { title: 'missing id' },
        meta: { request_id: 'req_2' },
      }),
    } as Response)

    await expect(
      apiPost<ApiResponse<ConversationData>>(
        '/api/conversations',
        { title: 'New conversation', kind: 'general' },
        (value) => decodeApiResponse(value, decodeConversationData),
      ),
    ).rejects.toThrow(/conversation.id/)
  })
})
