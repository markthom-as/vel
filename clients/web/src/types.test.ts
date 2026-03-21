import { describe, expect, it } from 'vitest'
import {
  decodeAgentInspectData,
  decodeAssistantContextData,
  decodeAssistantEntryResponse,
  decodeActionItemData,
  decodeApiResponse,
  decodeCommitmentData,
  decodeCreateMessageResponse,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeClusterWorkersData,
  decodeDailyLoopSessionData,
  decodeExecutionHandoffRecordData,
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
  decodePlanningProfileEditProposalData,
  decodePlanningProfileResponseData,
  decodeProjectRecordData,
  decodeRecallContextData,
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

  it('decodes assistant-entry responses with typed route outcomes', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: {
          route_target: 'inline',
          entry_intent: 'question',
          continuation_category: 'review_apply',
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
          assistant_context: {
            query_text: 'accountant follow up',
            summary:
              'Found 1 relevant recalled item across note (1). 1 open commitment with canonical scheduler rules remain available.',
            focus_lines: ['note projects/tax/accountant.md: Need accountant follow up.'],
            commitments: [
              {
                id: 'com_1',
                text: 'Deep work @30m',
                source_type: 'todoist',
                source_id: 'task_1',
                status: 'open',
                due_at: null,
                project: 'tax',
                commitment_kind: 'todo',
                created_at: '2026-03-20T00:00:00Z',
                resolved_at: null,
                scheduler_rules: {
                  block_target: 'focus',
                  duration_minutes: 30,
                  calendar_free: true,
                  fixed_start: false,
                  time_window: 'prenoon',
                  local_urgency: true,
                  local_defer: false,
                },
                metadata: {
                  labels: ['block:focus', '@cal:free', 'time:prenoon', '@urgent'],
                  scheduler_rules: {
                    block_target: 'focus',
                    duration_minutes: 30,
                    calendar_free: true,
                    fixed_start: false,
                    time_window: 'prenoon',
                    local_urgency: true,
                    local_defer: false,
                  },
                },
              },
            ],
            recall: {
              query_text: 'accountant follow up',
              hit_count: 1,
              source_counts: [{ source_kind: 'note', count: 1 }],
              hits: [
                {
                  record_id: 'sem_note_1',
                  source_kind: 'note',
                  source_id: 'projects/tax/accountant.md',
                  snippet: 'Need accountant follow up.',
                  lexical_score: 0.4,
                  semantic_score: 0.9,
                  combined_score: 0.775,
                  provenance: { note_path: 'projects/tax/accountant.md' },
                },
              ],
            },
          },
          conversation: {
            id: 'conv_1',
            title: 'Conversation',
            kind: 'general',
            pinned: false,
            archived: false,
            created_at: 0,
            updated_at: 2,
            continuation: {
              thread_id: 'thr_assistant_proposal_msg_user',
              thread_type: 'assistant_proposal',
              lifecycle_stage: 'staged',
              continuation: {
                escalation_reason:
                  'This assistant proposal became multi-step and remains in Threads for explicit follow-through.',
                continuation_context: {
                  source_message_id: 'msg_user',
                  action_item_id: 'act_intervention_intv_1',
                },
                review_requirements: [
                  'Operator confirmation is required before the proposal can be applied.',
                ],
                bounded_capability_state: 'proposal_review_gated',
                continuation_category: 'review_apply',
                open_target: 'thread',
              },
            },
          },
          proposal: {
            action_item_id: 'act_intervention_intv_1',
            state: 'staged',
            kind: 'intervention',
            permission_mode: 'user_confirm',
            scope_affinity: 'global',
            title: 'Inbox intervention',
            summary: 'Needs operator review from the intervention queue.',
            project_id: null,
            project_label: null,
            project_family: null,
            thread_route: {
              target: 'existing_thread',
              label: 'Continue in Threads',
              thread_id: 'thr_action_intervention_intv_1',
              thread_type: 'action_resolution',
              project_id: null,
            },
          },
          planning_profile_proposal: {
            source_surface: 'assistant',
            state: 'staged',
            mutation: {
              kind: 'upsert_planning_constraint',
              data: {
                id: 'constraint_default_prenoon',
                label: 'Default prenoon',
                kind: 'default_time_window',
                detail: null,
                time_window: 'prenoon',
                minutes: null,
                max_items: null,
                active: true,
              },
            },
            summary: 'Stage planning constraint.',
            requires_confirmation: true,
            continuity: 'thread',
            outcome_summary: null,
            thread_id: 'thr_planning_profile_edit_msg_1',
            thread_type: 'planning_profile_edit',
          },
          daily_loop_session: {
            id: 'dls_1',
            session_date: '2026-03-19',
            phase: 'morning_overview',
            status: 'waiting_for_input',
            start: {
              source: 'manual',
              surface: 'web',
            },
            turn_state: 'waiting_for_input',
            continuity_summary:
              'Morning overview is waiting on question 1 of 3 with 0 captured signal(s).',
            allowed_actions: ['accept', 'defer', 'choose', 'close'],
            current_prompt: {
              prompt_id: 'morning_prompt_1',
              kind: 'intent_question',
              text: 'What matters most today?',
              ordinal: 1,
              allow_skip: true,
            },
            state: {
              phase: 'morning_overview',
              snapshot: 'Snapshot',
              friction_callouts: [],
              signals: [],
              check_in_history: [],
            },
            outcome: null,
          },
          end_of_day: {
            date: '2026-03-19',
            what_was_done: [
              {
                capture_id: 'cap_1',
                capture_type: 'quick_note',
                content_text: 'finished draft',
                occurred_at: '2026-03-19T18:00:00Z',
                source_device: 'desktop',
              },
            ],
            what_remains_open: ['follow up'],
            what_may_matter_tomorrow: ['budget review'],
          },
        },
        meta: { request_id: 'req_assistant_entry_1' },
      },
      decodeAssistantEntryResponse,
    )

    expect(response.data?.route_target).toBe('inline')
    expect(response.data?.entry_intent).toBe('question')
    expect(response.data?.continuation_category).toBe('review_apply')
    expect(response.data?.conversation.id).toBe('conv_1')
    expect(response.data?.conversation.continuation?.thread_type).toBe('assistant_proposal')
    expect(
      response.data?.conversation.continuation?.continuation.continuation_category,
    ).toBe('review_apply')
    expect(response.data?.assistant_message?.id).toBe('msg_assistant')
    expect(response.data?.assistant_context?.focus_lines[0]).toContain('accountant')
    expect(response.data?.assistant_context?.commitments[0]?.scheduler_rules.block_target).toBe(
      'focus',
    )
    expect(response.data?.proposal?.action_item_id).toBe('act_intervention_intv_1')
    expect(response.data?.proposal?.state).toBe('staged')
    expect(response.data?.planning_profile_proposal?.state).toBe('staged')
    expect(response.data?.planning_profile_proposal?.thread_type).toBe('planning_profile_edit')
    expect(response.data?.daily_loop_session?.phase).toBe('morning_overview')
    expect(response.data?.end_of_day?.date).toBe('2026-03-19')
    expect(response.data?.end_of_day?.what_was_done[0]?.content_text).toBe('finished draft')
    expect(response.data?.end_of_day?.what_remains_open).toEqual(['follow up'])
    expect(response.data?.end_of_day?.what_may_matter_tomorrow).toEqual(['budget review'])
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

  it('decodes planning-profile responses with routine blocks and constraints', () => {
    const response = decodeApiResponse(
      {
        ok: true,
        data: {
          profile: {
            routine_blocks: [
              {
                id: 'routine_focus',
                label: 'Focus block',
                source: 'operator_declared',
                local_timezone: 'America/Denver',
                start_local_time: '09:00',
                end_local_time: '11:00',
                days_of_week: [1, 2, 3, 4, 5],
                protected: true,
                active: true,
              },
            ],
            planning_constraints: [
              {
                id: 'constraint_default_window',
                label: 'Morning default',
                kind: 'default_time_window',
                detail: 'Default to the morning block first.',
                time_window: 'prenoon',
                minutes: null,
                max_items: null,
                active: true,
              },
            ],
          },
          proposal_summary: {
            pending_count: 1,
            latest_pending: {
              thread_id: 'thr_planning_profile_edit_1',
              state: 'staged',
              title: 'Add shutdown block',
              summary: 'Add a protected shutdown block.',
              outcome_summary: null,
              updated_at: 1710000000,
            },
            latest_applied: null,
            latest_failed: null,
          },
        },
        meta: { request_id: 'req_planning_profile_1' },
      },
      decodePlanningProfileResponseData,
    )

    expect(response.data?.profile.routine_blocks[0]?.local_timezone).toBe('America/Denver')
    expect(response.data?.profile.planning_constraints[0]?.kind).toBe('default_time_window')
    expect(response.data?.profile.planning_constraints[0]?.time_window).toBe('prenoon')
    expect(response.data?.proposal_summary?.pending_count).toBe(1)
    expect(response.data?.proposal_summary?.latest_pending?.thread_id).toBe(
      'thr_planning_profile_edit_1',
    )
  })

  it('decodes assistant-capable planning-profile edit proposals', () => {
    const proposal = decodePlanningProfileEditProposalData({
      source_surface: 'assistant',
      state: 'staged',
      mutation: {
        kind: 'upsert_routine_block',
        data: {
          id: 'routine_shutdown',
          label: 'Shutdown',
          source: 'operator_declared',
          local_timezone: 'America/Denver',
          start_local_time: '17:00',
          end_local_time: '17:30',
          days_of_week: [1, 2, 3, 4, 5],
          protected: true,
          active: true,
        },
      },
      summary: 'Add a protected weekday shutdown block from 17:00 to 17:30.',
      requires_confirmation: true,
      continuity: 'thread',
      outcome_summary: null,
    })

    expect(proposal.source_surface).toBe('assistant')
    expect(proposal.state).toBe('staged')
    expect(proposal.mutation.kind).toBe('upsert_routine_block')
    expect(proposal.requires_confirmation).toBe(true)
    expect(proposal.continuity).toBe('thread')
    expect(proposal.outcome_summary).toBeNull()
  })

  it('decodes recall-context packs with typed source counts and hits', () => {
    const recall = decodeRecallContextData({
      query_text: 'accountant follow up',
      hit_count: 1,
      source_counts: [{ source_kind: 'note', count: 1 }],
      hits: [
        {
          record_id: 'sem_note_1',
          source_kind: 'note',
          source_id: 'projects/tax/accountant.md',
          snippet: 'Need accountant follow up on quarterly estimate.',
          lexical_score: 0.4,
          semantic_score: 0.9,
          combined_score: 0.775,
          provenance: {
            note_path: 'projects/tax/accountant.md',
          },
        },
      ],
    })

    expect(recall.query_text).toBe('accountant follow up')
    expect(recall.source_counts[0]).toEqual({ source_kind: 'note', count: 1 })
    expect(recall.hits[0].combined_score).toBe(0.775)
    expect(recall.hits[0].provenance).toEqual({
      note_path: 'projects/tax/accountant.md',
    })
  })

  it('decodes assistant-context packs with recall summary and nested recall', () => {
    const context = decodeAssistantContextData({
      query_text: 'accountant follow up',
      summary: 'Found 1 relevant recalled item across note sources.',
      focus_lines: ['note projects/tax/accountant.md: Need accountant follow up.'],
      recall: {
        query_text: 'accountant follow up',
        hit_count: 1,
        source_counts: [{ source_kind: 'note', count: 1 }],
        hits: [
          {
            record_id: 'sem_note_1',
            source_kind: 'note',
            source_id: 'projects/tax/accountant.md',
            snippet: 'Need accountant follow up.',
            lexical_score: 0.4,
            semantic_score: 0.9,
            combined_score: 0.775,
            provenance: { note_path: 'projects/tax/accountant.md' },
          },
        ],
      },
    })

    expect(context.summary).toContain('recalled item')
    expect(context.focus_lines[0]).toContain('accountant')
    expect(context.recall.hits[0].combined_score).toBe(0.775)
  })

  it('decodes action items with typed thread routing hints', () => {
    const item = decodeActionItemData({
      id: 'act_project_review_1',
      surface: 'now',
      kind: 'review',
      permission_mode: 'user_confirm',
      scope_affinity: 'project',
      title: 'Review project Vel',
      summary: 'Weekly review keeps the project anchored in Now and Inbox.',
      project_id: 'proj_vel',
      project_label: 'Vel',
      project_family: 'work',
      state: 'active',
      rank: 60,
      surfaced_at: '2026-03-19T02:10:00Z',
      snoozed_until: null,
      evidence: [],
      thread_route: {
        target: 'filtered_threads',
        label: 'Open related threads',
        thread_id: null,
        thread_type: 'project_review',
        project_id: 'proj_vel',
      },
    })

    expect(item.thread_route?.target).toBe('filtered_threads')
    expect(item.thread_route?.thread_type).toBe('project_review')
    expect(item.thread_route?.project_id).toBe('proj_vel')
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

  it('decodes persisted execution handoff review records', () => {
    const handoff = decodeExecutionHandoffRecordData({
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
          inputs: { project: 'vel' },
          constraints: ['sidecar only'],
          read_scopes: ['/tmp/vel', '/tmp/vel/notes'],
          write_scopes: ['/tmp/vel'],
          project_id: 'proj_exec',
          task_kind: 'implementation',
          agent_profile: 'quality',
          token_budget: 'large',
          review_gate: 'operator_approval',
          repo_root: {
            path: '/tmp/vel',
            label: 'vel',
            branch: 'main',
            head_rev: 'abc123',
          },
          allowed_tools: ['rg', 'cargo test'],
          capability_scope: {
            read_scopes: ['/tmp/vel'],
            write_scopes: ['/tmp/vel'],
          },
          deadline: null,
          expected_output_schema: {
            artifacts: ['patch', 'summary'],
          },
        },
        project_id: 'proj_exec',
        task_kind: 'implementation',
        agent_profile: 'quality',
        token_budget: 'large',
        review_gate: 'operator_approval',
        repo: {
          path: '/tmp/vel',
          label: 'vel',
          branch: 'main',
          head_rev: 'abc123',
        },
        notes_root: {
          path: '/tmp/vel/notes',
          label: 'vel-notes',
          kind: 'notes_root',
        },
        manifest_id: 'local-codex',
      },
      routing: {
        task_kind: 'implementation',
        agent_profile: 'quality',
        token_budget: 'large',
        review_gate: 'operator_approval',
        read_scopes: ['/tmp/vel'],
        write_scopes: ['/tmp/vel'],
        allowed_tools: ['rg', 'cargo test'],
        reasons: [
          {
            code: 'write_scope_requires_approval',
            message: 'write scopes require explicit operator approval before launch',
          },
        ],
      },
      manifest_id: 'local-codex',
      requested_by: 'operator_shell',
      reviewed_by: null,
      decision_reason: null,
      reviewed_at: null,
      launched_at: null,
      created_at: '2026-03-19T00:00:00Z',
      updated_at: '2026-03-19T00:00:00Z',
    })

    expect(handoff.handoff.handoff.objective).toBe('Implement the next safe slice')
    expect(handoff.routing.reasons[0]?.code).toBe('write_scope_requires_approval')
    expect(handoff.review_state).toBe('pending_review')
  })

  it('decodes agent inspect payloads with blockers and capability groups', () => {
    const inspect = decodeAgentInspectData({
      grounding: {
        generated_at: 1710000000,
        now: {
          computed_at: 1710000000,
          timezone: 'America/Denver',
          overview: {
            dominant_action: null,
            today_timeline: [],
            visible_nudge: null,
            why_state: [],
            suggestions: [],
            decision_options: ['accept', 'choose', 'thread', 'close'],
          },
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
          review_snapshot: {
            open_action_count: 1,
            triage_count: 0,
            projects_needing_review: 0,
            pending_execution_reviews: 0,
          },
          pending_writebacks: [],
          conflicts: [],
          trust_readiness: {
            level: 'ok',
            headline: 'Ready',
            summary: 'Backup trust and current context are healthy.',
            backup: {
              level: 'ok',
              label: 'Backup',
              detail: 'Backup trust is healthy.',
            },
            freshness: {
              level: 'ok',
              label: 'Freshness',
              detail: 'Current context and integrations look fresh enough to trust.',
            },
          review: {
            open_action_count: 1,
            pending_execution_reviews: 0,
            pending_writeback_count: 0,
            conflict_count: 0,
          },
          guidance: ['Backup trust is healthy.'],
          follow_through: [],
        },
          people: [],
          reasons: [],
          debug: { raw_context: {}, signals_used: [], commitments_used: [], risk_used: [] },
        },
        current_context: {
          computed_at: 1710000000,
          mode: 'focus',
          morning_state: 'engaged',
          current_context_path: '/v1/context/current',
          explain_context_path: '/v1/explain/context',
          explain_drift_path: '/v1/explain/drift',
        },
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
            kind: 'mutation_actions',
            label: 'Bounded mutation affordances',
            entries: [
              {
                key: 'integration_writeback',
                label: 'Request integration writeback',
                summary: 'Bounded upstream mutations remain subject to SAFE MODE and review gates.',
                available: false,
                blocked_reason: {
                  code: 'safe_mode_enabled',
                  message: 'SAFE MODE keeps writeback disabled.',
                  escalation_hint: 'Enable writeback in Settings before retrying.',
                },
                requires_review_gate: 'operator_preview',
                requires_writeback_enabled: true,
              },
            ],
          },
        ],
      },
      blockers: [
        {
          code: 'writeback_disabled',
          message: 'Writeback-dependent mutation requests are unavailable while SAFE MODE is enabled.',
          escalation_hint: 'Enable writeback or stay within read/review lanes.',
        },
      ],
      explainability: {
        persisted_record_kinds: ['now'],
        supporting_paths: ['/v1/agent/inspect'],
        raw_context_json_supporting_only: true,
      },
    })

    expect(inspect.capabilities.groups[0]?.entries[0]?.blocked_reason?.code).toBe('safe_mode_enabled')
    expect(inspect.blockers[0]?.escalation_hint).toContain('Enable writeback')
  })

  it('rejects malformed agent inspect capability payloads', () => {
    expect(() =>
      decodeAgentInspectData({
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
              kind: 'mutation_actions',
              label: 'Broken group',
              entries: [
                {
                  key: 'integration_writeback',
                  label: 'Broken entry',
                  summary: 'broken',
                  available: 'nope',
                  blocked_reason: null,
                  requires_review_gate: null,
                  requires_writeback_enabled: false,
                },
              ],
            },
          ],
        },
        blockers: [],
        explainability: {
          persisted_record_kinds: [],
          supporting_paths: [],
          raw_context_json_supporting_only: true,
        },
      }),
    ).toThrow()
  })

  it('decodes morning daily-loop sessions with typed state and outcome payloads', () => {
    const session = decodeDailyLoopSessionData({
      id: 'dls_morning_1',
      session_date: '2026-03-19',
      phase: 'morning_overview',
      status: 'completed',
      start: {
        source: 'manual',
        surface: 'web',
      },
      turn_state: 'completed',
      continuity_summary:
        'Morning overview continuity is available.',
      allowed_actions: ['accept', 'choose', 'close'],
      current_prompt: null,
      state: {
        phase: 'morning_overview',
        snapshot: 'Two meetings before noon. Todoist backlog is heavier than normal.',
        friction_callouts: [
          {
            label: 'Packed morning',
            detail: 'Back-to-back calendar blocks until 11:30.',
          },
        ],
        signals: [
          {
            kind: 'must_do_hint',
            text: 'Ship Phase 10.',
          },
        ],
      },
      outcome: {
        phase: 'morning_overview',
        signals: [
          {
            kind: 'focus_intent',
            text: 'Protect a two-hour focus block after lunch.',
          },
        ],
      },
    })

    expect(session.phase).toBe('morning_overview')
    expect(session.start.surface).toBe('web')
    expect(session.continuity_summary).toBe('Morning overview continuity is available.')
    expect(session.allowed_actions).toEqual(['accept', 'choose', 'close'])
    expect(session.state.phase).toBe('morning_overview')
    expect(session.state.snapshot).toContain('Todoist backlog')
    expect(session.outcome?.phase).toBe('morning_overview')
    expect(session.outcome?.signals[0]?.kind).toBe('focus_intent')
  })

  it('decodes standup daily-loop sessions with commitment and focus-block outcomes', () => {
    const session = decodeDailyLoopSessionData({
      id: 'dls_standup_1',
      session_date: '2026-03-19',
      phase: 'standup',
      status: 'completed',
      start: {
        source: 'manual',
        surface: 'web',
      },
      turn_state: 'completed',
      continuity_summary:
        'Standup continuity is available.',
      allowed_actions: ['accept', 'choose', 'close'],
      current_prompt: null,
      state: {
        phase: 'standup',
        commitments: [
          {
            title: 'Ship Phase 10',
            bucket: 'must',
            source_ref: 'todo_1',
          },
          {
            title: 'Review runtime PR',
            bucket: 'should',
            source_ref: null,
          },
        ],
        deferred_tasks: [
          {
            title: 'Inbox cleanup',
            source_ref: null,
            reason: 'Not part of the top three commitments.',
          },
        ],
        confirmed_calendar: ['Design review at 10:00'],
        focus_blocks: [
          {
            label: 'Deep work',
            start_at: '2026-03-19T13:00:00Z',
            end_at: '2026-03-19T15:00:00Z',
            reason: 'Protect implementation time.',
          },
        ],
      },
      outcome: {
        phase: 'standup',
        commitments: [
          {
            title: 'Ship Phase 10',
            bucket: 'must',
            source_ref: 'todo_1',
          },
        ],
        deferred_tasks: [],
        confirmed_calendar: ['Design review at 10:00'],
        focus_blocks: [],
      },
    })

    expect(session.phase).toBe('standup')
    expect(session.continuity_summary).toBe('Standup continuity is available.')
    expect(session.allowed_actions).toEqual(['accept', 'choose', 'close'])
    expect(session.state.phase).toBe('standup')
    expect(session.state.commitments).toHaveLength(2)
    expect(session.state.commitments[0]?.bucket).toBe('must')
    expect(session.state.focus_blocks[0]?.label).toBe('Deep work')
    expect(session.outcome?.phase).toBe('standup')
    expect(session.outcome?.commitments[0]?.title).toBe('Ship Phase 10')
  })

  it('treats omitted daily-loop outcome payloads as null', () => {
    const session = decodeDailyLoopSessionData({
      id: 'dls_pending_1',
      session_date: '2026-03-19',
      phase: 'morning_overview',
      status: 'waiting_for_input',
      start: {
        source: 'manual',
        surface: 'web',
      },
      turn_state: 'waiting_for_input',
      continuity_summary:
        'Morning overview is waiting on question 1 of 3 with 0 captured signal(s).',
      allowed_actions: ['accept', 'defer', 'choose', 'close'],
      current_prompt: {
        prompt_id: 'prompt_pending_1',
        kind: 'intent_question',
        text: 'What most needs to happen before noon?',
        ordinal: 1,
        allow_skip: true,
      },
      state: {
        phase: 'morning_overview',
        snapshot: 'Two meetings before noon.',
        friction_callouts: [],
        signals: [],
      },
    })

    expect(session.outcome).toBeNull()
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
        lan_base_url_auto_discovered: true,
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
        backup: {
          default_output_root: 'var/backups',
          trust: {
            level: 'warn',
            status: {
              state: 'stale',
              last_backup_id: 'bkp_123',
              last_backup_at: '2026-03-18T18:20:00Z',
              output_root: '/tmp/backups/bkp_123',
              artifact_coverage: {
                included: ['artifacts/captures'],
                omitted: ['artifacts/cache'],
                notes: ['Transient cache directories are excluded.'],
              },
              config_coverage: {
                included: ['config/public-settings.json', 'config/runtime-config.json'],
                omitted: ['integration_google_calendar_secrets'],
                notes: ['Secret-bearing settings are omitted.'],
              },
              verification_summary: {
                verified: true,
                checksum_algorithm: 'sha256',
                checksum: 'abc123',
                checked_paths: ['/tmp/backups/bkp_123/manifest.json'],
                notes: ['Verified from the snapshot and manifest.'],
              },
              warnings: ['last successful backup is stale'],
            },
            freshness: {
              state: 'stale',
              age_seconds: 200000,
              stale_after_seconds: 172800,
            },
            guidance: ['Create or verify a fresh backup before risky maintenance.'],
          },
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
      lan_base_url_auto_discovered: true,
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
      backup: {
        default_output_root: 'var/backups',
        trust: {
          level: 'warn',
          status: {
            state: 'stale',
            last_backup_id: 'bkp_123',
            last_backup_at: '2026-03-18T18:20:00Z',
            output_root: '/tmp/backups/bkp_123',
            artifact_coverage: {
              included: ['artifacts/captures'],
              omitted: ['artifacts/cache'],
              notes: ['Transient cache directories are excluded.'],
            },
            config_coverage: {
              included: ['config/public-settings.json', 'config/runtime-config.json'],
              omitted: ['integration_google_calendar_secrets'],
              notes: ['Secret-bearing settings are omitted.'],
            },
            verification_summary: {
              verified: true,
              checksum_algorithm: 'sha256',
              checksum: 'abc123',
              checked_paths: ['/tmp/backups/bkp_123/manifest.json'],
              notes: ['Verified from the snapshot and manifest.'],
            },
            warnings: ['last successful backup is stale'],
          },
          freshness: {
            state: 'stale',
            age_seconds: 200000,
            stale_after_seconds: 172800,
          },
          guidance: ['Create or verify a fresh backup before risky maintenance.'],
        },
      },
    })
  })

  it('decodes legacy backup coverage arrays in settings payloads', () => {
    expect(
      decodeSettingsData({
        backup: {
          default_output_root: 'var/backups',
          trust: {
            level: 'warn',
            status: {
              state: 'stale',
              last_backup_id: 'bkp_123',
              last_backup_at: '2026-03-18T18:20:00Z',
              output_root: '/tmp/backups/bkp_123',
              artifact_coverage: ['artifacts/captures', 'artifacts/threads'],
              config_coverage: ['config/runtime-config.json'],
              verification_summary: null,
              warnings: [],
            },
            freshness: {
              state: 'stale',
              age_seconds: 200000,
              stale_after_seconds: 172800,
            },
            guidance: [],
          },
        },
      }),
    ).toEqual({
      backup: {
        default_output_root: 'var/backups',
        trust: {
          level: 'warn',
          status: {
            state: 'stale',
            last_backup_id: 'bkp_123',
            last_backup_at: '2026-03-18T18:20:00Z',
            output_root: '/tmp/backups/bkp_123',
            artifact_coverage: {
              included: ['artifacts/captures', 'artifacts/threads'],
              omitted: [],
              notes: [],
            },
            config_coverage: {
              included: ['config/runtime-config.json'],
              omitted: [],
              notes: [],
            },
            verification_summary: null,
            warnings: [],
          },
          freshness: {
            state: 'stale',
            age_seconds: 200000,
            stale_after_seconds: 172800,
          },
          guidance: [],
        },
      },
    })
  })

  it('decodes legacy backup coverage strings in settings payloads', () => {
    expect(
      decodeSettingsData({
        backup: {
          default_output_root: 'var/backups',
          trust: {
            level: 'warn',
            status: {
              state: 'stale',
              last_backup_id: 'bkp_123',
              last_backup_at: '2026-03-18T18:20:00Z',
              output_root: '/tmp/backups/bkp_123',
              artifact_coverage: 'legacy summary only',
              config_coverage: 0,
              verification_summary: null,
              warnings: [],
            },
            freshness: {
              state: 'stale',
              age_seconds: 200000,
              stale_after_seconds: 172800,
            },
            guidance: [],
          },
        },
      }),
    ).toEqual({
      backup: {
        default_output_root: 'var/backups',
        trust: {
          level: 'warn',
          status: {
            state: 'stale',
            last_backup_id: 'bkp_123',
            last_backup_at: '2026-03-18T18:20:00Z',
            output_root: '/tmp/backups/bkp_123',
            artifact_coverage: {
              included: [],
              omitted: [],
              notes: ['legacy summary only'],
            },
            config_coverage: {
              included: [],
              omitted: [],
              notes: [],
            },
            verification_summary: null,
            warnings: [],
          },
          freshness: {
            state: 'stale',
            age_seconds: 200000,
            stale_after_seconds: 172800,
          },
          guidance: [],
        },
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
    const now = decodeNowData({
        computed_at: 1710000000,
        timezone: 'America/Denver',
        header: {
          title: "Jove's Now",
          buckets: [
            {
              kind: 'needs_input',
              count: 2,
              count_display: 'show_nonzero',
              urgent: true,
              route_target: {
                bucket: 'needs_input',
                thread_id: null,
              },
            },
          ],
        },
        mesh_summary: {
          authority_node_id: 'vel-desktop',
          authority_label: 'Vel Desktop',
          sync_state: 'stale',
          linked_node_count: 2,
          queued_write_count: 1,
          last_sync_at: 1710000000,
          urgent: true,
          repair_route: {
            target: 'settings_recovery',
            summary:
              'Sync or queued-write posture needs review before trusting all cross-client state.',
          },
        },
        status_row: {
          date_label: 'Thu Mar 20',
          time_label: '9:15 AM',
          context_label: 'Standup',
          elapsed_label: '12m',
        },
        context_line: {
          text: 'Standup is active and one supervised review is still pending.',
          thread_id: 'thr_day_2026_03_20',
          fallback_used: false,
        },
        nudge_bars: [
          {
            id: 'nudge_1',
            kind: 'needs_input',
            title: 'One answer is still needed',
            summary: 'Standup is waiting on the next short answer.',
            urgent: true,
            primary_thread_id: 'thr_check_in_dls_1_standup_prompt_1',
            actions: [
              {
                kind: 'accept',
                label: 'Continue standup',
              },
              {
                kind: 'snooze',
                label: 'Snooze',
              },
            ],
          },
        ],
        task_lane: {
          active: {
            id: 'task_active_1',
            task_kind: 'commitment',
            text: 'Standup check-in',
            state: 'active',
            project: 'Vel',
            primary_thread_id: 'thr_day_2026_03_20',
          },
          pending: [
            {
              id: 'task_pending_1',
              task_kind: 'task',
              text: 'Review operator queue',
              state: 'pending',
              project: 'Vel',
              primary_thread_id: null,
            },
          ],
          recent_completed: [
            {
              id: 'task_done_1',
              task_kind: 'task',
              text: 'Check calendar drift',
              state: 'completed',
              project: null,
              primary_thread_id: null,
            },
          ],
          overflow_count: 3,
        },
        docked_input: {
          supported_intents: [
            'task',
            'question',
            'note',
            'command',
            'continuation',
            'reflection',
            'scheduling',
          ],
          day_thread_id: 'thr_day_2026_03_20',
          raw_capture_thread_id: 'thr_capture_1',
        },
        overview: {
          dominant_action: {
            kind: 'check_in',
            title: 'Standup check-in',
            summary: 'Name the one to three commitments that matter most today.',
            reference_id: 'act_check_in_1',
          },
          today_timeline: [
            {
              kind: 'now',
              title: 'Current time',
              timestamp: 1710000000,
              detail: null,
            },
            {
              kind: 'calendar_event',
              title: 'Standup',
              timestamp: 1710000300,
              detail: 'Desk',
            },
          ],
          visible_nudge: {
            kind: 'freshness',
            title: 'Review operator queue',
            summary: 'One supervised review is still pending.',
          },
          why_state: [
            { label: 'Mode', detail: 'Day' },
            { label: 'Attention', detail: 'On task' },
          ],
          suggestions: [],
          decision_options: ['accept', 'choose', 'thread', 'close'],
        },
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
        trust_readiness: {
          level: 'warn',
          headline: 'Review is pending',
          summary: '1 conflict(s) and 1 supervised review(s) still need operator attention.',
          backup: {
            level: 'ok',
            label: 'Backup',
            detail: 'Backup trust is healthy.',
          },
          freshness: {
            level: 'ok',
            label: 'Freshness',
            detail: 'Current context and integrations look fresh enough to trust.',
          },
          review: {
            open_action_count: 1,
            pending_execution_reviews: 1,
            pending_writeback_count: 1,
            conflict_count: 1,
          },
          guidance: [
            'Backup trust is healthy.',
            'Review the remaining conflicts or supervised execution handoffs before risky actions.',
          ],
          follow_through: [
            {
              id: 'act_recovery_backup',
              surface: 'inbox',
              kind: 'recovery',
              permission_mode: 'user_confirm',
              scope_affinity: 'global',
              title: 'Backup is stale',
              summary:
                'Backup trust is degraded. Create or verify a fresh backup before risky maintenance.',
              project_id: null,
              project_label: null,
              project_family: null,
              state: 'active',
              rank: 88,
              surfaced_at: '2023-11-14T22:13:20Z',
              snoozed_until: null,
              thread_route: null,
              evidence: [
                {
                  source_kind: 'backup_trust',
                  source_id: 'warn',
                  label: 'Backup trust',
                  detail:
                    'Backup trust is degraded. Create or verify a fresh backup before risky maintenance.',
                },
              ],
            },
          ],
        },
        commitment_scheduling_summary: {
          pending_count: 1,
          latest_pending: {
            thread_id: 'thr_day_plan_apply_1',
            state: 'staged',
            title: 'Apply focus block shift',
            summary: 'Move the focus block after the calendar anchor.',
            outcome_summary: null,
            updated_at: 1710000000,
          },
          latest_applied: {
            thread_id: 'thr_reflow_edit_0',
            state: 'applied',
            title: 'Clear stale due time',
            summary: 'Remove the stale due time from one commitment.',
            outcome_summary:
              'Commitment scheduling proposal applied through canonical mutation seam.',
            updated_at: 1709990000,
          },
          latest_failed: null,
        },
        check_in: {
          id: 'act_check_in_1',
          source_kind: 'daily_loop',
          phase: 'standup',
          session_id: 'dls_1',
          title: 'Standup check-in',
          summary: 'Vel needs one short answer before the standup can continue.',
          prompt_id: 'standup_prompt_1',
          prompt_text: 'Name the one to three commitments that matter most today.',
          suggested_action_label: 'Continue standup',
          suggested_response: null,
          allow_skip: true,
          blocking: true,
          submit_target: {
            kind: 'daily_loop_turn',
            reference_id: 'dls_1',
          },
          escalation: {
            target: 'threads',
            label: 'Continue in Threads',
            thread_id: 'thr_check_in_dls_1_standup_prompt_1',
          },
          transitions: [
            {
              kind: 'submit',
              label: 'Continue standup',
              target: 'daily_loop_turn',
              reference_id: 'dls_1',
              requires_response: true,
              requires_note: false,
            },
            {
              kind: 'bypass',
              label: 'Skip for now',
              target: 'daily_loop_turn',
              reference_id: 'dls_1',
              requires_response: false,
              requires_note: true,
            },
            {
              kind: 'escalate',
              label: 'Continue in Threads',
              target: 'threads',
              reference_id: 'dls_1',
              requires_response: false,
              requires_note: false,
            },
          ],
        },
        reflow: {
          id: 'act_reflow_1',
          title: 'Day changed',
          summary:
            'A scheduled event appears to have slipped past without the plan being updated.',
          trigger: 'missed_event',
          severity: 'critical',
          accept_mode: 'confirm_required',
          suggested_action_label: 'Accept',
          preview_lines: [
            'Next scheduled event started 20 minutes ago.',
            'Leave-by threshold passed 10 minutes ago.',
          ],
          edit_target: {
            target: 'threads',
            label: 'Edit',
          },
          proposal: {
            headline: 'Remaining day needs repair',
            summary:
              'Vel can now carry a typed remaining-day recovery proposal over the reflow seam before full schedule recomputation lands.',
            moved_count: 0,
            unscheduled_count: 0,
            needs_judgment_count: 1,
            changes: [
              {
                kind: 'needs_judgment',
                title: 'Scheduled time already passed',
                detail: 'Next scheduled event started 20 minutes ago.',
                project_label: null,
                scheduled_start_ts: 1700000000,
              },
            ],
            rule_facets: [
              {
                kind: 'fixed_start',
                label: 'Fixed start',
                detail:
                  'A due datetime or schedule anchor should stay explicit in the recomputed day.',
              },
            ],
          },
          transitions: [
            {
              kind: 'accept',
              label: 'Accept',
              target: 'apply_suggestion',
              confirm_required: true,
            },
            {
              kind: 'edit',
              label: 'Edit',
              target: 'threads',
              confirm_required: false,
            },
          ],
        },
        reflow_status: {
          kind: 'editing',
          trigger: 'missed_event',
          severity: 'critical',
          headline: 'Reflow moved to Threads',
          detail:
            'Vel opened a thread-backed reflow follow-up so the day plan can be shaped before anything else changes.',
          recorded_at: 1700000300,
          preview_lines: ['Next scheduled event started 20 minutes ago.'],
          thread_id: 'thr_reflow_1',
        },
        action_items: [
          {
            id: 'act_1',
            surface: 'inbox',
            kind: 'intervention',
            permission_mode: 'user_confirm',
            scope_affinity: 'project',
            title: 'Inbox intervention',
            summary: 'Review operator intervention',
            project_id: 'proj_1',
            project_label: 'Vel',
            project_family: 'work',
            state: 'active',
            rank: 80,
            surfaced_at: '2026-03-19T02:10:00Z',
            snoozed_until: null,
            thread_route: null,
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
          pending_execution_reviews: 1,
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

    expect(now).toEqual({
      computed_at: 1710000000,
      timezone: 'America/Denver',
      header: {
        title: "Jove's Now",
        buckets: [
          {
            kind: 'needs_input',
            count: 2,
            count_display: 'show_nonzero',
            urgent: true,
            route_target: {
              bucket: 'needs_input',
              thread_id: null,
            },
          },
        ],
      },
      mesh_summary: {
        authority_node_id: 'vel-desktop',
        authority_label: 'Vel Desktop',
        sync_state: 'stale',
        linked_node_count: 2,
        queued_write_count: 1,
        last_sync_at: 1710000000,
        urgent: true,
        repair_route: {
          target: 'settings_recovery',
          summary:
            'Sync or queued-write posture needs review before trusting all cross-client state.',
        },
      },
      status_row: {
        date_label: 'Thu Mar 20',
        time_label: '9:15 AM',
        context_label: 'Standup',
        elapsed_label: '12m',
      },
      context_line: {
        text: 'Standup is active and one supervised review is still pending.',
        thread_id: 'thr_day_2026_03_20',
        fallback_used: false,
      },
      nudge_bars: [
        {
          id: 'nudge_1',
          kind: 'needs_input',
          title: 'One answer is still needed',
          summary: 'Standup is waiting on the next short answer.',
          urgent: true,
          primary_thread_id: 'thr_check_in_dls_1_standup_prompt_1',
          actions: [
            {
              kind: 'accept',
              label: 'Continue standup',
            },
            {
              kind: 'snooze',
              label: 'Snooze',
            },
          ],
        },
      ],
      task_lane: {
        active: {
          id: 'task_active_1',
          task_kind: 'commitment',
          text: 'Standup check-in',
          state: 'active',
          project: 'Vel',
          primary_thread_id: 'thr_day_2026_03_20',
        },
        pending: [
          {
            id: 'task_pending_1',
            task_kind: 'task',
            text: 'Review operator queue',
            state: 'pending',
            project: 'Vel',
            primary_thread_id: null,
          },
        ],
        recent_completed: [
          {
            id: 'task_done_1',
            task_kind: 'task',
            text: 'Check calendar drift',
            state: 'completed',
            project: null,
            primary_thread_id: null,
          },
        ],
        overflow_count: 3,
      },
      docked_input: {
        supported_intents: [
          'task',
          'question',
          'note',
          'command',
          'continuation',
          'reflection',
          'scheduling',
        ],
        day_thread_id: 'thr_day_2026_03_20',
        raw_capture_thread_id: 'thr_capture_1',
      },
      overview: {
        dominant_action: {
          kind: 'check_in',
          title: 'Standup check-in',
          summary: 'Name the one to three commitments that matter most today.',
          reference_id: 'act_check_in_1',
        },
        today_timeline: [
          {
            kind: 'now',
            title: 'Current time',
            timestamp: 1710000000,
            detail: null,
          },
          {
            kind: 'calendar_event',
            title: 'Standup',
            timestamp: 1710000300,
            detail: 'Desk',
          },
        ],
        visible_nudge: {
          kind: 'freshness',
          title: 'Review operator queue',
          summary: 'One supervised review is still pending.',
        },
        why_state: [
          { label: 'Mode', detail: 'Day' },
          { label: 'Attention', detail: 'On task' },
        ],
        suggestions: [],
        decision_options: ['accept', 'choose', 'thread', 'close'],
      },
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
      trust_readiness: {
        level: 'warn',
        headline: 'Review is pending',
        summary: '1 conflict(s) and 1 supervised review(s) still need operator attention.',
        backup: {
          level: 'ok',
          label: 'Backup',
          detail: 'Backup trust is healthy.',
        },
        freshness: {
          level: 'ok',
          label: 'Freshness',
          detail: 'Current context and integrations look fresh enough to trust.',
        },
        review: {
          open_action_count: 1,
          pending_execution_reviews: 1,
          pending_writeback_count: 1,
          conflict_count: 1,
        },
        guidance: [
          'Backup trust is healthy.',
          'Review the remaining conflicts or supervised execution handoffs before risky actions.',
        ],
        follow_through: [
          {
            id: 'act_recovery_backup',
            surface: 'inbox',
            kind: 'recovery',
            permission_mode: 'user_confirm',
            scope_affinity: 'global',
            title: 'Backup is stale',
            summary:
              'Backup trust is degraded. Create or verify a fresh backup before risky maintenance.',
            project_id: null,
            project_label: null,
            project_family: null,
            state: 'active',
            rank: 88,
            surfaced_at: '2023-11-14T22:13:20Z',
            snoozed_until: null,
            thread_route: null,
            evidence: [
              {
                source_kind: 'backup_trust',
                source_id: 'warn',
                label: 'Backup trust',
                detail:
                  'Backup trust is degraded. Create or verify a fresh backup before risky maintenance.',
              },
            ],
          },
        ],
      },
      commitment_scheduling_summary: {
        pending_count: 1,
        latest_pending: {
          thread_id: 'thr_day_plan_apply_1',
          state: 'staged',
          title: 'Apply focus block shift',
          summary: 'Move the focus block after the calendar anchor.',
          outcome_summary: null,
          updated_at: 1710000000,
        },
        latest_applied: {
          thread_id: 'thr_reflow_edit_0',
          state: 'applied',
          title: 'Clear stale due time',
          summary: 'Remove the stale due time from one commitment.',
          outcome_summary:
            'Commitment scheduling proposal applied through canonical mutation seam.',
          updated_at: 1709990000,
        },
        latest_failed: null,
      },
      check_in: {
        id: 'act_check_in_1',
        source_kind: 'daily_loop',
        phase: 'standup',
        session_id: 'dls_1',
        title: 'Standup check-in',
        summary: 'Vel needs one short answer before the standup can continue.',
        prompt_id: 'standup_prompt_1',
        prompt_text: 'Name the one to three commitments that matter most today.',
        suggested_action_label: 'Continue standup',
        suggested_response: null,
        allow_skip: true,
        blocking: true,
        submit_target: {
          kind: 'daily_loop_turn',
          reference_id: 'dls_1',
        },
        escalation: {
          target: 'threads',
          label: 'Continue in Threads',
          thread_id: 'thr_check_in_dls_1_standup_prompt_1',
        },
        transitions: [
          {
            kind: 'submit',
            label: 'Continue standup',
            target: 'daily_loop_turn',
            reference_id: 'dls_1',
            requires_response: true,
            requires_note: false,
          },
          {
            kind: 'bypass',
            label: 'Skip for now',
            target: 'daily_loop_turn',
            reference_id: 'dls_1',
            requires_response: false,
            requires_note: true,
          },
          {
            kind: 'escalate',
            label: 'Continue in Threads',
            target: 'threads',
            reference_id: 'dls_1',
            requires_response: false,
            requires_note: false,
          },
        ],
      },
      day_plan: null,
      reflow: {
        id: 'act_reflow_1',
        title: 'Day changed',
        summary:
          'A scheduled event appears to have slipped past without the plan being updated.',
        trigger: 'missed_event',
        severity: 'critical',
        accept_mode: 'confirm_required',
        suggested_action_label: 'Accept',
        preview_lines: [
          'Next scheduled event started 20 minutes ago.',
          'Leave-by threshold passed 10 minutes ago.',
        ],
        edit_target: {
          target: 'threads',
          label: 'Edit',
        },
        proposal: {
          headline: 'Remaining day needs repair',
          summary:
            'Vel can now carry a typed remaining-day recovery proposal over the reflow seam before full schedule recomputation lands.',
          moved_count: 0,
          unscheduled_count: 0,
          needs_judgment_count: 1,
          changes: [
            {
              kind: 'needs_judgment',
              title: 'Scheduled time already passed',
              detail: 'Next scheduled event started 20 minutes ago.',
              project_label: null,
              scheduled_start_ts: 1700000000,
            },
          ],
          rule_facets: [
            {
              kind: 'fixed_start',
              label: 'Fixed start',
              detail:
                'A due datetime or schedule anchor should stay explicit in the recomputed day.',
            },
          ],
        },
        transitions: [
          {
            kind: 'accept',
            label: 'Accept',
            target: 'apply_suggestion',
            confirm_required: true,
          },
          {
            kind: 'edit',
            label: 'Edit',
            target: 'threads',
            confirm_required: false,
          },
        ],
      },
      reflow_status: {
        kind: 'editing',
        trigger: 'missed_event',
        severity: 'critical',
        headline: 'Reflow moved to Threads',
        detail:
          'Vel opened a thread-backed reflow follow-up so the day plan can be shaped before anything else changes.',
        recorded_at: 1700000300,
        preview_lines: ['Next scheduled event started 20 minutes ago.'],
        thread_id: 'thr_reflow_1',
      },
      action_items: [
        {
          id: 'act_1',
          surface: 'inbox',
          kind: 'intervention',
          permission_mode: 'user_confirm',
          scope_affinity: 'project',
          title: 'Inbox intervention',
          summary: 'Review operator intervention',
          project_id: 'proj_1',
          project_label: 'Vel',
          project_family: 'work',
          state: 'active',
          rank: 80,
          surfaced_at: '2026-03-19T02:10:00Z',
          snoozed_until: null,
          thread_route: null,
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
        pending_execution_reviews: 1,
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
    expect(now.reflow?.proposal?.needs_judgment_count).toBe(1)
    expect(now.reflow?.transitions.map((transition) => transition.target)).toEqual([
      'apply_suggestion',
      'threads',
    ])
    expect(now.reflow?.transitions[0]?.confirm_required).toBe(true)
    expect(now.reflow_status?.thread_id).toBe('thr_reflow_1')
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
        pending_execution_reviews: 0,
      }),
    ).toEqual({
      open_action_count: 4,
      triage_count: 2,
      projects_needing_review: 1,
      pending_execution_reviews: 0,
    })

    expect(
      decodeActionItemData({
        id: 'act_review',
        surface: 'now',
        kind: 'review',
        permission_mode: 'user_confirm',
        scope_affinity: 'project',
        title: 'Review project Vel',
        summary: 'Weekly review keeps the project anchored.',
        project_id: 'proj_1',
        project_label: 'Vel',
        project_family: 'work',
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
      permission_mode: 'user_confirm',
      scope_affinity: 'project',
      title: 'Review project Vel',
      summary: 'Weekly review keeps the project anchored.',
      project_id: 'proj_1',
      project_label: 'Vel',
      project_family: 'work',
      state: 'active',
      rank: 60,
      surfaced_at: '2026-03-19T02:11:00Z',
      snoozed_until: null,
      thread_route: null,
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
              permission_mode: 'user_confirm',
              scope_affinity: 'global',
              title: 'Linked node Remote needs review',
              summary: 'Review linked node trust',
              project_id: null,
              project_label: null,
              project_family: null,
              state: 'active',
              rank: 85,
              surfaced_at: '2026-03-19T02:12:00Z',
              snoozed_until: null,
              thread_route: null,
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
            permission_mode: 'user_confirm',
            scope_affinity: 'global',
            title: 'Linked node Remote needs review',
            summary: 'Review linked node trust',
            project_id: null,
            project_label: null,
            project_family: null,
            state: 'active',
            rank: 85,
            surfaced_at: '2026-03-19T02:12:00Z',
            snoozed_until: null,
            thread_route: null,
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
            issuer_sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            issuer_sync_transport: 'tailscale',
            issuer_tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            issuer_lan_base_url: 'http://192.168.1.50:4130',
            issuer_localhost_base_url: 'http://127.0.0.1:4130',
            issuer_public_base_url: null,
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
            issuer_sync_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            issuer_sync_transport: 'tailscale',
            issuer_tailscale_base_url: 'http://vel-desktop.tailnet.ts.net:4130',
            issuer_lan_base_url: 'http://192.168.1.50:4130',
            issuer_localhost_base_url: 'http://127.0.0.1:4130',
            issuer_public_base_url: null,
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
