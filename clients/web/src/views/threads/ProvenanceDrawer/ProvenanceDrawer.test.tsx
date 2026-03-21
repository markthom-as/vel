import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../../api/client'
import { clearQueryCache } from '../../../data/query'
import { ProvenanceDrawer } from './ProvenanceDrawer'

vi.mock('../../../api/client', () => ({
  apiGet: vi.fn(),
}))

describe('ProvenanceDrawer cache reuse', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
  })

  afterEach(() => {
    cleanup()
  })

  it('reuses cached provenance when the same message is reopened', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: {
        message_id: 'msg_1',
        events: [],
        signals: [],
        policy_decisions: [],
        linked_objects: [],
      },
      meta: { request_id: 'req_1' },
    })

    const view = render(<ProvenanceDrawer messageId="msg_1" onClose={() => {}} />)

    await waitFor(() => {
      expect(screen.getByText('Message')).toBeInTheDocument()
      expect(screen.getByText('msg_1')).toBeInTheDocument()
    })

    view.rerender(<ProvenanceDrawer messageId={null} onClose={() => {}} />)
    view.rerender(<ProvenanceDrawer messageId="msg_1" onClose={() => {}} />)

    await waitFor(() => {
      expect(screen.getByText('msg_1')).toBeInTheDocument()
    })

    expect(api.apiGet).toHaveBeenCalledTimes(1)
  })

  it('renders signals, policy decisions, and linked objects when present', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: {
        message_id: 'msg_2',
        events: [
          {
            id: 'evt_1',
            event_name: 'message.created',
            created_at: 1,
            payload: { role: 'assistant' },
          },
        ],
        signals: [
          {
            signal_type: 'git_activity',
            source: 'git',
            summary: 'Touched vel runtime',
            repo: 'vel',
          },
        ],
        policy_decisions: [
          {
            decision: 'intervention_created',
            reason: 'assistant card was actionable',
            status: 'active',
          },
        ],
        linked_objects: [
          {
            object_type: 'conversation',
            object_id: 'conv_1',
            summary: 'Primary thread',
          },
        ],
      },
      meta: { request_id: 'req_2' },
    })

    render(<ProvenanceDrawer messageId="msg_2" onClose={() => {}} />)

    await waitFor(() => {
      expect(screen.getByText('Touched vel runtime')).toBeInTheDocument()
    })

    expect(screen.getByText('Policy decisions')).toBeInTheDocument()
    expect(screen.getByText('Linked objects')).toBeInTheDocument()
    expect(screen.getByText('git_activity')).toBeInTheDocument()
    expect(screen.getByText('Touched vel runtime')).toBeInTheDocument()
    expect(screen.getByText('intervention_created')).toBeInTheDocument()
    expect(screen.getByText('assistant card was actionable')).toBeInTheDocument()
    expect(screen.getByText('conversation')).toBeInTheDocument()
    expect(screen.getByText('conv_1')).toBeInTheDocument()
  })

  it('allows raw payload inspection per structured item', async () => {
    vi.mocked(api.apiGet).mockResolvedValue({
      ok: true,
      data: {
        message_id: 'msg_3',
        events: [],
        signals: [
          {
            signal_type: 'message_thread',
            source: 'messaging',
            summary: 'Waiting on reply',
            payload_hint: 'thread-42',
          },
        ],
        policy_decisions: [],
        linked_objects: [],
      },
      meta: { request_id: 'req_3' },
    })

    const view = render(<ProvenanceDrawer messageId="msg_3" onClose={() => {}} />)

    await waitFor(() => {
      expect(within(view.container).getByText('Waiting on reply')).toBeInTheDocument()
    })

    fireEvent.click(within(view.container).getByRole('button', { name: /show raw/i }))
    expect(within(view.container).getByText('thread-42')).toBeInTheDocument()
  })
})
