import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { resetWsQuerySyncForTests } from '../../data/ws-sync'
import { InboxView } from './InboxView'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

vi.mock('../../realtime/ws', () => ({
  subscribeWs: () => () => {},
}))

describe('InboxView', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  it('renders the compact queue surface and routes open thread actions', async () => {
    const onOpenThread = vi.fn()

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/inbox') {
        return {
          ok: true,
          data: [
            {
              id: 'act_1',
              message_id: 'msg_1',
              kind: 'reminder',
              state: 'active',
              surfaced_at: 1710000000,
              snoozed_until: null,
              confidence: 0.92,
              conversation_id: 'conv_1',
              title: 'Reply before the review window closes',
              summary: 'The next meeting depends on a response from this thread.',
              project_id: 'proj_ops',
              project_label: 'Ops',
              available_actions: ['acknowledge', 'snooze', 'dismiss', 'open_thread'],
              evidence: [{ source_kind: 'thread', source_id: 'thr_1', label: 'Unanswered stakeholder thread', detail: null }],
            },
          ],
          meta: { request_id: 'req_inbox' },
        } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'act_1', state: 'acknowledged' },
      meta: { request_id: 'req_post' },
    } as never)

    render(<InboxView onOpenThread={onOpenThread} />)

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: 'Queue' })).toBeInTheDocument()
    })
    expect(screen.queryByText(/triage what still needs a decision/i)).not.toBeInTheDocument()
    expect(screen.getByText('Reply before the review window closes')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: /Open thread/i }))
    expect(onOpenThread).toHaveBeenCalledWith('conv_1')
  })

  it('shows the compact zero state', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: [],
      meta: { request_id: 'req_empty' },
    } as never)

    render(<InboxView />)

    await waitFor(() => {
      expect(screen.getByText('No open queue items.')).toBeInTheDocument()
    })
  })
})
