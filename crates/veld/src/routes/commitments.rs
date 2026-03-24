use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::str::FromStr;
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, CommitmentCreateRequest, CommitmentData, CommitmentDependencyCreateRequest,
    CommitmentDependencyData, CommitmentUpdateRequest,
};
use vel_core::CommitmentStatus;
use vel_storage::CommitmentInsert;

use crate::{
    errors::AppError,
    services::commitment_write_bridge::{bridge_commitment_write, CommitmentWriteBridgeRequest},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListCommitmentsQuery {
    pub status: Option<String>,
    pub project: Option<String>,
    pub kind: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

pub async fn list_commitments(
    State(state): State<AppState>,
    Query(q): Query<ListCommitmentsQuery>,
) -> Result<Json<ApiResponse<Vec<CommitmentData>>>, AppError> {
    let status_filter = q
        .status
        .as_deref()
        .map(CommitmentStatus::from_str)
        .transpose()
        .map_err(|_| AppError::bad_request("invalid status"))?;
    let commitments = state
        .storage
        .list_commitments(
            status_filter,
            q.project.as_deref(),
            q.kind.as_deref(),
            q.limit,
        )
        .await?;
    let data: Vec<CommitmentData> = commitments.into_iter().map(CommitmentData::from).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_commitment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CommitmentData>>, AppError> {
    let commitment = state
        .storage
        .get_commitment_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CommitmentData::from(commitment),
        request_id,
    )))
}

pub async fn create_commitment(
    State(state): State<AppState>,
    Json(payload): Json<CommitmentCreateRequest>,
) -> Result<Json<ApiResponse<CommitmentData>>, AppError> {
    if payload.text.trim().is_empty() {
        return Err(AppError::bad_request("commitment text must not be empty"));
    }
    let id = state
        .storage
        .insert_commitment(CommitmentInsert {
            text: payload.text.trim().to_string(),
            source_type: payload.source_type,
            source_id: payload.source_id.unwrap_or_default(),
            status: CommitmentStatus::Open,
            due_at: payload.due_at,
            project: payload.project,
            commitment_kind: payload.commitment_kind,
            metadata_json: Some(payload.metadata),
        })
        .await?;
    let commitment = state
        .storage
        .get_commitment_by_id(id.as_ref())
        .await?
        .ok_or_else(|| AppError::internal("commitment not found after insert"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CommitmentData::from(commitment),
        request_id,
    )))
}

pub async fn update_commitment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<CommitmentUpdateRequest>,
) -> Result<Json<ApiResponse<CommitmentData>>, AppError> {
    let existing = state
        .storage
        .get_commitment_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let status = payload
        .status
        .as_deref()
        .map(CommitmentStatus::from_str)
        .transpose()
        .map_err(|_| AppError::bad_request("invalid status"))?;
    let requested_change = commitment_requested_change(&payload);

    let _write_intent = bridge_commitment_write(
        state.storage.sql_pool(),
        &CommitmentWriteBridgeRequest {
            object_id: id.trim().to_string(),
            object_status: existing.status.to_string(),
            requested_change,
            dry_run: false,
        },
    )
    .await?;

    state
        .storage
        .update_commitment(
            id.trim(),
            None,
            status,
            payload.due_at,
            payload.project.as_deref(),
            payload.commitment_kind.as_deref(),
            payload.metadata.as_ref(),
        )
        .await?;
    let commitment = state
        .storage
        .get_commitment_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CommitmentData::from(commitment),
        request_id,
    )))
}

fn commitment_requested_change(payload: &CommitmentUpdateRequest) -> JsonValue {
    let mut change = JsonMap::new();
    if let Some(status) = payload.status.as_ref() {
        change.insert("status".to_string(), JsonValue::String(status.clone()));
    }
    if let Some(due_at) = payload.due_at.as_ref() {
        change.insert(
            "due_at".to_string(),
            due_at
                .map(|value| JsonValue::String(value.to_string()))
                .unwrap_or(JsonValue::Null),
        );
    }
    if let Some(project) = payload.project.as_ref() {
        change.insert("project".to_string(), JsonValue::String(project.clone()));
    }
    if let Some(commitment_kind) = payload.commitment_kind.as_ref() {
        change.insert(
            "commitment_kind".to_string(),
            JsonValue::String(commitment_kind.clone()),
        );
    }
    if let Some(metadata) = payload.metadata.as_ref() {
        change.insert("metadata".to_string(), metadata.clone());
    }
    JsonValue::Object(change)
}

pub async fn list_commitment_dependencies(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Vec<CommitmentDependencyData>>>, AppError> {
    let parent_id = id.trim();
    let _ = state
        .storage
        .get_commitment_by_id(parent_id)
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let rows = state
        .storage
        .list_commitment_dependencies_by_parent(parent_id)
        .await?;
    let data: Vec<CommitmentDependencyData> = rows
        .into_iter()
        .map(
            |(dep_id, child_id, dep_type, created_at)| CommitmentDependencyData {
                id: dep_id,
                parent_commitment_id: parent_id.to_string(),
                child_commitment_id: child_id,
                dependency_type: dep_type,
                created_at,
            },
        )
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn add_commitment_dependency(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<CommitmentDependencyCreateRequest>,
) -> Result<Json<ApiResponse<CommitmentDependencyData>>, AppError> {
    let parent_id = id.trim();
    let child_id = payload.child_commitment_id.trim();
    let _ = state
        .storage
        .get_commitment_by_id(parent_id)
        .await?
        .ok_or_else(|| AppError::not_found("parent commitment not found"))?;
    let _ = state
        .storage
        .get_commitment_by_id(child_id)
        .await?
        .ok_or_else(|| AppError::not_found("child commitment not found"))?;
    let dep_id = state
        .storage
        .insert_commitment_dependency(parent_id, child_id, &payload.dependency_type)
        .await?;
    let rows = state
        .storage
        .list_commitment_dependencies_by_parent(parent_id)
        .await?;
    let (_, _, dep_type, created_at) = rows
        .into_iter()
        .find(|(did, _, _, _)| did == &dep_id)
        .ok_or_else(|| AppError::internal("dependency not found after insert"))?;
    let data = CommitmentDependencyData {
        id: dep_id,
        parent_commitment_id: parent_id.to_string(),
        child_commitment_id: child_id.to_string(),
        dependency_type: dep_type,
        created_at,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
