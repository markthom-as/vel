import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api/client'
import { clearQueryCache } from '../data/query'
import { SuggestionsView } from './SuggestionsView'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
}))

describe('SuggestionsView', () => {
  beforeEach(() => {
    cleanup()
    clearQueryCache()

    let listCalls = 0
    vi.mocked(api.apiGet).mockReset()
    vi.mocked(api.apiPatch).mockReset()
    vi.mocked(api.apiGet).mockImplementation(async (path: string) => {
      if (path === '/v1/suggestions?state=pending&limit=50') {
        listCalls += 1
        return {
          ok: true,
          data: listCalls === 1
            ? [
                {
                  id: 'sug_1',
                  suggestion_type: 'increase_commute_buffer',
                  state: 'pending',
                  title: 'Increase commute buffer',
                  summary: 'Leave a bit earlier for recurring Dimitri meetings.',
                  priority: 55,
                  confidence: 'medium',
                  evidence_count: 2,
                  decision_context_summary: 'Repeated commute danger nudges.',
                  decision_context: null,
                  evidence: null,
                  adaptive_policy: {
                    policy_key: 'commute_buffer',
                    suggested_minutes: 30,
                    current_minutes: 20,
                    is_active_source: false,
                    active_override: {
                      policy_key: 'commute_buffer',
                      value_minutes: 25,
                      source_suggestion_id: 'sug_old',
                      source_title: 'Existing commute override',
                      source_accepted_at: 1709999900,
                    },
                  },
                  payload: {
                    type: 'increase_commute_buffer',
                    current_minutes: 20,
                    suggested_minutes: 30,
                  },
                  created_at: 1710000000,
                  resolved_at: null,
                },
              ]
            : [],
          meta: { request_id: `req_list_${listCalls}` },
        } as never
      }
      if (path === '/v1/suggestions/sug_1') {
        return {
          ok: true,
          data: {
            id: 'sug_1',
            suggestion_type: 'increase_commute_buffer',
            state: 'pending',
            title: 'Increase commute buffer',
            summary: 'Leave a bit earlier for recurring Dimitri meetings.',
            priority: 55,
            confidence: 'medium',
            evidence_count: 2,
            decision_context_summary: 'Repeated commute danger nudges.',
            decision_context: {
              trigger: 'resolved_commute_danger',
              count: 2,
            },
            evidence: [
              {
                id: 'sugev_1',
                evidence_type: 'nudge',
                ref_id: 'nud_1',
                evidence: { level: 'danger' },
                weight: 1,
                created_at: 1710000000,
              },
            ],
            adaptive_policy: {
              policy_key: 'commute_buffer',
              suggested_minutes: 30,
              current_minutes: 20,
              is_active_source: true,
              active_override: {
                policy_key: 'commute_buffer',
                value_minutes: 30,
                source_suggestion_id: 'sug_1',
                source_title: 'Increase commute buffer',
                source_accepted_at: 1710000300,
              },
            },
            payload: {
              type: 'increase_commute_buffer',
              current_minutes: 20,
              suggested_minutes: 30,
            },
            created_at: 1710000000,
            resolved_at: null,
          },
          meta: { request_id: 'req_detail' },
        } as never
      }
      throw new Error(`unexpected apiGet path: ${path}`)
    })

    vi.mocked(api.apiPatch).mockImplementation(async () => ({
      ok: true,
      data: {
        id: 'sug_1',
        suggestion_type: 'increase_commute_buffer',
        state: 'accepted',
        title: 'Increase commute buffer',
        summary: 'Leave a bit earlier for recurring Dimitri meetings.',
        priority: 55,
        confidence: 'medium',
        evidence_count: 2,
        decision_context_summary: 'Repeated commute danger nudges.',
        decision_context: null,
        evidence: null,
        adaptive_policy: {
          policy_key: 'commute_buffer',
          suggested_minutes: 30,
          current_minutes: 20,
          is_active_source: true,
          active_override: {
            policy_key: 'commute_buffer',
            value_minutes: 30,
            source_suggestion_id: 'sug_1',
            source_title: 'Increase commute buffer',
            source_accepted_at: 1710000300,
          },
        },
        payload: {
          type: 'increase_commute_buffer',
          current_minutes: 20,
          suggested_minutes: 30,
        },
        created_at: 1710000000,
        resolved_at: 1710000300,
      },
      meta: { request_id: 'req_patch' },
    }) as never)
  })

  it('renders pending suggestions, loads detail, and accepts a suggestion', async () => {
    render(<SuggestionsView />)

    await waitFor(() => {
      expect(screen.getAllByText('Increase commute buffer').length).toBeGreaterThan(0)
    })

    expect(screen.getByText(/Repeated commute danger nudges\./i)).toBeInTheDocument()
    expect(screen.getByText(/nudge · nud_1/i)).toBeInTheDocument()
    expect(screen.getByText('Adaptive policy provenance')).toBeInTheDocument()
    expect(screen.getByText('Policy: commute_buffer')).toBeInTheDocument()
    expect(screen.getByText('This suggestion is the active policy source.')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: 'Accept' }))

    await waitFor(() => {
      expect(vi.mocked(api.apiPatch)).toHaveBeenCalledWith(
        '/v1/suggestions/sug_1',
        { state: 'accepted' },
        expect.any(Function),
      )
    })

    await waitFor(() => {
      expect(screen.getByText('No pending suggestions right now.')).toBeInTheDocument()
    })
  })
})
