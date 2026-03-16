import { render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { NowView } from './NowView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
}))

describe('NowView', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/context/current') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            context: {
              mode: 'focus',
              morning_state: 'engaged',
              meds_status: 'pending',
              global_risk_level: 'medium',
              global_risk_score: 0.72,
              next_event_start_ts: 1710003600,
              leave_by_ts: 1710001800,
              attention_state: 'on_task',
              drift_type: null,
              message_waiting_on_me_count: 2,
              message_waiting_on_others_count: 1,
            },
          },
          meta: { request_id: 'req_ctx' },
        } as never
      }
      if (path === '/v1/explain/context') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            mode: 'focus',
            morning_state: 'engaged',
            context: {},
            signals_used: ['sig_cal_1'],
            signal_summaries: [
              {
                signal_id: 'sig_cal_1',
                signal_type: 'calendar_event',
                source: 'google_calendar',
                timestamp: 1710003600,
                summary: {
                  title: 'Design review',
                  location: 'Room 4B',
                },
              },
            ],
            commitments_used: ['commit_1'],
            risk_used: ['risk_1'],
            reasons: ['prep window active', 'calendar event soon'],
          },
          meta: { request_id: 'req_explain' },
        } as never
      }
      if (path === '/v1/explain/drift') {
        return {
          ok: true,
          data: {
            attention_state: 'on_task',
            drift_type: null,
            drift_severity: null,
            confidence: 0.8,
            reasons: ['recent git activity indicates active work'],
            signals_used: [],
            signal_summaries: [],
            commitments_used: [],
          },
          meta: { request_id: 'req_drift' },
        } as never
      }
      if (path === '/v1/commitments?limit=12') {
        return {
          ok: true,
          data: [
            {
              id: 'commit_todoist_1',
              text: 'Reply to Dimitri',
              source_type: 'todoist',
              source_id: 'todoist_1',
              status: 'open',
              due_at: '2026-03-16T19:00:00Z',
              project: 'Ops',
              commitment_kind: 'todo',
              metadata_json: {},
              created_at: 1710000000,
              updated_at: 1710000000,
            },
            {
              id: 'commit_local_1',
              text: 'Write weekly review',
              source_type: 'capture',
              source_id: null,
              status: 'open',
              due_at: null,
              project: null,
              commitment_kind: 'writing',
              metadata_json: {},
              created_at: 1710000000,
              updated_at: 1710000000,
            },
          ],
          meta: { request_id: 'req_commitments' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })
  })

  it('renders current-state summaries, calendar events, and open commitments', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText('What matters right now')).toBeInTheDocument()
    })

    expect(screen.getAllByText('focus').length).toBeGreaterThan(0)
    expect(screen.getAllByText('engaged').length).toBeGreaterThan(0)
    expect(screen.getByText('pending')).toBeInTheDocument()
    expect(screen.getByText(/medium · 72%/i)).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Room 4B')).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    expect(screen.getByText('Write weekly review')).toBeInTheDocument()
    expect(screen.getByText('prep window active')).toBeInTheDocument()
    expect(screen.getByText('recent git activity indicates active work')).toBeInTheDocument()
  })
})
