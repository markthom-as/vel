import { render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../../api/client'
import { clearQueryCache } from '../../data/query'
import { StatsView } from './StatsView'

vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
}))

describe('StatsView', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/api/integrations') {
        return {
          ok: true,
          data: {
            google_calendar: {
              configured: true,
              connected: true,
              has_client_id: true,
              has_client_secret: true,
              calendars: [],
              all_calendars_selected: true,
              last_sync_at: 1710000000,
              last_sync_status: 'success',
              last_error: null,
              last_item_count: 4,
              guidance: null,
            },
            todoist: {
              configured: true,
              connected: false,
              has_api_token: true,
              last_sync_at: 1710000200,
              last_sync_status: 'error',
              last_error: 'token expired',
              last_item_count: 2,
              guidance: null,
            },
            activity: localIntegration(),
            health: localIntegration(),
            git: localIntegration('success'),
            messaging: localIntegration(),
            notes: localIntegration(),
            transcripts: localIntegration(),
          },
          meta: { request_id: 'req_integrations' },
        } as never
      }
      if (path === '/v1/runs?limit=20') {
        return {
          ok: true,
          data: [
            {
              id: 'run_1',
              kind: 'evaluate',
              status: 'succeeded',
              automatic_retry_supported: true,
              automatic_retry_reason: null,
              unsupported_retry_override: false,
              unsupported_retry_override_reason: null,
              created_at: '2026-03-17T10:00:00Z',
              started_at: '2026-03-17T10:00:01Z',
              finished_at: '2026-03-17T10:00:04Z',
              duration_ms: 3000,
              retry_scheduled_at: null,
              retry_reason: null,
              blocked_reason: null,
            },
          ],
          meta: { request_id: 'req_runs' },
        } as never
      }
      if (path === '/v1/loops') {
        return {
          ok: true,
          data: [
            {
              kind: 'evaluate_current_state',
              enabled: true,
              interval_seconds: 300,
              last_started_at: 1710000000,
              last_finished_at: 1710000010,
              last_status: 'success',
              last_error: null,
              next_due_at: 1710000300,
            },
          ],
          meta: { request_id: 'req_loops' },
        } as never
      }
      if (path === '/api/components') {
        return {
          ok: true,
          data: [
            {
              id: 'component_1',
              name: 'evaluate',
              description: 'Current-state evaluation worker',
              status: 'healthy',
              last_restarted_at: 1710000100,
              last_error: null,
              restart_count: 1,
            },
          ],
          meta: { request_id: 'req_components' },
        } as never
      }
      if (path === '/v1/context/current') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            mode: 'focus',
            morning_state: 'engaged',
            context: { global_risk_level: 'medium' },
            source_summaries: {},
            adaptive_policy_overrides: [],
            reasons: [],
          },
          meta: { request_id: 'req_current_context' },
        } as never
      }
      if (path === '/v1/explain/context') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            mode: 'focus',
            morning_state: 'engaged',
            context: { global_risk_level: 'medium' },
            source_summaries: {},
            adaptive_policy_overrides: [],
            signals_used: ['sig_1'],
            signal_summaries: [],
            commitments_used: ['commit_1'],
            risk_used: ['risk_1'],
            reasons: ['mode: focus'],
          },
          meta: { request_id: 'req_context_explain' },
        } as never
      }
      if (path === '/v1/explain/drift') {
        return {
          ok: true,
          data: {
            attention_state: 'on_task',
            drift_type: 'none',
            drift_severity: 'low',
            confidence: 0.91,
            reasons: ['recent activity'],
            signals_used: ['sig_1'],
            signal_summaries: [],
            commitments_used: ['commit_1'],
          },
          meta: { request_id: 'req_drift' },
        } as never
      }

      throw new Error(`unexpected apiGet path: ${path}`)
    })
  })

  it('renders aggregated runtime observability from existing endpoint contracts', async () => {
    render(<StatsView />)

    await waitFor(() => {
      expect(screen.getByText('System observability')).toBeInTheDocument()
    })

    expect(screen.getByText('Google Calendar')).toBeInTheDocument()
    expect(screen.getByText(/token expired/i)).toBeInTheDocument()
    expect(screen.getByText('Runtime loops')).toBeInTheDocument()
    expect(screen.getByText('evaluate_current_state')).toBeInTheDocument()
    expect(screen.getByText('Components')).toBeInTheDocument()
    expect(screen.getByText('Current-state evaluation worker')).toBeInTheDocument()
    expect(screen.getByText('Recent runs')).toBeInTheDocument()
    expect(screen.getAllByText('evaluate').length).toBeGreaterThan(0)
    expect(screen.getByText('91%')).toBeInTheDocument()
  })
})

function localIntegration(status: string | null = null) {
  return {
    configured: true,
    source_path: '/tmp/source.json',
    last_sync_at: 1710000000,
    last_sync_status: status,
    last_error: null,
    last_item_count: 1,
    guidance: null,
  }
}
