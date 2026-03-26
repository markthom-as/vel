use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    AdaptivePolicyOverrideData, ApiResponse, ArtifactData, CaptureCreateResponse,
    CommandCompleteRequest, CommandCompleteResponseData, CommandDelegationHintsData,
    CommandExecuteRequest, CommandExecutionPayloadData, CommandExecutionPlanData,
    CommandExecutionResultData, CommandIntentHintsData, CommandParsedData, CommandPlanModeData,
    CommandPlanRequest, CommandPlanStepData, CommandPlannedLinkData, CommandPlannedRecordData,
    CommandRegistryEntryData, CommandReviewSummaryData, CommandValidationData,
    CommandValidationIssueCodeData, CommandValidationIssueData, CommitmentData,
    CommitmentExplainData, ContextCapture, ContextExplainData, ContextSourceSummariesData,
    ContextSourceSummaryData, DriftExplainData, PlanningArtifactCreatedData, SignalExplainSummary,
    SynthesisWeekData, ThreadData,
};
use vel_command_lang::{completion, explain, infer, preview, registry, shell};

use crate::{errors::AppError, services, state::AppState};

fn shell_completion_to_data(completion: shell::ShellCompletion) -> CommandCompleteResponseData {
    CommandCompleteResponseData {
        input: completion.input,
        completion_hints: completion.completion_hints,
        registry: completion
            .registry
            .into_iter()
            .map(|entry| CommandRegistryEntryData {
                kind: entry.kind,
                aliases: entry.aliases,
                selectors: entry.selectors,
                operations: entry.operations,
            })
            .collect(),
        parsed: completion.parsed.map(|parsed| CommandParsedData {
            family: parsed.family,
            verb: parsed.verb,
            target_tokens: parsed.target_tokens,
            source_text: parsed.source_text,
        }),
        resolved_command: None,
        local_preview: completion.local_preview,
        local_explanation: completion.local_explanation,
        intent_hints: completion.intent_hints.map(|hints| CommandIntentHintsData {
            target_kind: hints.target_kind,
            mode: hints.mode,
            suggestions: hints.suggestions,
        }),
        parse_error: completion.parse_error,
    }
}

pub async fn complete_command(
    Json(body): Json<CommandCompleteRequest>,
) -> Result<Json<ApiResponse<CommandCompleteResponseData>>, AppError> {
    if let Some(text) = body.text.as_deref() {
        if let Some(completion) = shell::shell_completion_for_text(text) {
            let request_id = format!("req_{}", Uuid::new_v4().simple());
            return Ok(Json(ApiResponse::success(
                shell_completion_to_data(completion),
                request_id,
            )));
        }
    }

    let input = body
        .text
        .as_deref()
        .and_then(shell::parse_explicit_command_input)
        .map(|parsed| parsed.tokens)
        .unwrap_or(body.input);

    if let Some(completion) = shell::shell_completion(&input) {
        let request_id = format!("req_{}", Uuid::new_v4().simple());
        return Ok(Json(ApiResponse::success(
            shell_completion_to_data(completion),
            request_id,
        )));
    }

    let completion_hints = completion::next_tokens(&input)
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let registry = registry::default_registry()
        .into_iter()
        .map(|entry| CommandRegistryEntryData {
            kind: entry.kind.to_string(),
            aliases: entry
                .aliases
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            selectors: entry
                .selectors
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            operations: entry
                .operations
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        })
        .collect();

    let data = match infer::parse_and_resolve(&input) {
        Ok(resolution) => CommandCompleteResponseData {
            input,
            completion_hints,
            registry,
            parsed: Some(CommandParsedData {
                family: format!("{:?}", resolution.parsed.family).to_ascii_lowercase(),
                verb: resolution.parsed.verb.to_string(),
                target_tokens: resolution.parsed.target_tokens.clone(),
                source_text: resolution.parsed.source_text.clone(),
            }),
            resolved_command: Some(resolution.resolved.clone()),
            local_preview: Some(preview::render(&resolution)),
            local_explanation: Some(explain::render_explanation(&resolution)),
            intent_hints: completion::intent_hints(&resolution).map(|hints| {
                CommandIntentHintsData {
                    target_kind: hints.target_kind,
                    mode: hints.mode.to_string(),
                    suggestions: hints.suggestions.into_iter().map(str::to_string).collect(),
                }
            }),
            parse_error: None,
        },
        Err(error) => CommandCompleteResponseData {
            input,
            completion_hints,
            registry,
            parsed: None,
            resolved_command: None,
            local_preview: None,
            local_explanation: None,
            intent_hints: None,
            parse_error: Some(error.to_string()),
        },
    };

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn plan_command(
    State(state): State<AppState>,
    Json(body): Json<CommandPlanRequest>,
) -> Result<Json<ApiResponse<CommandExecutionPlanData>>, AppError> {
    let plan = services::command_lang::build_execution_plan(&body.command);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    let mut response = ApiResponse::success(plan_to_data(plan.clone()), request_id);
    if body.persist_preview {
        let run_id =
            services::command_lang::record_dry_run_preview(&state, &body.command, &plan).await?;
        response
            .warnings
            .push(format!("dry_run_preview_run_id={}", run_id.as_ref()));
    }
    Ok(Json(response))
}

pub async fn execute_command(
    State(state): State<AppState>,
    Json(body): Json<CommandExecuteRequest>,
) -> Result<Json<ApiResponse<CommandExecutionResultData>>, AppError> {
    let result = services::command_lang::execute_command_with_options(
        &state,
        &body.command,
        &services::command_lang::CommandExecutionOptions {
            dry_run: body.dry_run,
            approve: body.approve,
            idempotency_key: body.idempotency_key.clone(),
            write_scope: body.write_scope.clone(),
        },
    )
    .await?;
    let services::command_lang::CommandExecutionResult { result, warnings } = result;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CommandExecutionResultData {
            result: payload_to_data(result),
            warnings,
        },
        request_id,
    )))
}

