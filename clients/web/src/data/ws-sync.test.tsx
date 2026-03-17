import { render, screen, waitFor } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import type { WsEnvelope } from '../types'
import { clearQueryCache, useQuery } from './query'
import { queryKeys } from './resources'
import { resetWsQuerySyncForTests, subscribeWsQuerySync } from './ws-sync'

const subscribeWs = vi.fn()

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

function CurrentContextProbe({ fetcher }: { fetcher: () => Promise<{ computed_at: number; context: { mode: string } }> }) {
  const { data } = useQuery(queryKeys.currentContext(), fetcher)
  return <div data-testid="current-context">{data?.context.mode ?? 'missing'}</div>
}

function ContextExplainProbe({ fetcher }: { fetcher: () => Promise<{ reasons: string[] }> }) {
  const { data } = useQuery(queryKeys.contextExplain(), fetcher)
  return <div data-testid="context-explain">{data?.reasons.join(',') ?? 'missing'}</div>
}

function DriftExplainProbe({ fetcher }: { fetcher: () => Promise<{ reasons: string[] }> }) {
  const { data } = useQuery(queryKeys.driftExplain(), fetcher)
  return <div data-testid="drift-explain">{data?.reasons.join(',') ?? 'missing'}</div>
}

function CommitmentsProbe({ fetcher }: { fetcher: () => Promise<Array<{ id: string }>> }) {
  const { data } = useQuery(queryKeys.commitments(12), fetcher)
  return <div data-testid="commitments">{data?.length ?? 0}</div>
}

describe('ws query sync', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    subscribeWs.mockReset()
  })

  afterEach(() => {
    resetWsQuerySyncForTests()
    clearQueryCache()
  })

  it('hydrates current context and refetches explain-oriented queries on context updates', async () => {
    let emit!: (event: WsEnvelope) => void
    subscribeWs.mockImplementation((listener) => {
      emit = listener
      return () => {}
    })

    const currentContextFetcher = vi
      .fn<() => Promise<{ computed_at: number; context: { mode: string } }>>()
      .mockResolvedValue({ computed_at: 1710000000, context: { mode: 'focus' } })
    const contextExplainFetcher = vi
      .fn<() => Promise<{ reasons: string[] }>>()
      .mockResolvedValueOnce({ reasons: ['prep window active'] })
      .mockResolvedValueOnce({ reasons: ['context refreshed from evaluate'] })
    const driftExplainFetcher = vi
      .fn<() => Promise<{ reasons: string[] }>>()
      .mockResolvedValueOnce({ reasons: ['recent git activity'] })
      .mockResolvedValueOnce({ reasons: ['calendar pressure increased'] })
    const commitmentsFetcher = vi
      .fn<() => Promise<Array<{ id: string }>>>()
      .mockResolvedValueOnce([{ id: 'commit_1' }])
      .mockResolvedValueOnce([{ id: 'commit_1' }, { id: 'commit_2' }])

    const unsubscribe = subscribeWsQuerySync()

    render(
      <>
        <CurrentContextProbe fetcher={currentContextFetcher} />
        <ContextExplainProbe fetcher={contextExplainFetcher} />
        <DriftExplainProbe fetcher={driftExplainFetcher} />
        <CommitmentsProbe fetcher={commitmentsFetcher} />
      </>,
    )

    await waitFor(() => {
      expect(screen.getByTestId('current-context')).toHaveTextContent('focus')
      expect(screen.getByTestId('context-explain')).toHaveTextContent('prep window active')
      expect(screen.getByTestId('drift-explain')).toHaveTextContent('recent git activity')
      expect(screen.getByTestId('commitments')).toHaveTextContent('1')
    })

    emit({
      type: 'context:updated',
      timestamp: '2026-03-16T12:10:00Z',
      payload: {
        computed_at: 1710000300,
        context: {
          mode: 'review',
        },
      },
    })

    await waitFor(() => {
      expect(screen.getByTestId('current-context')).toHaveTextContent('review')
      expect(screen.getByTestId('context-explain')).toHaveTextContent('context refreshed from evaluate')
      expect(screen.getByTestId('drift-explain')).toHaveTextContent('calendar pressure increased')
      expect(screen.getByTestId('commitments')).toHaveTextContent('2')
    })

    expect(currentContextFetcher).toHaveBeenCalledTimes(1)
    expect(contextExplainFetcher).toHaveBeenCalledTimes(2)
    expect(driftExplainFetcher).toHaveBeenCalledTimes(2)
    expect(commitmentsFetcher).toHaveBeenCalledTimes(2)

    unsubscribe()
  })
})
