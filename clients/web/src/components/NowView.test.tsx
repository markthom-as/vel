import { cleanup, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { NowView } from './NowView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
}))

describe('NowView', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/now') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            summary: {
              mode: { key: 'day_mode', label: 'Day' },
              phase: { key: 'engaged', label: 'Engaged' },
              meds: { key: 'pending', label: 'Pending' },
              risk: { level: 'medium', score: 0.72, label: 'medium · 72%' },
            },
            schedule: {
              next_event: {
                title: 'Design review',
                start_ts: 1710003600,
                end_ts: 1710007200,
                location: 'Room 4B',
                prep_minutes: 15,
                travel_minutes: 0,
                leave_by_ts: 1710003600,
              },
              upcoming_events: [
                {
                  title: 'Design review',
                  start_ts: 1710003600,
                  end_ts: 1710007200,
                  location: 'Room 4B',
                  prep_minutes: 15,
                  travel_minutes: 0,
                  leave_by_ts: 1710003600,
                },
              ],
            },
            tasks: {
              todoist: [
                {
                  id: 'commit_todoist_1',
                  text: 'Reply to Dimitri',
                  source_type: 'todoist',
                  due_at: '2026-03-16T19:00:00Z',
                  project: 'Ops',
                  commitment_kind: 'todo',
                },
              ],
              other_open: [
                {
                  id: 'commit_local_1',
                  text: 'Write weekly review',
                  source_type: 'capture',
                  due_at: null,
                  project: null,
                  commitment_kind: 'writing',
                },
              ],
              next_commitment: {
                id: 'commit_local_1',
                text: 'Write weekly review',
                source_type: 'capture',
                due_at: null,
                project: null,
                commitment_kind: 'writing',
              },
            },
            attention: {
              state: { key: 'on_task', label: 'On task' },
              drift: { key: 'none', label: 'None' },
              severity: { key: 'none', label: 'None' },
              confidence: 0.8,
              reasons: ['recent git activity indicates active work'],
            },
            freshness: {
              overall_status: 'fresh',
              sources: [
                {
                  key: 'context',
                  label: 'Context',
                  status: 'fresh',
                  last_sync_at: 1710000000,
                  age_seconds: 10,
                },
              ],
            },
            reasons: ['Prep window active', 'recent git activity indicates active work'],
            debug: {
              raw_context: {},
              signals_used: ['sig_cal_1'],
              commitments_used: ['commit_1'],
              risk_used: ['risk_1'],
            },
          },
          meta: { request_id: 'req_now' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })
  })

  it('renders the consolidated now snapshot', async () => {
    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText('What matters right now')).toBeInTheDocument()
    })

    expect(screen.getAllByText('Day').length).toBeGreaterThan(0)
    expect(screen.getByText('Engaged')).toBeInTheDocument()
    expect(screen.getByText('Pending')).toBeInTheDocument()
    expect(screen.getByText(/medium · 72%/i)).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Room 4B')).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getByText('Prep window active')).toBeInTheDocument()
    expect(screen.getByText('recent git activity indicates active work')).toBeInTheDocument()
    expect(screen.getAllByText('Fresh').length).toBeGreaterThan(0)
  })

  it('surfaces degraded freshness warnings while keeping stale data visible', async () => {
    vi.mocked(api.apiGet).mockImplementationOnce(async () => ({
      ok: true,
      data: {
        computed_at: 1710000000,
        timezone: 'America/Denver',
        summary: {
          mode: { key: 'day_mode', label: 'Day' },
          phase: { key: 'engaged', label: 'Engaged' },
          meds: { key: 'pending', label: 'Pending' },
          risk: { level: 'medium', score: 0.72, label: 'medium · 72%' },
        },
        schedule: {
          next_event: {
            title: 'Design review',
            start_ts: 1710003600,
            end_ts: 1710007200,
            location: 'Room 4B',
            prep_minutes: 15,
            travel_minutes: 0,
            leave_by_ts: 1710003600,
          },
          upcoming_events: [
            {
              title: 'Design review',
              start_ts: 1710003600,
              end_ts: 1710007200,
              location: 'Room 4B',
              prep_minutes: 15,
              travel_minutes: 0,
              leave_by_ts: 1710003600,
            },
          ],
        },
        tasks: {
          todoist: [
            {
              id: 'commit_todoist_1',
              text: 'Reply to Dimitri',
              source_type: 'todoist',
              due_at: '2026-03-16T19:00:00Z',
              project: 'Ops',
              commitment_kind: 'todo',
            },
          ],
          other_open: [],
          next_commitment: null,
        },
        attention: {
          state: { key: 'on_task', label: 'On task' },
          drift: { key: 'none', label: 'None' },
          severity: { key: 'none', label: 'None' },
          confidence: 0.8,
          reasons: [],
        },
        freshness: {
          overall_status: 'stale',
          sources: [
            {
              key: 'context',
              label: 'Context',
              status: 'aging',
              last_sync_at: 1709999400,
              age_seconds: 600,
            },
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'stale',
              last_sync_at: 1709990000,
              age_seconds: 10000,
            },
            {
              key: 'todoist',
              label: 'Todoist',
              status: 'error',
              last_sync_at: 1709995000,
              age_seconds: 5000,
            },
          ],
        },
        reasons: ['Prep window active'],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
      meta: { request_id: 'req_now_degraded' },
    }) as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText(/Some inputs are degraded/i)).toBeInTheDocument()
    })

    expect(screen.getByText(/Calendar: Stale/i)).toBeInTheDocument()
    expect(screen.getByText(/Todoist: Error/i)).toBeInTheDocument()
    expect(
      screen.getByText('Calendar is stale. Upcoming events may be out of date.'),
    ).toBeInTheDocument()
    expect(
      screen.getByText('Todoist sync last failed. Backlog state may be incomplete.'),
    ).toBeInTheDocument()
    expect(
      screen.getByText('Current context is aging. Evaluate soon if you need fresher state.'),
    ).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
  })
})
