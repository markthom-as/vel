//! Nudge engine (policy layer): consumes current context + risk + nudges + policy. No duplicate signal/context logic.
//! See docs/specs/vel-policy-engine-spec.md and vel-detailed-next-steps-and-ios-repo-guidance.md.

use time::OffsetDateTime;
use vel_storage::{NudgeInsert, Storage};
use vel_core::CommitmentStatus;

use crate::policy_config::PolicyConfig;

/// Evaluate and create/update nudges from **current context**, risk snapshots, commitments, and active nudges.
/// Does not recompute meds/prep/morning from raw signals; reads from context.
pub async fn evaluate(
    storage: &Storage,
    policy_config: &PolicyConfig,
    _inferred_states_count: usize,
) -> Result<u32, crate::errors::AppError> {
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();

    let (_, context_json) = storage
        .get_current_context()
        .await?
        .unwrap_or((0, "{}".to_string()));
    let context: serde_json::Value = serde_json::from_str(&context_json).unwrap_or(serde_json::json!({}));

    let meds_status = context.get("meds_status").and_then(|v| v.as_str()).unwrap_or("none");
    let meds_pending = meds_status == "pending";
    let prep_window_active = context.get("prep_window_active").and_then(|v| v.as_bool()).unwrap_or(false);
    let commute_window_active = context.get("commute_window_active").and_then(|v| v.as_bool()).unwrap_or(false);
    let morning_state = context.get("morning_state").and_then(|v| v.as_str()).unwrap_or("inactive");
    let morning_started = morning_state != "inactive" && morning_state != "awake_unstarted";
    let leave_by_ts: Option<i64> = context.get("leave_by_ts").and_then(|v| v.as_i64());
    let next_event_start_ts: Option<i64> = context.get("next_event_start_ts").and_then(|v| v.as_i64());
    let event_started = next_event_start_ts.map(|t| now_ts >= t).unwrap_or(true);

    let open_commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 200)
        .await?;
    let existing_nudges = storage.list_nudges(None, 100).await?;
    let risk_rows = storage.list_commitment_risk_latest_all().await.unwrap_or_default();

    let mut count = 0u32;

    // --- Resolution policies first (from context + risk, not raw signals) ---
    for n in &existing_nudges {
        if n.state != "active" && n.state != "snoozed" {
            continue;
        }
        let should_resolve = match n.nudge_type.as_str() {
            "meds_not_logged" => {
                let com_id = n.related_commitment_id.as_deref();
                !meds_pending || com_id.and_then(|id| open_commitments.iter().find(|c| c.id.as_ref() == id)).is_none()
            }
            "meeting_prep_window" => !prep_window_active,
            "morning_drift" => morning_started,
            "commute_leave_time" => event_started || open_commitments.is_empty(),
            _ => false,
        };
        if should_resolve {
            if let Err(e) = storage.update_nudge_state(&n.nudge_id, "resolved", None, Some(now_ts)).await {
                tracing::warn!(nudge_id = %n.nudge_id, error = %e, "resolve nudge");
            } else {
                let _ = storage.insert_nudge_event(&n.nudge_id, "nudge_resolved", r#"{"reason":"policy_resolution"}"#, now_ts).await;
                let _ = storage.emit_event("NUDGE_RESOLVED", "nudge", Some(&n.nudge_id), r#"{}"#).await;
                count += 1;
            }
        }
    }

    let has_active_meds_nudge = existing_nudges
        .iter()
        .any(|n| n.nudge_type == "meds_not_logged" && (n.state == "active" || n.state == "snoozed"));
    if meds_pending && !has_active_meds_nudge {
        let com = open_commitments.iter().find(|c| c.commitment_kind.as_deref() == Some("medication"));
        let explanation = serde_json::json!({
            "policy": "meds_not_logged",
            "decision": "create_nudge",
            "level": "gentle",
            "reasons": ["meds commitment open (from context)", "no completion signal today"],
            "suppressed_reasons": []
        });
        let nudge_id = storage
            .insert_nudge(NudgeInsert {
                nudge_type: "meds_not_logged".to_string(),
                level: "gentle".to_string(),
                state: "active".to_string(),
                related_commitment_id: com.map(|c| c.id.as_ref().to_string()),
                message: "Meds not logged yet.".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: Some(serde_json::json!({ "meds_pending": true, "source": "context" }).to_string()),
                metadata_json: Some(explanation),
            })
            .await?;
        let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "meds_not_logged" }).to_string(), now_ts).await;
        if let Err(e) = storage.emit_event("NUDGE_GENERATED", "nudge", None, r#"{"nudge_type":"meds_not_logged"}"#).await {
            tracing::warn!(error = %e, "emit NUDGE_GENERATED");
        }
        count += 1;
    }

    let has_prep_nudge = existing_nudges
        .iter()
        .any(|n| n.nudge_type == "meeting_prep_window" && (n.state == "active" || n.state == "snoozed"));
    if prep_window_active && !has_prep_nudge {
        let meeting_risk_high = risk_rows
            .iter()
            .any(|(_, _, _, level, _, _)| level == "high" || level == "critical");
        let prep_level = if meeting_risk_high { "warning" } else { "gentle" };
        let reasons: Vec<&str> = if meeting_risk_high {
            vec!["prep window active (from context)", "first meeting today", "meeting_risk >= high"]
        } else {
            vec!["prep window active (from context)", "first meeting today"]
        };
        let explanation = serde_json::json!({
            "policy": "meeting_prep_window",
            "decision": "create_nudge",
            "level": prep_level,
            "reasons": reasons,
            "suppressed_reasons": []
        });
        let nudge_id = storage
            .insert_nudge(NudgeInsert {
                nudge_type: "meeting_prep_window".to_string(),
                level: prep_level.to_string(),
                state: "active".to_string(),
                related_commitment_id: None,
                message: "Prep window for your first meeting has started.".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: Some(serde_json::json!({ "prep_window_active": true, "source": "context" }).to_string()),
                metadata_json: Some(explanation),
            })
            .await?;
        let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "meeting_prep_window" }).to_string(), now_ts).await;
        count += 1;
    }

    // --- commute_leave_time: only when context has commute_window_active and leave_by_ts (travel_minutes was set) ---
    let commute_cfg = policy_config.commute_leave_time();
    let has_commute_nudge = existing_nudges
        .iter()
        .any(|n| n.nudge_type == "commute_leave_time" && (n.state == "active" || n.state == "snoozed"));
    if commute_cfg.map(|c| c.enabled && c.require_travel_minutes).unwrap_or(false)
        && commute_window_active
        && leave_by_ts.is_some()
        && !has_commute_nudge
    {
        let leave_by = leave_by_ts.unwrap();
        let gentle_before = commute_cfg.map(|c| c.gentle_before_minutes as i64).unwrap_or(20);
        let warning_before = commute_cfg.map(|c| c.warning_before_minutes as i64).unwrap_or(5);
        let danger_before = commute_cfg.map(|c| c.danger_before_minutes as i64).unwrap_or(0);
        let level_message: Option<(&str, &str)> = if now_ts >= leave_by - danger_before * 60 {
            Some(("danger", "You may be late unless you leave now."))
        } else if now_ts >= leave_by - warning_before * 60 {
            Some(("warning", "You should leave soon."))
        } else if now_ts >= leave_by - gentle_before * 60 {
            Some(("gentle", "Leave-by time is approaching."))
        } else {
            None
        };
        if let Some((lvl, msg)) = level_message {
            let explanation = serde_json::json!({
                "policy": "commute_leave_time",
                "decision": "create_nudge",
                "level": lvl,
                "reasons": ["leave-by window active (from context)", "travel_minutes set"],
                "suppressed_reasons": []
            });
            let nudge_id = storage
                .insert_nudge(NudgeInsert {
                    nudge_type: "commute_leave_time".to_string(),
                    level: lvl.to_string(),
                    state: "active".to_string(),
                    related_commitment_id: None,
                    message: msg.to_string(),
                    snoozed_until: None,
                    resolved_at: None,
                    signals_snapshot_json: None,
                    inference_snapshot_json: Some(serde_json::json!({ "leave_by": leave_by, "source": "context" }).to_string()),
                    metadata_json: Some(explanation),
                })
                .await?;
            let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "commute_leave_time" }).to_string(), now_ts).await;
            count += 1;
        }
    }

    let morning_drift = !morning_started
        && context
            .get("attention_state")
            .and_then(|v| v.as_str())
            .map(|s| s == "drifting")
            .unwrap_or(false)
        && context
            .get("drift_type")
            .and_then(|v| v.as_str())
            .map(|s| s == "morning_drift")
            .unwrap_or(false);
    let has_drift_nudge = existing_nudges
        .iter()
        .any(|n| n.nudge_type == "morning_drift" && (n.state == "active" || n.state == "snoozed"));
    if morning_drift && !has_drift_nudge {
        let explanation = serde_json::json!({
            "policy": "morning_drift",
            "decision": "create_nudge",
            "level": "gentle",
            "reasons": ["morning not started (from context)", "drift_type morning_drift"],
            "suppressed_reasons": []
        });
        let nudge_id = storage
            .insert_nudge(NudgeInsert {
                nudge_type: "morning_drift".to_string(),
                level: "gentle".to_string(),
                state: "active".to_string(),
                related_commitment_id: None,
                message: "Morning hasn't started yet.".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: Some(serde_json::json!({ "morning_drift": true, "source": "context" }).to_string()),
                metadata_json: Some(explanation),
            })
            .await?;
        let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "morning_drift" }).to_string(), now_ts).await;
        count += 1;
    }

    Ok(count)
}
