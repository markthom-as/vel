import { describe, expect, it } from 'vitest'

import { buildOperatorReviewStatus } from './operator'

describe('buildOperatorReviewStatus', () => {
  it('derives pending writebacks, conflicts, and people needing review from now plus settings', () => {
    const status = buildOperatorReviewStatus(
      {
        computed_at: 1710000000,
        timezone: 'America/Denver',
        summary: {
          mode: { key: 'focus', label: 'Focus' },
          phase: { key: 'engaged', label: 'Engaged' },
          meds: { key: 'ok', label: 'OK' },
          risk: { level: 'low', score: 0.2, label: 'low' },
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
        freshness: {
          overall_status: 'fresh',
          sources: [],
        },
        action_items: [
          {
            id: 'act_person',
            surface: 'now',
            kind: 'next_step',
            title: 'Reply to Annie',
            summary: 'Draft review pending',
            project_id: null,
            state: 'active',
            rank: 72,
            surfaced_at: '2026-03-18T18:00:00Z',
            snoozed_until: null,
            evidence: [
              {
                source_kind: 'person',
                source_id: 'per_annie',
                label: 'Annie Case',
                detail: null,
              },
            ],
          },
        ],
        review_snapshot: {
          open_action_count: 1,
          triage_count: 0,
          projects_needing_review: 0,
        },
        pending_writebacks: [
          {
            id: 'wb_1',
            kind: 'email_create_draft_reply',
            risk: 'safe',
            status: 'queued',
            target: {
              family: 'messaging',
              provider_key: 'email',
              project_id: null,
              connection_id: 'icn_email',
              external_id: 'thread_1',
            },
            requested_payload: {},
            result_payload: null,
            provenance: [],
            conflict_case_id: null,
            requested_by_node_id: 'vel-local',
            requested_at: '2026-03-18T18:00:00Z',
            applied_at: null,
            updated_at: '2026-03-18T18:00:00Z',
          },
        ],
        conflicts: [
          {
            id: 'conf_1',
            kind: 'upstream_vs_local',
            status: 'open',
            target: {
              family: 'messaging',
              provider_key: 'email',
              project_id: null,
              connection_id: 'icn_email',
              external_id: 'thread_1',
            },
            summary: 'Conflict',
            local_payload: {},
            upstream_payload: null,
            resolution_payload: null,
            opened_at: '2026-03-18T18:00:00Z',
            resolved_at: null,
            updated_at: '2026-03-18T18:00:00Z',
          },
        ],
        people: [
          {
            id: 'per_annie',
            display_name: 'Annie Case',
            given_name: 'Annie',
            family_name: 'Case',
            relationship_context: null,
            birthday: null,
            last_contacted_at: null,
            aliases: [],
            links: [],
          },
        ],
        reasons: [],
        debug: {
          raw_context: {},
          signals_used: [],
          commitments_used: [],
          risk_used: [],
        },
      },
      { writeback_enabled: false },
    )

    expect(status.writeback_enabled).toBe(false)
    expect(status.pending_writebacks).toHaveLength(1)
    expect(status.open_conflicts).toHaveLength(1)
    expect(status.people_needing_review.map((person) => person.display_name)).toEqual(['Annie Case'])
  })
})
