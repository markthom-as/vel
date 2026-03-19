use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_core::{
    AgentProfile, ExecutionHandoff, ExecutionReviewGate, ExecutionTaskKind, HandoffEnvelope,
    ProjectId, ProjectRecord, RepoWorktreeRef, TokenBudgetClass, TraceId,
};

use crate::{errors::AppError, state::AppState};

const REVIEW_PENDING: &str = "pending_review";
const REVIEW_APPROVED: &str = "approved";
const REVIEW_REJECTED: &str = "rejected";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOriginKind {
    HumanToAgent,
    AgentToAgent,
}

impl HandoffOriginKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::HumanToAgent => "human_to_agent",
            Self::AgentToAgent => "agent_to_agent",
        }
    }
}

impl std::str::FromStr for HandoffOriginKind {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "human_to_agent" => Ok(Self::HumanToAgent),
            "agent_to_agent" => Ok(Self::AgentToAgent),
            _ => Err(AppError::bad_request("invalid handoff origin kind")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReviewState {
    PendingReview,
    Approved,
    Rejected,
}

impl HandoffReviewState {
    fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => REVIEW_PENDING,
            Self::Approved => REVIEW_APPROVED,
            Self::Rejected => REVIEW_REJECTED,
        }
    }
}

impl std::str::FromStr for HandoffReviewState {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            REVIEW_PENDING => Ok(Self::PendingReview),
            REVIEW_APPROVED => Ok(Self::Approved),
            REVIEW_REJECTED => Ok(Self::Rejected),
            _ => Err(AppError::bad_request("invalid handoff review state")),
        }
    }
}

fn task_kind_str(value: ExecutionTaskKind) -> &'static str {
    match value {
        ExecutionTaskKind::Planning => "planning",
        ExecutionTaskKind::Implementation => "implementation",
        ExecutionTaskKind::Debugging => "debugging",
        ExecutionTaskKind::Review => "review",
        ExecutionTaskKind::Research => "research",
        ExecutionTaskKind::Documentation => "documentation",
    }
}

fn agent_profile_str(value: AgentProfile) -> &'static str {
    match value {
        AgentProfile::Budget => "budget",
        AgentProfile::Balanced => "balanced",
        AgentProfile::Quality => "quality",
        AgentProfile::Inherit => "inherit",
    }
}

fn token_budget_str(value: TokenBudgetClass) -> &'static str {
    match value {
        TokenBudgetClass::Small => "small",
        TokenBudgetClass::Medium => "medium",
        TokenBudgetClass::Large => "large",
        TokenBudgetClass::Xlarge => "xlarge",
    }
}

