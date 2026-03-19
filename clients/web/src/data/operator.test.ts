import { describe, expect, it } from 'vitest'

import type { IntegrationsData } from '../types'
import { buildBackupTrustProjection, buildOperatorReviewStatus, buildSettingsOnboardingGuide } from './operator'

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

describe('buildSettingsOnboardingGuide', () => {
  it('surfaces the next linking and source-path action from existing operator payloads', () => {
    const guide = buildSettingsOnboardingGuide({
      clusterBootstrap: {
        node_id: 'vel-desktop',
        node_display_name: 'Vel Desktop',
        active_authority_node_id: 'vel-desktop',
        active_authority_epoch: 1,
        sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        sync_transport: 'tailscale',
        tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        lan_base_url: 'http://192.168.1.50:4130',
        localhost_base_url: 'http://127.0.0.1:4130',
        capabilities: ['read_context'],
        linked_nodes: [],
        projects: [],
        action_items: [],
      },
      clusterWorkers: {
        active_authority_node_id: 'vel-desktop',
        active_authority_epoch: 1,
        generated_at: 1710000000,
        workers: [
          {
            worker_id: 'vel-desktop',
            node_id: 'vel-desktop',
            node_display_name: 'Vel Desktop',
            client_kind: 'veld',
            client_version: '0.1.0',
            protocol_version: '1',
            build_id: 'build_local',
            worker_classes: ['sync'],
            capabilities: ['sync_bootstrap'],
            status: 'ok',
            queue_depth: 0,
            reachability: 'reachable',
            latency_class: 'low',
            compute_class: 'standard',
            power_class: 'ac_or_unknown',
            recent_failure_rate: 0,
            tailscale_preferred: true,
            last_heartbeat_at: 1710000000,
            started_at: 1709999900,
            sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            sync_transport: 'tailscale',
            tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            preferred_tailnet_endpoint: null,
            tailscale_reachable: true,
            lan_base_url: 'http://192.168.1.50:4130',
            localhost_base_url: 'http://127.0.0.1:4130',
            ping_ms: 5,
            sync_status: 'ready',
            last_upstream_sync_at: null,
            last_downstream_sync_at: null,
            last_sync_error: null,
            incoming_linking_prompt: null,
            capacity: {
              max_concurrency: 2,
              current_load: 0,
              available_concurrency: 2,
            },
          },
          {
            worker_id: 'worker_remote',
            node_id: 'node_remote',
            node_display_name: 'Remote Mac',
            client_kind: 'vel_macos',
            client_version: '0.1.0',
            protocol_version: '1',
            build_id: 'build_remote',
            worker_classes: ['sync'],
            capabilities: ['sync_bootstrap'],
            status: 'ok',
            queue_depth: 0,
            reachability: 'reachable',
            latency_class: 'low',
            compute_class: 'standard',
            power_class: 'ac_or_unknown',
            recent_failure_rate: 0,
            tailscale_preferred: true,
            last_heartbeat_at: 1710000000,
            started_at: 1709999900,
            sync_base_url: 'http://remote.tailnet.ts.net:4130',
            sync_transport: 'tailscale',
            tailscale_base_url: 'http://remote.tailnet.ts.net:4130',
            preferred_tailnet_endpoint: null,
            tailscale_reachable: true,
            lan_base_url: null,
            localhost_base_url: null,
            ping_ms: 12,
            sync_status: 'ready',
            last_upstream_sync_at: null,
            last_downstream_sync_at: null,
            last_sync_error: null,
            incoming_linking_prompt: null,
            capacity: {
              max_concurrency: 2,
              current_load: 0,
              available_concurrency: 2,
            },
          },
        ],
      },
      linkedNodes: [],
      integrations: buildIntegrations({
        notes: {
          configured: false,
          source_path: null,
          selected_paths: [],
          available_paths: ['/Users/test/Vault'],
          internal_paths: ['/Users/test/Library/Application Support/Vel/notes'],
          suggested_paths: ['/Users/test/Vault'],
          source_kind: 'directory',
          last_sync_at: null,
          last_sync_status: null,
          last_error: null,
          last_item_count: null,
          guidance: null,
        },
      }),
    })

    expect(guide.headline).toMatch(/next unfinished step/i)
    expect(guide.nextAction).toMatch(/Link a companion device/i)
    expect(guide.steps.map((step) => step.title)).toEqual([
      'Reach the daemon',
      'Link a companion device',
      'Confirm local source paths',
      'Validate Apple and macOS export paths',
    ])
    expect(guide.steps.find((step) => step.id === 'linking')?.status).toBe('ready')
    expect(guide.steps.find((step) => step.id === 'local-sources')?.supportPath).toBe('docs/user/integrations/local-sources.md')
  })
})

function buildIntegrations(overrides: Partial<IntegrationsData> = {}): IntegrationsData {
  const baseLocal = {
    configured: false,
    source_path: null,
    selected_paths: [],
    available_paths: [],
    internal_paths: [],
    suggested_paths: [],
    source_kind: 'file',
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  }

  return {
    google_calendar: {
      configured: false,
      connected: false,
      has_client_id: false,
      has_client_secret: false,
      calendars: [],
      all_calendars_selected: true,
      last_sync_at: null,
      last_sync_status: null,
      last_error: null,
      last_item_count: null,
      guidance: null,
    },
    todoist: {
      configured: false,
      connected: false,
      has_api_token: false,
      last_sync_at: null,
      last_sync_status: null,
      last_error: null,
      last_item_count: null,
      guidance: null,
    },
    activity: { ...baseLocal },
    health: { ...baseLocal },
    git: { ...baseLocal },
    messaging: { ...baseLocal },
    reminders: { ...baseLocal },
    notes: { ...baseLocal, source_kind: 'directory' },
    transcripts: { ...baseLocal },
    ...overrides,
  }
}
