import { render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { ContextPanel } from './ContextPanel'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
}))

describe('ContextPanel', () => {
  beforeEach(() => {
    clearQueryCache()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/explain/context') {
        return {
          ok: true,
          data: {
            computed_at: 1710000000,
            mode: 'focus',
            morning_state: 'engaged',
            context: {
              next_commitment_id: 'commit_1',
              meds_status: 'pending',
              prep_window_active: true,
              global_risk_level: 'medium',
            },
            source_summaries: {
              git_activity: {
                timestamp: 1710000000,
                summary: {
                  repo: 'vel',
                  branch: 'main',
                  operation: 'commit',
                },
              },
              note_document: {
                timestamp: 1710000030,
                summary: {
                  title: 'Today',
                  path: 'daily/today.md',
                },
              },
              assistant_message: {
                timestamp: 1710000060,
                summary: {
                  conversation_id: 'conv_context',
                  role: 'assistant',
                  source: 'chatgpt',
                },
              },
            },
            signals_used: ['sig_1'],
            signal_summaries: [
              {
                signal_id: 'sig_1',
                signal_type: 'git_activity',
                source: 'git',
                timestamp: 1710000000,
                summary: {
                  repo: 'vel',
                  branch: 'main',
                  operation: 'commit',
                },
              },
            ],
            commitments_used: ['commit_1'],
            risk_used: ['risk_1'],
            reasons: ['mode: focus', 'prep window active'],
          },
          meta: { request_id: 'req_ctx' },
        } as never
      }
      if (path === '/v1/explain/drift') {
        return {
          ok: true,
          data: {
            attention_state: 'on_task',
            drift_type: null,
            drift_severity: null,
            confidence: 0.82,
            reasons: ['Recent git activity indicates active work'],
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

  it('renders explain-backed context and drift details', async () => {
    render(<ContextPanel />)

    await waitFor(() => {
      expect(screen.getByText('Why this context')).toBeInTheDocument()
    })

    expect(screen.getByText('focus')).toBeInTheDocument()
    expect(screen.getByText('engaged')).toBeInTheDocument()
    expect(screen.getByText('on_task')).toBeInTheDocument()
    expect(screen.getByText('mode: focus')).toBeInTheDocument()
    expect(screen.getByText('Recent git activity indicates active work')).toBeInTheDocument()
    expect(screen.getByText('git_activity')).toBeInTheDocument()
    expect(screen.getAllByText(/repo: vel/i).length).toBeGreaterThan(0)
    expect(screen.getByText('Source summaries')).toBeInTheDocument()
    expect(screen.getByText('Recent note')).toBeInTheDocument()
    expect(screen.getByText(/path: daily\/today.md/i)).toBeInTheDocument()
    expect(screen.getByText('Recent transcript')).toBeInTheDocument()
    expect(screen.getByText(/conversation id: conv_context/i)).toBeInTheDocument()
    expect(screen.getByText('commit_1')).toBeInTheDocument()
    expect(screen.getByText('pending')).toBeInTheDocument()
  })
})
