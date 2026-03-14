use axum::{
    extract::{Path, State},
    Json,
};
use time::OffsetDateTime;
use tracing::warn;
use uuid::Uuid;
use vel_api_types::{ApiResponse, CaptureCreateRequest, CaptureCreateResponse, ContextCapture};
use vel_core::PrivacyClass;
use vel_storage::CaptureInsert;

use crate::{errors::AppError, state::AppState};

pub async fn get_capture(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ContextCapture>>, AppError> {
    let capture = state
        .storage
        .get_capture_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("capture not found"))?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(capture, request_id)))
}

pub async fn create_capture(
    State(state): State<AppState>,
    Json(payload): Json<CaptureCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    if payload.content_text.trim().is_empty() {
        return Err(AppError::bad_request("capture text must not be empty"));
    }

    let capture_id = state
        .storage
        .insert_capture(CaptureInsert {
            content_text: payload.content_text.trim().to_string(),
            capture_type: payload.capture_type,
            source_device: payload.source_device,
            privacy_class: PrivacyClass::Private,
        })
        .await?;

    let payload_json = serde_json::json!({ "capture_id": capture_id.to_string() }).to_string();
    if let Err(e) = state
        .storage
        .emit_event("CAPTURE_CREATED", "capture", Some(capture_id.as_ref()), &payload_json)
        .await
    {
        warn!(error = %e, "failed to emit CAPTURE_CREATED event");
    }

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CaptureCreateResponse {
            capture_id,
            accepted_at: OffsetDateTime::now_utc(),
        },
        request_id,
    )))
}

