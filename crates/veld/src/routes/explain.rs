//! Explainability: why a nudge fired, context state, etc.

use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, ContextExplainData, NudgeExplainData};

use crate::{errors::AppError, state::AppState};

/// GET /v1/explain/context — current context plus explanation (signals/commitments/risk that shaped it).
pub async fn explain_context(State(state): State<AppState>) -> Result<Json<ApiResponse<ContextExplainData>>, AppError> {
    let row = state.storage.get_current_context().await?;
    let (computed_at, context_json) = row
        .map(|(ts, s)| (ts, s))
        .unwrap_or((0, "{}".to_string()));
    let context: serde_json::Value = serde_json::from_str(&context_json).unwrap_or(serde_json::json!({}));
    let signals_used: Vec<String> = context
        .get("signals_used")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let commitments_used: Vec<String> = context
        .get("commitments_used")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let risk_used: Vec<String> = context
        .get("risk_used")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let mode = context.get("mode").and_then(|v| v.as_str()).map(String::from);
    let morning_state = context.get("morning_state").and_then(|v| v.as_str()).map(String::from);
    let mut reasons: Vec<String> = Vec::new();
    if let Some(ref m) = mode {
        reasons.push(format!("mode: {}", m));
    }
    if let Some(ref s) = morning_state {
        reasons.push(format!("morning_state: {}", s));
    }
    if context.get("prep_window_active").and_then(|v| v.as_bool()).unwrap_or(false) {
        reasons.push("prep window active".to_string());
    }
    if context.get("next_commitment_id").and_then(|v| v.as_str()).filter(|s| !s.is_empty()).is_some() {
        reasons.push("upcoming commitment".to_string());
    }
    if context.get("meds_status").and_then(|v| v.as_str()) == Some("pending") {
        reasons.push("meds commitment still open".to_string());
    }
    if reasons.is_empty() {
        reasons.push("Derived from signals, commitments, and active nudges.".to_string());
    }
    reasons.push("Run `vel evaluate` to recompute; run `vel context timeline` for history.".to_string());
    let data = ContextExplainData {
        computed_at,
        mode,
        morning_state,
        context,
        signals_used,
        commitments_used,
        risk_used,
        reasons,
    };
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
    let data = NudgeExplainData {
        nudge_id: nudge.nudge_id,
        nudge_type: nudge.nudge_type,
        level: nudge.level,
        state: nudge.state,
        message: nudge.message,
        inference_snapshot,
        signals_snapshot,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
