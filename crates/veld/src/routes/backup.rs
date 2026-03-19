use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, BackupStatusData};

use crate::{
    errors::AppError,
    services::backup::{
        self, BackupCreateResultData, BackupInspectResultData, BackupRootInput,
        BackupVerifyResultData, CreateBackupInput,
    },
    state::AppState,
};

pub async fn create_backup(
    State(state): State<AppState>,
    Json(payload): Json<CreateBackupInput>,
) -> Result<Json<ApiResponse<BackupCreateResultData>>, AppError> {
    let result = backup::create_backup(&state, payload).await?;
    Ok(Json(ApiResponse::success(
        result,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn inspect_backup(
    State(state): State<AppState>,
    Json(payload): Json<BackupRootInput>,
) -> Result<Json<ApiResponse<BackupInspectResultData>>, AppError> {
    let result = backup::inspect_backup(&state, payload).await?;
    Ok(Json(ApiResponse::success(
        result,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn verify_backup(
    State(state): State<AppState>,
    Json(payload): Json<BackupRootInput>,
) -> Result<Json<ApiResponse<BackupVerifyResultData>>, AppError> {
    let result = backup::verify_backup(&state, payload).await?;
    Ok(Json(ApiResponse::success(
        result,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn backup_status(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<BackupStatusData>>, AppError> {
    let result = backup::backup_status(&state).await?;
    Ok(Json(ApiResponse::success(
        result,
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}
