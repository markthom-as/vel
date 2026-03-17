//! Inference engine: signals + commitments + time -> inferred state and canonical current context (Phase C).
//! See docs/specs/vel-current-context-spec.md for canonical shape and material-change rules.
//!
//! **Boundary: recompute-and-persist.** This module must only be called from the evaluate
//! orchestration (e.g. [crate::services::evaluate::run]). Never call from explain or read routes.

use time::OffsetDateTime;
use vel_core::{CommitmentStatus, RiskSnapshot};
use vel_storage::{InferredStateInsert, Storage};

const RECENT_GIT_ACTIVITY_WINDOW_SECS: i64 = 90 * 60;
const DEFAULT_PREP_MINUTES: i64 = 15;
const DEFAULT_TRAVEL_MINUTES: i64 = 0;
const COMMUTE_WINDOW_LEAD_SECS: i64 = 15 * 60;
struct InferenceInputs {
    open_commitments: Vec<vel_core::Commitment>,
    medication_commitments: Vec<vel_core::Commitment>,
    signals_today: Vec<vel_storage::SignalRecord>,
    active_nudges: Vec<vel_storage::NudgeRecord>,
    snoozed_nudges: Vec<vel_storage::NudgeRecord>,
    risk_snapshots: Vec<RiskSnapshot>,
}

struct TemporalWindows {
    prep_window_active: bool,
    commute_window_active: bool,
    leave_by_ts: Option<i64>,
    next_event_start_ts: Option<i64>,
}

struct MessageSummary {
    waiting_on_me_count: usize,
    waiting_on_others_count: usize,
    scheduling_thread_count: usize,
    urgent_thread_count: usize,
    top_threads: Vec<serde_json::Value>,
}

struct AttentionState {
    attention_state: &'static str,
    drift_type: Option<&'static str>,
    drift_severity: Option<&'static str>,
    confidence: f64,
    reasons: Vec<&'static str>,
}

struct GlobalRiskSummary {
    level: &'static str,
    score: Option<f64>,
    missing: bool,
}

