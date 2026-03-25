use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vel_api_types::ApiResponse;

use crate::{
    errors::AppError,
    services::{
        connect_runtime,
        execution_context::{self, ExecutionContextData},
        execution_launch,
        execution_routing::{
            self, CreateExecutionHandoffInput, HandoffOriginKind, HandoffReviewState,
            ReviewExecutionHandoffInput,
        },
    },
    state::AppState,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SaveExecutionContextRequest {
    pub objective: String,
    #[serde(default)]
    pub repo_brief: String,
    #[serde(default)]
    pub notes_brief: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionArtifactRequest {
    #[serde(default)]
    pub output_dir: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListExecutionHandoffsQuery {
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExecutionHandoffRequest {
    pub project_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub origin_kind: String,
    pub objective: String,
    #[serde(default)]
    pub task_kind: Option<vel_core::ExecutionTaskKind>,
    #[serde(default)]
    pub agent_profile: Option<vel_core::AgentProfile>,
    #[serde(default)]
    pub token_budget: Option<vel_core::TokenBudgetClass>,
    #[serde(default)]
    pub review_gate: Option<vel_core::ExecutionReviewGate>,
    #[serde(default)]
    pub read_scopes: Vec<String>,
    #[serde(default)]
    pub write_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub inputs: serde_json::Value,
    #[serde(default)]
    pub expected_output_schema: serde_json::Value,
    #[serde(default)]
    pub manifest_id: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewExecutionHandoffRequest {
    pub reviewed_by: String,
    #[serde(default)]
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchExecutionHandoffRequest {
    pub runtime_kind: String,
    #[serde(default)]
    pub actor_id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub writable_roots: Vec<String>,
    #[serde(default)]
    pub capability_allowlist: Vec<vel_core::CapabilityDescriptor>,
    #[serde(default)]
    pub lease_seconds: Option<i64>,
}

pub async fn get_execution_context(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<ExecutionContextData>>, AppError> {
    let context = execution_context::get_execution_context(&state, project_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("execution context not found"))?;
    Ok(Json(ApiResponse::success(
        context,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn save_execution_context(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(payload): Json<SaveExecutionContextRequest>,
) -> Result<Json<ApiResponse<ExecutionContextData>>, AppError> {
    let context = execution_context::save_execution_context(
        &state,
        project_id.trim(),
        execution_context::ExecutionContextInput {
            objective: payload.objective,
            repo_brief: payload.repo_brief,
            notes_brief: payload.notes_brief,
            constraints: payload.constraints,
            expected_outputs: payload.expected_outputs,
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(
        context,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn preview_execution_artifacts(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(payload): Json<ExecutionArtifactRequest>,
) -> Result<Json<ApiResponse<execution_context::ExecutionArtifactPackData>>, AppError> {
    let pack = execution_context::preview_gsd_artifacts(
        &state,
        project_id.trim(),
        payload.output_dir.as_deref(),
    )
    .await?;

    Ok(Json(ApiResponse::success(
        pack,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn export_execution_artifacts(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(payload): Json<ExecutionArtifactRequest>,
) -> Result<Json<ApiResponse<execution_context::ExecutionExportResultData>>, AppError> {
    let exported = execution_context::export_gsd_artifacts(
        &state,
        project_id.trim(),
        payload.output_dir.as_deref(),
    )
    .await?;

    Ok(Json(ApiResponse::success(
        exported,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn create_execution_handoff(
    State(state): State<AppState>,
    Json(payload): Json<CreateExecutionHandoffRequest>,
) -> Result<Json<ApiResponse<execution_routing::ExecutionHandoffRecordData>>, AppError> {
    let origin_kind: HandoffOriginKind = payload.origin_kind.parse()?;
    let handoff = execution_routing::create_execution_handoff(
        &state,
        CreateExecutionHandoffInput {
            project_id: payload.project_id,
            from_agent: payload.from_agent,
            to_agent: payload.to_agent,
            origin_kind,
            objective: payload.objective,
            task_kind: payload.task_kind,
            agent_profile: payload.agent_profile,
            token_budget: payload.token_budget,
            review_gate: payload.review_gate,
            read_scopes: payload.read_scopes,
            write_scopes: payload.write_scopes,
            allowed_tools: payload.allowed_tools,
            constraints: payload.constraints,
            inputs: payload.inputs,
            expected_output_schema: payload.expected_output_schema,
            manifest_id: payload.manifest_id,
            requested_by: payload.requested_by,
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(
        handoff,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn list_execution_handoffs(
    State(state): State<AppState>,
    Query(query): Query<ListExecutionHandoffsQuery>,
) -> Result<Json<ApiResponse<Vec<execution_routing::ExecutionHandoffRecordData>>>, AppError> {
    let review_state = query
        .state
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::parse::<HandoffReviewState>)
        .transpose()?;
    let handoffs = execution_routing::list_execution_handoffs(
        &state,
        query.project_id.as_deref(),
        review_state,
    )
    .await?;

    Ok(Json(ApiResponse::success(
        handoffs,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn preview_execution_handoff_launch(
    State(state): State<AppState>,
    Path(handoff_id): Path<String>,
) -> Result<Json<ApiResponse<execution_routing::ExecutionLaunchPreviewData>>, AppError> {
    let preview = execution_routing::preview_launch(&state, handoff_id.trim()).await?;
    Ok(Json(ApiResponse::success(
        preview,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn approve_execution_handoff(
    State(state): State<AppState>,
    Path(handoff_id): Path<String>,
    Json(payload): Json<ReviewExecutionHandoffRequest>,
) -> Result<Json<ApiResponse<execution_routing::ExecutionHandoffRecordData>>, AppError> {
    let handoff = execution_routing::approve_execution_handoff(
        &state,
        handoff_id.trim(),
        ReviewExecutionHandoffInput {
            reviewed_by: payload.reviewed_by,
            decision_reason: payload.decision_reason,
        },
    )
    .await?;
    Ok(Json(ApiResponse::success(
        handoff,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn reject_execution_handoff(
    State(state): State<AppState>,
    Path(handoff_id): Path<String>,
    Json(payload): Json<ReviewExecutionHandoffRequest>,
) -> Result<Json<ApiResponse<execution_routing::ExecutionHandoffRecordData>>, AppError> {
    let handoff = execution_routing::reject_execution_handoff(
        &state,
        handoff_id.trim(),
        ReviewExecutionHandoffInput {
            reviewed_by: payload.reviewed_by,
            decision_reason: payload.decision_reason,
        },
    )
    .await?;
    Ok(Json(ApiResponse::success(
        handoff,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn launch_execution_handoff(
    State(state): State<AppState>,
    Path(handoff_id): Path<String>,
    Json(payload): Json<LaunchExecutionHandoffRequest>,
) -> Result<Json<ApiResponse<vel_api_types::ConnectInstanceData>>, AppError> {
    let launched = execution_launch::launch_approved_handoff(
        &state,
        handoff_id.trim(),
        execution_launch::LaunchApprovedHandoffRequest {
            runtime_kind: payload.runtime_kind,
            actor_id: payload.actor_id,
            display_name: payload.display_name,
            command: payload.command,
            working_dir: payload.working_dir,
            writable_roots: payload.writable_roots,
            capability_allowlist: payload.capability_allowlist,
            lease_seconds: payload.lease_seconds,
        },
    )
    .await?;

    let _ = connect_runtime::reconcile_connect_runtime_state(&state).await;

    Ok(Json(ApiResponse::success(
        vel_api_types::ConnectInstanceData::from(launched),
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}
