import { describe, expect, it } from 'vitest'
import {
  decodeApiResponse,
  decodeCommitmentData,
  decodeCreateMessageResponse,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeIntegrationsData,
  decodeLoopData,
  decodeNowData,
  decodeNullable,
  decodeArray,
  decodeRiskCardContent,
  decodeRunSummaryData,
  decodeSettingsData,
  decodeSuggestionData,
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
            last_sync_at: 12,
            last_sync_status: 'ok',
            last_error: null,
            last_item_count: 4,
            guidance: null,
          },
          health: {
            configured: true,
            source_path: '/tmp/health.json',
            last_sync_at: 14,
            last_sync_status: 'ok',
            last_error: null,
            last_item_count: 2,
            guidance: null,
          },
          git: {
            configured: false,
            source_path: null,
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
            last_sync_at: null,
            last_sync_status: null,
            last_error: null,
            last_item_count: null,
            guidance: null,
          },
          notes: {
            configured: false,
            source_path: null,
            last_sync_at: null,
            last_sync_status: null,
            last_error: null,
            last_item_count: null,
            guidance: null,
          },
          transcripts: {
            configured: false,
            source_path: null,
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
    expect(response.data?.google_calendar.guidance?.action).toBe('Save credentials')
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
      reasons: ['Prep window active'],
      debug: {
        raw_context: { mode: 'day_mode' },
        signals_used: ['sig_1'],
        commitments_used: ['commit_1'],
        risk_used: ['risk_1'],
      },
    })
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

  it('requires RFC3339 run summary datetime fields', () => {
    expect(() =>
      decodeRunSummaryData({
        id: 'run_1',
        kind: 'search',
        status: 'blocked',
        automatic_retry_supported: false,
        automatic_retry_reason: 'search runs do not have an automatic retry executor',
        unsupported_retry_override: false,
        unsupported_retry_override_reason: null,
        created_at: [2026, 76, 12, 0, 0, 0],
        started_at: null,
        finished_at: '2026-03-16T12:04:00Z',
        duration_ms: 240000,
        retry_scheduled_at: null,
        retry_reason: null,
        blocked_reason: 'waiting_on_dependency',
      }),
    ).toThrow(/run summary\.created_at/)
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
