use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, CaptureCreateResponse, MoodJournalCreateRequest, PainJournalCreateRequest,
    WatchSignalCreateRequest,
};

use crate::{errors::AppError, services::journal, state::AppState};

pub async fn create_mood_journal(
    State(state): State<AppState>,
    Json(payload): Json<MoodJournalCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    let data = journal::record_mood(
        &state.storage,
        journal::MoodJournalInput {
            score: payload.score,
            label: payload.label,
            note: payload.note,
            source_device: payload.source_device,
        },
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CaptureCreateResponse {
            capture_id: data.capture_id,
            accepted_at: data.accepted_at,
        },
        request_id,
    )))
}

pub async fn create_pain_journal(
    State(state): State<AppState>,
    Json(payload): Json<PainJournalCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    let data = journal::record_pain(
        &state.storage,
        journal::PainJournalInput {
            severity: payload.severity,
            location: payload.location,
            note: payload.note,
            source_device: payload.source_device,
        },
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CaptureCreateResponse {
            capture_id: data.capture_id,
            accepted_at: data.accepted_at,
        },
        request_id,
    )))
}

pub async fn create_watch_signal_journal(
    State(state): State<AppState>,
    Json(payload): Json<WatchSignalCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    let data = journal::record_watch_signal(
        &state.storage,
        journal::WatchSignalJournalInput {
            signal_type: payload.signal_type,
            note: payload.note,
            context: payload.context,
            source_device: payload.source_device,
        },
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CaptureCreateResponse {
            capture_id: data.capture_id,
            accepted_at: data.accepted_at,
        },
        request_id,
    )))
}
