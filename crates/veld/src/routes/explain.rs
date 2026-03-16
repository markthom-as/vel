//! Explainability: why a nudge fired, context state, commitment risk, drift.

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, CommitmentExplainData, ContextExplainData, DriftExplainData, NudgeExplainData,
    SignalExplainSummary,
};
use vel_storage::SignalRecord;

use crate::{errors::AppError, state::AppState};

/// GET /v1/explain/context — current context plus explanation (signals/commitments/risk that shaped it).
pub async fn explain_context(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ContextExplainData>>, AppError> {
    let row = state.storage.get_current_context().await?;
    let (computed_at, context_json) = row.map(|(ts, s)| (ts, s)).unwrap_or((0, "{}".to_string()));
    let context: serde_json::Value =
        serde_json::from_str(&context_json).unwrap_or(serde_json::json!({}));
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
    let mode = context
        .get("mode")
        .and_then(|v| v.as_str())
        .map(String::from);
    let morning_state = context
        .get("morning_state")
        .and_then(|v| v.as_str())
        .map(String::from);
    let mut reasons: Vec<String> = Vec::new();
    if let Some(ref m) = mode {
        reasons.push(format!("mode: {}", m));
    }
    if let Some(ref s) = morning_state {
        reasons.push(format!("morning_state: {}", s));
    }
    if context
        .get("prep_window_active")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        reasons.push("prep window active".to_string());
    }
    if context
        .get("next_commitment_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .is_some()
    {
        reasons.push("upcoming commitment".to_string());
    }
    if context.get("meds_status").and_then(|v| v.as_str()) == Some("pending") {
        reasons.push("meds commitment still open".to_string());
    }
    if reasons.is_empty() {
        reasons.push("Derived from signals, commitments, and active nudges.".to_string());
    }
    reasons.push(
        "Run `vel evaluate` to recompute; run `vel context timeline` for history.".to_string(),
    );
    let signal_summaries = hydrate_signal_summaries(&state, &signals_used).await?;
    let data = ContextExplainData {
        computed_at,
        mode,
        morning_state,
        context,
        signals_used,
        signal_summaries,
        commitments_used,
        risk_used,
        reasons,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/explain/commitment/:id — commitment details, latest risk, why it appears in context.
pub async fn explain_commitment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CommitmentExplainData>>, AppError> {
    let id = id.trim();
    let commitment = state
        .storage
        .get_commitment_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    // Read-only: use latest persisted risk snapshot for this commitment (do not call risk::run).
    let risk_rows = state.storage.list_commitment_risk_recent(id, 1).await?;
    let risk_value = match risk_rows.first() {
        Some((_, risk_score, risk_level, factors_json, _)) => {
            let mut factors: serde_json::Value =
                serde_json::from_str(factors_json).unwrap_or(serde_json::json!({}));
            factors["risk_score"] = serde_json::json!(risk_score);
            factors["risk_level"] = serde_json::json!(risk_level);
            factors
        }
        None => serde_json::json!({}),
    };
    let has_risk = !risk_rows.is_empty();
    let row = state.storage.get_current_context().await?;
    let context_json = row.map(|(_, s)| s).unwrap_or_else(|| "{}".to_string());
    let context: serde_json::Value =
        serde_json::from_str(&context_json).unwrap_or(serde_json::json!({}));
    let commitments_used: Vec<String> = context
        .get("commitments_used")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let top_risk: Vec<String> = context
        .get("top_risk_commitment_ids")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let mut in_context_reasons: Vec<String> = Vec::new();
    if commitments_used.iter().any(|c| c == id) {
        in_context_reasons.push("In commitments_used for current context.".to_string());
    }
    if top_risk.iter().any(|c| c == id) {
        in_context_reasons.push("In top_risk_commitment_ids.".to_string());
    }
    if in_context_reasons.is_empty() {
        in_context_reasons
            .push("Not in current context snapshot (run `vel evaluate` to refresh).".to_string());
    }
    let commitment_json = serde_json::json!({
        "id": commitment.id.as_ref(),
        "text": commitment.text,
        "status": format!("{:?}", commitment.status),
        "due_at": commitment.due_at.map(|t| t.unix_timestamp()),
        "project": commitment.project,
        "commitment_kind": commitment.commitment_kind,
    });
    let data = CommitmentExplainData {
        commitment_id: id.to_string(),
        commitment: commitment_json,
        risk: if has_risk { Some(risk_value) } else { None },
        in_context_reasons,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/explain/drift — current attention/drift state from context.
pub async fn explain_drift(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DriftExplainData>>, AppError> {
    let row = state.storage.get_current_context().await?;
    let (_, context_json) = row.unwrap_or((0, "{}".to_string()));
    let context: serde_json::Value =
        serde_json::from_str(&context_json).unwrap_or(serde_json::json!({}));
    let attention_state = context
        .get("attention_state")
        .and_then(|v| v.as_str())
        .map(String::from);
    let drift_type = context
        .get("drift_type")
        .and_then(|v| v.as_str())
        .map(String::from);
    let drift_severity = context
        .get("drift_severity")
        .and_then(|v| v.as_str())
        .map(String::from);
    let attention_confidence = context.get("attention_confidence").and_then(|v| v.as_f64());
    let reasons: Vec<String> = context
        .get("attention_reasons")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let signals_used: Vec<String> = context
        .get("signals_used")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let commitments_used: Vec<String> = context
        .get("commitments_used")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let signal_summaries = hydrate_signal_summaries(&state, &signals_used).await?;
    let data = DriftExplainData {
        attention_state,
        drift_type,
        drift_severity,
        confidence: attention_confidence,
        reasons,
        signals_used,
        signal_summaries,
        commitments_used,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

async fn hydrate_signal_summaries(
    state: &AppState,
    signal_ids: &[String],
) -> Result<Vec<SignalExplainSummary>, AppError> {
    let signals = state.storage.list_signals_by_ids(signal_ids).await?;
    Ok(signals.iter().map(signal_summary).collect())
}

fn signal_summary(signal: &SignalRecord) -> SignalExplainSummary {
    let payload = &signal.payload_json;
    let summary = match signal.signal_type.as_str() {
        "git_activity" => serde_json::json!({
            "repo": payload.get("repo_name").and_then(|value| value.as_str())
                .or_else(|| payload.get("repo").and_then(|value| value.as_str())),
            "branch": payload.get("branch").and_then(|value| value.as_str()),
            "operation": payload.get("operation").and_then(|value| value.as_str()),
            "message": payload.get("message").and_then(|value| value.as_str()),
            "files_changed": payload.get("files_changed").and_then(|value| value.as_u64()),
        }),
        "shell_login" | "computer_activity" | "idle_state" | "vel_invocation" => {
            serde_json::json!({
                "host": payload.get("host").and_then(|value| value.as_str()),
                "activity": payload.get("activity").and_then(|value| value.as_str()),
                "state": payload.get("state").and_then(|value| value.as_str()),
                "command": payload.get("command").and_then(|value| value.as_str()),
            })
        }
        "calendar_event" => serde_json::json!({
            "title": payload.get("title").and_then(|value| value.as_str()),
            "prep_minutes": payload.get("prep_minutes").and_then(|value| value.as_i64()),
            "travel_minutes": payload.get("travel_minutes").and_then(|value| value.as_i64()),
            "location": payload.get("location").and_then(|value| value.as_str()),
        }),
        "assistant_message" => serde_json::json!({
            "conversation_id": payload.get("conversation_id").and_then(|value| value.as_str()),
            "role": payload.get("role").and_then(|value| value.as_str()),
            "source": payload.get("source").and_then(|value| value.as_str()),
        }),
        "message_thread" => serde_json::json!({
            "platform": payload.get("platform").and_then(|value| value.as_str()),
            "title": payload.get("title").and_then(|value| value.as_str()),
            "waiting_state": payload.get("waiting_state").and_then(|value| value.as_str()),
            "scheduling_related": payload.get("scheduling_related").and_then(|value| value.as_bool()),
            "urgent": payload.get("urgent").and_then(|value| value.as_bool()),
            "snippet": payload.get("snippet").and_then(|value| value.as_str()),
        }),
        "note_document" => serde_json::json!({
            "path": payload.get("path").and_then(|value| value.as_str()),
            "title": payload.get("title").and_then(|value| value.as_str()),
        }),
        "external_task" => serde_json::json!({
            "task_id": payload.get("task_id").and_then(|value| value.as_str()),
            "content": payload.get("content").and_then(|value| value.as_str()),
            "project": payload.get("project").and_then(|value| value.as_str()),
            "checked": payload.get("checked").and_then(|value| value.as_bool()),
        }),
        _ => payload.clone(),
    };

    SignalExplainSummary {
        signal_id: signal.signal_id.clone(),
        signal_type: signal.signal_type.clone(),
        source: signal.source.clone(),
        timestamp: signal.timestamp,
        summary,
    }
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
