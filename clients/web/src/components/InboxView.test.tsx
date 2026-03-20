import { describe, it, expect, vi, beforeEach } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { InboxView } from './InboxView'
import * as api from '../api/client'
import type { WsEnvelope } from '../types'
import { clearQueryCache } from '../data/query'
import { resetWsQuerySyncForTests } from '../data/ws-sync'

const subscribeWs = vi.fn()

function requireWsListener(listener: ((event: WsEnvelope) => void) | null): (event: WsEnvelope) => void {
  expect(listener).not.toBeNull()
  return listener as (event: WsEnvelope) => void
}

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

vi.mock('../realtime/ws', () => ({
  subscribeWs: (listener: (event: WsEnvelope) => void) => subscribeWs(listener),
}))

describe('InboxView realtime sync', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()
    resetWsQuerySyncForTests()
    subscribeWs.mockReset()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
  })

  it('renders dense triage actions and routes open thread to the canonical thread surface', async () => {
    let wsListener: ((event: WsEnvelope) => void) | null = null
    subscribeWs.mockImplementation((listener) => {
      wsListener = listener
      return () => {}
    })

    vi.mocked(api.apiGet).mockResolvedValueOnce({
      ok: true,
      data: [
        {
          id: 'intv_1',
          message_id: 'msg_1',
          kind: 'reminder',
          state: 'active',
          surfaced_at: 1_710_000_000,
          snoozed_until: null,
          confidence: 0.92,
          conversation_id: 'conv_1',
          title: 'Reply before the review window closes',
          summary: 'The next meeting depends on a response from this thread.',
          project_id: 'proj_ops',
          project_label: 'Ops',
          available_actions: ['acknowledge', 'snooze', 'dismiss', 'open_thread'],
          evidence: [
            {
              source_kind: 'message',
              source_id: 'msg_1',
              label: 'Unanswered stakeholder thread',
              detail: null,
            },
          ],
        },
      ],
      meta: { request_id: 'req_1' },
    })

    const onOpenThread = vi.fn()
    render(<InboxView onOpenThread={onOpenThread} />)

    await waitFor(() => {
      expect(screen.getByText('Reply before the review window closes')).toBeInTheDocument()
    })

    expect(screen.getByText('Triage what still needs a decision')).toBeInTheDocument()
    expect(screen.getByText(/triage first here\. history and longer back-and-forth stay in threads\./i)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Acknowledge' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Snooze 10m' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Dismiss' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Open thread' })).toBeInTheDocument()
    expect(screen.getByText('Evidence')).toBeInTheDocument()
    expect(screen.getByText('Unanswered stakeholder thread')).toBeInTheDocument()
    expect(screen.getByText(/keep the decision here unless you need continuity, history, or a longer follow-up\./i)).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'Open thread' }))
    expect(onOpenThread).toHaveBeenCalledWith('conv_1')

    expect(wsListener).not.toBeNull()
  })

  it('shows the Phase 05 empty state and accepts realtime interventions', async () => {
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
      expect(screen.getByText('Inbox is clear')).toBeInTheDocument()
    })
    expect(screen.getByText('Triage what still needs a decision')).toBeInTheDocument()
    expect(screen.getByText(/No actions need triage right now/i)).toBeInTheDocument()
    expect(screen.getByText(/this surface stays focused on unresolved decisions, not archive browsing\./i)).toBeInTheDocument()

    requireWsListener(wsListener)({
      type: 'interventions:new',
      timestamp: '2026-03-16T12:01:00Z',
      payload: {
        id: 'intv_2',
        message_id: 'msg_2',
        kind: 'risk',
        state: 'active',
        surfaced_at: 1_710_000_060,
        snoozed_until: null,
        confidence: 0.88,
        conversation_id: 'conv_2',
        title: 'Clarify the latest blocker',
        summary: 'The system surfaced a new risk without an explicit owner.',
        project_id: null,
        project_label: null,
        available_actions: ['acknowledge', 'open_thread'],
        evidence: [
          {
            source_kind: 'run',
            source_id: 'run_2',
            label: 'Fresh failure event',
            detail: null,
          },
        ],
      },
    })

    await waitFor(() => {
      expect(screen.getByText('Clarify the latest blocker')).toBeInTheDocument()
    })
    expect(screen.getByRole('button', { name: 'Acknowledge' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Open thread' })).toBeInTheDocument()
  })
})
