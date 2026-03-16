import { renderHook, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { clearQueryCache, invalidateQuery, useQuery } from './query'

describe('shared query cache', () => {
  beforeEach(() => {
    clearQueryCache()
  })

  it('shares one in-flight fetch across subscribers for the same key', async () => {
    const key = ['shared', 'resource'] as const
    const fetcher = vi.fn().mockResolvedValue(['value'])

    const first = renderHook(() => useQuery(key, fetcher))
    const second = renderHook(() => useQuery(key, fetcher))

    await waitFor(() => {
      expect(first.result.current.data).toEqual(['value'])
      expect(second.result.current.data).toEqual(['value'])
    })

    expect(fetcher).toHaveBeenCalledTimes(1)
  })

  it('keeps cached data while invalidation triggers a background refetch', async () => {
    const key = ['shared', 'refreshing'] as const
    const fetcher = vi
      .fn<() => Promise<string[]>>()
      .mockResolvedValueOnce(['first'])
      .mockResolvedValueOnce(['second'])

    const query = renderHook(() => useQuery(key, fetcher))

    await waitFor(() => {
      expect(query.result.current.data).toEqual(['first'])
    })

    invalidateQuery(key, { refetch: true })

    expect(query.result.current.data).toEqual(['first'])
    expect(query.result.current.loading).toBe(false)

    await waitFor(() => {
      expect(query.result.current.data).toEqual(['second'])
    })

    expect(fetcher).toHaveBeenCalledTimes(2)
  })
})
