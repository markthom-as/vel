import { describe, expect, it } from 'vitest'
import {
  decodeActionItemData,
  decodeApiResponse,
  decodeCommitmentData,
  decodeCreateMessageResponse,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeClusterWorkersData,
  decodeInboxItemData,
  decodeIntegrationConnectionData,
  decodeIntegrationConnectionEventData,
  decodeIntegrationsData,
  decodeLinkedNodeData,
  decodeLoopData,
  decodeNowData,
  decodeNullable,
  decodeArray,
  decodePairingTokenData,
  decodeProjectRecordData,
  decodeReviewSnapshotData,
  decodeRiskCardContent,
  decodeRunSummaryData,
  decodeSettingsData,
  decodeSuggestionData,
  decodeSyncBootstrapData,
  decodeUncertaintyData,
  decodeWsEvent,
} from './types'

describe('transport decoders', () => {
  it('decodes create-message API responses with optional assistant data', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: {
          user_message: {
            id: 'msg_user',
            conversation_id: 'conv_1',
            role: 'user',
            kind: 'text',
            content: { text: 'hello' },
            status: null,
            importance: null,
            created_at: 1,
            updated_at: null,
          },
          assistant_message: {
            id: 'msg_assistant',
            conversation_id: 'conv_1',
            role: 'assistant',
            kind: 'text',
            content: { text: 'hi' },
            status: null,
            importance: null,
            created_at: 2,
            updated_at: null,
          },
          assistant_error: null,
        },
        meta: { request_id: 'req_1' },
      },
      decodeCreateMessageResponse,
    )

    expect(response.data?.user_message.id).toBe('msg_user')
    expect(response.data?.assistant_message?.id).toBe('msg_assistant')
  })

  it('decodes current-context responses with nullable data', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: {
          computed_at: 123,
          context: {
            inferred_activity: 'coding',
            git_activity_summary: 'commit on main',
          },
        },
        meta: { request_id: 'req_2' },
      },
      (value) => decodeNullable(value, decodeCurrentContextData),
    )

    expect(response.data?.computed_at).toBe(123)
    expect(response.data?.context).toEqual({
      inferred_activity: 'coding',
      git_activity_summary: 'commit on main',
    })
  })

  it('decodes integrations responses with local adapter sections', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: {
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
            guidance: {
              title: 'Calendar credentials missing',
              detail: 'Add credentials first.',
              action: 'Save credentials',
            },
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
          activity: {
            configured: true,
            source_path: '/tmp/activity.json',
            selected_paths: [],
            available_paths: ['/tmp/activity.json', '/home/test/.zsh_history'],
            internal_paths: ['var/integrations/activity/snapshot.json'],
            suggested_paths: ['/tmp/activity.json'],
            source_kind: 'file',
            last_sync_at: 12,
            last_sync_status: 'ok',
            last_error: null,
            last_item_count: 4,
            guidance: null,
          },
          health: {
            configured: true,
            source_path: '/tmp/health.json',
            selected_paths: [],
            available_paths: ['/tmp/health.json'],
            internal_paths: [],
            suggested_paths: ['/tmp/health.json'],
            source_kind: 'file',
            last_sync_at: 14,
            last_sync_status: 'ok',
            last_error: null,
            last_item_count: 2,
            guidance: null,
          },
          git: {
            configured: false,
            source_path: null,
            selected_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
            available_paths: ['/Users/test/code/vel', '/Users/test/code/other'],
            internal_paths: ['var/integrations/git/snapshot.json'],
            suggested_paths: ['/tmp/git.json'],
            source_kind: 'file',
            last_sync_at: null,
            last_sync_status: null,
            last_error: null,
            last_item_count: null,
            guidance: {
              title: 'Local source missing',
              detail: 'Configure a source path.',
              action: 'Set source path',
            },
          },
          messaging: {
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
          },
          reminders: {
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
          },
          notes: {
            configured: false,
            source_path: null,
            selected_paths: [],
            available_paths: ['/Users/test/Vault'],
            internal_paths: ['~/Library/Application Support/Vel/notes'],
            suggested_paths: ['/Users/test/Vault'],
            source_kind: 'directory',
            last_sync_at: null,
            last_sync_status: null,
            last_error: null,
            last_item_count: null,
            guidance: null,
          },
          transcripts: {
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
          },
        },
        meta: { request_id: 'req_integrations' },
      },
      decodeIntegrationsData,
    )

    expect(response.data?.activity.source_path).toBe('/tmp/activity.json')
    expect(response.data?.health.source_path).toBe('/tmp/health.json')
    expect(response.data?.activity.last_item_count).toBe(4)
    expect(response.data?.activity.available_paths).toEqual(['/tmp/activity.json', '/home/test/.zsh_history'])
    expect(response.data?.activity.internal_paths).toEqual(['var/integrations/activity/snapshot.json'])
    expect(response.data?.git.selected_paths).toEqual(['/Users/test/code/vel', '/Users/test/code/other'])
    expect(response.data?.notes.suggested_paths).toEqual(['/Users/test/Vault'])
    expect(response.data?.notes.source_kind).toBe('directory')
    expect(response.data?.google_calendar.guidance?.action).toBe('Save credentials')
  })

  it('decodes canonical integration connection data', () => {
    expect(
      decodeIntegrationConnectionData({
        id: 'icn_1',
        family: 'messaging',
        provider_key: 'signal',
        status: 'connected',
        display_name: 'Signal personal',
        account_ref: '+15555550123',
        metadata: { scope: 'personal' },
        created_at: 10,
        updated_at: 11,
        setting_refs: [
          {
            setting_key: 'messaging_snapshot_path',
            setting_value: '/tmp/signal.json',
            created_at: 12,
          },
        ],
      }),
    ).toEqual({
      id: 'icn_1',
      family: 'messaging',
      provider_key: 'signal',
      status: 'connected',
      display_name: 'Signal personal',
      account_ref: '+15555550123',
      metadata: { scope: 'personal' },
      created_at: 10,
      updated_at: 11,
      setting_refs: [
        {
          setting_key: 'messaging_snapshot_path',
          setting_value: '/tmp/signal.json',
          created_at: 12,
        },
      ],
    })
  })

  it('decodes integration connection events', () => {
    expect(
      decodeIntegrationConnectionEventData({
        id: 'icev_1',
        connection_id: 'icn_1',
        event_type: 'sync_succeeded',
        payload: { items: 42 },
        timestamp: 100,
        created_at: 101,
      }),
    ).toEqual({
      id: 'icev_1',
      connection_id: 'icn_1',
      event_type: 'sync_succeeded',
      payload: { items: 42 },
      timestamp: 100,
      created_at: 101,
    })
  })

  it('decodes context explain source summaries', () => {
    expect(
      decodeContextExplainData({
        computed_at: 1710000000,
        mode: 'focus',
        morning_state: 'engaged',
        context: { meds_status: 'pending' },
        source_summaries: {
          git_activity: {
            timestamp: 1710000000,
            summary: { repo: 'vel', branch: 'main' },
          },
          health: {
            timestamp: 1710000030,
            summary: { metric_type: 'sleep_hours', value: 7.5 },
          },
          note_document: {
            timestamp: 1710000060,
            summary: { path: 'daily/today.md' },
          },
          assistant_message: {
            timestamp: 1710000120,
            summary: { conversation_id: 'conv_context' },
          },
        },
        adaptive_policy_overrides: [],
        signals_used: ['sig_1'],
        signal_summaries: [],
        commitments_used: ['commit_1'],
        risk_used: ['risk_1'],
        reasons: ['mode: focus'],
      }),
    ).toEqual({
      computed_at: 1710000000,
      mode: 'focus',
      morning_state: 'engaged',
      context: { meds_status: 'pending' },
      source_summaries: {
        git_activity: {
          timestamp: 1710000000,
          summary: { repo: 'vel', branch: 'main' },
        },
        health: {
          timestamp: 1710000030,
          summary: { metric_type: 'sleep_hours', value: 7.5 },
        },
        note_document: {
          timestamp: 1710000060,
          summary: { path: 'daily/today.md' },
        },
        assistant_message: {
          timestamp: 1710000120,
          summary: { conversation_id: 'conv_context' },
        },
        },
        adaptive_policy_overrides: [],
        signals_used: ['sig_1'],
        signal_summaries: [],
        commitments_used: ['commit_1'],
        risk_used: ['risk_1'],
        reasons: ['mode: focus'],
    })
  })

  it('decodes settings with adaptive policy overrides', () => {
    expect(
      decodeSettingsData({
        disable_proactive: false,
        toggle_risks: true,
        toggle_reminders: true,
        timezone: 'America/Denver',
        node_display_name: 'Vel Desktop',
        writeback_enabled: false,
        tailscale_preferred: true,
        tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        tailscale_base_url_auto_discovered: true,
        lan_base_url: 'http://192.168.1.50:4130',
        adaptive_policy_overrides: {
          commute_buffer_minutes: 30,
          default_prep_minutes: 45,
          commute_buffer_source_suggestion_id: 'sug_commute',
          commute_buffer_source_title: 'Increase commute buffer',
          commute_buffer_source_accepted_at: 1710000100,
          default_prep_source_suggestion_id: 'sug_prep',
          default_prep_source_title: 'Increase prep window',
          default_prep_source_accepted_at: 1710000200,
        },
      }),
    ).toEqual({
      disable_proactive: false,
      toggle_risks: true,
      toggle_reminders: true,
      timezone: 'America/Denver',
      node_display_name: 'Vel Desktop',
      writeback_enabled: false,
      tailscale_preferred: true,
      tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
      tailscale_base_url_auto_discovered: true,
      lan_base_url: 'http://192.168.1.50:4130',
      adaptive_policy_overrides: {
        commute_buffer_minutes: 30,
        default_prep_minutes: 45,
        commute_buffer_source_suggestion_id: 'sug_commute',
        commute_buffer_source_title: 'Increase commute buffer',
        commute_buffer_source_accepted_at: 1710000100,
        default_prep_source_suggestion_id: 'sug_prep',
        default_prep_source_title: 'Increase prep window',
        default_prep_source_accepted_at: 1710000200,
      },
    })
  })

  it('decodes pairing token suggestions', () => {
    expect(
      decodePairingTokenData({
        token_id: 'token_1',
        token_code: 'VEL-PAIR-123',
        issued_at: '2026-03-16T18:20:00Z',
        expires_at: '2026-03-16T18:35:00Z',
        issued_by_node_id: 'vel-desktop',
        scopes: {
          read_context: true,
          write_safe_actions: false,
          execute_repo_tasks: false,
        },
        suggested_targets: [
          {
            label: 'Tailscale',
            base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            transport_hint: 'tailscale',
            recommended: true,
            redeem_command_hint:
              'vel --base-url http://vel-desktop.tailnet.ts.net:4130 node link redeem VEL-PAIR-123 --node-id <node_id> --node-display-name <name> --transport-hint tailscale',
          },
        ],
      }),
    ).toEqual({
      token_id: 'token_1',
      token_code: 'VEL-PAIR-123',
      issued_at: '2026-03-16T18:20:00Z',
      expires_at: '2026-03-16T18:35:00Z',
      issued_by_node_id: 'vel-desktop',
      scopes: {
        read_context: true,
        write_safe_actions: false,
        execute_repo_tasks: false,
      },
      suggested_targets: [
        {
          label: 'Tailscale',
          base_url: 'http://vel-desktop.tailnet.ts.net:4130',
          transport_hint: 'tailscale',
          recommended: true,
          redeem_command_hint:
            'vel --base-url http://vel-desktop.tailnet.ts.net:4130 node link redeem VEL-PAIR-123 --node-id <node_id> --node-display-name <name> --transport-hint tailscale',
        },
      ],
    })
  })

  it('decodes consolidated now responses', () => {
    expect(
      decodeNowData({
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
          reasons: ['recent git activity'],
        },
        sources: {
          git_activity: {
            label: 'Git activity',
            timestamp: 1710000000,
            summary: { repo: 'vel' },
          },
          health: {
            label: 'Recent health signal',
            timestamp: 1710000030,
            summary: { metric_type: 'sleep_hours', value: 7.5 },
          },
          mood: null,
          pain: null,
          note_document: {
            label: 'Recent note',
            timestamp: 1710000060,
            summary: { path: 'daily/today.md' },
          },
          assistant_message: {
            label: 'Recent transcript',
            timestamp: 1710000120,
            summary: { conversation_id: 'conv_external' },
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
        action_items: [
          {
            id: 'act_1',
            surface: 'inbox',
            kind: 'intervention',
            title: 'Inbox intervention',
            summary: 'Review operator intervention',
            project_id: 'proj_1',
            state: 'active',
            rank: 80,
            surfaced_at: '2026-03-19T02:10:00Z',
            snoozed_until: null,
            evidence: [
              {
                source_kind: 'intervention',
                source_id: 'intv_1',
                label: 'risk',
                detail: 'message_id=msg_1',
              },
            ],
          },
        ],
        review_snapshot: {
          open_action_count: 3,
          triage_count: 2,
          projects_needing_review: 1,
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
              project_id: 'proj_1',
              connection_id: 'icn_email',
              external_id: 'thread_1',
            },
            requested_payload: { body: 'draft' },
            result_payload: null,
            provenance: [],
            conflict_case_id: null,
            requested_by_node_id: 'vel-local',
            requested_at: '2026-03-19T02:00:00Z',
            applied_at: null,
            updated_at: '2026-03-19T02:05:00Z',
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
              project_id: 'proj_1',
              connection_id: 'icn_email',
              external_id: 'thread_1',
            },
            summary: 'Conflict needs review',
            local_payload: { body: 'draft' },
            upstream_payload: { body: 'upstream' },
            resolution_payload: null,
            opened_at: '2026-03-19T02:05:00Z',
            resolved_at: null,
            updated_at: '2026-03-19T02:06:00Z',
          },
        ],
        people: [
          {
            id: 'per_1',
            display_name: 'Annie Case',
            given_name: 'Annie',
            family_name: 'Case',
            relationship_context: 'teammate',
            birthday: null,
            last_contacted_at: null,
            aliases: [],
            links: [],
          },
        ],
        reasons: ['Prep window active'],
        debug: {
          raw_context: { mode: 'day_mode' },
          signals_used: ['sig_1'],
          commitments_used: ['commit_1'],
          risk_used: ['risk_1'],
        },
      }),
    ).toEqual({
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
        reasons: ['recent git activity'],
      },
      sources: {
        git_activity: {
          label: 'Git activity',
          timestamp: 1710000000,
          summary: { repo: 'vel' },
        },
        health: {
          label: 'Recent health signal',
          timestamp: 1710000030,
          summary: { metric_type: 'sleep_hours', value: 7.5 },
        },
        mood: null,
        pain: null,
        note_document: {
          label: 'Recent note',
          timestamp: 1710000060,
          summary: { path: 'daily/today.md' },
        },
        assistant_message: {
          label: 'Recent transcript',
          timestamp: 1710000120,
          summary: { conversation_id: 'conv_external' },
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
      action_items: [
        {
          id: 'act_1',
          surface: 'inbox',
          kind: 'intervention',
          title: 'Inbox intervention',
          summary: 'Review operator intervention',
          project_id: 'proj_1',
          state: 'active',
          rank: 80,
          surfaced_at: '2026-03-19T02:10:00Z',
          snoozed_until: null,
          evidence: [
            {
              source_kind: 'intervention',
              source_id: 'intv_1',
              label: 'risk',
              detail: 'message_id=msg_1',
            },
          ],
        },
      ],
      review_snapshot: {
        open_action_count: 3,
        triage_count: 2,
        projects_needing_review: 1,
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
            project_id: 'proj_1',
            connection_id: 'icn_email',
            external_id: 'thread_1',
          },
          requested_payload: { body: 'draft' },
          result_payload: null,
          provenance: [],
          conflict_case_id: null,
          requested_by_node_id: 'vel-local',
          requested_at: '2026-03-19T02:00:00Z',
          applied_at: null,
          updated_at: '2026-03-19T02:05:00Z',
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
            project_id: 'proj_1',
            connection_id: 'icn_email',
            external_id: 'thread_1',
          },
          summary: 'Conflict needs review',
          local_payload: { body: 'draft' },
          upstream_payload: { body: 'upstream' },
          resolution_payload: null,
          opened_at: '2026-03-19T02:05:00Z',
          resolved_at: null,
          updated_at: '2026-03-19T02:06:00Z',
        },
      ],
      people: [
        {
          id: 'per_1',
          display_name: 'Annie Case',
          given_name: 'Annie',
          family_name: 'Case',
          relationship_context: 'teammate',
          birthday: null,
          last_contacted_at: null,
          aliases: [],
          links: [],
        },
      ],
      reasons: ['Prep window active'],
      debug: {
        raw_context: { mode: 'day_mode' },
        signals_used: ['sig_1'],
        commitments_used: ['commit_1'],
        risk_used: ['risk_1'],
      },
    })
  })

  it('decodes expanded inbox items with conversation_id, available_actions, and evidence', () => {
    expect(
      decodeInboxItemData({
        id: 'intv_1',
        message_id: 'msg_1',
        kind: 'risk',
        state: 'active',
        surfaced_at: 1710000000,
        snoozed_until: null,
        confidence: 0.9,
        conversation_id: 'conv_1',
        title: 'Link trust degraded',
        summary: 'Inspect the degraded trust state',
        project_id: 'proj_1',
        project_label: 'Vel',
        available_actions: ['acknowledge', 'snooze', 'dismiss', 'open_thread'],
        evidence: [
          {
            source_kind: 'intervention',
            source_id: 'intv_1',
            label: 'risk',
            detail: null,
          },
        ],
      }),
    ).toEqual({
      id: 'intv_1',
      message_id: 'msg_1',
      kind: 'risk',
      state: 'active',
      surfaced_at: 1710000000,
      snoozed_until: null,
      confidence: 0.9,
      conversation_id: 'conv_1',
      title: 'Link trust degraded',
      summary: 'Inspect the degraded trust state',
      project_id: 'proj_1',
      project_label: 'Vel',
      available_actions: ['acknowledge', 'snooze', 'dismiss', 'open_thread'],
      evidence: [
        {
          source_kind: 'intervention',
          source_id: 'intv_1',
          label: 'risk',
          detail: null,
        },
      ],
    })
  })

  it('decodes named review snapshot and action item contracts', () => {
    expect(
      decodeReviewSnapshotData({
        open_action_count: 4,
        triage_count: 2,
        projects_needing_review: 1,
      }),
    ).toEqual({
      open_action_count: 4,
      triage_count: 2,
      projects_needing_review: 1,
    })

    expect(
      decodeActionItemData({
        id: 'act_review',
        surface: 'now',
        kind: 'review',
        title: 'Review project Vel',
        summary: 'Weekly review keeps the project anchored.',
        project_id: 'proj_1',
        state: 'active',
        rank: 60,
        surfaced_at: '2026-03-19T02:11:00Z',
        snoozed_until: null,
        evidence: [
          {
            source_kind: 'project',
            source_id: 'proj_1',
            label: 'Vel',
            detail: 'family=work',
          },
        ],
      }),
    ).toEqual({
      id: 'act_review',
      surface: 'now',
      kind: 'review',
      title: 'Review project Vel',
      summary: 'Weekly review keeps the project anchored.',
      project_id: 'proj_1',
      state: 'active',
      rank: 60,
      surfaced_at: '2026-03-19T02:11:00Z',
      snoozed_until: null,
      evidence: [
        {
          source_kind: 'project',
          source_id: 'proj_1',
          label: 'Vel',
          detail: 'family=work',
        },
      ],
    })
  })

  it('decodes expanded project, linking, and sync bootstrap payloads', () => {
    const project = decodeProjectRecordData({
      id: 'proj_1',
      slug: 'vel',
      name: 'Vel',
      family: 'work',
      status: 'active',
      primary_repo: { path: '/tmp/vel', label: 'vel', kind: 'repo' },
      primary_notes_root: { path: '/tmp/notes/vel', label: 'vel', kind: 'notes_root' },
      secondary_repos: [],
      secondary_notes_roots: [],
      upstream_ids: { github: 'vel' },
      pending_provision: { create_repo: true, create_notes_root: false },
      created_at: '2026-03-19T02:00:00Z',
      updated_at: '2026-03-19T02:05:00Z',
      archived_at: null,
    })

    expect(project.slug).toBe('vel')

    const linkedNode = decodeLinkedNodeData({
      node_id: 'node_remote',
      node_display_name: 'Remote',
      status: 'pending',
      scopes: {
        read_context: true,
        write_safe_actions: false,
        execute_repo_tasks: false,
      },
      linked_at: '2026-03-19T02:01:00Z',
      last_seen_at: '2026-03-19T02:02:00Z',
      transport_hint: 'tailscale',
    })

    expect(linkedNode.status).toBe('pending')

    expect(
      decodeSyncBootstrapData({
        cluster: {
          node_id: 'vel-desktop',
          node_display_name: 'Vel Desktop',
          active_authority_node_id: 'vel-desktop',
          active_authority_epoch: 1,
          sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
          sync_transport: 'tailscale',
          tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
          lan_base_url: 'http://192.168.1.50:4130',
          localhost_base_url: 'http://127.0.0.1:4130',
          capabilities: ['build_test_profiles'],
          linked_nodes: [linkedNode],
          projects: [project],
          action_items: [
            {
              id: 'act_1',
              surface: 'inbox',
              kind: 'linking',
              title: 'Linked node Remote needs review',
              summary: 'Review linked node trust',
              project_id: null,
              state: 'active',
              rank: 85,
              surfaced_at: '2026-03-19T02:12:00Z',
              snoozed_until: null,
              evidence: [
                {
                  source_kind: 'linked_node',
                  source_id: 'node_remote',
                  label: 'Remote',
                  detail: 'status=pending',
                },
              ],
            },
          ],
        },
        current_context: null,
        nudges: [],
        commitments: [],
        linked_nodes: [linkedNode],
        projects: [project],
        action_items: [],
      }),
    ).toEqual({
      cluster: {
        node_id: 'vel-desktop',
        node_display_name: 'Vel Desktop',
        active_authority_node_id: 'vel-desktop',
        active_authority_epoch: 1,
        sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        sync_transport: 'tailscale',
        tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
        lan_base_url: 'http://192.168.1.50:4130',
        localhost_base_url: 'http://127.0.0.1:4130',
        capabilities: ['build_test_profiles'],
        linked_nodes: [linkedNode],
        projects: [project],
        action_items: [
          {
            id: 'act_1',
            surface: 'inbox',
            kind: 'linking',
            title: 'Linked node Remote needs review',
            summary: 'Review linked node trust',
            project_id: null,
            state: 'active',
            rank: 85,
            surfaced_at: '2026-03-19T02:12:00Z',
            snoozed_until: null,
            evidence: [
              {
                source_kind: 'linked_node',
                source_id: 'node_remote',
                label: 'Remote',
                detail: 'status=pending',
              },
            ],
          },
        ],
      },
      current_context: null,
      nudges: [],
      commitments: [],
      linked_nodes: [linkedNode],
      projects: [project],
      action_items: [],
    })
  })

  it('decodes cluster worker presence payloads', () => {
    const decoded = decodeClusterWorkersData({
      active_authority_node_id: 'vel-desktop',
      active_authority_epoch: 1,
      generated_at: 1_710_000_100,
      workers: [
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
          last_heartbeat_at: 1_710_000_090,
          started_at: 1_710_000_000,
          sync_base_url: 'http://remote.tailnet.ts.net:4130',
          sync_transport: 'tailscale',
          tailscale_base_url: 'http://remote.tailnet.ts.net:4130',
          preferred_tailnet_endpoint: null,
          tailscale_reachable: true,
          lan_base_url: null,
          localhost_base_url: null,
          ping_ms: 14,
          sync_status: 'ready',
          last_upstream_sync_at: null,
          last_downstream_sync_at: null,
          last_sync_error: null,
          incoming_linking_prompt: {
            target_node_id: 'node_remote',
            target_node_display_name: 'Remote Mac',
            issued_by_node_id: 'vel-desktop',
            issued_by_node_display_name: 'Vel Desktop',
            issued_at: '2026-03-16T18:20:00Z',
            expires_at: '2026-03-16T18:35:00Z',
            scopes: {
              read_context: true,
              write_safe_actions: false,
              execute_repo_tasks: false,
            },
          },
          capacity: {
            max_concurrency: 2,
            current_load: 0,
            available_concurrency: 2,
          },
        },
      ],
    })

    expect(decoded).toMatchObject({
      active_authority_node_id: 'vel-desktop',
      active_authority_epoch: 1,
      generated_at: 1_710_000_100,
      workers: [
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
          last_heartbeat_at: 1_710_000_090,
          started_at: 1_710_000_000,
          sync_base_url: 'http://remote.tailnet.ts.net:4130',
          sync_transport: 'tailscale',
          tailscale_base_url: 'http://remote.tailnet.ts.net:4130',
          preferred_tailnet_endpoint: null,
          tailscale_reachable: true,
          lan_base_url: null,
          localhost_base_url: null,
          ping_ms: 14,
          sync_status: 'ready',
          last_upstream_sync_at: null,
          last_downstream_sync_at: null,
          last_sync_error: null,
          incoming_linking_prompt: {
            target_node_id: 'node_remote',
            target_node_display_name: 'Remote Mac',
            issued_by_node_id: 'vel-desktop',
            issued_by_node_display_name: 'Vel Desktop',
            issued_at: '2026-03-16T18:20:00Z',
            expires_at: '2026-03-16T18:35:00Z',
            scopes: {
              read_context: true,
              write_safe_actions: false,
              execute_repo_tasks: false,
            },
          },
          capacity: {
            max_concurrency: 2,
            current_load: 0,
            available_concurrency: 2,
          },
        },
      ],
    })
    expect(decoded.workers[0].incoming_linking_prompt?.target_node_display_name).toBe('Remote Mac')
  })

  it('decodes suggestion detail payloads with evidence', () => {
    expect(
      decodeSuggestionData({
        id: 'sug_1',
        suggestion_type: 'increase_commute_buffer',
        state: 'pending',
        title: 'Increase commute buffer',
        summary: 'Leave earlier for repeated commute danger.',
        priority: 55,
        confidence: 'medium',
        evidence_count: 2,
        decision_context_summary: 'Repeated commute danger nudges.',
        decision_context: {
          trigger: 'resolved_commute_danger',
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
        payload: {
          type: 'increase_commute_buffer',
          current_minutes: 20,
          suggested_minutes: 30,
        },
        latest_feedback_outcome: 'accepted_and_policy_changed',
        latest_feedback_notes: 'helpful',
        adaptive_policy: null,
        created_at: 1710000000,
        resolved_at: null,
      }),
    ).toEqual({
      id: 'sug_1',
      suggestion_type: 'increase_commute_buffer',
      state: 'pending',
      title: 'Increase commute buffer',
      summary: 'Leave earlier for repeated commute danger.',
      priority: 55,
      confidence: 'medium',
      evidence_count: 2,
      decision_context_summary: 'Repeated commute danger nudges.',
      decision_context: {
        trigger: 'resolved_commute_danger',
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
      payload: {
        type: 'increase_commute_buffer',
        current_minutes: 20,
        suggested_minutes: 30,
      },
      latest_feedback_outcome: 'accepted_and_policy_changed',
      latest_feedback_notes: 'helpful',
      adaptive_policy: null,
      created_at: 1710000000,
      resolved_at: null,
    })
  })

  it('decodes runtime loop payloads', () => {
    expect(
      decodeLoopData({
        kind: 'evaluate_current_state',
        enabled: true,
        interval_seconds: 300,
        last_started_at: 1710000000,
        last_finished_at: 1710000030,
        last_status: 'success',
        last_error: null,
        next_due_at: 1710000300,
      }),
    ).toEqual({
      kind: 'evaluate_current_state',
      enabled: true,
      interval_seconds: 300,
      last_started_at: 1710000000,
      last_finished_at: 1710000030,
      last_status: 'success',
      last_error: null,
      next_due_at: 1710000300,
    })
  })

  it('decodes uncertainty payloads', () => {
    expect(
      decodeUncertaintyData({
        id: 'unc_1',
        subject_type: 'suggestion_candidate',
        subject_id: 'followup_block',
        decision_kind: 'add_followup_block',
        confidence_band: 'borderline',
        confidence_score: 0.47,
        reasons: { summary: 'Weak evidence for follow-up scheduling' },
        missing_evidence: { needed: ['recent_response_debt'] },
        resolution_mode: 'operator_review',
        status: 'open',
        created_at: 1710000100,
        resolved_at: null,
      }),
    ).toEqual({
      id: 'unc_1',
      subject_type: 'suggestion_candidate',
      subject_id: 'followup_block',
      decision_kind: 'add_followup_block',
      confidence_band: 'borderline',
      confidence_score: 0.47,
      reasons: { summary: 'Weak evidence for follow-up scheduling' },
      missing_evidence: { needed: ['recent_response_debt'] },
      resolution_mode: 'operator_review',
      status: 'open',
      created_at: 1710000100,
      resolved_at: null,
    })
  })

  it('decodes component arrays with restart metadata', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: [
          {
            id: 'evaluate',
            name: 'Evaluate',
            description: 'Run full evaluation',
            status: 'ok',
            last_restarted_at: 1_700_000_000,
            last_error: null,
            restart_count: 3,
          },
        ],
        meta: { request_id: 'req_components' },
      },
      (value) => decodeArray(value, decodeComponentData),
    )

    expect(response.data?.[0].id).toBe('evaluate')
    expect(response.data?.[0].restart_count).toBe(3)
  })

  it('decodes component log events with created_at and payload', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: {
          id: 'log_eval_1',
          component_id: 'evaluate',
          event_name: 'component.restart.completed',
          status: 'success',
          message: 'Evaluate complete',
          payload: { requested_at: 1_700_000_100 },
          created_at: 1_700_000_200,
        },
        meta: { request_id: 'req_component_log' },
      },
      decodeComponentLogEventData,
    )

    expect(response.data?.component_id).toBe('evaluate')
    expect(response.data?.payload).toEqual({ requested_at: 1_700_000_100 })
    expect(response.data?.created_at).toBe(1_700_000_200)
  })

  it('requires RFC3339 commitment datetime fields', () => {
    expect(() =>
      decodeCommitmentData({
        id: 'commit_1',
        text: 'Ship feature',
        source_type: 'manual',
        source_id: null,
        status: 'open',
        due_at: [2026, 75, 9, 30, 0, 0],
        project: null,
        commitment_kind: 'todo',
        created_at: [2026, 75, 8, 0, 0, 0],
        resolved_at: null,
        metadata: {},
      }),
    ).toThrow(/commitment\.due_at/)
  })

  it('decodes websocket message events', () => {
    const event = decodeWsEvent({
      type: 'messages:new',
      timestamp: '2026-03-16T12:00:00Z',
      payload: {
        id: 'msg_1',
        conversation_id: 'conv_1',
        role: 'assistant',
        kind: 'text',
        content: { text: 'reply' },
        status: null,
        importance: null,
        created_at: 1,
        updated_at: null,
      },
    })

    expect(event.type).toBe('messages:new')
    if (event.type === 'messages:new') {
      expect(event.payload.content).toEqual({ text: 'reply' })
    }
  })

  it('decodes websocket run update events', () => {
    const event = decodeWsEvent({
      type: 'runs:updated',
      timestamp: '2026-03-16T12:05:00Z',
      payload: {
        id: 'run_1',
        kind: 'search',
        status: 'blocked',
        trace_id: 'trace_1',
        parent_run_id: null,
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T12:00:00Z',
        started_at: null,
        finished_at: '2026-03-16T12:04:00Z',
        duration_ms: 240000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'waiting_on_dependency',
      },
    })

    expect(event.type).toBe('runs:updated')
    if (event.type === 'runs:updated') {
      expect(event.payload).toEqual({
        id: 'run_1',
        kind: 'search',
        status: 'blocked',
        trace_id: 'trace_1',
        parent_run_id: null,
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: '2026-03-16T12:00:00Z',
        started_at: null,
        finished_at: '2026-03-16T12:04:00Z',
        duration_ms: 240000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'waiting_on_dependency',
      })
    }
  })

  it('normalizes unix timestamp run summary datetime fields', () => {
    expect(
      decodeRunSummaryData({
        id: 'run_1',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: 1710590400,
        started_at: null,
        finished_at: 1710590640,
        duration_ms: 240000,
        retry_scheduled_at: 1710590700,
        retry_reason: null,
        blocked_reason: 'waiting_on_dependency',
      }),
    ).toEqual({
      id: 'run_1',
      kind: 'search',
      status: 'blocked',
      trace_id: 'run_1',
      parent_run_id: null,
      automatic_retry_supported: false,
      automatic_retry_reason: 'search runs do not have an automatic retry executor',
      unsupported_retry_override: false,
      unsupported_retry_override_reason: null,
      created_at: '2024-03-16T12:00:00.000Z',
      started_at: null,
      finished_at: '2024-03-16T12:04:00.000Z',
      duration_ms: 240000,
      retry_scheduled_at: '2024-03-16T12:05:00.000Z',
      retry_reason: null,
      blocked_reason: 'waiting_on_dependency',
    })
  })

  it('falls back when run summary created_at is missing', () => {
    expect(
      decodeRunSummaryData({
        id: 'run_legacy',
        kind: 'search',
        status: 'completed',
        automatic_retry_supported: false,
        automatic_retry_reason: null,
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: null,
        started_at: 1710590400,
        finished_at: 1710590640,
        duration_ms: 240000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: null,
      }),
    ).toEqual({
      id: 'run_legacy',
      kind: 'search',
      status: 'completed',
      trace_id: 'run_legacy',
      parent_run_id: null,
      automatic_retry_supported: false,
      automatic_retry_reason: null,
      unsupported_retry_override: false,
      unsupported_retry_override_reason: null,
      created_at: '2024-03-16T12:00:00.000Z',
      started_at: '2024-03-16T12:00:00.000Z',
      finished_at: '2024-03-16T12:04:00.000Z',
      duration_ms: 240000,
      retry_scheduled_at: null,
      retry_reason: null,
      blocked_reason: null,
    })
  })

  it('decodes canonical risk card payloads', () => {
    expect(
      decodeRiskCardContent({
        commitment_id: 'commit_42',
        risk_level: 'danger',
        risk_score: 0.82,
        factors: {
          reasons: ['long-stale open commitment'],
          dependency_ids: ['dep_1', 'dep_2'],
        },
      }),
    ).toEqual({
      commitment_title: 'commit_42',
      risk_level: 'danger',
      risk_score: 0.82,
      top_drivers: ['long-stale open commitment'],
      dependency_ids: ['dep_1', 'dep_2'],
      proposed_next_step: undefined,
    })
  })

  it('decodes websocket context update events', () => {
    const event = decodeWsEvent({
      type: 'context:updated',
      timestamp: '2026-03-16T12:08:00Z',
      payload: {
        computed_at: 1710000000,
        context: {
          mode: 'focus',
          global_risk_level: 'high',
        },
      },
    })

    expect(event.type).toBe('context:updated')
    if (event.type === 'context:updated') {
      expect(event.payload.computed_at).toBe(1710000000)
      expect(event.payload.context).toEqual({
        mode: 'focus',
        global_risk_level: 'high',
      })
    }
  })

  it('decodes websocket linking update events', () => {
    const event = decodeWsEvent({
      type: 'linking:updated',
      timestamp: '2026-03-16T12:09:00Z',
      payload: {
        reason: 'pairing_prompt_saved',
      },
    })

    expect(event.type).toBe('linking:updated')
    if (event.type === 'linking:updated') {
      expect(event.payload).toEqual({ reason: 'pairing_prompt_saved' })
    }
  })

  it('decodes websocket component update events', () => {
    const event = decodeWsEvent({
      type: 'components:updated',
      timestamp: '2026-03-16T12:10:00Z',
      payload: {
        id: 'evaluate',
        name: 'Evaluate',
        description: 'Evaluate all pipelines',
        status: 'running',
        last_restarted_at: 1_700_000_300,
        last_error: null,
        restart_count: 4,
      },
    })

    expect(event.type).toBe('components:updated')
    if (event.type === 'components:updated') {
      expect(event.payload.id).toBe('evaluate')
      expect(event.payload.status).toBe('running')
      expect(event.payload.restart_count).toBe(4)
    }
  })

  it('rejects malformed websocket payloads for known event types', () => {
    expect(() =>
      decodeWsEvent({
        type: 'interventions:new',
        timestamp: '2026-03-16T12:00:00Z',
        payload: { id: 'intv_1', state: 'active' },
      }),
    ).toThrow(/message_id/)
  })

  it('rejects malformed websocket timestamps', () => {
    expect(() =>
      decodeWsEvent({
        type: 'messages:new',
        timestamp: '1700000000',
        payload: {
          id: 'msg_1',
          conversation_id: 'conv_1',
          role: 'assistant',
          kind: 'text',
          content: { text: 'reply' },
          status: null,
          importance: null,
          created_at: 1,
          updated_at: null,
        },
      }),
    ).toThrow(/RFC3339/)
  })
})