fn payload_to_data(
    result: services::command_lang::CommandExecutionPayload,
) -> CommandExecutionPayloadData {
    match result {
        services::command_lang::CommandExecutionPayload::CaptureCreated(payload) => {
            CommandExecutionPayloadData::CaptureCreated(CaptureCreateResponse {
                capture_id: payload.capture_id,
                accepted_at: payload.accepted_at,
            })
        }
        services::command_lang::CommandExecutionPayload::CommitmentCreated(payload) => {
            let scheduler_rules = vel_core::CanonicalScheduleRules::from_commitment_parts(
                &payload.text,
                &payload.metadata,
                payload.due_at,
            );
            CommandExecutionPayloadData::CommitmentCreated(CommitmentData {
                id: payload.id,
                text: payload.text,
                source_type: payload.source_type,
                source_id: payload.source_id,
                status: payload.status,
                due_at: payload.due_at,
                project: payload.project,
                commitment_kind: payload.commitment_kind,
                created_at: payload.created_at,
                resolved_at: payload.resolved_at,
                scheduler_rules: scheduler_rules.into(),
                metadata: payload.metadata,
            })
        }
        services::command_lang::CommandExecutionPayload::ArtifactCreated(payload) => {
            CommandExecutionPayloadData::ArtifactCreated(map_artifact_to_data(payload))
        }
        services::command_lang::CommandExecutionPayload::SpecDraftCreated(payload) => {
            CommandExecutionPayloadData::SpecDraftCreated(map_planning_artifact_to_data(payload))
        }
        services::command_lang::CommandExecutionPayload::ExecutionPlanCreated(payload) => {
            CommandExecutionPayloadData::ExecutionPlanCreated(map_planning_artifact_to_data(
                payload,
            ))
        }
        services::command_lang::CommandExecutionPayload::DelegationPlanCreated(payload) => {
            CommandExecutionPayloadData::DelegationPlanCreated(map_planning_artifact_to_data(
                payload,
            ))
        }
        services::command_lang::CommandExecutionPayload::SynthesisCreated(payload) => {
            CommandExecutionPayloadData::SynthesisCreated(SynthesisWeekData {
                run_id: payload.run_id,
                artifact_id: payload.artifact_id,
            })
        }
        services::command_lang::CommandExecutionPayload::ContextExplained(payload) => {
            CommandExecutionPayloadData::ContextExplained(ContextExplainData {
                computed_at: payload.computed_at,
                mode: payload.mode,
                morning_state: payload.morning_state,
                context: payload.context,
                source_summaries: ContextSourceSummariesData {
                    git_activity: payload.source_summaries.git_activity.map(|summary| {
                        ContextSourceSummaryData {
                            timestamp: summary.timestamp,
                            summary: summary.summary,
                        }
                    }),
                    health: payload.source_summaries.health.map(|summary| {
                        ContextSourceSummaryData {
                            timestamp: summary.timestamp,
                            summary: summary.summary,
                        }
                    }),
                    mood: payload
                        .source_summaries
                        .mood
                        .map(|summary| ContextSourceSummaryData {
                            timestamp: summary.timestamp,
                            summary: summary.summary,
                        }),
                    pain: payload
                        .source_summaries
                        .pain
                        .map(|summary| ContextSourceSummaryData {
                            timestamp: summary.timestamp,
                            summary: summary.summary,
                        }),
                    note_document: payload.source_summaries.note_document.map(|summary| {
                        ContextSourceSummaryData {
                            timestamp: summary.timestamp,
                            summary: summary.summary,
                        }
                    }),
                    assistant_message: payload.source_summaries.assistant_message.map(|summary| {
                        ContextSourceSummaryData {
                            timestamp: summary.timestamp,
                            summary: summary.summary,
                        }
                    }),
                },
                adaptive_policy_overrides: payload
                    .adaptive_policy_overrides
                    .into_iter()
                    .map(|item| AdaptivePolicyOverrideData {
                        policy_key: item.policy_key,
                        value_minutes: item.value_minutes,
                        source_suggestion_id: item.source_suggestion_id,
                        source_title: item.source_title,
                        source_accepted_at: item.source_accepted_at,
                    })
                    .collect(),
                signals_used: payload.signals_used,
                signal_summaries: payload
                    .signal_summaries
                    .into_iter()
                    .map(|summary| SignalExplainSummary {
                        signal_id: summary.signal_id,
                        signal_type: summary.signal_type,
                        source: summary.source,
                        timestamp: summary.timestamp,
                        summary: summary.summary,
                    })
                    .collect(),
                commitments_used: payload.commitments_used,
                risk_used: payload.risk_used,
                reasons: payload.reasons,
            })
        }
        services::command_lang::CommandExecutionPayload::CommitmentExplained(payload) => {
            CommandExecutionPayloadData::CommitmentExplained(CommitmentExplainData {
                commitment_id: payload.commitment_id,
                commitment: payload.commitment,
                risk: payload.risk,
                in_context_reasons: payload.in_context_reasons,
            })
        }
        services::command_lang::CommandExecutionPayload::DriftExplained(payload) => {
            CommandExecutionPayloadData::DriftExplained(DriftExplainData {
                attention_state: payload.attention_state,
                drift_type: payload.drift_type,
                drift_severity: payload.drift_severity,
                confidence: payload.confidence,
                reasons: payload.reasons,
                signals_used: payload.signals_used,
                signal_summaries: payload
                    .signal_summaries
                    .into_iter()
                    .map(|summary| SignalExplainSummary {
                        signal_id: summary.signal_id,
                        signal_type: summary.signal_type,
                        source: summary.source,
                        timestamp: summary.timestamp,
                        summary: summary.summary,
                    })
                    .collect(),
                commitments_used: payload.commitments_used,
            })
        }
        services::command_lang::CommandExecutionPayload::ReviewToday(payload) => {
            CommandExecutionPayloadData::ReviewToday(map_review_summary_to_data(payload))
        }
        services::command_lang::CommandExecutionPayload::ReviewWeek(payload) => {
            CommandExecutionPayloadData::ReviewWeek(map_review_summary_to_data(payload))
        }
    }
}

