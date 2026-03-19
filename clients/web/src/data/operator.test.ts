import { describe, expect, it } from 'vitest'

import { buildBackupTrustProjection, buildOperatorReviewStatus } from './operator'

describe('buildOperatorReviewStatus', () => {
  it('derives writeback, handoff, conflict, and people review state from now plus settings', () => {
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
      [
        {
          id: 'handoff_1',
          project_id: 'proj_exec',
          origin_kind: 'human_to_agent',
          review_state: 'pending_review',
          handoff: {
            handoff: {
              task_id: 'task_1',
              trace_id: 'trace_1',
              from_agent: 'operator',
              to_agent: 'codex-local',
              objective: 'Implement the next safe slice',
              inputs: {},
              constraints: [],
              read_scopes: ['/tmp/vel'],
              write_scopes: ['/tmp/vel'],
              project_id: 'proj_exec',
              task_kind: 'implementation',
              agent_profile: 'quality',
              token_budget: 'large',
              review_gate: 'operator_approval',
              repo_root: {
                path: '/tmp/vel',
                label: 'vel',
                branch: null,
                head_rev: null,
              },
              allowed_tools: ['rg'],
              capability_scope: {},
              deadline: null,
              expected_output_schema: { artifacts: ['patch'] },
            },
            project_id: 'proj_exec',
            task_kind: 'implementation',
            agent_profile: 'quality',
            token_budget: 'large',
            review_gate: 'operator_approval',
            repo: {
              path: '/tmp/vel',
              label: 'vel',
              branch: null,
              head_rev: null,
            },
            notes_root: {
              path: '/tmp/vel/notes',
              label: 'vel-notes',
              kind: 'notes_root',
            },
            manifest_id: null,
          },
          routing: {
            task_kind: 'implementation',
            agent_profile: 'quality',
            token_budget: 'large',
            review_gate: 'operator_approval',
            read_scopes: ['/tmp/vel'],
            write_scopes: ['/tmp/vel'],
            allowed_tools: ['rg'],
            reasons: [
              {
                code: 'write_scope_requires_approval',
                message: 'write scopes require explicit operator approval before launch',
              },
            ],
          },
          manifest_id: null,
          requested_by: 'operator_shell',
          reviewed_by: null,
          decision_reason: null,
          reviewed_at: null,
          launched_at: null,
          created_at: '2026-03-18T18:00:00Z',
          updated_at: '2026-03-18T18:00:00Z',
        },
      ],
    )

    expect(status.writeback_enabled).toBe(false)
    expect(status.pending_writebacks).toHaveLength(1)
    expect(status.open_conflicts).toHaveLength(1)
    expect(status.people_needing_review.map((person) => person.display_name)).toEqual(['Annie Case'])
    expect(status.pending_execution_handoffs).toHaveLength(1)
  })
})

describe('buildBackupTrustProjection', () => {
  it('summarizes backend-owned backup trust for settings surfaces', () => {
    const projection = buildBackupTrustProjection({
      default_output_root: 'var/backups',
      trust: {
        level: 'warn',
        status: {
          state: 'stale',
          last_backup_id: 'bkp_123',
          last_backup_at: '2026-03-18T18:20:00Z',
          output_root: '/tmp/backups/bkp_123',
          artifact_coverage: {
            included: ['artifacts/captures', 'artifacts/exports'],
            omitted: ['artifacts/cache'],
            notes: [],
          },
          config_coverage: {
            included: ['config/public-settings.json', 'config/runtime-config.json'],
            omitted: ['integration_google_calendar_secrets'],
            notes: [],
          },
          verification_summary: {
            verified: true,
            checksum_algorithm: 'sha256',
            checksum: 'abc123',
            checked_paths: [],
            notes: [],
          },
          warnings: ['last successful backup is stale'],
        },
        freshness: {
          state: 'stale',
          age_seconds: 60 * 60 * 50,
          stale_after_seconds: 60 * 60 * 48,
        },
        guidance: ['Create or verify a fresh backup before risky maintenance.'],
      },
    })

    expect(projection).toEqual({
      level: 'warn',
      statusLabel: 'Backup trust needs attention',
      freshnessLabel: 'stale (50h old)',
      outputRoot: '/tmp/backups/bkp_123',
      lastBackupAt: '2026-03-18T18:20:00Z',
      artifactSummary: 'Artifacts: 2 included, 1 omitted',
      configSummary: 'Config: 2 included, 1 omitted',
      warnings: ['last successful backup is stale'],
      guidance: ['Create or verify a fresh backup before risky maintenance.'],
      commandHints: [
        'vel backup create',
        'vel backup inspect <backup_root>',
        'vel backup verify <backup_root>',
        'vel backup restore-check <backup_root>',
      ],
    })
  })
})
