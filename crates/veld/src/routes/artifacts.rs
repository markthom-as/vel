use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{ApiResponse, ArtifactCreateRequest, ArtifactCreateResponse, ArtifactData};
use vel_storage::{ArtifactInsert, ArtifactRecord};

use crate::{errors::AppError, state::AppState};

fn artifact_record_to_data(r: ArtifactRecord) -> Result<ArtifactData, AppError> {
    Ok(ArtifactData {
        artifact_id: r.artifact_id,
        artifact_type: r.artifact_type,
        title: r.title,
        mime_type: r.mime_type,
        storage_uri: r.storage_uri,
        storage_kind: r.storage_kind.to_string(),
        privacy_class: r.privacy_class,
        sync_class: r.sync_class,
        content_hash: r.content_hash,
        size_bytes: r.size_bytes,
        created_at: OffsetDateTime::from_unix_timestamp(r.created_at)
            .map_err(|e| AppError::internal(e.to_string()))?,
        updated_at: OffsetDateTime::from_unix_timestamp(r.updated_at)
            .map_err(|e| AppError::internal(e.to_string()))?,
    })
}

#[derive(Debug, Deserialize)]
pub struct LatestArtifactQuery {
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct ListArtifactsQuery {
    #[serde(default = "default_artifact_limit")]
    pub limit: u32,
}

fn default_artifact_limit() -> u32 {
    500
}

pub async fn list_artifacts(
    State(state): State<AppState>,
    Query(q): Query<ListArtifactsQuery>,
) -> Result<Json<ApiResponse<Vec<ArtifactData>>>, AppError> {
    let limit = q.limit.min(1000);
    let records = state.storage.list_artifacts(limit).await?;
    let mut data = Vec::with_capacity(records.len());
    for r in records {
        data.push(ArtifactData {
            artifact_id: r.artifact_id,
            artifact_type: r.artifact_type,
            title: r.title,
            mime_type: r.mime_type,
            storage_uri: r.storage_uri,
            storage_kind: r.storage_kind.to_string(),
            privacy_class: r.privacy_class,
            sync_class: r.sync_class,
            content_hash: r.content_hash,
            size_bytes: r.size_bytes,
            created_at: OffsetDateTime::from_unix_timestamp(r.created_at)
                .map_err(|e| AppError::internal(e.to_string()))?,
            updated_at: OffsetDateTime::from_unix_timestamp(r.updated_at)
                .map_err(|e| AppError::internal(e.to_string()))?,
        });
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn create_artifact(
    State(state): State<AppState>,
    Json(payload): Json<ArtifactCreateRequest>,
) -> Result<Json<ApiResponse<ArtifactCreateResponse>>, AppError> {
    if payload.storage_uri.trim().is_empty() {
        return Err(AppError::bad_request("storage_uri must not be empty"));
    }

    let artifact_id = state
        .storage
        .create_artifact(ArtifactInsert {
            artifact_type: payload.artifact_type,
            title: payload.title,
            mime_type: payload.mime_type,
            storage_uri: payload.storage_uri.trim().to_string(),
            storage_kind: payload.storage_kind,
            privacy_class: payload.privacy_class,
            sync_class: payload.sync_class,
            content_hash: payload.content_hash,
            size_bytes: None,
            metadata_json: None,
        })
        .await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        ArtifactCreateResponse {
            artifact_id,
            created_at: OffsetDateTime::now_utc(),
        },
        request_id,
    )))
}

pub async fn get_artifact_latest(
    State(state): State<AppState>,
    Query(q): Query<LatestArtifactQuery>,
) -> Result<Json<ApiResponse<Option<ArtifactData>>>, AppError> {
    let record = state
        .storage
        .get_latest_artifact_by_type(q.r#type.trim())
        .await?;
    let data = record.map(artifact_record_to_data).transpose()?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_artifact(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ArtifactData>>, AppError> {
    let record = state
        .storage
        .get_artifact_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("artifact not found"))?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        ArtifactData {
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
                .map_err(|e| AppError::internal(e.to_string()))?,
            updated_at: OffsetDateTime::from_unix_timestamp(record.updated_at)
                .map_err(|e| AppError::internal(e.to_string()))?,
        },
        request_id,
    )))
}
