//! Inference engine: signals + commitments + time -> inferred state and canonical current context (Phase C).
//! See docs/specs/vel-current-context-spec.md for canonical shape and material-change rules.
//!
//! **Boundary: recompute-and-persist.** This module must only be called from the evaluate
//! orchestration (e.g. [crate::services::evaluate::run]). Never call from explain or read routes.

use time::OffsetDateTime;
use vel_core::CommitmentStatus;
use vel_storage::{InferredStateInsert, Storage};

const RECENT_GIT_ACTIVITY_WINDOW_SECS: i64 = 90 * 60;

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

    let open_commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 200)
        .await?;
    let signals_today = storage
        .list_signals(None, Some(start_of_today), 500)
        .await?;
    let active_nudges = storage.list_nudges(Some("active"), 50).await?;
    let snoozed_nudges = storage.list_nudges(Some("snoozed"), 50).await?;

    let has_workstation_activity = signals_today.iter().any(|s| {
        matches!(
            s.signal_type.as_str(),
            "vel_invocation" | "shell_login" | "computer_activity" | "git_activity"
        )
    });
    let meds_open = open_commitments
        .iter()
        .any(|c| c.commitment_kind.as_deref() == Some("medication"));
    let all_commitments = storage
        .list_commitments(None, None, Some("medication"), 100)
        .await?;
    let meds_done_today = all_commitments.iter().any(|c| {
        c.status == vel_core::CommitmentStatus::Done
            && c.resolved_at
                .map(|t| t.date() == now.date())
                .unwrap_or(false)
    });
    let meds_pending = meds_open && !meds_done_today;
    let meds_status = if meds_done_today {
        "done"
    } else if meds_pending {
        "pending"
    } else {
        "none"
    };

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
    // Next relevant future event (at or after now), not earliest event of the day.
    let first_event = calendar_events
        .iter()
        .filter(|s| s.timestamp >= now_ts)
        .min_by_key(|s| s.timestamp)
        .or_else(|| {
            // Fallback: currently active event (most recent past event if we're inside its window).
            calendar_events
                .iter()
                .filter(|s| s.timestamp <= now_ts)
                .max_by_key(|s| s.timestamp)
        });
    let prep_minutes = first_event
        .and_then(|e| e.payload_json.get("prep_minutes").and_then(|p| p.as_i64()))
        .unwrap_or(15);
    let travel_minutes = first_event
        .and_then(|e| {
            e.payload_json
                .get("travel_minutes")
                .and_then(|p| p.as_i64())
        })
        .unwrap_or(0);
    let prep_start = first_event.map(|e| e.timestamp - prep_minutes * 60);
    let leave_by = first_event.map(|e| e.timestamp - travel_minutes * 60);
    let prep_window_active = prep_start
        .map(|ps| now_ts >= ps && first_event.map(|e| now_ts < e.timestamp).unwrap_or(false))
        .unwrap_or(false);
    let commute_window_active = leave_by
        .map(|lb| {
            now_ts >= lb - 15 * 60 && first_event.map(|e| now_ts < e.timestamp).unwrap_or(false)
        })
        .unwrap_or(false);

    let morning_started = has_workstation_activity || meds_done_today;
    let state_name = if prep_window_active && !morning_started {
        "at_risk"
    } else if morning_started {
        "engaged"
    } else if first_event.is_some() && !morning_started {
        "awake_unstarted"
    } else {
        "inactive"
    };

    let recent_git_summary = latest_git_activity
        .and_then(|signal| build_git_activity_summary(signal))
        .filter(|summary| now_ts - summary.timestamp <= RECENT_GIT_ACTIVITY_WINDOW_SECS);
    let git_activity_summary = latest_git_activity.and_then(|signal| build_git_activity_summary(signal));
    let inferred_activity = if recent_git_summary.is_some() {
        "coding"
    } else if has_workstation_activity {
        "computer_active"
    } else {
        "unknown"
    };
    let waiting_on_me_threads: Vec<_> = message_threads
        .iter()
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
    let top_message_threads: Vec<serde_json::Value> = waiting_on_me_threads
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

    let mode = if prep_window_active {
        "meeting_mode"
    } else if commute_window_active {
        "commute_mode"
    } else if state_name == "at_risk" || state_name == "awake_unstarted" {
        "morning_mode"
    } else {
        "morning_mode"
    };

    let next_commitment_id = open_commitments.first().map(|c| c.id.as_ref().to_string());
    let next_commitment_due_at = open_commitments
        .first()
        .and_then(|c| c.due_at.map(|t| t.unix_timestamp()));

    let active_nudge_ids: Vec<String> = active_nudges
        .iter()
        .chain(snoozed_nudges.iter())
        .map(|n| n.nudge_id.clone())
        .collect();

    let risk_rows = storage
        .list_commitment_risk_latest_all()
        .await
        .unwrap_or_default();
    let top_risk_commitment_ids: Vec<String> = risk_rows
        .iter()
        .map(|(_, cid, _, _, _, _)| cid.clone())
        .take(10)
        .collect();
    let risk_used: Vec<String> = risk_rows
        .iter()
        .map(|(_, cid, _, _, _, _)| cid.clone())
        .take(50)
        .collect();
    let (global_risk_level, global_risk_score) =
        if let Some((_, _, score, level, _, _)) = risk_rows.first() {
            (level.as_str(), *score)
        } else if prep_window_active && meds_pending {
            ("high", 0.78)
        } else if prep_window_active || meds_pending {
            ("medium", 0.5)
        } else {
            ("low", 0.2)
        };

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

    let morning_started = state_name != "inactive" && state_name != "awake_unstarted";
    let (attention_state, drift_type, drift_severity, attention_confidence, attention_reasons): (
        &str,
        Option<&str>,
        Option<&str>,
        f64,
        Vec<&str>,
    ) = if prep_window_active && !morning_started {
        (
            "drifting",
            Some("prep_drift"),
            Some("high"),
            0.75,
            vec![
                "prep window active",
                "prep dependency unresolved",
                "no progress signal",
            ],
        )
    } else if state_name == "at_risk" || (state_name == "awake_unstarted" && meds_pending) {
        (
            "drifting",
            Some("morning_drift"),
            Some(if prep_window_active { "high" } else { "medium" }),
            0.7,
            vec![
                "morning not started",
                "meds commitment open",
                "no workstation signal",
            ],
        )
    } else if morning_started && prep_window_active {
        (
            "aligned",
            None,
            None,
            0.8,
            vec!["morning underway", "prep window active"],
        )
    } else if morning_started {
        ("aligned", None, None, 0.8, vec!["morning underway"])
    } else if state_name == "inactive" {
        ("unknown", None, None, 0.3, vec![])
    } else {
        ("neutral_transition", None, None, 0.5, vec![])
    };

    let next_event_start_ts = first_event.map(|e| e.timestamp);
    let leave_by_ts = leave_by;
    let attention_reasons_json: Vec<String> =
        attention_reasons.iter().map(|s| (*s).to_string()).collect();
    let context = serde_json::json!({
        "computed_at": now_ts,
        "mode": mode,
        "morning_state": state_name,
        "inferred_activity": inferred_activity,
        "next_commitment_id": next_commitment_id,
        "next_commitment_due_at": next_commitment_due_at,
        "prep_window_active": prep_window_active,
        "commute_window_active": commute_window_active,
        "meds_status": meds_status,
        "active_nudge_ids": active_nudge_ids,
        "top_risk_commitment_ids": top_risk_commitment_ids,
        "global_risk_level": global_risk_level,
        "global_risk_score": global_risk_score,
        "signals_used": signals_used,
        "commitments_used": commitments_used,
        "risk_used": risk_used,
        "attention_state": attention_state,
        "drift_type": drift_type,
        "drift_severity": drift_severity,
        "attention_confidence": attention_confidence,
        "attention_reasons": attention_reasons_json,
        "git_activity_summary": git_activity_summary.as_ref().map(|summary| serde_json::json!({
            "timestamp": summary.timestamp,
            "repo": summary.repo,
            "branch": summary.branch,
            "operation": summary.operation,
            "message": summary.message,
            "files_changed": summary.files_changed,
            "insertions": summary.insertions,
            "deletions": summary.deletions,
        })),
        "message_waiting_on_me_count": waiting_on_me_threads.len(),
        "message_waiting_on_others_count": waiting_on_others_count,
        "message_scheduling_thread_count": scheduling_thread_count,
        "message_urgent_thread_count": urgent_thread_count,
        "message_summary": {
            "waiting_on_me_count": waiting_on_me_threads.len(),
            "waiting_on_others_count": waiting_on_others_count,
            "scheduling_thread_count": scheduling_thread_count,
            "urgent_thread_count": urgent_thread_count,
            "top_threads": top_message_threads,
        },
        "leave_by_ts": leave_by_ts,
        "next_event_start_ts": next_event_start_ts,
    });

    let context_str = context.to_string();

    let prev = storage.get_current_context().await?;
    let material = is_material_context_change(prev.as_ref().map(|(_, s)| s.as_str()), &context_str);
    if material {
        if let Err(e) = storage
            .insert_context_timeline(now_ts, &context_str, None)
            .await
        {
            tracing::warn!(error = %e, "insert_context_timeline");
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

    if let Err(e) = storage.set_current_context(now_ts, &context_str).await {
        tracing::warn!(error = %e, "set_current_context");
    }

    if let Err(e) = storage
        .emit_event(
            "STATE_CHANGED",
            "inferred_state",
            None,
            &serde_json::json!({ "state_name": state_name }).to_string(),
        )
        .await
    {
        tracing::warn!(error = %e, "emit STATE_CHANGED");
    }

    Ok(1)
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
}