/// **Recompute-and-persist.** Run inference once: compute morning state, meds status, prep window;
/// build canonical current context; persist inferred_state and current_context; append to context_timeline on material change.
/// Returns count of state records written. Only call from evaluate orchestration.
pub async fn run(storage: &Storage) -> Result<usize, crate::errors::AppError> {
    let now = OffsetDateTime::now_utc();
    let now_ts = now.unix_timestamp();
    let start_of_today = now
        .date()
        .with_hms(0, 0, 0)
        .map_err(|e| crate::errors::AppError::internal(e.to_string()))?
        .assume_utc()
        .unix_timestamp();

    let inputs = collect_inputs(storage, start_of_today).await?;
    let InferenceInputs {
        open_commitments,
        medication_commitments,
        signals_today,
        active_nudges,
        snoozed_nudges,
        risk_snapshots,
    } = inputs;

    let has_workstation_activity = signals_today.iter().any(|s| {
        matches!(
            s.signal_type.as_str(),
            "vel_invocation" | "shell_login" | "computer_activity" | "git_activity"
        )
    });
    let meds_status = derive_meds_status(&open_commitments, &medication_commitments, now);
    let meds_done_today = meds_status == "done";
    let meds_pending = meds_status == "pending";

    let calendar_events: Vec<_> = signals_today
        .iter()
        .filter(|s| s.signal_type == "calendar_event")
        .collect();
    let message_threads: Vec<_> = signals_today
        .iter()
        .filter(|s| s.signal_type == "message_thread")
        .collect();
    let latest_git_activity = signals_today
        .iter()
        .filter(|s| s.signal_type == "git_activity")
        .max_by_key(|s| s.timestamp);
    let first_event = select_next_event(&calendar_events, now_ts);
    let temporal_windows = derive_temporal_windows(first_event, now_ts);

    let morning_started = has_workstation_activity || meds_done_today;
    let state_name = if temporal_windows.prep_window_active && !morning_started {
        "at_risk"
    } else if morning_started {
        "engaged"
    } else if first_event.is_some() && !morning_started {
        "awake_unstarted"
    } else {
        "inactive"
    };

    let recent_git_summary = latest_git_activity
        .and_then(build_git_activity_summary)
        .filter(|summary| now_ts - summary.timestamp <= RECENT_GIT_ACTIVITY_WINDOW_SECS);
    let git_activity_summary = latest_git_activity.and_then(build_git_activity_summary);
    let inferred_activity = if recent_git_summary.is_some() {
        "coding"
    } else if has_workstation_activity {
        "computer_active"
    } else {
        "unknown"
    };
    let message_summary = derive_message_summary(&message_threads);

    let mode = if temporal_windows.prep_window_active {
        "meeting_mode"
    } else if temporal_windows.commute_window_active {
        "commute_mode"
    } else {
        "morning_mode"
    };

    let next_commitment = select_next_commitment(&open_commitments, &risk_snapshots, now_ts);
    let next_commitment_id = next_commitment.map(|c| c.id.as_ref().to_string());
    let next_commitment_due_at = next_commitment.and_then(|c| c.due_at.map(|t| t.unix_timestamp()));

    let active_nudge_ids: Vec<String> = active_nudges
        .iter()
        .chain(snoozed_nudges.iter())
        .map(|n| n.nudge_id.clone())
        .collect();

    let top_risk_commitment_ids: Vec<String> = risk_snapshots
        .iter()
        .map(|snapshot| snapshot.commitment_id.clone())
        .take(10)
        .collect();
    let risk_used: Vec<String> = risk_snapshots
        .iter()
        .map(|snapshot| snapshot.commitment_id.clone())
        .take(50)
        .collect();
    let global_risk = derive_global_risk_summary(&risk_snapshots);

    let signals_used: Vec<String> = signals_today
        .iter()
        .filter(|s| {
            matches!(
                s.signal_type.as_str(),
                "calendar_event"
                    | "vel_invocation"
                    | "shell_login"
                    | "computer_activity"
                    | "git_activity"
                    | "message_thread"
            )
        })
        .take(50)
        .map(|s| s.signal_id.clone())
        .collect();
    let commitments_used: Vec<String> = open_commitments
        .iter()
        .take(20)
        .map(|c| c.id.as_ref().to_string())
        .collect();

    let attention = derive_attention_state(
        state_name,
        temporal_windows.prep_window_active,
        meds_pending,
    );

    let context = build_current_context(
        now_ts,
        mode,
        state_name,
        inferred_activity,
        next_commitment_id,
        next_commitment_due_at,
        meds_status,
        &active_nudge_ids,
        &top_risk_commitment_ids,
        &global_risk,
        &signals_used,
        &commitments_used,
        &risk_used,
        &attention,
        git_activity_summary.as_ref(),
        &message_summary,
        &temporal_windows,
    );

    persist_inference_outputs(storage, now_ts, state_name, &context).await?;

    Ok(1)
}

async fn collect_inputs(
    storage: &Storage,
    start_of_today: i64,
) -> Result<InferenceInputs, crate::errors::AppError> {
    Ok(InferenceInputs {
        open_commitments: storage
            .list_commitments(Some(CommitmentStatus::Open), None, None, 200)
            .await?,
        medication_commitments: storage
            .list_commitments(None, None, Some("medication"), 100)
            .await?,
        signals_today: storage
            .list_signals(None, Some(start_of_today), 500)
            .await?,
        active_nudges: storage.list_nudges(Some("active"), 50).await?,
        snoozed_nudges: storage.list_nudges(Some("snoozed"), 50).await?,
        risk_snapshots: crate::services::risk::list_latest_snapshots(storage)
            .await
            .unwrap_or_default(),
    })
}

#[derive(Clone)]
struct GitActivitySummary {
    timestamp: i64,
    repo: String,
    branch: Option<String>,
    operation: Option<String>,
    message: Option<String>,
    files_changed: Option<u32>,
    insertions: Option<u32>,
    deletions: Option<u32>,
}

fn derive_meds_status(
    open_commitments: &[vel_core::Commitment],
    medication_commitments: &[vel_core::Commitment],
    now: OffsetDateTime,
) -> &'static str {
    let meds_open = open_commitments
        .iter()
        .any(|commitment| commitment.commitment_kind.as_deref() == Some("medication"));
    let meds_done_today = medication_commitments.iter().any(|commitment| {
        commitment.status == vel_core::CommitmentStatus::Done
            && commitment
                .resolved_at
                .map(|resolved_at| resolved_at.date() == now.date())
                .unwrap_or(false)
    });

    if meds_done_today {
        "done"
    } else if meds_open {
        "pending"
    } else {
        "none"
    }
}