fn map_artifact_to_data(payload: services::command_lang::ArtifactPayload) -> ArtifactData {
    ArtifactData {
        artifact_id: payload.artifact_id,
        artifact_type: payload.artifact_type,
        title: payload.title,
        mime_type: payload.mime_type,
        storage_uri: payload.storage_uri,
        storage_kind: payload.storage_kind,
        privacy_class: payload.privacy_class,
        sync_class: payload.sync_class,
        content_hash: payload.content_hash,
        size_bytes: payload.size_bytes,
        created_at: payload.created_at,
        updated_at: payload.updated_at,
    }
}

fn map_thread_to_data(payload: services::command_lang::ThreadPayload) -> ThreadData {
    ThreadData {
        id: payload.id,
        thread_type: payload.thread_type,
        title: payload.title,
        status: payload.status,
        planning_kind: payload.planning_kind,
        lifecycle_stage: payload.lifecycle_stage,
        created_at: payload.created_at,
        updated_at: payload.updated_at,
        continuation: None,
        metadata: None,
        links: payload.links.map(|links| {
            links
                .into_iter()
                .map(|link| vel_api_types::ThreadLinkData {
                    id: link.id,
                    entity_type: link.entity_type,
                    entity_id: link.entity_id,
                    relation_type: link.relation_type,
                })
                .collect()
        }),
        project_id: None,
        project_label: None,
    }
}

