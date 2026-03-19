use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vel_api_types::ApiResponse;

use crate::{
    errors::AppError,
    services::execution_context::{self, ExecutionContextData},
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