fn derive_temporal_windows(
    first_event: Option<&vel_storage::SignalRecord>,
    now_ts: i64,
) -> TemporalWindows {
    let prep_minutes = first_event
        .and_then(|event| {
            event
                .payload_json
                .get("prep_minutes")
                .and_then(|value| value.as_i64())
        })
        .unwrap_or(DEFAULT_PREP_MINUTES);
    let travel_minutes = first_event
        .and_then(|event| {
            event
                .payload_json
                .get("travel_minutes")
                .and_then(|value| value.as_i64())
        })
        .unwrap_or(DEFAULT_TRAVEL_MINUTES);
    let prep_start = first_event.map(|event| event.timestamp - prep_minutes * 60);
    let leave_by_ts = first_event.map(|event| event.timestamp - travel_minutes * 60);
    let next_event_start_ts = first_event.map(|event| event.timestamp);
    let prep_window_active = prep_start
        .map(|start_ts| {
            now_ts >= start_ts
                && next_event_start_ts
                    .map(|event_ts| now_ts < event_ts)
                    .unwrap_or(false)
        })
        .unwrap_or(false);
    let commute_window_active = leave_by_ts
        .map(|leave_by| {
            now_ts >= leave_by - COMMUTE_WINDOW_LEAD_SECS
                && next_event_start_ts
                    .map(|event_ts| now_ts < event_ts)
                    .unwrap_or(false)
        })
        .unwrap_or(false);

    TemporalWindows {
        prep_window_active,
        commute_window_active,
        leave_by_ts,
        next_event_start_ts,
    }
}

