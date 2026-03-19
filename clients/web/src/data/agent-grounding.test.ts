import { describe, expect, it, vi, beforeEach } from 'vitest'

import { loadAgentInspect } from './agent-grounding'
import * as client from '../api/client'

vi.mock('../api/client', () => ({
  apiGet: vi.fn(),
}))

describe('agent grounding loader', () => {
  beforeEach(() => {
    vi.mocked(client.apiGet).mockReset()
  })

  it('loads and decodes the shared agent inspect contract', async () => {
    vi.mocked(client.apiGet).mockImplementation(async () => ({
      ok: true,
      data: {
        grounding: {
          generated_at: 1710000000,
          now: {
            computed_at: 1710000000,
            timezone: 'America/Denver',
            summary: {
              mode: { key: 'focus', label: 'Focus' },
              phase: { key: 'engaged', label: 'Engaged' },
              meds: { key: 'ok', label: 'OK' },
              risk: { level: 'low', score: 0.2, label: 'low' },
            },
            schedule: { empty_message: null, next_event: null, upcoming_events: [] },
            tasks: { todoist: [], other_open: [], next_commitment: null },
            attention: {
              state: { key: 'on_task', label: 'On task' },
              drift: { key: 'none', label: 'None' },
              severity: { key: 'none', label: 'None' },
              confidence: null,
              reasons: [],
            },
            sources: {
              git_activity: null,
              health: null,
              mood: null,
              pain: null,
              note_document: null,
              assistant_message: null,
            },
            freshness: { overall_status: 'fresh', sources: [] },
            action_items: [],
            review_snapshot: { open_action_count: 1, triage_count: 0, projects_needing_review: 0 },
            pending_writebacks: [],
            conflicts: [],
            people: [],
            reasons: [],
            debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
          },
          current_context: null,
          projects: [],
          people: [],
          commitments: [],
          review: {
            review_snapshot: { open_action_count: 1, triage_count: 0, projects_needing_review: 0 },
            pending_writebacks: [],
            conflicts: [],
            pending_execution_handoffs: [],
          },
        },
        capabilities: {
          groups: [
            {
              kind: 'read_context',
              label: 'Read current Vel state',
              entries: [],
            },
          ],
        },
        blockers: [],
        explainability: {
          persisted_record_kinds: ['now'],
          supporting_paths: ['/v1/agent/inspect'],
          raw_context_json_supporting_only: true,
        },
      },
      meta: { request_id: 'req_agent_inspect' },
    }) as never)

    const response = await loadAgentInspect()

    expect(client.apiGet).toHaveBeenCalledWith('/v1/agent/inspect', expect.any(Function))
    expect(response.data?.capabilities.groups[0]?.kind).toBe('read_context')
  })
})
