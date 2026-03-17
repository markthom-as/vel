import { afterEach, describe, expect, it, vi } from 'vitest'
import { apiGet, apiPatch, apiPost } from './client'
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

  it('preserves structured API error messages for non-2xx POST responses', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: false,
      status: 409,
      headers: new Headers({ 'content-type': 'application/json' }),
      json: async () => ({
        ok: false,
        error: { code: 'conflict', message: 'run cannot be retried automatically' },
        meta: { request_id: 'req_3' },
      }),
    } as Response)

    await expect(apiPost('/v1/runs/run_1', {})).rejects.toThrow(
      'API 409: run cannot be retried automatically',
    )
  })

  it('falls back to status and path when a non-2xx PATCH response is not JSON', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: false,
      status: 500,
      headers: new Headers({ 'content-type': 'text/plain' }),
      json: async () => {
        throw new Error('not json')
      },
    } as unknown as Response)

    await expect(apiPatch('/api/settings', { disable_proactive: true })).rejects.toThrow(
      'API 500: /api/settings',
    )
  })
})