fn derive_message_summary(message_threads: &[&vel_storage::SignalRecord]) -> MessageSummary {
    let waiting_on_me_threads: Vec<_> = message_threads
        .iter()
        .copied()
        .filter(|signal| {
            signal
                .payload_json
                .get("waiting_state")
                .and_then(|value| value.as_str())
                == Some("me")
        })
        .collect();
    let waiting_on_others_count = message_threads
        .iter()
        .filter(|signal| {
            signal
                .payload_json
                .get("waiting_state")
                .and_then(|value| value.as_str())
                == Some("others")
        })
        .count();
    let scheduling_thread_count = message_threads
        .iter()
        .filter(|signal| {
            signal
                .payload_json
                .get("scheduling_related")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .count();
    let urgent_thread_count = message_threads
        .iter()
        .filter(|signal| {
            signal
                .payload_json
                .get("urgent")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .count();
    let top_threads = waiting_on_me_threads
        .iter()
        .take(3)
        .map(|signal| {
            serde_json::json!({
                "thread_id": signal.payload_json.get("thread_id").and_then(|value| value.as_str()),
                "platform": signal.payload_json.get("platform").and_then(|value| value.as_str()),
                "title": signal.payload_json.get("title").and_then(|value| value.as_str()),
                "waiting_state": signal.payload_json.get("waiting_state").and_then(|value| value.as_str()),
                "scheduling_related": signal.payload_json.get("scheduling_related").and_then(|value| value.as_bool()),
                "urgent": signal.payload_json.get("urgent").and_then(|value| value.as_bool()),
                "latest_timestamp": signal.payload_json.get("latest_timestamp").and_then(|value| value.as_i64()),
                "snippet": signal.payload_json.get("snippet").and_then(|value| value.as_str()),
            })
        })
        .collect();

    MessageSummary {
        waiting_on_me_count: waiting_on_me_threads.len(),
        waiting_on_others_count,
        scheduling_thread_count,
        urgent_thread_count,
        top_threads,
    }
}

fn derive_attention_state(
    state_name: &str,
    prep_window_active: bool,
    meds_pending: bool,
) -> AttentionState {
    let morning_started = state_name != "inactive" && state_name != "awake_unstarted";
    if prep_window_active && !morning_started {
        return AttentionState {
            attention_state: "drifting",
            drift_type: Some("prep_drift"),
            drift_severity: Some("high"),
            confidence: 0.75,
            reasons: vec![
                "prep window active",
                "prep dependency unresolved",
                "no progress signal",
            ],
        };
    }
    if state_name == "at_risk" || (state_name == "awake_unstarted" && meds_pending) {
        return AttentionState {
            attention_state: "drifting",
            drift_type: Some("morning_drift"),
            drift_severity: Some(if prep_window_active { "high" } else { "medium" }),
            confidence: 0.7,
            reasons: vec![
                "morning not started",
                "meds commitment open",
                "no workstation signal",
            ],
        };
    }
    if morning_started && prep_window_active {
        return AttentionState {
            attention_state: "aligned",
            drift_type: None,
            drift_severity: None,
            confidence: 0.8,
            reasons: vec!["morning underway", "prep window active"],
        };
    }
    if morning_started {
        return AttentionState {
            attention_state: "aligned",
            drift_type: None,
            drift_severity: None,
            confidence: 0.8,
            reasons: vec!["morning underway"],
        };
    }
    if state_name == "inactive" {
        return AttentionState {
            attention_state: "unknown",
            drift_type: None,
            drift_severity: None,
            confidence: 0.3,
            reasons: vec![],
        };
    }
    AttentionState {
        attention_state: "neutral_transition",
        drift_type: None,
        drift_severity: None,
        confidence: 0.5,
        reasons: vec![],
    }
}

fn derive_global_risk_summary(risk_snapshots: &[RiskSnapshot]) -> GlobalRiskSummary {
    if let Some(snapshot) = risk_snapshots.first() {
        return GlobalRiskSummary {
            level: match snapshot.risk_level.as_str() {
                "low" => "low",
                "medium" => "medium",
                "high" => "high",
                "critical" => "critical",
                _ => "unknown",
            },
            score: Some(snapshot.risk_score),
            missing: false,
        };
    }
    GlobalRiskSummary {
        level: "unknown",
        score: None,
        missing: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn build_current_context(
    now_ts: i64,
    mode: &str,
    state_name: &str,
    inferred_activity: &str,
    next_commitment_id: Option<String>,
    next_commitment_due_at: Option<i64>,
    meds_status: &str,
    active_nudge_ids: &[String],
    top_risk_commitment_ids: &[String],
    global_risk: &GlobalRiskSummary,
    signals_used: &[String],
    commitments_used: &[String],
    risk_used: &[String],
    attention: &AttentionState,
    git_activity_summary: Option<&GitActivitySummary>,
    message_summary: &MessageSummary,
    temporal_windows: &TemporalWindows,
) -> serde_json::Value {
    let attention_reasons_json: Vec<String> = attention
        .reasons
        .iter()
        .map(|reason| (*reason).to_string())
        .collect();
    serde_json::json!({
        "computed_at": now_ts,
        "mode": mode,
        "morning_state": state_name,
        "inferred_activity": inferred_activity,
        "next_commitment_id": next_commitment_id,
        "next_commitment_due_at": next_commitment_due_at,
        "prep_window_active": temporal_windows.prep_window_active,
        "commute_window_active": temporal_windows.commute_window_active,
        "meds_status": meds_status,
        "active_nudge_ids": active_nudge_ids,
        "top_risk_commitment_ids": top_risk_commitment_ids,
        "global_risk_level": global_risk.level,
        "global_risk_score": global_risk.score,
        "global_risk_missing": global_risk.missing,
        "signals_used": signals_used,
        "commitments_used": commitments_used,
        "risk_used": risk_used,
        "attention_state": attention.attention_state,
        "drift_type": attention.drift_type,
        "drift_severity": attention.drift_severity,
        "attention_confidence": attention.confidence,
        "attention_reasons": attention_reasons_json,
        "git_activity_summary": git_activity_summary.map(|summary| serde_json::json!({
            "timestamp": summary.timestamp,
            "repo": summary.repo,
            "branch": summary.branch,
            "operation": summary.operation,
            "message": summary.message,
            "files_changed": summary.files_changed,
            "insertions": summary.insertions,
            "deletions": summary.deletions,
        })),
        "message_waiting_on_me_count": message_summary.waiting_on_me_count,
        "message_waiting_on_others_count": message_summary.waiting_on_others_count,
        "message_scheduling_thread_count": message_summary.scheduling_thread_count,
        "message_urgent_thread_count": message_summary.urgent_thread_count,
        "message_summary": {
            "waiting_on_me_count": message_summary.waiting_on_me_count,
            "waiting_on_others_count": message_summary.waiting_on_others_count,
            "scheduling_thread_count": message_summary.scheduling_thread_count,
            "urgent_thread_count": message_summary.urgent_thread_count,
            "top_threads": message_summary.top_threads,
        },
        "leave_by_ts": temporal_windows.leave_by_ts,
        "next_event_start_ts": temporal_windows.next_event_start_ts,
    })
}

async fn persist_inference_outputs(
    storage: &Storage,
    now_ts: i64,
    state_name: &str,
    context: &serde_json::Value,
) -> Result<(), crate::errors::AppError> {
    let context_str = context.to_string();
    let prev = storage.get_current_context().await?;
    let material =
        is_material_context_change(prev.as_ref().map(|(_, json)| json.as_str()), &context_str);
    if material {
        if let Err(error) = storage
            .insert_context_timeline(now_ts, &context_str, None)
            .await
        {
            tracing::warn!(error = %error, "insert_context_timeline");
        }
    }

    storage
        .insert_inferred_state(InferredStateInsert {
            state_name: state_name.to_string(),
            confidence: Some("medium".to_string()),
            timestamp: now_ts,
            context_json: Some(context.clone()),
        })
        .await
        .map_err(crate::errors::AppError::from)?;

    if let Err(error) = storage.set_current_context(now_ts, &context_str).await {
        tracing::warn!(error = %error, "set_current_context");
    }

    if let Err(error) = storage
        .emit_event(
            "STATE_CHANGED",
            "inferred_state",
            None,
            &serde_json::json!({ "state_name": state_name }).to_string(),
        )
        .await
    {
        tracing::warn!(error = %error, "emit STATE_CHANGED");
    }

    Ok(())
}

fn select_next_event<'a>(
    calendar_events: &'a [&vel_storage::SignalRecord],
    now_ts: i64,
) -> Option<&'a vel_storage::SignalRecord> {
    calendar_events
        .iter()
        .copied()
        .filter(|signal| signal.timestamp >= now_ts)
        .min_by_key(|signal| signal.timestamp)
        .or_else(|| {
            calendar_events
                .iter()
                .copied()
                .filter(|signal| signal.timestamp <= now_ts)
                .max_by_key(|signal| signal.timestamp)
        })
}

fn select_next_commitment<'a>(
    open_commitments: &'a [vel_core::Commitment],
    risk_snapshots: &[RiskSnapshot],
    now_ts: i64,
) -> Option<&'a vel_core::Commitment> {
    open_commitments
        .iter()
        .min_by_key(|commitment| commitment_sort_key(commitment, risk_snapshots, now_ts))
}

fn commitment_sort_key(
    commitment: &vel_core::Commitment,
    risk_snapshots: &[RiskSnapshot],
    now_ts: i64,
) -> (u8, i64, u8, u32, String) {
    let due_at = commitment.due_at.map(|value| value.unix_timestamp());
    let due_bucket = if due_at.is_some() { 0 } else { 1 };
    let due_sort = due_at.unwrap_or(i64::MAX);
    let anchor_bucket = if is_externally_anchored(commitment) {
        0
    } else {
        1
    };
    let risk_bucket = u32::MAX - commitment_risk_rank(commitment, risk_snapshots, now_ts);
    (
        due_bucket,
        due_sort,
        anchor_bucket,
        risk_bucket,
        commitment.id.as_ref().to_string(),
    )
}

fn is_externally_anchored(commitment: &vel_core::Commitment) -> bool {
    let kind = commitment.commitment_kind.as_deref().unwrap_or_default();
    commitment.source_type == "calendar" || matches!(kind, "meeting" | "prep" | "commute")
}

fn commitment_risk_rank(
    commitment: &vel_core::Commitment,
    risk_snapshots: &[RiskSnapshot],
    now_ts: i64,
) -> u32 {
    risk_snapshots
        .iter()
        .find(|snapshot| snapshot.commitment_id == commitment.id.as_ref())
        .map(|snapshot| (snapshot.risk_score * 1000.0).round() as u32)
        .unwrap_or_else(|| fallback_commitment_risk_rank(commitment, now_ts))
}

fn fallback_commitment_risk_rank(commitment: &vel_core::Commitment, now_ts: i64) -> u32 {
    let due_rank = commitment
        .due_at
        .map(|value| value.unix_timestamp() - now_ts)
        .map(|seconds_until_due| {
            if seconds_until_due <= 0 {
                950
            } else if seconds_until_due <= 30 * 60 {
                900
            } else if seconds_until_due <= 2 * 60 * 60 {
                700
            } else {
                350
            }
        })
        .unwrap_or(200);
    let anchor_bonus = if is_externally_anchored(commitment) {
        100
    } else {
        0
    };
    (due_rank + anchor_bonus).min(1000)
}

fn build_git_activity_summary(signal: &vel_storage::SignalRecord) -> Option<GitActivitySummary> {
    let payload = &signal.payload_json;
    let repo = payload
        .get("repo_name")
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
        .or_else(|| {
            payload
                .get("repo")
                .and_then(serde_json::Value::as_str)
                .and_then(repo_basename)
        })?;

    Some(GitActivitySummary {
        timestamp: signal.timestamp,
        repo,
        branch: payload
            .get("branch")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
        operation: payload
            .get("operation")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
        message: payload
            .get("message")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
        files_changed: payload
            .get("files_changed")
            .and_then(serde_json::Value::as_u64)
            .map(|value| value as u32),
        insertions: payload
            .get("insertions")
            .and_then(serde_json::Value::as_u64)
            .map(|value| value as u32),
        deletions: payload
            .get("deletions")
            .and_then(serde_json::Value::as_u64)
            .map(|value| value as u32),
    })
}

fn repo_basename(path: &str) -> Option<String> {
    path.rsplit('/')
        .find(|segment| !segment.trim().is_empty())
        .map(ToString::to_string)
}

/// Returns true if the new context represents a material change vs previous (for timeline append).
fn is_material_context_change(prev_json: Option<&str>, new_json: &str) -> bool {
    let Some(prev) = prev_json else { return true };
    let Ok(prev_val) = serde_json::from_str::<serde_json::Value>(prev) else {
        return true;
    };
    let Ok(new_val) = serde_json::from_str::<serde_json::Value>(new_json) else {
        return false;
    };
    for key in [
        "morning_state",
        "mode",
        "next_commitment_id",
        "prep_window_active",
        "commute_window_active",
        "meds_status",
        "global_risk_level",
        "active_nudge_ids",
    ] {
        if prev_val.get(key) != new_val.get(key) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_core::{Commitment, CommitmentId, CommitmentStatus};
    use vel_storage::SignalRecord;

    fn test_signal(signal_id: &str, signal_type: &str, timestamp: i64) -> SignalRecord {
        SignalRecord {
            signal_id: signal_id.to_string(),
            signal_type: signal_type.to_string(),
            source: "test".to_string(),
            source_ref: None,
            timestamp,
            payload_json: json!({}),
            created_at: timestamp,
        }
    }

    fn test_commitment(id: &str, due_at: Option<i64>, kind: Option<&str>) -> Commitment {
        Commitment {
            id: CommitmentId::from(id.to_string()),
            text: format!("commitment {id}"),
            source_type: if matches!(kind, Some("meeting")) {
                "calendar".to_string()
            } else {
                "test".to_string()
            },
            source_id: None,
            status: CommitmentStatus::Open,
            due_at: due_at
                .and_then(|timestamp| OffsetDateTime::from_unix_timestamp(timestamp).ok()),
            project: None,
            commitment_kind: kind.map(ToString::to_string),
            created_at: OffsetDateTime::UNIX_EPOCH,
            resolved_at: None,
            metadata_json: json!({}),
        }
    }

    fn test_medication_commitment(
        id: &str,
        status: CommitmentStatus,
        resolved_at: Option<i64>,
    ) -> Commitment {
        Commitment {
            id: CommitmentId::from(id.to_string()),
            text: format!("medication {id}"),
            source_type: "test".to_string(),
            source_id: None,
            status,
            due_at: None,
            project: None,
            commitment_kind: Some("medication".to_string()),
            created_at: OffsetDateTime::UNIX_EPOCH,
            resolved_at: resolved_at
                .and_then(|timestamp| OffsetDateTime::from_unix_timestamp(timestamp).ok()),
            metadata_json: json!({}),
        }
    }

    #[test]
    fn material_change_identical_returns_false() {
        let ctx =
            r#"{"morning_state":"underway","mode":"morning_mode","prep_window_active":false}"#;
        assert!(!is_material_context_change(Some(ctx), ctx));
    }

    #[test]
    fn material_change_different_state_returns_true() {
        let prev = r#"{"morning_state":"inactive"}"#;
        let new = r#"{"morning_state":"underway"}"#;
        assert!(is_material_context_change(Some(prev), new));
    }

    #[test]
    fn material_change_no_prev_returns_true() {
        assert!(is_material_context_change(None, r#"{}"#));
    }

    #[test]
    fn select_next_event_prefers_nearest_future_event() {
        let now_ts = 1_700_000_000;
        let signals = vec![
            test_signal("sig_past", "calendar_event", now_ts - 60),
            test_signal("sig_later", "calendar_event", now_ts + 600),
            test_signal("sig_next", "calendar_event", now_ts + 120),
        ];
        let calendar_events: Vec<_> = signals.iter().collect();

        let selected = select_next_event(&calendar_events, now_ts).expect("expected next event");

        assert_eq!(selected.signal_id, "sig_next");
    }

    #[test]
    fn select_next_event_falls_back_to_latest_past_event_when_no_future_event_exists() {
        let now_ts = 1_700_000_000;
        let signals = vec![
            test_signal("sig_old", "calendar_event", now_ts - 600),
            test_signal("sig_recent", "calendar_event", now_ts - 60),
        ];
        let calendar_events: Vec<_> = signals.iter().collect();

        let selected =
            select_next_event(&calendar_events, now_ts).expect("expected fallback past event");

        assert_eq!(selected.signal_id, "sig_recent");
    }

    #[test]
    fn select_next_commitment_is_not_insertion_order_based() {
        let now_ts = 1_700_000_000;
        let commitments = vec![
            test_commitment("com_later", Some(now_ts + 7200), None),
            test_commitment("com_sooner", Some(now_ts + 900), None),
        ];

        let selected =
            select_next_commitment(&commitments, &[], now_ts).expect("expected a commitment");

        assert_eq!(selected.id.as_ref(), "com_sooner");
    }

    #[test]
    fn select_next_commitment_prefers_anchored_high_risk_work_over_undated_items() {
        let now_ts = 1_700_000_000;
        let commitments = vec![
            test_commitment("com_undated", None, Some("todo")),
            test_commitment("com_meeting", None, Some("meeting")),
        ];
        let risk_snapshots = vec![RiskSnapshot {
            commitment_id: "com_meeting".to_string(),
            risk_score: 0.92,
            risk_level: "critical".to_string(),
            factors: vel_core::RiskFactors {
                consequence: 0.9,
                proximity: 0.9,
                dependency_pressure: 0.0,
                reasons: vec![],
                dependency_ids: vec![],
            },
            computed_at: Some(now_ts),
        }];

        let selected = select_next_commitment(&commitments, &risk_snapshots, now_ts)
            .expect("expected a commitment");

        assert_eq!(selected.id.as_ref(), "com_meeting");
    }

    #[test]
    fn derive_meds_status_prefers_done_today_over_open_medication_commitment() {
        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let open_commitments = vec![test_medication_commitment(
            "com_open",
            CommitmentStatus::Open,
            None,
        )];
        let medication_commitments = vec![
            test_medication_commitment("com_open", CommitmentStatus::Open, None),
            test_medication_commitment(
                "com_done",
                CommitmentStatus::Done,
                Some(now.unix_timestamp()),
            ),
        ];

        let meds_status = derive_meds_status(&open_commitments, &medication_commitments, now);

        assert_eq!(meds_status, "done");
    }

    #[test]
    fn derive_temporal_windows_marks_prep_and_commute_windows_before_event() {
        let now_ts = 1_700_000_000;
        let event_ts = now_ts + 10 * 60;
        let event = SignalRecord {
            signal_id: "sig_event".to_string(),
            signal_type: "calendar_event".to_string(),
            source: "test".to_string(),
            source_ref: None,
            timestamp: event_ts,
            payload_json: json!({
                "prep_minutes": 30,
                "travel_minutes": 20,
            }),
            created_at: event_ts,
        };

        let windows = derive_temporal_windows(Some(&event), now_ts);

        assert!(windows.prep_window_active);
        assert!(windows.commute_window_active);
        assert_eq!(windows.next_event_start_ts, Some(event_ts));
        assert_eq!(windows.leave_by_ts, Some(event_ts - 20 * 60));
    }

    #[test]
    fn derive_attention_state_marks_morning_drift_when_unstarted_and_meds_pending() {
        let attention = derive_attention_state("awake_unstarted", false, true);

        assert_eq!(attention.attention_state, "drifting");
        assert_eq!(attention.drift_type, Some("morning_drift"));
        assert_eq!(attention.drift_severity, Some("medium"));
    }

    #[test]
    fn derive_global_risk_summary_uses_fallback_when_no_risk_rows_exist() {
        let summary = derive_global_risk_summary(&[]);

        assert_eq!(summary.level, "unknown");
        assert_eq!(summary.score, None);
        assert!(summary.missing);
    }
}
