use axum::{
    extract::{Path, State},
    Json,
};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{ApiResponse, NudgeData, NudgeSnoozeRequest};

use crate::{errors::AppError, state::AppState};

fn nudge_record_to_data(r: vel_storage::NudgeRecord) -> NudgeData {
    NudgeData {
        nudge_id: r.nudge_id,
        nudge_type: r.nudge_type,
        level: r.level,
        state: r.state,
        related_commitment_id: r.related_commitment_id,
        message: r.message,
        created_at: r.created_at,
        snoozed_until: r.snoozed_until,
        resolved_at: r.resolved_at,
    }
}

pub async fn list_nudges(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<NudgeData>>>, AppError> {
    // Return operator-facing nudge history with unresolved items first.
    let active = state.storage.list_nudges(Some("active"), 50).await?;
    let pending = state.storage.list_nudges(Some("pending"), 50).await?;
    let snoozed = state.storage.list_nudges(Some("snoozed"), 50).await?;
    let resolved = state.storage.list_nudges(Some("resolved"), 50).await?;
    let mut data: Vec<NudgeData> = active.into_iter().map(nudge_record_to_data).collect();
    data.extend(pending.into_iter().map(nudge_record_to_data));
    data.extend(snoozed.into_iter().map(nudge_record_to_data));
    data.extend(resolved.into_iter().map(nudge_record_to_data));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_nudge(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<NudgeData>>, AppError> {
    let nudge = state
        .storage
        .get_nudge(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("nudge not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        nudge_record_to_data(nudge),
        request_id,
    )))
}

pub async fn nudge_done(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<NudgeData>>, AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    state
        .storage
        .update_nudge_state(id.trim(), "resolved", None, Some(now))
        .await?;
    let _ = state
        .storage
        .insert_nudge_event(id.trim(), "nudge_resolved", "{}", now)
        .await;
    if let Err(e) = state
        .storage
        .emit_event("NUDGE_RESOLVED", "nudge", Some(id.trim()), "{}")
        .await
    {
        tracing::warn!(error = %e, "emit NUDGE_RESOLVED");
    }
    let nudge = state
        .storage
        .get_nudge(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("nudge not found"))?;
    if let Some(com_id) = &nudge.related_commitment_id {
        let _ = state
            .storage
            .update_commitment(
                com_id,
                None,
                Some(vel_core::CommitmentStatus::Done),
                None,
                None,
                None,
                None,
            )
            .await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        nudge_record_to_data(nudge),
        request_id,
    )))
}

pub async fn nudge_snooze(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<NudgeSnoozeRequest>,
) -> Result<Json<ApiResponse<NudgeData>>, AppError> {
    let now = OffsetDateTime::now_utc();
    let now_ts = now.unix_timestamp();
    let snoozed_until = now + time::Duration::minutes(payload.minutes as i64);
    let ts = snoozed_until.unix_timestamp();
    state
        .storage
        .update_nudge_state(id.trim(), "snoozed", Some(ts), None)
        .await?;
    let _ = state
        .storage
        .insert_nudge_event(
            id.trim(),
            "nudge_snoozed",
            &serde_json::json!({ "snoozed_until": ts, "minutes": payload.minutes }).to_string(),
            now_ts,
        )
        .await;
    let nudge = state
        .storage
        .get_nudge(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("nudge not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        nudge_record_to_data(nudge),
        request_id,
    )))
}
