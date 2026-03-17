use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, CommandDelegationHintsData, CommandExecuteRequest, CommandExecutionPayloadData,
    CommandExecutionPlanData, CommandExecutionResultData, CommandIntentHintsData,
    CommandPlanModeData, CommandPlanRequest, CommandPlanStepData, CommandPlannedLinkData,
    CommandPlannedRecordData, CommandValidationData, CommandValidationIssueCodeData,
    CommandValidationIssueData,
};

use crate::{errors::AppError, services, state::AppState};

pub async fn plan_command(
    State(_state): State<AppState>,
    Json(body): Json<CommandPlanRequest>,
) -> Result<Json<ApiResponse<CommandExecutionPlanData>>, AppError> {
    let plan = services::command_lang::build_execution_plan(&body.command);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(plan_to_data(plan), request_id)))
}

pub async fn execute_command(
    State(state): State<AppState>,
    Json(body): Json<CommandExecuteRequest>,
) -> Result<Json<ApiResponse<CommandExecutionResultData>>, AppError> {
    let result = services::command_lang::execute_command(&state, &body.command).await?;
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
            CommandExecutionPayloadData::CaptureCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::CommitmentCreated(payload) => {
            CommandExecutionPayloadData::CommitmentCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::ArtifactCreated(payload) => {
            CommandExecutionPayloadData::ArtifactCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::SpecDraftCreated(payload) => {
            CommandExecutionPayloadData::SpecDraftCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::ExecutionPlanCreated(payload) => {
            CommandExecutionPayloadData::ExecutionPlanCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::DelegationPlanCreated(payload) => {
            CommandExecutionPayloadData::DelegationPlanCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::SynthesisCreated(payload) => {
            CommandExecutionPayloadData::SynthesisCreated(payload)
        }
        services::command_lang::CommandExecutionPayload::ContextExplained(payload) => {
            CommandExecutionPayloadData::ContextExplained(payload)
        }
        services::command_lang::CommandExecutionPayload::CommitmentExplained(payload) => {
            CommandExecutionPayloadData::CommitmentExplained(payload)
        }
        services::command_lang::CommandExecutionPayload::DriftExplained(payload) => {
            CommandExecutionPayloadData::DriftExplained(payload)
        }
        services::command_lang::CommandExecutionPayload::ReviewToday(payload) => {
            CommandExecutionPayloadData::ReviewToday(payload)
        }
        services::command_lang::CommandExecutionPayload::ReviewWeek(payload) => {
            CommandExecutionPayloadData::ReviewWeek(payload)
        }
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
