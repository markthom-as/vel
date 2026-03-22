import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
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
  afterEach(() => {
    cleanup()
  })

  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  it('renders the compact queue surface and routes open thread actions', async () => {
    const onOpenThread = vi.fn()

    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path.startsWith('/api/inbox')) {
        return {
          ok: true,
          data: [
            {
              id: 'intv_1',
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
              available_actions: ['acknowledge', 'snooze', 'resolve', 'dismiss', 'open_thread'],
              evidence: [{ source_kind: 'intervention', source_id: 'intv_1', label: 'reminder', detail: null }],
            },
          ],
          meta: { request_id: 'req_inbox' },
        } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    vi.mocked(api.apiPost).mockResolvedValue({
      ok: true,
      data: { id: 'intv_1', state: 'acknowledged' },
      meta: { request_id: 'req_post' },
    } as never)

    render(<InboxView onOpenThread={onOpenThread} />)

    await waitFor(() => {
      expect(screen.getByRole('heading', { name: 'Queue' })).toBeInTheDocument()
    })
    const status = screen.getByRole('group', { name: 'Queue state' })
    expect(within(status).getByText('New')).toBeInTheDocument()
    expect(within(status).getByText('Opened')).toBeInTheDocument()
    expect(within(status).getByText('Archived')).toBeInTheDocument()
    expect(within(status).getByText('All')).toBeInTheDocument()
    expect(within(status).getAllByText('1')).toHaveLength(2)
    expect(screen.queryByText(/triage what still needs a decision/i)).not.toBeInTheDocument()
    expect(screen.getByText('Reply before the review window closes')).toBeInTheDocument()
    const row = screen.getByRole('article')
    expect(within(row).getByRole('button', { name: /^Archive$/ })).toBeInTheDocument()

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
    const emptyStatus = screen.getByRole('group', { name: 'Queue state' })
    expect(within(emptyStatus).getAllByText('0')).toHaveLength(4)
  })

  it('filters queue rows by state bucket', async () => {
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path.startsWith('/api/inbox')) {
        return {
          ok: true,
          data: [
            {
              id: 'intv_new',
              message_id: 'msg_n',
              kind: 'reminder',
              state: 'active',
              surfaced_at: 1710000000,
              snoozed_until: null,
              confidence: null,
              conversation_id: null,
              title: 'New item',
              summary: 'Still unread.',
              project_id: null,
              project_label: null,
              available_actions: [],
              evidence: [],
            },
            {
              id: 'intv_open',
              message_id: 'msg_o',
              kind: 'follow_up',
              state: 'acknowledged',
              surfaced_at: 1710000001,
              snoozed_until: null,
              confidence: null,
              conversation_id: null,
              title: 'Opened item',
              summary: 'Already seen.',
              project_id: 'p1',
              project_label: 'Alpha',
              available_actions: [],
              evidence: [],
            },
          ],
          meta: { request_id: 'req_inbox_2' },
        } as never
      }
      throw new Error(`Unexpected GET ${path}`)
    })

    render(<InboxView />)

    await waitFor(() => {
      expect(screen.getByText('New item')).toBeInTheDocument()
    })
    expect(screen.getByText('Opened item')).toBeInTheDocument()

    const stateFilters = screen.getByRole('group', { name: 'Queue state' })
    fireEvent.click(within(stateFilters).getByRole('button', { name: /New,/ }))
    expect(screen.getByText('New item')).toBeInTheDocument()
    expect(screen.queryByText('Opened item')).not.toBeInTheDocument()

    fireEvent.click(within(stateFilters).getByRole('button', { name: /Opened,/ }))
    expect(screen.queryByText('New item')).not.toBeInTheDocument()
    expect(screen.getByText('Opened item')).toBeInTheDocument()

    fireEvent.click(within(stateFilters).getByRole('button', { name: /All,/ }))
    expect(screen.getByText('New item')).toBeInTheDocument()
    expect(screen.getByText('Opened item')).toBeInTheDocument()
  })
})
