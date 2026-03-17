//! Read-only explain helpers shared by API routes and command execution.

use vel_core::{CurrentContextV1};
use vel_storage::SignalRecord;

use crate::{errors::AppError, services::risk::snapshot_from_row, state::AppState};

#[derive(Debug, Clone, serde::Serialize)]
pub struct AdaptivePolicyOverride {
    pub policy_key: String,
    pub value_minutes: u32,
    pub source_suggestion_id: Option<String>,
    pub source_title: Option<String>,
    pub source_accepted_at: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SignalSummary {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub timestamp: i64,
    pub summary: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextSourceSummary {
    pub timestamp: i64,
    pub summary: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextSourceSummaries {
    pub git_activity: Option<ContextSourceSummary>,
    pub health: Option<ContextSourceSummary>,
    pub mood: Option<ContextSourceSummary>,
    pub pain: Option<ContextSourceSummary>,
    pub note_document: Option<ContextSourceSummary>,
    pub assistant_message: Option<ContextSourceSummary>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextExplain {
    pub computed_at: i64,
    pub mode: String,
    pub morning_state: String,
    pub context: serde_json::Value,
    pub source_summaries: ContextSourceSummaries,
    pub adaptive_policy_overrides: Vec<AdaptivePolicyOverride>,
    pub signals_used: Vec<String>,
    pub signal_summaries: Vec<SignalSummary>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CommitmentExplain {
    pub commitment_id: String,
    pub commitment: serde_json::Value,
    pub risk: Option<serde_json::Value>,
    pub in_context_reasons: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DriftExplain {
    pub attention_state: String,
    pub drift_type: Option<String>,
    pub drift_severity: Option<String>,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
    pub signals_used: Vec<String>,
    pub signal_summaries: Vec<SignalSummary>,
    pub commitments_used: Vec<String>,
}

pub async fn explain_context_data(state: &AppState) -> Result<ContextExplain, AppError> {
    let row = state.storage.get_current_context().await?;
    let (computed_at, context) = row.unwrap_or((0, CurrentContextV1::default()));
    let signals_used = context.signals_used.clone();
    let commitments_used = context.commitments_used.clone();
    let risk_used = context.risk_used.clone();
    
    let mut reasons: Vec<String> = Vec::new();
    if !context.mode.is_empty() {
        reasons.push(format!("mode: {}", context.mode));
    }
    if !context.morning_state.is_empty() {
        reasons.push(format!("morning_state: {}", context.morning_state));
    }
    if context.prep_window_active {
        reasons.push("prep window active".to_string());
    }
    if context.next_commitment_id.is_some() {
        reasons.push("upcoming commitment".to_string());
    }
    if context.meds_status == "pending" {
        reasons.push("meds commitment still open".to_string());
    }
    if reasons.is_empty() {
        reasons.push("Derived from signals, commitments, and active nudges.".to_string());
    }
    reasons.push(
        "Run `vel evaluate` to recompute; run `vel context timeline` for history.".to_string(),
    );

    Ok(ContextExplain {
        computed_at,
        mode: context.mode.clone(),
        morning_state: context.morning_state.clone(),
        context: context.clone().into_json(),
        source_summaries: context_source_summaries(&context),
        adaptive_policy_overrides: adaptive_policy_overrides(state).await?,
        signals_used: signals_used.clone(),
        signal_summaries: hydrate_signal_summaries(state, &signals_used).await?,
        commitments_used,
        risk_used,
        reasons,
    })
}

pub async fn explain_commitment_data(
    state: &AppState,
    id: &str,
) -> Result<CommitmentExplain, AppError> {
    let id = id.trim();
    let commitment = state
        .storage
        .get_commitment_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let risk_rows = state.storage.list_commitment_risk_recent(id, 1).await?;
    let risk_value = match risk_rows.first() {
        Some((_, risk_score, risk_level, factors_json, _)) => {
            let snapshot = snapshot_from_row(
                id.to_string(),
                *risk_score,
                risk_level.clone(),
                factors_json,
                None,
            );
            serde_json::json!({
                "commitment_id": snapshot.commitment_id,
                "risk_score": snapshot.risk_score,
                "risk_level": snapshot.normalized_level(),
                "factors": {
                    "consequence": snapshot.factors.consequence,
                    "proximity": snapshot.factors.proximity,
                    "dependency_pressure": snapshot.factors.dependency_pressure,
                    "external_anchor": snapshot.factors.external_anchor,
                    "stale_open_age": snapshot.factors.stale_open_age,
                    "reasons": snapshot.factors.reasons,
                    "dependency_ids": snapshot.factors.dependency_ids,
                },
                "computed_at": snapshot.computed_at,
            })
        }
        None => serde_json::json!({}),
    };
    let has_risk = !risk_rows.is_empty();
    let row = state.storage.get_current_context().await?;
    let context = row.map(|(_, c)| c).unwrap_or_else(CurrentContextV1::default);
    
    let commitments_used = context.commitments_used.clone();
    let top_risk = context.top_risk_commitment_ids.clone();
    
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

    Ok(CommitmentExplain {
        commitment_id: id.to_string(),
        commitment: commitment_json,
        risk: if has_risk { Some(risk_value) } else { None },
        in_context_reasons,
    })
}

pub async fn explain_drift_data(state: &AppState) -> Result<DriftExplain, AppError> {
    let row = state.storage.get_current_context().await?;
    let (_, context) = row.unwrap_or((0, CurrentContextV1::default()));
    
    let signals_used = context.signals_used.clone();
    let commitments_used = context.commitments_used.clone();

    Ok(DriftExplain {
        attention_state: context.attention_state.clone(),
        drift_type: context.drift_type.clone(),
        drift_severity: context.drift_severity.clone(),
        confidence: context.attention_confidence,
        reasons: context.attention_reasons.clone(),
        signals_used: signals_used.clone(),
        signal_summaries: hydrate_signal_summaries(state, &signals_used).await?,
        commitments_used,
    })
}

async fn hydrate_signal_summaries(
    state: &AppState,
    signal_ids: &[String],
) -> Result<Vec<SignalSummary>, AppError> {
    let signals = state.storage.list_signals_by_ids(signal_ids).await?;
    Ok(signals.iter().map(signal_summary).collect())
}

fn context_source_summaries(
    context: &CurrentContextV1,
) -> ContextSourceSummaries {
    ContextSourceSummaries {
        git_activity: context_source_summary(
            context,
            "git_activity_summary",
            context.git_activity_summary.clone(),
        ),
        health: context_source_summary(
            context,
            "health_summary",
            context.health_summary.clone(),
        ),
        mood: context_source_summary(
            context,
            "mood_summary",
            context.mood_summary.clone(),
        ),
        pain: context_source_summary(
            context,
            "pain_summary",
            context.pain_summary.clone(),
        ),
        note_document: context_source_summary(
            context,
            "note_document_summary",
            context.note_document_summary.clone(),
        ),
        assistant_message: context_source_summary(
            context,
            "assistant_message_summary",
            context.assistant_message_summary.clone(),
        ),
    }
}

async fn adaptive_policy_overrides(
    state: &AppState,
) -> Result<Vec<AdaptivePolicyOverride>, AppError> {
    let overrides = crate::services::adaptive_policies::load(&state.storage).await?;
    let mut items = Vec::new();
    if let Some(value_minutes) = overrides.commute_buffer_minutes {
        items.push(AdaptivePolicyOverride {
            policy_key: "commute_buffer".to_string(),
            value_minutes,
            source_suggestion_id: overrides.commute_buffer_source_suggestion_id,
            source_title: overrides.commute_buffer_source_title,
            source_accepted_at: overrides.commute_buffer_source_accepted_at,
        });
    }
    if let Some(value_minutes) = overrides.default_prep_minutes {
        items.push(AdaptivePolicyOverride {
            policy_key: "default_prep".to_string(),
            value_minutes,
            source_suggestion_id: overrides.default_prep_source_suggestion_id,
            source_title: overrides.default_prep_source_title,
            source_accepted_at: overrides.default_prep_source_accepted_at,
        });
    }
    Ok(items)
}

fn context_source_summary(
    context: &CurrentContextV1,
    key: &str,
    typed_summary: Option<serde_json::Value>,
) -> Option<ContextSourceSummary> {
    let summary = typed_summary.or_else(|| context.extra.get(key).cloned())?;
    let timestamp = summary.get("timestamp").and_then(|value| value.as_i64()).unwrap_or(context.computed_at);
    Some(ContextSourceSummary { timestamp, summary })
}

fn signal_summary(signal: &SignalRecord) -> SignalSummary {
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
        "health_metric" => serde_json::json!({
            "metric_type": payload.get("metric_type").and_then(|value| value.as_str()),
            "value": payload.get("value"),
            "unit": payload.get("unit").and_then(|value| value.as_str()),
            "source_app": payload.get("source_app").and_then(|value| value.as_str()),
            "device": payload.get("device").and_then(|value| value.as_str()),
        }),
        "mood_log" => serde_json::json!({
            "score": payload.get("score").and_then(|value| value.as_u64()),
            "label": payload.get("label").and_then(|value| value.as_str()),
            "note": payload.get("note").and_then(|value| value.as_str()),
        }),
        "pain_log" => serde_json::json!({
            "severity": payload.get("severity").and_then(|value| value.as_u64()),
            "location": payload.get("location").and_then(|value| value.as_str()),
            "note": payload.get("note").and_then(|value| value.as_str()),
        }),
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

    SignalSummary {
        signal_id: signal.signal_id.clone(),
        signal_type: signal.signal_type.clone(),
        source: signal.source.clone(),
        timestamp: signal.timestamp,
        summary,
    }
}
