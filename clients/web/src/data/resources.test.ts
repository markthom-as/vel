import { afterEach, describe, expect, it, vi } from 'vitest'
import { runEvaluate, syncSource } from './resources'

describe('resource payload decoders', () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('decodes sync responses from the API envelope payload', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: true,
      json: async () => ({
        ok: true,
        data: {
          source: 'notes',
          signals_ingested: 4,
        },
        meta: { request_id: 'req_sync' },
      }),
    } as Response)

    const response = await syncSource('notes')

    expect(response.ok).toBe(true)
    expect(response.data).toEqual({
      source: 'notes',
      signals_ingested: 4,
    })
  })

  it('decodes evaluate responses from the API envelope payload', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValue({
      ok: true,
      json: async () => ({
        ok: true,
        data: {
          inferred_states: 1,
          nudges_created_or_updated: 2,
        },
        meta: { request_id: 'req_eval' },
      }),
    } as Response)

    const response = await runEvaluate()

    expect(response.ok).toBe(true)
    expect(response.data).toEqual({
      inferred_states: 1,
      nudges_created_or_updated: 2,
    })
  })
})
