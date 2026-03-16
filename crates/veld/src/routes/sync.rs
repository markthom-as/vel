use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, SyncResultData};

use crate::{adapters, errors::AppError, state::AppState};

pub async fn sync_calendar(State(state): State<AppState>) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = adapters::calendar::ingest(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "calendar".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_todoist(State(state): State<AppState>) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = adapters::todoist::ingest(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "todoist".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_activity(State(state): State<AppState>) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = adapters::activity::ingest(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "activity".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_transcripts(State(state): State<AppState>) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = adapters::transcripts::ingest(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "transcripts".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}
