//! Explainability: why a nudge fired, context state, commitment risk, drift.

use crate::routes::response;
use crate::{errors::AppError, services, state::AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use vel_api_types::{
    ApiResponse, CommitmentExplainData, ContextExplainData, DriftExplainData, NudgeEventData,
    NudgeExplainData,
};

fn map_signal_summary(
    value: services::explain::SignalSummary,
) -> vel_api_types::SignalExplainSummary {
    vel_api_types::SignalExplainSummary {
        signal_id: value.signal_id,
        signal_type: value.signal_type,
        source: value.source,
        timestamp: value.timestamp,
        summary: value.summary,
    }
}

fn map_context_source_summary(
    value: services::explain::ContextSourceSummary,
) -> vel_api_types::ContextSourceSummaryData {
    vel_api_types::ContextSourceSummaryData {
        timestamp: value.timestamp,
        summary: value.summary,
    }
}

fn map_context_explain(value: services::explain::ContextExplain) -> ContextExplainData {
    ContextExplainData {
        computed_at: value.computed_at,
        mode: value.mode,
        morning_state: value.morning_state,
        context: value.context,
        source_summaries: vel_api_types::ContextSourceSummariesData {
            git_activity: value
                .source_summaries
                .git_activity
                .map(map_context_source_summary),
            health: value
                .source_summaries
                .health
                .map(map_context_source_summary),
            mood: value.source_summaries.mood.map(map_context_source_summary),
            pain: value.source_summaries.pain.map(map_context_source_summary),
            note_document: value
                .source_summaries
                .note_document
                .map(map_context_source_summary),
            assistant_message: value
                .source_summaries
                .assistant_message
                .map(map_context_source_summary),
        },
        adaptive_policy_overrides: value
            .adaptive_policy_overrides
            .into_iter()
            .map(|item| vel_api_types::AdaptivePolicyOverrideData {
                policy_key: item.policy_key,
                value_minutes: item.value_minutes,
                source_suggestion_id: item.source_suggestion_id,
                source_title: item.source_title,
                source_accepted_at: item.source_accepted_at,
            })
            .collect(),
        signals_used: value.signals_used,
        signal_summaries: value
            .signal_summaries
            .into_iter()
            .map(map_signal_summary)
            .collect(),
        commitments_used: value.commitments_used,
        risk_used: value.risk_used,
        reasons: value.reasons,
    }
}

fn map_commitment_explain(value: services::explain::CommitmentExplain) -> CommitmentExplainData {
    CommitmentExplainData {
        commitment_id: value.commitment_id,
        commitment: value.commitment,
        risk: value.risk,
        in_context_reasons: value.in_context_reasons,
    }
}

fn map_drift_explain(value: services::explain::DriftExplain) -> DriftExplainData {
    DriftExplainData {
        attention_state: value.attention_state,
        drift_type: value.drift_type,
        drift_severity: value.drift_severity,
        confidence: value.confidence,
        reasons: value.reasons,
        signals_used: value.signals_used,
        signal_summaries: value
            .signal_summaries
            .into_iter()
            .map(map_signal_summary)
            .collect(),
        commitments_used: value.commitments_used,
    }
}

/// GET /v1/explain/context — current context plus explanation (signals/commitments/risk that shaped it).
pub async fn explain_context(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ContextExplainData>>, AppError> {
    let data = map_context_explain(services::explain::explain_context_data(&state).await?);
    Ok(response::success(data))
}

/// GET /v1/explain/commitment/:id — commitment details, latest risk, why it appears in context.
pub async fn explain_commitment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CommitmentExplainData>>, AppError> {
    let data = map_commitment_explain(
        services::explain::explain_commitment_data(&state, id.trim()).await?,
    );
    Ok(response::success(data))
}

/// GET /v1/explain/drift — current attention/drift state from context.
pub async fn explain_drift(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DriftExplainData>>, AppError> {
    let data = map_drift_explain(services::explain::explain_drift_data(&state).await?);
    Ok(response::success(data))
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
    Ok(response::success(data))
}
