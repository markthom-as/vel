use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use time::OffsetDateTime;
use tracing::warn;
use uuid::Uuid;
use vel_api_types::{ApiResponse, CaptureCreateRequest, CaptureCreateResponse, ContextCapture};
use vel_core::{CommitmentStatus, PrivacyClass};
use vel_storage::{CaptureInsert, CommitmentInsert, SignalInsert};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct RecentCapturesQuery {
    #[serde(default = "default_recent_limit")]
    pub limit: u32,
    #[serde(default)]
    pub today: bool,
}

fn default_recent_limit() -> u32 {
    20
}

pub async fn list_captures(
    State(state): State<AppState>,
    Query(q): Query<RecentCapturesQuery>,
) -> Result<Json<ApiResponse<Vec<ContextCapture>>>, AppError> {
    let captures = state.storage.list_captures_recent(q.limit, q.today).await?;
    let data: Vec<vel_api_types::ContextCapture> = captures
        .into_iter()
        .map(vel_api_types::ContextCapture::from)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

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
    Ok(Json(ApiResponse::success(
        vel_api_types::ContextCapture::from(capture),
        request_id,
    )))
}

pub async fn create_capture(
    State(state): State<AppState>,
    Json(payload): Json<CaptureCreateRequest>,
) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    if payload.content_text.trim().is_empty() {
        return Err(AppError::bad_request("capture text must not be empty"));
    }

    let capture_type = payload.capture_type.clone();
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
        .emit_event(
            "CAPTURE_CREATED",
            "capture",
            Some(capture_id.as_ref()),
            &payload_json,
        )
        .await
    {
        warn!(error = %e, "failed to emit CAPTURE_CREATED event");
    }

    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let signal_payload = serde_json::json!({
        "capture_id": capture_id.to_string(),
        "content": payload.content_text.trim(),
        "tags": []
    });
    if let Err(e) = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: "capture_created".to_string(),
            source: "vel".to_string(),
            source_ref: Some(capture_id.to_string()),
            timestamp: now_ts,
            payload_json: Some(signal_payload),
        })
        .await
    {
        warn!(error = %e, "failed to insert capture_created signal");
    }

    // Capture promotion: todo captures auto-create an open commitment.
    if capture_type == "todo" {
        let content = payload.content_text.trim().to_string();
        if let Err(e) = state
            .storage
            .insert_commitment(CommitmentInsert {
                text: content,
                source_type: "capture".to_string(),
                source_id: Some(capture_id.to_string()),
                status: CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({ "capture_id": capture_id.to_string() })),
            })
            .await
        {
            warn!(error = %e, "failed to create commitment from todo capture");
        }
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
