//! Nudge engine (policy layer): consumes current context + risk + nudges + policy. No duplicate signal/context logic.
//! See docs/specs/vel-policy-engine-spec.md and vel-detailed-next-steps-and-ios-repo-guidance.md.
//!
//! **Boundary: recompute-and-persist.** [evaluate] creates/updates nudges and nudge_events. Only call from evaluate orchestration.

use time::OffsetDateTime;
use vel_core::CommitmentStatus;
use vel_storage::{NudgeInsert, Storage};

use crate::policy_config::{PolicyCommuteLeaveTime, PolicyConfig};
use crate::services::risk::list_latest_snapshots;

fn nudge_level_rank(level: &str) -> u8 {
    match level {
        "gentle" => 1,
        "warning" => 2,
        "danger" => 3,
        _ => 0,
    }
}

fn suppresses_new_nudge(nudge: &vel_storage::NudgeRecord, nudge_type: &str, now_ts: i64) -> bool {
    nudge.nudge_type == nudge_type
        && (nudge.state == "active"
            || (nudge.state == "snoozed"
                && nudge
                    .snoozed_until
                    .map(|snoozed_until| snoozed_until > now_ts)
                    .unwrap_or(true)))
}

fn commute_level_message(
    now_ts: i64,
    leave_by_ts: i64,
    config: &PolicyCommuteLeaveTime,
) -> Option<(&'static str, &'static str)> {
    if now_ts >= leave_by_ts - i64::from(config.danger_before_minutes) * 60 {
        Some(("danger", "You may be late unless you leave now."))
    } else if now_ts >= leave_by_ts - i64::from(config.warning_before_minutes) * 60 {
        Some(("warning", "You should leave soon."))
    } else if now_ts >= leave_by_ts - i64::from(config.gentle_before_minutes) * 60 {
        Some(("gentle", "Leave-by time is approaching."))
    } else {
        None
    }
}

async fn reactivate_snoozed_nudge_for_higher_urgency(
    storage: &Storage,
    existing_nudges: &[vel_storage::NudgeRecord],
    nudge_type: &str,
    desired_level: &str,
    desired_message: &str,
    inference_snapshot: serde_json::Value,
    metadata_json: serde_json::Value,
    now_ts: i64,
) -> Result<bool, crate::errors::AppError> {
    let Some(nudge) = existing_nudges.iter().find(|nudge| {
        nudge.nudge_type == nudge_type
            && nudge.state == "snoozed"
            && nudge
                .snoozed_until
                .map(|snoozed_until| snoozed_until > now_ts)
                .unwrap_or(false)
    }) else {
        return Ok(false);
    };

    if nudge_level_rank(desired_level) <= nudge_level_rank(&nudge.level) {
        return Ok(false);
    }

    let inference_snapshot_json = inference_snapshot.to_string();
    storage
        .update_nudge_lifecycle(
            &nudge.nudge_id,
            desired_level,
            "active",
            desired_message,
            None,
            None,
            Some(&inference_snapshot_json),
            &metadata_json,
        )
        .await?;
    let _ = storage
        .insert_nudge_event(
            &nudge.nudge_id,
            "nudge_reactivated",
            &serde_json::json!({
                "reason": "higher_urgency",
                "from_level": nudge.level,
                "to_level": desired_level,
            })
            .to_string(),
            now_ts,
        )
        .await;
    let _ = storage
        .insert_nudge_event(
            &nudge.nudge_id,
            "nudge_escalated",
            &serde_json::json!({
                "nudge_type": nudge_type,
                "from_level": nudge.level,
                "to_level": desired_level,
            })
            .to_string(),
            now_ts,
        )
        .await;
    if let Err(e) = storage
        .emit_event(
            "NUDGE_ESCALATED",
            "nudge",
            Some(&nudge.nudge_id),
            &serde_json::json!({
                "nudge_type": nudge_type,
                "from_level": nudge.level,
                "to_level": desired_level,
                "reactivated": true,
            })
            .to_string(),
        )
        .await
    {
        tracing::warn!(error = %e, nudge_id = %nudge.nudge_id, "emit NUDGE_ESCALATED");
    }
    Ok(true)
}