fn review_gate_str(value: ExecutionReviewGate) -> &'static str {
    match value {
        ExecutionReviewGate::None => "none",
        ExecutionReviewGate::OperatorApproval => "operator_approval",
        ExecutionReviewGate::OperatorPreview => "operator_preview",
        ExecutionReviewGate::PostRunReview => "post_run_review",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingReasonData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRoutingDecisionData {
    pub task_kind: ExecutionTaskKind,
    pub agent_profile: AgentProfile,
    pub token_budget: TokenBudgetClass,
    pub review_gate: ExecutionReviewGate,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub reasons: Vec<RoutingReasonData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionHandoffRecordData {
    pub id: String,
    pub project_id: ProjectId,
    pub origin_kind: HandoffOriginKind,
    pub review_state: HandoffReviewState,
    pub handoff: ExecutionHandoff,
    pub routing: ExecutionRoutingDecisionData,
    #[serde(default)]
    pub manifest_id: Option<String>,
    pub requested_by: String,
    #[serde(default)]
    pub reviewed_by: Option<String>,
    #[serde(default)]
    pub decision_reason: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub reviewed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub launched_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionLaunchPreviewData {
    pub handoff_id: String,
    pub review_state: HandoffReviewState,
    pub launch_ready: bool,
    #[serde(default)]
    pub blockers: Vec<String>,
    pub handoff: ExecutionHandoff,
    pub routing: ExecutionRoutingDecisionData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateExecutionHandoffInput {
    pub project_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub origin_kind: HandoffOriginKind,
    pub objective: String,
    #[serde(default)]
    pub task_kind: Option<ExecutionTaskKind>,
    #[serde(default)]
    pub agent_profile: Option<AgentProfile>,
    #[serde(default)]
    pub token_budget: Option<TokenBudgetClass>,
    #[serde(default)]
    pub review_gate: Option<ExecutionReviewGate>,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub inputs: JsonValue,
    #[serde(default)]
    pub expected_output_schema: JsonValue,
    #[serde(default)]
    pub manifest_id: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewExecutionHandoffInput {
    pub reviewed_by: String,
    #[serde(default)]
    pub decision_reason: Option<String>,
}

pub async fn create_execution_handoff(
    state: &AppState,
    input: CreateExecutionHandoffInput,
) -> Result<ExecutionHandoffRecordData, AppError> {
    let project = state
        .storage
        .get_project(input.project_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("project not found"))?;
    validate_create_input(&project, &input)?;

    let now = OffsetDateTime::now_utc();
    let requested_by = input
        .requested_by
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("operator_shell")
        .to_string();
    let routing = compute_routing(&input);
    let review_gate = routing.review_gate;
    let task_id = format!("handoff_{}", uuid::Uuid::new_v4().simple());
    let repo = RepoWorktreeRef {
        path: project.primary_repo.path.clone(),
        label: project.primary_repo.label.clone(),
        branch: None,
        head_rev: None,
    };
    let handoff = ExecutionHandoff {
        handoff: HandoffEnvelope {
            task_id: task_id.clone(),
            trace_id: TraceId::new(),
            from_agent: input.from_agent.trim().to_string(),
            to_agent: input.to_agent.trim().to_string(),
            objective: input.objective.trim().to_string(),
            inputs: input.inputs.clone(),
            constraints: normalize_strings(input.constraints),
            read_scopes: routing.read_scopes.clone(),
            write_scopes: routing.write_scopes.clone(),
            project_id: Some(project.id.clone()),
            task_kind: Some(routing.task_kind),
            agent_profile: Some(routing.agent_profile),
            token_budget: Some(routing.token_budget),
            review_gate: Some(review_gate),
            repo_root: Some(repo.clone()),
            allowed_tools: routing.allowed_tools.clone(),
            capability_scope: json!({
                "read_scopes": routing.read_scopes,
                "write_scopes": routing.write_scopes,
            }),
            deadline: None,
            expected_output_schema: input.expected_output_schema.clone(),
        },
        project_id: project.id.clone(),
        task_kind: routing.task_kind,
        agent_profile: routing.agent_profile,
        token_budget: routing.token_budget,
        review_gate,
        repo,
        notes_root: project.primary_notes_root.clone(),
        manifest_id: input.manifest_id.clone(),
    };
    handoff
        .validate()
        .map_err(|error| AppError::bad_request(error.to_string()))?;

    let handoff_json =
        serde_json::to_value(&handoff).map_err(|error| AppError::internal(error.to_string()))?;
    let routing_json =
        serde_json::to_value(&routing).map_err(|error| AppError::internal(error.to_string()))?;
    let handoff_id = state
        .storage
        .create_execution_handoff(
            &project.id,
            &handoff_json,
            task_kind_str(routing.task_kind),
            agent_profile_str(routing.agent_profile),
            token_budget_str(routing.token_budget),
            review_gate_str(routing.review_gate),
            input.origin_kind.as_str(),
            REVIEW_PENDING,
            &routing_json,
            input.manifest_id.as_deref(),
            &requested_by,
            now,
        )
        .await?;

    hydrate_record(
        state
            .storage
            .get_execution_handoff(&handoff_id)
            .await?
            .ok_or_else(|| AppError::internal("handoff missing after insert"))?,
    )
}

pub async fn list_execution_handoffs(
    state: &AppState,
    project_id: Option<&str>,
    review_state: Option<HandoffReviewState>,
) -> Result<Vec<ExecutionHandoffRecordData>, AppError> {
    let rows = state
        .storage
        .list_execution_handoffs(
            project_id.map(str::trim).filter(|value| !value.is_empty()),
            review_state.map(HandoffReviewState::as_str),
        )
        .await?;

    rows.into_iter().map(hydrate_record).collect()
}

pub async fn get_execution_handoff(
    state: &AppState,
    handoff_id: &str,
) -> Result<Option<ExecutionHandoffRecordData>, AppError> {
    state
        .storage
        .get_execution_handoff(handoff_id.trim())
        .await?
        .map(hydrate_record)
        .transpose()
}

pub async fn preview_launch(
    state: &AppState,
    handoff_id: &str,
) -> Result<ExecutionLaunchPreviewData, AppError> {
    let record = get_execution_handoff(state, handoff_id)
        .await?
        .ok_or_else(|| AppError::not_found("execution handoff not found"))?;
    let mut blockers = Vec::new();
    match record.review_state {
        HandoffReviewState::PendingReview => {
            blockers.push("handoff review is still pending".to_string())
        }
        HandoffReviewState::Rejected => blockers.push("handoff was rejected".to_string()),
        HandoffReviewState::Approved => {}
    }

    Ok(ExecutionLaunchPreviewData {
        handoff_id: record.id.clone(),
        review_state: record.review_state,
        launch_ready: blockers.is_empty(),
        blockers,
        handoff: record.handoff,
        routing: record.routing,
    })
}

pub async fn approve_execution_handoff(
    state: &AppState,
    handoff_id: &str,
    input: ReviewExecutionHandoffInput,
) -> Result<ExecutionHandoffRecordData, AppError> {
    review_execution_handoff(state, handoff_id, HandoffReviewState::Approved, input).await
}

pub async fn reject_execution_handoff(
    state: &AppState,
    handoff_id: &str,
    input: ReviewExecutionHandoffInput,
) -> Result<ExecutionHandoffRecordData, AppError> {
    review_execution_handoff(state, handoff_id, HandoffReviewState::Rejected, input).await
}

async fn review_execution_handoff(
    state: &AppState,
    handoff_id: &str,
    review_state: HandoffReviewState,
    input: ReviewExecutionHandoffInput,
) -> Result<ExecutionHandoffRecordData, AppError> {
    let reviewed_by = input.reviewed_by.trim();
    if reviewed_by.is_empty() {
        return Err(AppError::bad_request("reviewed_by must not be empty"));
    }

    let existing = get_execution_handoff(state, handoff_id)
        .await?
        .ok_or_else(|| AppError::not_found("execution handoff not found"))?;
    if existing.review_state != HandoffReviewState::PendingReview {
        return Err(AppError::bad_request(
            "execution handoff is no longer pending review",
        ));
    }

    let now = OffsetDateTime::now_utc();
    let updated = state
        .storage
        .update_execution_handoff_review(
            handoff_id.trim(),
            review_state.as_str(),
            Some(reviewed_by),
            input.decision_reason.as_deref(),
            Some(now),
            None,
            now,
        )
        .await?
        .ok_or_else(|| AppError::not_found("execution handoff not found"))?;

    hydrate_record(updated)
}

fn validate_create_input(
    project: &ProjectRecord,
    input: &CreateExecutionHandoffInput,
) -> Result<(), AppError> {
    if input.objective.trim().is_empty() {
        return Err(AppError::bad_request("handoff objective must not be empty"));
    }
    if input.from_agent.trim().is_empty() {
        return Err(AppError::bad_request("from_agent must not be empty"));
    }
    if input.to_agent.trim().is_empty() {
        return Err(AppError::bad_request("to_agent must not be empty"));
    }
    if normalize_strings(input.allowed_tools.clone()).is_empty() {
        return Err(AppError::bad_request(
            "handoff allowed_tools must not be empty",
        ));
    }
    if is_empty_json(&input.expected_output_schema) {
        return Err(AppError::bad_request(
            "handoff expected output schema must not be empty",
        ));
    }

    let read_scopes = normalize_strings(input.read_scopes.clone());
    let write_scopes = normalize_strings(input.write_scopes.clone());
    if read_scopes.is_empty() && write_scopes.is_empty() {
        return Err(AppError::bad_request(
            "handoff must declare at least one read or write scope",
        ));
    }

    for scope in read_scopes.iter().chain(write_scopes.iter()) {
        if !scope_within_project(project, scope) {
            return Err(AppError::bad_request(format!(
                "scope {} is outside declared project roots",
                scope
            )));
        }
    }

    Ok(())
}

fn compute_routing(input: &CreateExecutionHandoffInput) -> ExecutionRoutingDecisionData {
    let read_scopes = normalize_strings(input.read_scopes.clone());
    let write_scopes = normalize_strings(input.write_scopes.clone());
    let allowed_tools = normalize_strings(input.allowed_tools.clone());
    let task_kind = input.task_kind.unwrap_or(ExecutionTaskKind::Implementation);

    let mut reasons = vec![RoutingReasonData {
        code: "task_kind_selected".to_string(),
        message: format!("task_kind resolved to {}", task_kind_str(task_kind)),
    }];

    let agent_profile = match input.agent_profile {
        Some(profile) => profile,
        None if !write_scopes.is_empty()
            && matches!(
                task_kind,
                ExecutionTaskKind::Implementation | ExecutionTaskKind::Debugging
            ) =>
        {
            reasons.push(RoutingReasonData {
                code: "write_scope_quality_profile".to_string(),
                message: "write scopes plus implementation work default to quality".to_string(),
            });
            AgentProfile::Quality
        }
        None if matches!(
            task_kind,
            ExecutionTaskKind::Review | ExecutionTaskKind::Documentation
        ) =>
        {
            reasons.push(RoutingReasonData {
                code: "lighter_review_profile".to_string(),
                message: "review/documentation work defaults to balanced".to_string(),
            });
            AgentProfile::Balanced
        }
        None => {
            reasons.push(RoutingReasonData {
                code: "default_balanced_profile".to_string(),
                message: "no explicit profile provided; defaulting to balanced".to_string(),
            });
            AgentProfile::Balanced
        }
    };

    let token_budget = match input.token_budget {
        Some(budget) => budget,
        None if write_scopes.len() > 1 => {
            reasons.push(RoutingReasonData {
                code: "multiple_write_roots_budget".to_string(),
                message: "multiple write roots raise the default budget to large".to_string(),
            });
            TokenBudgetClass::Large
        }
        None if matches!(
            task_kind,
            ExecutionTaskKind::Research | ExecutionTaskKind::Planning
        ) =>
        {
            reasons.push(RoutingReasonData {
                code: "planning_budget_default".to_string(),
                message: "planning/research work defaults to medium budget".to_string(),
            });
            TokenBudgetClass::Medium
        }
        None => {
            reasons.push(RoutingReasonData {
                code: "default_medium_budget".to_string(),
                message: "defaulting token budget to medium".to_string(),
            });
            TokenBudgetClass::Medium
        }
    };

    let review_gate = match input.review_gate {
        Some(gate) => gate,
        None if !write_scopes.is_empty() => {
            reasons.push(RoutingReasonData {
                code: "write_scope_requires_approval".to_string(),
                message: "write scopes require explicit operator approval before launch"
                    .to_string(),
            });
            ExecutionReviewGate::OperatorApproval
        }
        None => {
            reasons.push(RoutingReasonData {
                code: "read_only_preview_gate".to_string(),
                message: "read-only work defaults to operator preview".to_string(),
            });
            ExecutionReviewGate::OperatorPreview
        }
    };

    ExecutionRoutingDecisionData {
        task_kind,
        agent_profile,
        token_budget,
        review_gate,
        read_scopes,
        write_scopes,
        allowed_tools,
        reasons,
    }
}

pub(crate) fn hydrate_record(
    row: (
        String,
        String,
        JsonValue,
        JsonValue,
        String,
        String,
        Option<String>,
        String,
        Option<String>,
        Option<String>,
        Option<OffsetDateTime>,
        Option<OffsetDateTime>,
        OffsetDateTime,
        OffsetDateTime,
    ),
) -> Result<ExecutionHandoffRecordData, AppError> {
    let (
        id,
        _project_id,
        handoff_json,
        routing_json,
        origin_kind,
        review_state,
        manifest_id,
        requested_by,
        reviewed_by,
        decision_reason,
        reviewed_at,
        launched_at,
        created_at,
        updated_at,
    ) = row;

    let handoff: ExecutionHandoff = serde_json::from_value(handoff_json)
        .map_err(|error| AppError::internal(error.to_string()))?;
    let routing: ExecutionRoutingDecisionData = serde_json::from_value(routing_json)
        .map_err(|error| AppError::internal(error.to_string()))?;

    Ok(ExecutionHandoffRecordData {
        id,
        project_id: handoff.project_id.clone(),
        origin_kind: origin_kind.parse()?,
        review_state: review_state.parse()?,
        manifest_id,
        handoff,
        routing,
        requested_by,
        reviewed_by,
        decision_reason,
        reviewed_at,
        launched_at,
        created_at,
        updated_at,
    })
}

fn normalize_strings(values: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !normalized.iter().any(|existing| existing == trimmed) {
            normalized.push(trimmed.to_string());
        }
    }
    normalized
}

fn is_empty_json(value: &JsonValue) -> bool {
    match value {
        JsonValue::Null => true,
        JsonValue::Object(map) => map.is_empty(),
        JsonValue::Array(items) => items.is_empty(),
        _ => false,
    }
}

fn scope_within_project(project: &ProjectRecord, scope: &str) -> bool {
    let scope = Path::new(scope);
    project_roots(project)
        .into_iter()
        .any(|root| scope.starts_with(root))
}

fn project_roots(project: &ProjectRecord) -> Vec<PathBuf> {
    let mut roots = vec![
        PathBuf::from(&project.primary_repo.path),
        PathBuf::from(&project.primary_notes_root.path),
    ];
    roots.extend(
        project
            .secondary_repos
            .iter()
            .map(|root| PathBuf::from(&root.path)),
    );
    roots.extend(
        project
            .secondary_notes_roots
            .iter()
            .map(|root| PathBuf::from(&root.path)),
    );
    roots
}
