//! Nudge engine (policy layer): resolution first, then create/update nudges (Phase D).
//! See docs/specs/vel-policy-engine-spec.md and vel-agent-next-steps-policy-config-commute.md.

use time::OffsetDateTime;
use vel_storage::{NudgeInsert, Storage};
use vel_core::CommitmentStatus;

use crate::policy_config::PolicyConfig;

/// Evaluate and create/update nudges from current state. Resolution policies run first, then creation.
/// Returns count of nudges created or updated.
pub async fn evaluate(
    storage: &Storage,
    policy_config: &PolicyConfig,
    _inferred_states_count: usize,
) -> Result<u32, crate::errors::AppError> {
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
    let signals_today = storage.list_signals(None, Some(start_of_today), 500).await?;
    let existing_nudges = storage.list_nudges(None, 100).await?;

    let mut count = 0u32;

    let meds_open = open_commitments.iter().any(|c| c.commitment_kind.as_deref() == Some("medication"));
    let all_meds = storage.list_commitments(None, None, Some("medication"), 100).await?;
    let meds_done_today = all_meds.iter().any(|c| {
        c.status == vel_core::CommitmentStatus::Done
            && c.resolved_at.map(|t| t.date() == now.date()).unwrap_or(false)
    });
    let meds_pending = meds_open && !meds_done_today;

    let calendar_events: Vec<_> = signals_today.iter().filter(|s| s.signal_type == "calendar_event").collect();
    let first_event = calendar_events.iter().min_by_key(|s| s.timestamp);
    let default_prep = policy_config
        .meeting_prep_window()
        .map(|p| p.default_prep_minutes as i64)
        .unwrap_or(30);
    let prep_minutes = first_event
        .and_then(|e| e.payload_json.get("prep_minutes").and_then(|p| p.as_i64()))
        .unwrap_or(default_prep);
    let prep_start = first_event.map(|e| e.timestamp - prep_minutes * 60);
    let prep_window_active = prep_start
        .map(|ps| now_ts >= ps && first_event.map(|e| now_ts < e.timestamp).unwrap_or(false))
        .unwrap_or(false);

    let has_vel = signals_today.iter().any(|s| s.signal_type == "vel_invocation");
    let morning_started = has_vel || meds_done_today;
    let morning_drift = !morning_started && now_ts > start_of_today + 8 * 3600;

    // --- Resolution policies first (policy engine spec: resolution before creation/escalation) ---
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
            "commute_leave_time" => {
                let event_started = first_event.map(|e| now_ts >= e.timestamp).unwrap_or(true);
                event_started || open_commitments.is_empty()
            }
            _ => false,
        };
        if should_resolve {
            if let Err(e) = storage.update_nudge_state(&n.nudge_id, "resolved", None).await {
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
            "reasons": ["meds commitment open", "no completion signal today"],
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
                inference_snapshot_json: Some(serde_json::json!({ "meds_pending": true }).to_string()),
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
        let explanation = serde_json::json!({
            "policy": "meeting_prep_window",
            "decision": "create_nudge",
            "level": "warning",
            "reasons": ["prep window active", "first meeting today"],
            "suppressed_reasons": []
        });
        let nudge_id = storage
            .insert_nudge(NudgeInsert {
                nudge_type: "meeting_prep_window".to_string(),
                level: "warning".to_string(),
                state: "active".to_string(),
                related_commitment_id: None,
                message: "Prep window for your first meeting has started.".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: Some(serde_json::json!({ "prep_window_active": true }).to_string()),
                metadata_json: Some(explanation),
            })
            .await?;
        let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "meeting_prep_window" }).to_string(), now_ts).await;
        count += 1;
    }

    // --- commute_leave_time: only when calendar event has travel_minutes (require_travel_minutes) ---
    let commute_cfg = policy_config.commute_leave_time();
    let has_commute_nudge = existing_nudges
        .iter()
        .any(|n| n.nudge_type == "commute_leave_time" && (n.state == "active" || n.state == "snoozed"));
    if commute_cfg.map(|c| c.enabled && c.require_travel_minutes).unwrap_or(false)
        && !has_commute_nudge
    {
        let gentle_before = commute_cfg.map(|c| c.gentle_before_minutes as i64).unwrap_or(20);
        let warning_before = commute_cfg.map(|c| c.warning_before_minutes as i64).unwrap_or(5);
        let danger_before = commute_cfg.map(|c| c.danger_before_minutes as i64).unwrap_or(0);
        if let Some(event) = first_event {
            let travel_minutes = event.payload_json.get("travel_minutes").and_then(|p| p.as_i64());
            if let Some(travel_m) = travel_minutes {
                let leave_by = event.timestamp - travel_m * 60;
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
                        "reasons": ["leave-by window active", "travel_minutes set"],
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
                            inference_snapshot_json: Some(serde_json::json!({ "leave_by": leave_by, "travel_minutes": travel_m }).to_string()),
                            metadata_json: Some(explanation),
                        })
                        .await?;
                    let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "commute_leave_time" }).to_string(), now_ts).await;
                    count += 1;
                }
            }
        }
    }

    let has_drift_nudge = existing_nudges
        .iter()
        .any(|n| n.nudge_type == "morning_drift" && (n.state == "active" || n.state == "snoozed"));
    if morning_drift && !has_drift_nudge {
        let explanation = serde_json::json!({
            "policy": "morning_drift",
            "decision": "create_nudge",
            "level": "gentle",
            "reasons": ["morning not started", "past typical wake window"],
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
                inference_snapshot_json: Some(serde_json::json!({ "morning_drift": true }).to_string()),
                metadata_json: Some(explanation),
            })
            .await?;
        let _ = storage.insert_nudge_event(&nudge_id, "nudge_created", &serde_json::json!({ "nudge_type": "morning_drift" }).to_string(), now_ts).await;
        count += 1;
    }

    Ok(count)
}