async fn reactivate_expired_snoozed_nudge(
    storage: &Storage,
    existing_nudges: &[vel_storage::NudgeRecord],
    nudge_type: &str,
    now_ts: i64,
) -> Result<bool, crate::errors::AppError> {
    let Some(nudge) = existing_nudges.iter().find(|nudge| {
        nudge.nudge_type == nudge_type
            && nudge.state == "snoozed"
            && nudge
                .snoozed_until
                .map(|snoozed_until| snoozed_until <= now_ts)
                .unwrap_or(false)
    }) else {
        return Ok(false);
    };

    storage
        .update_nudge_state(&nudge.nudge_id, "active", None, None)
        .await?;
    let _ = storage
        .insert_nudge_event(
            &nudge.nudge_id,
            "nudge_reactivated",
            r#"{"reason":"snooze_expired"}"#,
            now_ts,
        )
        .await;
    Ok(true)
}

async fn escalate_active_nudge(
    storage: &Storage,
    existing_nudges: &[vel_storage::NudgeRecord],
    nudge_type: &str,
    desired_level: &str,
    desired_message: &str,
    inference_snapshot: serde_json::Value,
    metadata_json: serde_json::Value,
    now_ts: i64,
) -> Result<bool, crate::errors::AppError> {
    let Some(nudge) = existing_nudges
        .iter()
        .find(|nudge| nudge.nudge_type == nudge_type && nudge.state == "active")
    else {
        return Ok(false);
    };

    if nudge_level_rank(&nudge.level) >= nudge_level_rank(desired_level) {
        return Ok(false);
    }

    let inference_snapshot_json = inference_snapshot.to_string();
    storage
        .update_nudge_lifecycle(
            &nudge.nudge_id,
            desired_level,
            "active",
            desired_message,
            None,
            None,
            Some(&inference_snapshot_json),
            &metadata_json,
        )
        .await?;
    let _ = storage
        .insert_nudge_event(
            &nudge.nudge_id,
            "nudge_escalated",
            &serde_json::json!({
                "nudge_type": nudge_type,
                "from_level": nudge.level,
                "to_level": desired_level,
            })
            .to_string(),
            now_ts,
        )
        .await;
    if let Err(e) = storage
        .emit_event(
            "NUDGE_ESCALATED",
            "nudge",
            Some(&nudge.nudge_id),
            &serde_json::json!({
                "nudge_type": nudge_type,
                "from_level": nudge.level,
                "to_level": desired_level,
            })
            .to_string(),
        )
        .await
    {
        tracing::warn!(error = %e, nudge_id = %nudge.nudge_id, "emit NUDGE_ESCALATED");
    }
    Ok(true)
}

