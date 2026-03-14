use axum::{
    extract::{Path, State},
    Json,
};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ArtifactCreateRequest, ArtifactCreateResponse, ArtifactData,
};
use vel_storage::ArtifactInsert;

use crate::{errors::AppError, state::AppState};

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
            created_at: OffsetDateTime::from_unix_timestamp(record.created_at)
                .map_err(|e| AppError::internal(e.to_string()))?,
            updated_at: OffsetDateTime::from_unix_timestamp(record.updated_at)
                .map_err(|e| AppError::internal(e.to_string()))?,
        },
        request_id,
    )))
}
