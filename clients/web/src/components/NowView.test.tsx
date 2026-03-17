import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { NowView } from './NowView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPost: vi.fn(),
}))

describe('NowView', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()
    vi.useRealTimers()
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPost).mockReset()
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
              empty_message: null,
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
            sources: {
              git_activity: {
                label: 'Git activity',
                timestamp: 1710000000,
                summary: {
                  repo: 'vel',
                  branch: 'main',
                  operation: 'commit',
                },
              },
              health: {
                label: 'Health',
                timestamp: 1710000030,
                summary: {
                  metric_type: 'resting_heart_rate',
                  value: 58,
                  unit: 'bpm',
                  source_app: 'Apple Health',
                  device: 'Apple Watch',
                },
              },
              note_document: {
                label: 'Recent note',
                timestamp: 1710000060,
                summary: {
                  title: 'Today',
                  path: 'daily/today.md',
                },
              },
              assistant_message: {
                label: 'Recent transcript',
                timestamp: 1710000120,
                summary: {
                  conversation_id: 'conv_external',
                  role: 'assistant',
                  source: 'chatgpt',
                },
              },
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
                  guidance: null,
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
    vi.mocked(api.apiPost).mockImplementation(async (path: string) => {
      if (path === '/v1/evaluate') {
        return {
          ok: true,
          data: {
            inferred_states: 4,
            nudges_created_or_updated: 1,
          },
          meta: { request_id: 'req_eval' },
        } as never
      }
      if (path === '/v1/sync/calendar') {
        return {
          ok: true,
          data: {
            source: 'calendar',
            signals_ingested: 3,
          },
          meta: { request_id: 'req_sync_calendar' },
        } as never
      }
      if (path === '/v1/sync/todoist') {
        return {
          ok: true,
          data: {
            source: 'todoist',
            signals_ingested: 5,
          },
          meta: { request_id: 'req_sync_todoist' },
        } as never
      }
      if (path === '/v1/sync/activity') {
        return {
          ok: true,
          data: {
            source: 'activity',
            signals_ingested: 2,
          },
          meta: { request_id: 'req_sync_activity' },
        } as never
      }
      if (path === '/v1/sync/messaging') {
        return {
          ok: true,
          data: {
            source: 'messaging',
            signals_ingested: 4,
          },
          meta: { request_id: 'req_sync_messaging' },
        } as never
      }
      throw new Error(`unexpected apiPost path: ${path}`)
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
    expect(screen.getByText(/prep 15m/i)).toBeInTheDocument()
    expect(screen.getByText(/travel 0m/i)).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
    expect(screen.getAllByText('Write weekly review').length).toBeGreaterThan(0)
    expect(screen.getByText('Prep window active')).toBeInTheDocument()
    expect(screen.getByText('recent git activity indicates active work')).toBeInTheDocument()
    expect(screen.getByText('Recent source activity')).toBeInTheDocument()
    expect(screen.getByText('repo: vel')).toBeInTheDocument()
    expect(screen.getByText('metric type: resting_heart_rate')).toBeInTheDocument()
    expect(screen.getByText('value: 58')).toBeInTheDocument()
    expect(screen.getByText('source app: Apple Health')).toBeInTheDocument()
    expect(screen.getByText('path: daily/today.md')).toBeInTheDocument()
    expect(screen.getByText('conversation id: conv_external')).toBeInTheDocument()
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
          empty_message: null,
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
        sources: {
          git_activity: null,
          note_document: null,
          assistant_message: null,
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
              guidance: 'Re-run evaluate soon.',
            },
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'stale',
              last_sync_at: 1709990000,
              age_seconds: 10000,
              guidance: 'Calendar sync failed earlier. Inspect history and retry sync.',
            },
            {
              key: 'todoist',
              label: 'Todoist',
              status: 'error',
              last_sync_at: 1709995000,
              age_seconds: 5000,
              guidance: 'Todoist sync failed. Inspect history and retry sync.',
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
    expect(screen.getAllByText(/Calendar sync failed earlier\. Inspect history and retry sync\./i).length).toBeGreaterThan(0)
    expect(screen.getAllByText(/Todoist sync failed\. Inspect history and retry sync\./i).length).toBeGreaterThan(0)
    expect(
      screen.getByText('Current context is aging. Evaluate soon if you need fresher state.'),
    ).toBeInTheDocument()
    expect(screen.getByText('Design review')).toBeInTheDocument()
    expect(screen.getByText('Reply to Dimitri')).toBeInTheDocument()
  })

  it('runs evaluate directly from degraded context warnings', async () => {
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce({
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
            empty_message: null,
            next_event: null,
            upcoming_events: [],
          },
          tasks: {
            todoist: [],
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
          sources: {
            git_activity: null,
            note_document: null,
            assistant_message: null,
          },
          freshness: {
            overall_status: 'aging',
            sources: [
              {
                key: 'context',
                label: 'Context',
                status: 'aging',
                last_sync_at: 1709999400,
                age_seconds: 600,
                guidance: 'Re-run evaluate soon.',
              },
            ],
          },
          reasons: [],
          debug: {
            raw_context: {},
            signals_used: [],
            commitments_used: [],
            risk_used: [],
          },
        },
        meta: { request_id: 'req_now_degraded_context' },
      } as never)
      .mockResolvedValueOnce({
        ok: true,
        data: {
          computed_at: 1710000300,
          timezone: 'America/Denver',
          summary: {
            mode: { key: 'day_mode', label: 'Day' },
            phase: { key: 'engaged', label: 'Engaged' },
            meds: { key: 'pending', label: 'Pending' },
            risk: { level: 'low', score: 0.32, label: 'low · 32%' },
          },
          schedule: {
            empty_message: null,
            next_event: null,
            upcoming_events: [],
          },
          tasks: {
            todoist: [],
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
          sources: {
            git_activity: null,
            note_document: null,
            assistant_message: null,
          },
          freshness: {
            overall_status: 'fresh',
            sources: [
              {
                key: 'context',
                label: 'Context',
                status: 'fresh',
                last_sync_at: 1710000300,
                age_seconds: 0,
                guidance: null,
              },
            ],
          },
          reasons: [],
          debug: {
            raw_context: {},
            signals_used: [],
            commitments_used: [],
            risk_used: [],
          },
        },
        meta: { request_id: 'req_now_refreshed_context' },
      } as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /re-run evaluate/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /re-run evaluate/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith('/v1/evaluate', {}, expect.any(Function))
    })
    await waitFor(() => {
      expect(screen.getByText('Context refreshed.')).toBeInTheDocument()
    })
    expect(screen.getAllByText('Fresh').length).toBeGreaterThan(0)
  })

  it('retries calendar sync directly from degraded freshness warnings', async () => {
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce({
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
            empty_message: null,
            next_event: null,
            upcoming_events: [],
          },
          tasks: {
            todoist: [],
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
          sources: {
            git_activity: null,
            note_document: null,
            assistant_message: null,
          },
          freshness: {
            overall_status: 'stale',
            sources: [
              {
                key: 'calendar',
                label: 'Calendar',
                status: 'stale',
                last_sync_at: 1709990000,
                age_seconds: 10000,
                guidance: 'Calendar sync failed earlier. Inspect history and retry sync.',
              },
            ],
          },
          reasons: [],
          debug: {
            raw_context: {},
            signals_used: [],
            commitments_used: [],
            risk_used: [],
          },
        },
        meta: { request_id: 'req_now_calendar_stale' },
      } as never)
      .mockResolvedValueOnce({
        ok: true,
        data: {
          computed_at: 1710000300,
          timezone: 'America/Denver',
          summary: {
            mode: { key: 'day_mode', label: 'Day' },
            phase: { key: 'engaged', label: 'Engaged' },
            meds: { key: 'pending', label: 'Pending' },
            risk: { level: 'medium', score: 0.72, label: 'medium · 72%' },
          },
          schedule: {
            empty_message: null,
            next_event: null,
            upcoming_events: [],
          },
          tasks: {
            todoist: [],
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
          sources: {
            git_activity: null,
            note_document: null,
            assistant_message: null,
          },
          freshness: {
            overall_status: 'fresh',
            sources: [
              {
                key: 'calendar',
                label: 'Calendar',
                status: 'fresh',
                last_sync_at: 1710000300,
                age_seconds: 0,
                guidance: null,
              },
            ],
          },
          reasons: [],
          debug: {
            raw_context: {},
            signals_used: [],
            commitments_used: [],
            risk_used: [],
          },
        },
        meta: { request_id: 'req_now_calendar_fresh' },
      } as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /sync calendar/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /sync calendar/i })[0] as HTMLElement)

    await waitFor(() => {
      expect(api.apiPost).toHaveBeenCalledWith('/v1/sync/calendar', {}, expect.any(Function))
    })
    await waitFor(() => {
      expect(screen.getByText('Calendar synced (3 signals).')).toBeInTheDocument()
    })
  })

  it('refetches on focus and reveals debug payload on demand', async () => {
    const initial = {
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
          empty_message: null,
          next_event: null,
          upcoming_events: [],
        },
        tasks: {
          todoist: [],
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
        sources: {
          git_activity: null,
          note_document: null,
          assistant_message: null,
        },
        freshness: {
          overall_status: 'fresh',
          sources: [],
        },
        reasons: [],
        debug: {
          raw_context: { mode: 'day_mode' },
          signals_used: ['sig_1'],
          commitments_used: ['commit_1'],
          risk_used: ['risk_1'],
        },
      },
      meta: { request_id: 'req_now_1' },
    }
    const refreshed = {
      ...initial,
      data: {
        ...initial.data,
        computed_at: 1710000300,
        summary: {
          ...initial.data.summary,
          mode: { key: 'meeting_mode', label: 'Meeting prep' },
        },
      },
      meta: { request_id: 'req_now_2' },
    }
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce(initial as never)
      .mockResolvedValueOnce(refreshed as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText('Day')).toBeInTheDocument()
    })

    fireEvent(window, new Event('focus'))

    await waitFor(() => {
      expect(screen.getByText('Meeting prep')).toBeInTheDocument()
    })

    fireEvent.click(screen.getByText(/show raw fields/i))
    expect(screen.getByText(/"signals_used": \[/i)).toBeInTheDocument()
    expect(screen.getByText(/"risk_used": \[/i)).toBeInTheDocument()
  })

  it('registers a background refresh interval', async () => {
    const setIntervalSpy = vi.spyOn(window, 'setInterval')
    const initial = {
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
          empty_message: null,
          next_event: null,
          upcoming_events: [],
        },
        tasks: {
          todoist: [],
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
        sources: {
          git_activity: null,
          note_document: null,
          assistant_message: null,
        },
        freshness: {
          overall_status: 'fresh',
          sources: [],
        },
        reasons: [],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
      meta: { request_id: 'req_now_1' },
    }
    const refreshed = {
      ...initial,
      data: {
        ...initial.data,
        summary: {
          ...initial.data.summary,
          phase: { key: 'underway', label: 'Underway' },
        },
      },
    }
    vi.mocked(api.apiGet)
      .mockResolvedValueOnce(initial as never)
      .mockResolvedValueOnce(refreshed as never)

    render(<NowView />)

    await waitFor(() => {
      expect(screen.getByText('Engaged')).toBeInTheDocument()
    })
    expect(setIntervalSpy).toHaveBeenCalledWith(expect.any(Function), 60_000)
    setIntervalSpy.mockRestore()
  })

  it('opens integration settings for non-retryable degraded sources', async () => {
    vi.mocked(api.apiGet).mockResolvedValueOnce({
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
          empty_message: null,
          next_event: null,
          upcoming_events: [],
        },
        tasks: {
          todoist: [],
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
        sources: {
          git_activity: null,
          note_document: null,
          assistant_message: null,
        },
        freshness: {
          overall_status: 'disconnected',
          sources: [
            {
              key: 'calendar',
              label: 'Calendar',
              status: 'disconnected',
              last_sync_at: null,
              age_seconds: null,
              guidance: 'Connect Google before syncing calendar data.',
            },
            {
              key: 'activity',
              label: 'Computer activity',
              status: 'missing',
              last_sync_at: null,
              age_seconds: null,
              guidance: 'Configure a source path for this local adapter before syncing it.',
            },
          ],
        },
        reasons: [],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
      meta: { request_id: 'req_now_settings' },
    } as never)

    const onOpenSettings = vi.fn()
    render(<NowView onOpenSettings={onOpenSettings} />)

    await waitFor(() => {
      expect(screen.getAllByRole('button', { name: /open google settings/i }).length).toBeGreaterThan(0)
    })

    fireEvent.click(screen.getAllByRole('button', { name: /open google settings/i })[0] as HTMLElement)
    fireEvent.click(screen.getAllByRole('button', { name: /open source settings/i })[0] as HTMLElement)

    expect(onOpenSettings).toHaveBeenNthCalledWith(1, { tab: 'integrations', integrationId: 'google' })
    expect(onOpenSettings).toHaveBeenNthCalledWith(2, { tab: 'integrations', integrationId: 'activity' })
  })
})
