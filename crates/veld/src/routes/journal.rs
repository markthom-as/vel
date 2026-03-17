use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, CaptureCreateResponse, MoodJournalCreateRequest, PainJournalCreateRequest,
};

use crate::{errors::AppError, services::journal, state::AppState};

pub async fn create_mood_journal(
    State(state): State<AppState>,
    Json(payload): Json<MoodJournalCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    let data = journal::record_mood(&state.storage, payload).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn create_pain_journal(
    State(state): State<AppState>,
    Json(payload): Json<PainJournalCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    let data = journal::record_pain(&state.storage, payload).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
