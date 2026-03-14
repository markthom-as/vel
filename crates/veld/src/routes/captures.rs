use axum::{extract::State, Json};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{ApiResponse, CaptureCreateRequest, CaptureCreateResponse};
use vel_core::PrivacyClass;
use vel_storage::CaptureInsert;

use crate::{errors::AppError, state::AppState};

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

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        CaptureCreateResponse {
            capture_id,
            accepted_at: OffsetDateTime::now_utc(),
        },
        request_id,
    )))
}