/// **Recompute-and-persist.** Evaluate and create/update nudges from current context, risk snapshots, commitments, and active nudges.
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
    let context: serde_json::Value =
        serde_json::from_str(&context_json).unwrap_or(serde_json::json!({}));

    let meds_status = context
        .get("meds_status")
        .and_then(|v| v.as_str())
        .unwrap_or("none");
    let meds_pending = meds_status == "pending";
    let prep_window_active = context
        .get("prep_window_active")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let commute_window_active = context
        .get("commute_window_active")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let morning_state = context
        .get("morning_state")
        .and_then(|v| v.as_str())
        .unwrap_or("inactive");
    let morning_started = morning_state != "inactive" && morning_state != "awake_unstarted";
    let leave_by_ts: Option<i64> = context.get("leave_by_ts").and_then(|v| v.as_i64());
    let next_event_start_ts: Option<i64> =
        context.get("next_event_start_ts").and_then(|v| v.as_i64());
    let event_started = next_event_start_ts.map(|t| now_ts >= t).unwrap_or(true);
    let message_waiting_on_me_count = context
        .get("message_waiting_on_me_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let message_scheduling_thread_count = context
        .get("message_scheduling_thread_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let message_urgent_thread_count = context
        .get("message_urgent_thread_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let open_commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 200)
        .await?;
    let existing_nudges = storage.list_nudges(None, 100).await?;
    let risk_snapshots = list_latest_snapshots(storage).await.unwrap_or_default();

    let mut count = 0u32;

    // --- Resolution policies first (from context + risk, not raw signals) ---
    for n in &existing_nudges {
        if n.state != "active" && n.state != "snoozed" {
            continue;
        }
        let should_resolve = match n.nudge_type.as_str() {
            "meds_not_logged" => {
                let com_id = n.related_commitment_id.as_deref();
                !meds_pending
                    || com_id
                        .and_then(|id| open_commitments.iter().find(|c| c.id.as_ref() == id))
                        .is_none()
            }
            "meeting_prep_window" => !prep_window_active,
            "morning_drift" => morning_started,
            "response_debt" => message_waiting_on_me_count == 0,
            "commute_leave_time" => {
                let related_resolved = n
                    .related_commitment_id
                    .as_deref()
                    .is_some_and(|cid| !open_commitments.iter().any(|c| c.id.as_ref() == cid));
                event_started || related_resolved
            }
            _ => false,
        };
        if should_resolve {
            if let Err(e) = storage
                .update_nudge_state(&n.nudge_id, "resolved", None, Some(now_ts))
                .await
            {
                tracing::warn!(nudge_id = %n.nudge_id, error = %e, "resolve nudge");
            } else {
                let _ = storage
                    .insert_nudge_event(
                        &n.nudge_id,
                        "nudge_resolved",
                        r#"{"reason":"policy_resolution"}"#,
                        now_ts,
                    )
                    .await;
                let _ = storage
                    .emit_event("NUDGE_RESOLVED", "nudge", Some(&n.nudge_id), r#"{}"#)
                    .await;
                count += 1;
            }
        }
    }

    let has_active_meds_nudge = existing_nudges
        .iter()
        .any(|n| suppresses_new_nudge(n, "meds_not_logged", now_ts));
    if meds_pending && !has_active_meds_nudge {
        if reactivate_expired_snoozed_nudge(storage, &existing_nudges, "meds_not_logged", now_ts)
            .await?
        {
            count += 1;
        } else {
            let com = open_commitments
                .iter()
                .find(|c| c.commitment_kind.as_deref() == Some("medication"));
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
                    inference_snapshot_json: Some(
                        serde_json::json!({ "meds_pending": true, "source": "context" })
                            .to_string(),
                    ),
                    metadata_json: Some(explanation),
                })
                .await?;
            let _ = storage
                .insert_nudge_event(
                    &nudge_id,
                    "nudge_created",
                    &serde_json::json!({ "nudge_type": "meds_not_logged" }).to_string(),
                    now_ts,
                )
                .await;
            if let Err(e) = storage
                .emit_event(
                    "NUDGE_GENERATED",
                    "nudge",
                    None,
                    r#"{"nudge_type":"meds_not_logged"}"#,
                )
                .await
            {
                tracing::warn!(error = %e, "emit NUDGE_GENERATED");
            }
            count += 1;
        }
    }

    let has_prep_nudge = existing_nudges
        .iter()
        .any(|n| suppresses_new_nudge(n, "meeting_prep_window", now_ts));
    if prep_window_active {
        let meeting_risk_high = risk_snapshots.iter().any(|snapshot| snapshot.is_high_or_worse());
        let prep_level = if meeting_risk_high {
            "warning"
        } else {
            "gentle"
        };
        let reasons: Vec<&str> = if meeting_risk_high {
            vec![
                "prep window active (from context)",
                "first meeting today",
                "meeting_risk >= high",
            ]
        } else {
            vec!["prep window active (from context)", "first meeting today"]
        };
        let explanation = serde_json::json!({
            "policy": "meeting_prep_window",
            "decision": if has_prep_nudge { "escalate_nudge" } else { "create_nudge" },
            "level": prep_level,
            "reasons": reasons,
            "suppressed_reasons": []
        });
        let inference_snapshot = serde_json::json!({
            "prep_window_active": true,
            "meeting_risk_high": meeting_risk_high,
            "source": "context"
        });
        if reactivate_expired_snoozed_nudge(
            storage,
            &existing_nudges,
            "meeting_prep_window",
            now_ts,
        )
        .await?
        {
            count += 1;
        } else if reactivate_snoozed_nudge_for_higher_urgency(
            storage,
            &existing_nudges,
            "meeting_prep_window",
            prep_level,
            "Prep window for your first meeting has started.",
            inference_snapshot.clone(),
            explanation.clone(),
            now_ts,
        )
        .await?
        {
            count += 1;
        } else if escalate_active_nudge(
            storage,
            &existing_nudges,
            "meeting_prep_window",
            prep_level,
            "Prep window for your first meeting has started.",
            inference_snapshot.clone(),
            explanation.clone(),
            now_ts,
        )
        .await?
        {
            count += 1;
        } else if !has_prep_nudge {
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
                    inference_snapshot_json: Some(inference_snapshot.to_string()),
                    metadata_json: Some(explanation),
                })
                .await?;
            let _ = storage
                .insert_nudge_event(
                    &nudge_id,
                    "nudge_created",
                    &serde_json::json!({ "nudge_type": "meeting_prep_window" }).to_string(),
                    now_ts,
                )
                .await;
            count += 1;
        }
    }

    // --- commute_leave_time: only when context has commute_window_active and leave_by_ts (travel_minutes was set) ---
    let commute_cfg = policy_config.commute_leave_time();
    let has_commute_nudge = existing_nudges
        .iter()
        .any(|n| suppresses_new_nudge(n, "commute_leave_time", now_ts));
    if commute_cfg
        .map(|c| c.enabled && c.require_travel_minutes)
        .unwrap_or(false)
        && commute_window_active
        && leave_by_ts.is_some()
    {
        if reactivate_expired_snoozed_nudge(storage, &existing_nudges, "commute_leave_time", now_ts)
            .await?
        {
            count += 1;
        } else {
            let leave_by = leave_by_ts.unwrap();
            let level_message =
                commute_cfg.and_then(|config| commute_level_message(now_ts, leave_by, config));
            if let Some((lvl, msg)) = level_message {
                let explanation = serde_json::json!({
                    "policy": "commute_leave_time",
                    "decision": if has_commute_nudge { "escalate_nudge" } else { "create_nudge" },
                    "level": lvl,
                    "reasons": ["leave-by window active (from context)", "travel_minutes set"],
                    "suppressed_reasons": []
                });
                let inference_snapshot = serde_json::json!({
                    "leave_by": leave_by,
                    "source": "context"
                });
                if reactivate_snoozed_nudge_for_higher_urgency(
                    storage,
                    &existing_nudges,
                    "commute_leave_time",
                    lvl,
                    msg,
                    inference_snapshot.clone(),
                    explanation.clone(),
                    now_ts,
                )
                .await?
                {
                    count += 1;
                } else if escalate_active_nudge(
                    storage,
                    &existing_nudges,
                    "commute_leave_time",
                    lvl,
                    msg,
                    inference_snapshot.clone(),
                    explanation.clone(),
                    now_ts,
                )
                .await?
                {
                    count += 1;
                } else if !has_commute_nudge {
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
                            inference_snapshot_json: Some(inference_snapshot.to_string()),
                            metadata_json: Some(explanation),
                        })
                        .await?;
                    let _ = storage
                        .insert_nudge_event(
                            &nudge_id,
                            "nudge_created",
                            &serde_json::json!({ "nudge_type": "commute_leave_time" }).to_string(),
                            now_ts,
                        )
                        .await;
                    count += 1;
                }
            }
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
        .any(|n| suppresses_new_nudge(n, "morning_drift", now_ts));
    if morning_drift && !has_drift_nudge {
        if reactivate_expired_snoozed_nudge(storage, &existing_nudges, "morning_drift", now_ts)
            .await?
        {
            count += 1;
        } else {
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
                    inference_snapshot_json: Some(
                        serde_json::json!({ "morning_drift": true, "source": "context" })
                            .to_string(),
                    ),
                    metadata_json: Some(explanation),
                })
                .await?;
            let _ = storage
                .insert_nudge_event(
                    &nudge_id,
                    "nudge_created",
                    &serde_json::json!({ "nudge_type": "morning_drift" }).to_string(),
                    now_ts,
                )
                .await;
            count += 1;
        }
    }

    let has_response_debt_nudge = existing_nudges
        .iter()
        .any(|n| suppresses_new_nudge(n, "response_debt", now_ts));
    if message_waiting_on_me_count > 0 {
        let level = if message_urgent_thread_count > 0 || message_waiting_on_me_count >= 3 {
            "warning"
        } else {
            "gentle"
        };
        let message = if message_scheduling_thread_count > 0 {
            "You have messages waiting on you, including scheduling follow-up."
        } else {
            "You have messages waiting on you."
        };
        let reasons = if message_scheduling_thread_count > 0 {
            vec![
                "messages waiting on me (from context)",
                "scheduling thread present",
            ]
        } else {
            vec!["messages waiting on me (from context)"]
        };
        let explanation = serde_json::json!({
            "policy": "response_debt",
            "decision": if has_response_debt_nudge { "escalate_nudge" } else { "create_nudge" },
            "level": level,
            "reasons": reasons,
            "suppressed_reasons": []
        });
        let inference_snapshot = serde_json::json!({
            "message_waiting_on_me_count": message_waiting_on_me_count,
            "message_scheduling_thread_count": message_scheduling_thread_count,
            "message_urgent_thread_count": message_urgent_thread_count,
            "source": "context"
        });
        if reactivate_expired_snoozed_nudge(storage, &existing_nudges, "response_debt", now_ts)
            .await?
        {
            count += 1;
        } else if reactivate_snoozed_nudge_for_higher_urgency(
            storage,
            &existing_nudges,
            "response_debt",
            level,
            message,
            inference_snapshot.clone(),
            explanation.clone(),
            now_ts,
        )
        .await?
        {
            count += 1;
        } else if escalate_active_nudge(
            storage,
            &existing_nudges,
            "response_debt",
            level,
            message,
            inference_snapshot.clone(),
            explanation.clone(),
            now_ts,
        )
        .await?
        {
            count += 1;
        } else if !has_response_debt_nudge {
            let nudge_id = storage
                .insert_nudge(NudgeInsert {
                    nudge_type: "response_debt".to_string(),
                    level: level.to_string(),
                    state: "active".to_string(),
                    related_commitment_id: None,
                    message: message.to_string(),
                    snoozed_until: None,
                    resolved_at: None,
                    signals_snapshot_json: None,
                    inference_snapshot_json: Some(inference_snapshot.to_string()),
                    metadata_json: Some(explanation),
                })
                .await?;
            let _ = storage
                .insert_nudge_event(
                    &nudge_id,
                    "nudge_created",
                    &serde_json::json!({ "nudge_type": "response_debt" }).to_string(),
                    now_ts,
                )
                .await;
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commute_level_message_uses_policy_thresholds() {
        let config = PolicyCommuteLeaveTime {
            enabled: true,
            require_travel_minutes: true,
            gentle_before_minutes: 20,
            warning_before_minutes: 5,
            danger_before_minutes: 0,
            default_snooze_minutes: 5,
        };
        let leave_by_ts = 10_000;

        assert_eq!(
            commute_level_message(leave_by_ts - 21 * 60, leave_by_ts, &config),
            None
        );
        assert_eq!(
            commute_level_message(leave_by_ts - 20 * 60, leave_by_ts, &config),
            Some(("gentle", "Leave-by time is approaching."))
        );
        assert_eq!(
            commute_level_message(leave_by_ts - 5 * 60, leave_by_ts, &config),
            Some(("warning", "You should leave soon."))
        );
        assert_eq!(
            commute_level_message(leave_by_ts, leave_by_ts, &config),
            Some(("danger", "You may be late unless you leave now."))
        );
    }
}
