//! Command-language planning and execution service.
//!
//! This module validates `vel_core::ResolvedCommand`, builds dry-run plans, and
//! executes the low-risk command slice currently supported by the shared route.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;
use tracing::warn;
use uuid::Uuid;
use vel_api_types::{
    ArtifactData, CaptureCreateResponse, CommandReviewSummaryData, CommitmentData,
    PlanningArtifactCreatedData, ThreadData,
};
use vel_core::{
    ArtifactStorageKind, CommitmentStatus, DomainKind, DomainOperation, PrivacyClass,
    ResolvedCommand, SyncClass,
};
use vel_storage::{ArtifactInsert, ArtifactRecord, CaptureInsert, CommitmentInsert, SignalInsert};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandPlanMode {
    Ready,
    DryRunOnly,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationIssueCode {
    UnsupportedOperation,
    MissingTargets,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandValidationIssue {
    pub code: ValidationIssueCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CommandValidation {
    pub is_valid: bool,
    pub issues: Vec<CommandValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPlanStep {
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandIntentHints {
    pub target_kind: DomainKind,
    pub mode: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDelegationHints {
    pub worker_roles: Vec<String>,
    pub coordination: String,
    pub approval_required: bool,
    pub linked_record_strategy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPlannedLink {
    pub entity_type: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPlannedRecord {
    pub record_type: String,
    pub title: String,
    pub links: Vec<CommandPlannedLink>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExecutionPlan {
    pub operation: DomainOperation,
    pub target_kinds: Vec<DomainKind>,
    pub mode: CommandPlanMode,
    pub summary: String,
    pub steps: Vec<CommandPlanStep>,
    pub intent_hints: Option<CommandIntentHints>,
    pub delegation_hints: Option<CommandDelegationHints>,
    pub planned_records: Vec<CommandPlannedRecord>,
    pub validation: CommandValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionResult {
    pub result: CommandExecutionPayload,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "result_kind", content = "data", rename_all = "snake_case")]
pub enum CommandExecutionPayload {
    CaptureCreated(CaptureCreateResponse),
    CommitmentCreated(CommitmentData),
    ArtifactCreated(ArtifactData),
    SpecDraftCreated(PlanningArtifactCreatedData),
    ExecutionPlanCreated(PlanningArtifactCreatedData),
    DelegationPlanCreated(PlanningArtifactCreatedData),
    ReviewToday(CommandReviewSummaryData),
    ReviewWeek(CommandReviewSummaryData),
}

pub fn build_execution_plan(command: &ResolvedCommand) -> CommandExecutionPlan {
    let validation = validate_command(command);
    let mode = if !validation.is_valid {
        CommandPlanMode::Unsupported
    } else if is_dry_run_only(command.operation) {
        CommandPlanMode::DryRunOnly
    } else {
        CommandPlanMode::Ready
    };

    let target_kinds = command.targets.iter().map(|t| t.kind).collect::<Vec<_>>();
    let steps = build_plan_steps(command);
    let summary = build_summary(command, mode, target_kinds.len());
    let intent_hints = build_intent_hints(command);
    let delegation_hints = build_delegation_hints(command);
    let planned_records = build_planned_records(command);

    CommandExecutionPlan {
        operation: command.operation,
        target_kinds,
        mode,
        summary,
        steps,
        intent_hints,
        delegation_hints,
        planned_records,
        validation,
    }
}

pub async fn execute_command(
    state: &AppState,
    command: &ResolvedCommand,
) -> Result<CommandExecutionResult, AppError> {
    let validation = validate_command(command);
    if !validation.is_valid {
        let message = validation
            .issues
            .into_iter()
            .map(|issue| issue.message)
            .collect::<Vec<_>>()
            .join("; ");
        return Err(AppError::bad_request(message));
    }

    match (
        command.operation,
        command.targets.first().map(|target| target.kind),
    ) {
        (DomainOperation::Create, Some(DomainKind::Capture)) => {
            execute_create_capture(state, command).await
        }
        (DomainOperation::Create, Some(DomainKind::Commitment)) => {
            execute_create_commitment(state, command).await
        }
        (DomainOperation::Create, Some(DomainKind::SpecDraft)) => {
            execute_create_planning_artifact(
                state,
                command,
                "spec_draft",
                CommandExecutionPayloadKind::SpecDraftCreated,
            )
            .await
        }
        (DomainOperation::Create, Some(DomainKind::ExecutionPlan)) => {
            execute_create_planning_artifact(
                state,
                command,
                "execution_plan",
                CommandExecutionPayloadKind::ExecutionPlanCreated,
            )
            .await
        }
        (DomainOperation::Create, Some(DomainKind::DelegationPlan)) => {
            execute_create_planning_artifact(
                state,
                command,
                "delegation_plan",
                CommandExecutionPayloadKind::DelegationPlanCreated,
            )
            .await
        }
        (DomainOperation::Execute, Some(DomainKind::Context)) => {
            execute_review_context(state, command).await
        }
        _ => Err(AppError::bad_request(format!(
            "command execution is not supported for operation `{}` and target kind `{:?}`",
            command.operation,
            command.targets.first().map(|target| target.kind)
        ))),
    }
}

pub fn validate_command(command: &ResolvedCommand) -> CommandValidation {
    let mut issues = Vec::new();

    if !is_supported_operation(command.operation) {
        issues.push(CommandValidationIssue {
            code: ValidationIssueCode::UnsupportedOperation,
            message: format!(
                "operation `{}` is not supported by the command-language service scaffold",
                command.operation
            ),
        });
    }

    if requires_targets(command.operation) && command.targets.is_empty() {
        issues.push(CommandValidationIssue {
            code: ValidationIssueCode::MissingTargets,
            message: format!(
                "operation `{}` requires at least one target",
                command.operation
            ),
        });
    }

    CommandValidation {
        is_valid: issues.is_empty(),
        issues,
    }
}

fn build_plan_steps(command: &ResolvedCommand) -> Vec<CommandPlanStep> {
    let mut steps = vec![CommandPlanStep {
        title: "Validate command".to_string(),
        detail: format!("Check operation `{}` and target shape", command.operation),
    }];

    steps.push(CommandPlanStep {
        title: "Resolve target mapping".to_string(),
        detail: format!(
            "Resolve {} target(s) to service domain calls",
            command.targets.len()
        ),
    });

    if command.targets.first().map(|target| target.kind) == Some(DomainKind::DelegationPlan) {
        steps.push(CommandPlanStep {
            title: "Derive delegation structure".to_string(),
            detail: "Infer worker roles, ownership boundaries, and review gates".to_string(),
        });
    }

    if is_dry_run_only(command.operation) {
        steps.push(CommandPlanStep {
            title: "Dry-run summary only".to_string(),
            detail: "No side effects in scaffold mode".to_string(),
        });
    } else {
        steps.push(CommandPlanStep {
            title: "Execute via service adapter".to_string(),
            detail: "Hook to be implemented by future command route/CLI integration".to_string(),
        });
    }

    steps
}

fn build_summary(command: &ResolvedCommand, mode: CommandPlanMode, target_count: usize) -> String {
    format!(
        "operation={} targets={} mode={}",
        command.operation,
        target_count,
        match mode {
            CommandPlanMode::Ready => "ready",
            CommandPlanMode::DryRunOnly => "dry_run_only",
            CommandPlanMode::Unsupported => "unsupported",
        }
    )
}

fn build_intent_hints(command: &ResolvedCommand) -> Option<CommandIntentHints> {
    let target_kind = command.targets.first()?.kind;
    let mode = match target_kind {
        DomainKind::Context => "execute",
        DomainKind::SpecDraft | DomainKind::ExecutionPlan | DomainKind::DelegationPlan => {
            "planning_artifact"
        }
        _ => "create",
    };
    let suggestions = match target_kind {
        DomainKind::Capture => vec!["quick capture", "feature request", "inbox note"],
        DomainKind::Commitment => vec!["open commitment", "project link", "due date"],
        DomainKind::Context => vec!["today review", "week review", "read only"],
        DomainKind::SpecDraft => vec!["planned doc", "suggested path", "design constraints"],
        DomainKind::ExecutionPlan => vec!["task breakdown", "ordered steps", "planning only"],
        DomainKind::DelegationPlan => vec!["worker split", "ownership", "review gate"],
        _ => vec!["typed target"],
    };

    Some(CommandIntentHints {
        target_kind,
        mode: mode.to_string(),
        suggestions: suggestions.into_iter().map(ToString::to_string).collect(),
    })
}

fn build_delegation_hints(command: &ResolvedCommand) -> Option<CommandDelegationHints> {
    if command.targets.first()?.kind != DomainKind::DelegationPlan {
        return None;
    }

    Some(CommandDelegationHints {
        worker_roles: vec![
            "planner".to_string(),
            "implementer".to_string(),
            "reviewer".to_string(),
        ],
        coordination: "review_gated".to_string(),
        approval_required: true,
        linked_record_strategy: "artifact_plus_thread".to_string(),
    })
}

fn build_planned_records(command: &ResolvedCommand) -> Vec<CommandPlannedRecord> {
    let Some(target) = command.targets.first() else {
        return Vec::new();
    };

    let (planning_title, _) = planning_title_for_target(command, target.kind);

    match target.kind {
        DomainKind::SpecDraft => vec![
            CommandPlannedRecord {
                record_type: "artifact".to_string(),
                title: format!("spec_draft: {}", planning_title),
                links: vec![],
            },
            CommandPlannedRecord {
                record_type: "thread".to_string(),
                title: format!("spec thread: {}", planning_title),
                links: vec![CommandPlannedLink {
                    entity_type: "artifact".to_string(),
                    relation_type: "primary".to_string(),
                }],
            },
        ],
        DomainKind::ExecutionPlan => vec![
            CommandPlannedRecord {
                record_type: "artifact".to_string(),
                title: format!("execution_plan: {}", planning_title),
                links: vec![],
            },
            CommandPlannedRecord {
                record_type: "thread".to_string(),
                title: format!("plan thread: {}", planning_title),
                links: vec![CommandPlannedLink {
                    entity_type: "artifact".to_string(),
                    relation_type: "primary".to_string(),
                }],
            },
        ],
        DomainKind::DelegationPlan => vec![
            CommandPlannedRecord {
                record_type: "artifact".to_string(),
                title: format!("delegation_plan: {}", planning_title),
                links: vec![],
            },
            CommandPlannedRecord {
                record_type: "thread".to_string(),
                title: format!("delegation thread: {}", planning_title),
                links: vec![CommandPlannedLink {
                    entity_type: "artifact".to_string(),
                    relation_type: "primary".to_string(),
                }],
            },
        ],
        _ => Vec::new(),
    }
}

fn planning_title_for_target(command: &ResolvedCommand, target_kind: DomainKind) -> (String, bool) {
    let explicit_title = command
        .targets
        .first()
        .and_then(|target| {
            target
                .attributes
                .get("topic")
                .and_then(|value| value.as_str())
                .or_else(|| target.attributes.get("goal").and_then(|value| value.as_str()))
                .or_else(|| target.attributes.get("text").and_then(|value| value.as_str()))
        })
        .or_else(|| {
            command
                .inferred
                .get("topic")
                .and_then(|value| value.as_str())
        })
        .or_else(|| {
            command
                .inferred
                .get("goal")
                .and_then(|value| value.as_str())
        })
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);

    match explicit_title {
        Some(title) => (title, false),
        None => (
            match target_kind {
                DomainKind::SpecDraft => "spec draft".to_string(),
                DomainKind::ExecutionPlan => "execution plan".to_string(),
                DomainKind::DelegationPlan => "delegation plan".to_string(),
                _ => "planned command".to_string(),
            },
            true,
        ),
    }
}

fn requires_targets(operation: DomainOperation) -> bool {
    matches!(
        operation,
        DomainOperation::Create
            | DomainOperation::Inspect
            | DomainOperation::Update
            | DomainOperation::Link
            | DomainOperation::Execute
    )
}

fn is_supported_operation(operation: DomainOperation) -> bool {
    matches!(
        operation,
        DomainOperation::Create
            | DomainOperation::Inspect
            | DomainOperation::List
            | DomainOperation::Explain
            | DomainOperation::Execute
    )
}

fn is_dry_run_only(operation: DomainOperation) -> bool {
    matches!(operation, DomainOperation::Execute)
}

async fn execute_create_capture(
    state: &AppState,
    command: &ResolvedCommand,
) -> Result<CommandExecutionResult, AppError> {
    let target = command
        .targets
        .first()
        .ok_or_else(|| AppError::bad_request("capture command requires a target"))?;
    let content_text = target
        .attributes
        .get("text")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| AppError::bad_request("capture command requires non-empty text"))?
        .to_string();
    let capture_type = command
        .inferred
        .get("capture_type")
        .and_then(|value| value.as_str())
        .or_else(|| {
            target
                .attributes
                .get("capture_type")
                .and_then(|value| value.as_str())
        })
        .unwrap_or("quick_note")
        .to_string();
    let source_device = command
        .inferred
        .get("source_device")
        .and_then(|value| value.as_str())
        .unwrap_or("vel-command")
        .to_string();

    let capture_id = state
        .storage
        .insert_capture(CaptureInsert {
            content_text: content_text.clone(),
            capture_type: capture_type.clone(),
            source_device: Some(source_device.clone()),
            privacy_class: PrivacyClass::Private,
        })
        .await?;

    let payload_json = json!({ "capture_id": capture_id.to_string() }).to_string();
    if let Err(error) = state
        .storage
        .emit_event(
            "CAPTURE_CREATED",
            "capture",
            Some(capture_id.as_ref()),
            &payload_json,
        )
        .await
    {
        warn!(error = %error, "failed to emit CAPTURE_CREATED event");
    }

    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let signal_payload = json!({
        "capture_id": capture_id.to_string(),
        "content": content_text,
        "tags": []
    });
    if let Err(error) = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: "capture_created".to_string(),
            source: "vel".to_string(),
            source_ref: Some(capture_id.to_string()),
            timestamp: now_ts,
            payload_json: Some(signal_payload),
        })
        .await
    {
        warn!(error = %error, "failed to insert capture_created signal");
    }

    if capture_type == "todo" {
        if let Err(error) = state
            .storage
            .insert_commitment(CommitmentInsert {
                text: target
                    .attributes
                    .get("text")
                    .and_then(|value| value.as_str())
                    .unwrap_or_default()
                    .trim()
                    .to_string(),
                source_type: "capture".to_string(),
                source_id: Some(capture_id.to_string()),
                status: CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(json!({ "capture_id": capture_id.to_string() })),
            })
            .await
        {
            warn!(error = %error, "failed to create commitment from todo capture");
        }
    }

    let response = CaptureCreateResponse {
        capture_id,
        accepted_at: OffsetDateTime::now_utc(),
    };

    Ok(CommandExecutionResult {
        result: CommandExecutionPayload::CaptureCreated(response),
        warnings: Vec::new(),
    })
}

async fn execute_review_context(
    state: &AppState,
    command: &ResolvedCommand,
) -> Result<CommandExecutionResult, AppError> {
    let scope = command
        .targets
        .first()
        .and_then(|target| target.attributes.get("scope"))
        .and_then(|value| value.as_str())
        .unwrap_or("today");
    let (limit, today) = match scope {
        "today" => (20, true),
        "week" => (50, false),
        other => {
            return Err(AppError::bad_request(format!(
                "unsupported review scope `{}`",
                other
            )));
        }
    };

    let captures = state.storage.list_captures_recent(limit, today).await?;
    let captures = captures
        .into_iter()
        .map(vel_api_types::ContextCapture::from)
        .collect::<Vec<_>>();
    let latest_context_artifact = state
        .storage
        .get_latest_artifact_by_type("context_brief")
        .await?
        .map(artifact_record_to_data)
        .transpose()?;
    let review = CommandReviewSummaryData {
        capture_count: captures.len(),
        captures,
        latest_context_artifact,
    };

    Ok(CommandExecutionResult {
        result: match scope {
            "today" => CommandExecutionPayload::ReviewToday(review),
            "week" => CommandExecutionPayload::ReviewWeek(review),
            _ => unreachable!("unsupported review scope already returned"),
        },
        warnings: Vec::new(),
    })
}

async fn execute_create_commitment(
    state: &AppState,
    command: &ResolvedCommand,
) -> Result<CommandExecutionResult, AppError> {
    let target = command
        .targets
        .first()
        .ok_or_else(|| AppError::bad_request("commitment command requires a target"))?;

    let text = target
        .attributes
        .get("text")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| AppError::bad_request("commitment text must not be empty"))?
        .to_string();

    let project = target
        .attributes
        .get("project")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
        .or_else(|| {
            command
                .inferred
                .get("project")
                .and_then(|value| value.as_str())
                .map(ToOwned::to_owned)
        });

    let commitment_kind = target
        .attributes
        .get("commitment_kind")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned);

    let commitment_id = state
        .storage
        .insert_commitment(CommitmentInsert {
            text,
            source_type: "vel-command".to_string(),
            source_id: None,
            status: CommitmentStatus::Open,
            due_at: None,
            project,
            commitment_kind,
            metadata_json: Some(json!({
                "command_operation": command.operation.to_string(),
                "target_kind": "commitment",
                "assumptions": command.assumptions,
            })),
        })
        .await?;

    let commitment = state
        .storage
        .get_commitment_by_id(commitment_id.as_ref())
        .await?
        .ok_or_else(|| AppError::internal("commitment not found after insert"))?;

    Ok(CommandExecutionResult {
        result: CommandExecutionPayload::CommitmentCreated(CommitmentData::from(commitment)),
        warnings: Vec::new(),
    })
}

async fn execute_create_planning_artifact(
    state: &AppState,
    command: &ResolvedCommand,
    artifact_type: &str,
    result_kind: CommandExecutionPayloadKind,
) -> Result<CommandExecutionResult, AppError> {
    let target = command
        .targets
        .first()
        .ok_or_else(|| AppError::bad_request("planning command requires a target"))?;
    let (title, title_defaulted) = planning_title_for_target(command, target.kind);

    let storage_uri = format!(
        "vel://command/{}/{}",
        artifact_type,
        Uuid::new_v4().simple()
    );

    let artifact_id = state
        .storage
        .create_artifact(ArtifactInsert {
            artifact_type: artifact_type.to_string(),
            title: Some(title),
            mime_type: Some("application/json".to_string()),
            storage_uri,
            storage_kind: ArtifactStorageKind::External,
            privacy_class: PrivacyClass::Private,
            sync_class: SyncClass::Warm,
            content_hash: None,
            size_bytes: None,
            metadata_json: Some(json!({
                "command": {
                    "operation": command.operation.to_string(),
                    "targets": command.targets,
                    "inferred": command.inferred,
                    "assumptions": command.assumptions,
                },
            })),
        })
        .await?;

    let artifact = state
        .storage
        .get_artifact_by_id(artifact_id.as_ref())
        .await?
        .ok_or_else(|| AppError::internal("artifact not found after insert"))?;
    let artifact = artifact_record_to_data(artifact)?;
    let thread = create_planning_thread(state, artifact_type, &artifact, command).await?;
    let planning = PlanningArtifactCreatedData { artifact, thread };

    Ok(CommandExecutionResult {
        result: match result_kind {
            CommandExecutionPayloadKind::ArtifactCreated => {
                CommandExecutionPayload::ArtifactCreated(planning.artifact)
            }
            CommandExecutionPayloadKind::SpecDraftCreated => {
                CommandExecutionPayload::SpecDraftCreated(planning)
            }
            CommandExecutionPayloadKind::ExecutionPlanCreated => {
                CommandExecutionPayload::ExecutionPlanCreated(planning)
            }
            CommandExecutionPayloadKind::DelegationPlanCreated => {
                CommandExecutionPayload::DelegationPlanCreated(planning)
            }
        },
        warnings: if title_defaulted {
            vec![format!(
                "no topic, goal, or text was provided; defaulted {} title",
                artifact_type
            )]
        } else {
            Vec::new()
        },
    })
}

#[derive(Debug, Clone, Copy)]
enum CommandExecutionPayloadKind {
    ArtifactCreated,
    SpecDraftCreated,
    ExecutionPlanCreated,
    DelegationPlanCreated,
}

fn artifact_record_to_data(
    record: ArtifactRecord,
) -> Result<vel_api_types::ArtifactData, AppError> {
    Ok(vel_api_types::ArtifactData {
        artifact_id: record.artifact_id,
        artifact_type: record.artifact_type,
        title: record.title,
        mime_type: record.mime_type,
        storage_uri: record.storage_uri,
        storage_kind: record.storage_kind.to_string(),
        privacy_class: record.privacy_class,
        sync_class: record.sync_class,
        content_hash: record.content_hash,
        size_bytes: record.size_bytes,
        created_at: OffsetDateTime::from_unix_timestamp(record.created_at)
            .map_err(|error| AppError::internal(error.to_string()))?,
        updated_at: OffsetDateTime::from_unix_timestamp(record.updated_at)
            .map_err(|error| AppError::internal(error.to_string()))?,
    })
}

async fn create_planning_thread(
    state: &AppState,
    artifact_type: &str,
    artifact: &ArtifactData,
    command: &ResolvedCommand,
) -> Result<ThreadData, AppError> {
    let thread_id = format!("thr_{}", Uuid::new_v4().simple());
    let thread_type = match artifact_type {
        "spec_draft" => "planning_spec",
        "execution_plan" => "planning_execution",
        "delegation_plan" => "planning_delegation",
        _ => "planning",
    };
    let title = artifact
        .title
        .clone()
        .unwrap_or_else(|| format!("{artifact_type} thread"));
    let metadata = json!({
        "source": "vel-command",
        "artifact_id": artifact.artifact_id,
        "operation": command.operation.to_string(),
        "target_kind": artifact_type,
    })
    .to_string();

    state
        .storage
        .insert_thread(&thread_id, thread_type, &title, "planned", &metadata)
        .await?;
    state
        .storage
        .insert_thread_link(
            &thread_id,
            "artifact",
            artifact.artifact_id.as_ref(),
            "primary",
        )
        .await?;
    let row = state
        .storage
        .get_thread_by_id(&thread_id)
        .await?
        .ok_or_else(|| AppError::internal("thread not found after insert"))?;
    Ok(thread_row_to_data(row))
}

fn thread_row_to_data(row: (String, String, String, String, String, i64, i64)) -> ThreadData {
    let (id, thread_type, title, status, _metadata_json, created_at, updated_at) = row;
    let planning_kind = match thread_type.as_str() {
        "planning_spec" => Some("spec".to_string()),
        "planning_execution" => Some("execution_plan".to_string()),
        "planning_delegation" => Some("delegation_plan".to_string()),
        _ => None,
    };
    let lifecycle_stage = planning_kind.as_ref().map(|_| status.clone());
    ThreadData {
        id,
        thread_type,
        title,
        status,
        planning_kind,
        lifecycle_stage,
        created_at,
        updated_at,
        links: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_config::PolicyConfig;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{TargetSelector, TypedTarget};
    use vel_storage::Storage;

    async fn test_state() -> AppState {
        let storage = Storage::connect(":memory:").await.expect("storage");
        storage.migrate().await.expect("migrate");
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    #[test]
    fn validate_create_with_target_is_valid() {
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::Capture,
                id: None,
                selector: Some(TargetSelector::Custom("inline_text".to_string())),
                attributes: serde_json::json!({"text":"hello"}),
            }],
            ..ResolvedCommand::default()
        };

        let validation = validate_command(&command);
        assert!(validation.is_valid);
        assert!(validation.issues.is_empty());
    }

    #[test]
    fn validate_update_is_unsupported() {
        let command = ResolvedCommand {
            operation: DomainOperation::Update,
            targets: vec![TypedTarget::new(DomainKind::Capture)],
            ..ResolvedCommand::default()
        };

        let validation = validate_command(&command);
        assert!(!validation.is_valid);
        assert!(validation
            .issues
            .iter()
            .any(|issue| issue.code == ValidationIssueCode::UnsupportedOperation));
    }

    #[test]
    fn build_plan_for_execute_is_dry_run_only() {
        let command = ResolvedCommand {
            operation: DomainOperation::Execute,
            targets: vec![TypedTarget::new(DomainKind::Context)],
            ..ResolvedCommand::default()
        };

        let plan = build_execution_plan(&command);
        assert_eq!(plan.mode, CommandPlanMode::DryRunOnly);
        assert!(plan.validation.is_valid);
        assert!(plan.summary.contains("dry_run_only"));
        assert_eq!(
            plan.intent_hints.as_ref().map(|hints| hints.mode.as_str()),
            Some("execute")
        );
    }

    #[test]
    fn build_plan_for_delegation_includes_planning_hints() {
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget::new(DomainKind::DelegationPlan)],
            ..ResolvedCommand::default()
        };

        let plan = build_execution_plan(&command);
        assert_eq!(plan.mode, CommandPlanMode::Ready);
        assert_eq!(
            plan.intent_hints.as_ref().map(|hints| hints.target_kind),
            Some(DomainKind::DelegationPlan)
        );
        assert_eq!(
            plan.intent_hints.as_ref().map(|hints| hints.mode.as_str()),
            Some("planning_artifact")
        );
        assert_eq!(
            plan.delegation_hints
                .as_ref()
                .map(|hints| hints.linked_record_strategy.as_str()),
            Some("artifact_plus_thread")
        );
        assert_eq!(plan.planned_records.len(), 2);
        assert_eq!(plan.planned_records[0].record_type, "artifact");
        assert_eq!(plan.planned_records[1].record_type, "thread");
    }

    #[tokio::test]
    async fn execute_create_capture_returns_capture_created_result() {
        let state = test_state().await;
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::Capture,
                id: None,
                selector: Some(TargetSelector::Custom("inline_text".to_string())),
                attributes: serde_json::json!({
                    "text": "test capture",
                    "capture_type": "quick_note"
                }),
            }],
            inferred: serde_json::json!({
                "capture_type": "quick_note",
                "source_device": "vel-command"
            }),
            ..ResolvedCommand::default()
        };

        let result = execute_command(&state, &command).await.expect("execute");
        match result.result {
            CommandExecutionPayload::CaptureCreated(payload) => {
                assert!(!payload.capture_id.as_ref().is_empty());
            }
            other => panic!("unexpected result payload: {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_create_commitment_returns_commitment_created_result() {
        let state = test_state().await;
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::Commitment,
                id: None,
                selector: Some(TargetSelector::Custom("inline_text".to_string())),
                attributes: serde_json::json!({
                    "text": "finish integration hardening",
                    "project": "vel"
                }),
            }],
            ..ResolvedCommand::default()
        };

        let result = execute_command(&state, &command).await.expect("execute");
        match result.result {
            CommandExecutionPayload::CommitmentCreated(payload) => {
                assert_eq!(payload.status, "open");
                assert_eq!(payload.project.as_deref(), Some("vel"));
            }
            other => panic!("unexpected result payload: {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_create_spec_draft_returns_spec_draft_created_result() {
        let state = test_state().await;
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::SpecDraft,
                id: None,
                selector: Some(TargetSelector::Custom("topic".to_string())),
                attributes: serde_json::json!({
                    "topic": "cluster sync"
                }),
            }],
            inferred: serde_json::json!({
                "planning_status": "planned"
            }),
            ..ResolvedCommand::default()
        };

        let result = execute_command(&state, &command).await.expect("execute");
        match result.result {
            CommandExecutionPayload::SpecDraftCreated(payload) => {
                assert_eq!(payload.artifact.artifact_type, "spec_draft");
                assert_eq!(payload.thread.thread_type, "planning_spec");
                assert_eq!(payload.thread.planning_kind.as_deref(), Some("spec"));
                assert_eq!(payload.thread.status, "planned");
            }
            other => panic!("unexpected result payload: {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_create_execution_plan_returns_execution_plan_created_result() {
        let state = test_state().await;
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::ExecutionPlan,
                id: None,
                selector: Some(TargetSelector::Custom("goal".to_string())),
                attributes: serde_json::json!({
                    "goal": "message backlog"
                }),
            }],
            inferred: serde_json::json!({
                "planning_status": "planned"
            }),
            ..ResolvedCommand::default()
        };

        let result = execute_command(&state, &command).await.expect("execute");
        match result.result {
            CommandExecutionPayload::ExecutionPlanCreated(payload) => {
                assert_eq!(payload.artifact.artifact_type, "execution_plan");
                assert_eq!(payload.thread.thread_type, "planning_execution");
                assert_eq!(
                    payload.thread.planning_kind.as_deref(),
                    Some("execution_plan")
                );
                assert_eq!(payload.thread.status, "planned");
            }
            other => panic!("unexpected result payload: {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_create_delegation_plan_returns_delegation_plan_created_result() {
        let state = test_state().await;
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::DelegationPlan,
                id: None,
                selector: Some(TargetSelector::Custom("goal".to_string())),
                attributes: serde_json::json!({
                    "goal": "queue cleanup"
                }),
            }],
            inferred: serde_json::json!({
                "planning_status": "planned"
            }),
            ..ResolvedCommand::default()
        };

        let result = execute_command(&state, &command).await.expect("execute");
        match result.result {
            CommandExecutionPayload::DelegationPlanCreated(payload) => {
                assert_eq!(payload.artifact.artifact_type, "delegation_plan");
                assert_eq!(payload.thread.thread_type, "planning_delegation");
                assert_eq!(
                    payload.thread.planning_kind.as_deref(),
                    Some("delegation_plan")
                );
                assert_eq!(payload.thread.status, "planned");
            }
            other => panic!("unexpected result payload: {other:?}"),
        }
    }

    #[test]
    fn planning_title_defaults_by_target_kind() {
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget::new(DomainKind::SpecDraft)],
            ..ResolvedCommand::default()
        };
        let (spec_title, spec_defaulted) = planning_title_for_target(&command, DomainKind::SpecDraft);
        assert_eq!(spec_title, "spec draft");
        assert!(spec_defaulted);

        let (plan_title, plan_defaulted) =
            planning_title_for_target(&command, DomainKind::ExecutionPlan);
        assert_eq!(plan_title, "execution plan");
        assert!(plan_defaulted);

        let (delegate_title, delegate_defaulted) =
            planning_title_for_target(&command, DomainKind::DelegationPlan);
        assert_eq!(delegate_title, "delegation plan");
        assert!(delegate_defaulted);
    }

    #[tokio::test]
    async fn execute_create_spec_draft_warns_when_title_is_defaulted() {
        let state = test_state().await;
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget::new(DomainKind::SpecDraft)],
            inferred: serde_json::json!({
                "planning_status": "planned"
            }),
            ..ResolvedCommand::default()
        };

        let result = execute_command(&state, &command).await.expect("execute");
        assert_eq!(
            result.warnings,
            vec!["no topic, goal, or text was provided; defaulted spec_draft title".to_string()]
        );
        match result.result {
            CommandExecutionPayload::SpecDraftCreated(payload) => {
                assert_eq!(payload.artifact.title.as_deref(), Some("spec draft"));
            }
            other => panic!("unexpected result payload: {other:?}"),
        }
    }
}
