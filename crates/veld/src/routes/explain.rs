//! Explainability: why a nudge fired, context state, commitment risk, drift.

use crate::{errors::AppError, services, state::AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, CommitmentExplainData, ContextExplainData, DriftExplainData, NudgeEventData,
    NudgeExplainData,
};

/// GET /v1/explain/context — current context plus explanation (signals/commitments/risk that shaped it).
pub async fn explain_context(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ContextExplainData>>, AppError> {
    let data = services::explain::explain_context_data(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/explain/commitment/:id — commitment details, latest risk, why it appears in context.
pub async fn explain_commitment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CommitmentExplainData>>, AppError> {
    let data = services::explain::explain_commitment_data(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/explain/drift — current attention/drift state from context.
pub async fn explain_drift(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DriftExplainData>>, AppError> {
    let data = services::explain::explain_drift_data(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn explain_nudge(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<NudgeExplainData>>, AppError> {
    let nudge = state
        .storage
        .get_nudge(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("nudge not found"))?;
    let inference_snapshot = nudge
        .inference_snapshot_json
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok());
    let signals_snapshot = nudge
        .signals_snapshot_json
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok());
    let events = state
        .storage
        .list_nudge_events(id.trim(), 50)
        .await?
        .into_iter()
        .map(|event| NudgeEventData {
            id: event.id,
            event_type: event.event_type,
            payload: event.payload_json,
            timestamp: event.timestamp,
            created_at: event.created_at,
        })
        .collect();
    let data = NudgeExplainData {
        nudge_id: nudge.nudge_id,
        nudge_type: nudge.nudge_type,
        level: nudge.level,
        state: nudge.state,
        message: nudge.message,
        inference_snapshot,
        signals_snapshot,
        events,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