fn map_planning_artifact_to_data(
    payload: services::command_lang::PlanningArtifactCreatedPayload,
) -> PlanningArtifactCreatedData {
    PlanningArtifactCreatedData {
        artifact: map_artifact_to_data(payload.artifact),
        thread: map_thread_to_data(payload.thread),
    }
}

fn map_review_summary_to_data(
    payload: services::command_lang::CommandReviewSummaryPayload,
) -> CommandReviewSummaryData {
    CommandReviewSummaryData {
        captures: payload
            .captures
            .into_iter()
            .map(|capture| ContextCapture {
                capture_id: capture.capture_id,
                capture_type: capture.capture_type,
                content_text: capture.content_text,
                occurred_at: capture.occurred_at,
                source_device: capture.source_device,
            })
            .collect(),
        capture_count: payload.capture_count,
        latest_context_artifact: payload.latest_context_artifact.map(map_artifact_to_data),
    }
}

fn plan_to_data(plan: services::command_lang::CommandExecutionPlan) -> CommandExecutionPlanData {
    CommandExecutionPlanData {
        operation: plan.operation.to_string(),
        target_kinds: plan
            .target_kinds
            .into_iter()
            .map(|kind| kind.to_string())
            .collect(),
        mode: match plan.mode {
            services::command_lang::CommandPlanMode::Ready => CommandPlanModeData::Ready,
            services::command_lang::CommandPlanMode::DryRunOnly => CommandPlanModeData::DryRunOnly,
            services::command_lang::CommandPlanMode::Unsupported => {
                CommandPlanModeData::Unsupported
            }
        },
        summary: plan.summary,
        steps: plan
            .steps
            .into_iter()
            .map(|step| CommandPlanStepData {
                title: step.title,
                detail: step.detail,
            })
            .collect(),
        intent_hints: plan.intent_hints.map(|hints| CommandIntentHintsData {
            target_kind: hints.target_kind.to_string(),
            mode: hints.mode,
            suggestions: hints.suggestions,
        }),
        delegation_hints: plan
            .delegation_hints
            .map(|hints| CommandDelegationHintsData {
                worker_roles: hints.worker_roles,
                coordination: hints.coordination,
                approval_required: hints.approval_required,
                linked_record_strategy: hints.linked_record_strategy,
            }),
        planned_records: plan
            .planned_records
            .into_iter()
            .map(|record| CommandPlannedRecordData {
                record_type: record.record_type,
                title: record.title,
                links: record
                    .links
                    .into_iter()
                    .map(|link| CommandPlannedLinkData {
                        entity_type: link.entity_type,
                        relation_type: link.relation_type,
                    })
                    .collect(),
            })
            .collect(),
        validation: CommandValidationData {
            is_valid: plan.validation.is_valid,
            issues: plan
                .validation
                .issues
                .into_iter()
                .map(|issue| CommandValidationIssueData {
                    code: match issue.code {
                        services::command_lang::ValidationIssueCode::UnsupportedOperation => {
                            CommandValidationIssueCodeData::UnsupportedOperation
                        }
                        services::command_lang::ValidationIssueCode::MissingTargets => {
                            CommandValidationIssueCodeData::MissingTargets
                        }
                    },
                    message: issue.message,
                })
                .collect(),
        },
    }
}
