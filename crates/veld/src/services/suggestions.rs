//! Suggestion triggers: create suggestions from repeated evidence (e.g. repeated commute danger, prep warnings).
//! See vel-agent-next-implementation-steps.md — trigger only on repeated evidence.
//!
//! **Boundary: recompute-and-persist.** Only call from evaluate orchestration.

use time::OffsetDateTime;
use vel_core::SuggestionConfidence;
use vel_storage::Storage;

const REPEAT_THRESHOLD: usize = 2;
const WINDOW_DAYS: i64 = 7;

/// **Recompute-and-persist.** After nudges are evaluated, check for repeated patterns and insert suggestions if not already present.
pub async fn evaluate_after_nudges(storage: &Storage) -> Result<u32, crate::errors::AppError> {
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let window_start = now_ts - WINDOW_DAYS * 86400;
    let mut created = 0u32;

    let nudges = storage.list_nudges(None, 500).await?;
    let resolved_commute_danger = nudges
        .iter()
        .filter(|n| {
            n.nudge_type == "commute_leave_time"
                && n.state == "resolved"
                && n.level == "danger"
                && n.resolved_at.map(|t| t >= window_start).unwrap_or(false)
        })
        .collect::<Vec<_>>();
    let existing_commute_suggestion = storage
        .find_recent_suggestion_by_dedupe_key("increase_commute_buffer")
        .await?
        .is_some_and(|suggestion| suggestion.state == "pending");
    if resolved_commute_danger.len() >= REPEAT_THRESHOLD && !existing_commute_suggestion {
        let payload = serde_json::json!({
            "type": "increase_commute_buffer",
            "current_minutes": 20,
            "suggested_minutes": 30
        });
        let suggestion_id = storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "increase_commute_buffer".to_string(),
                state: "pending".to_string(),
                title: Some("Increase commute buffer".to_string()),
                summary: Some(
                    "Repeated commute danger nudges suggest the leave-time threshold is too tight."
                        .to_string(),
                ),
                priority: 70,
                confidence: Some(SuggestionConfidence::Medium.to_string()),
                dedupe_key: Some("increase_commute_buffer".to_string()),
                payload_json: payload,
                decision_context_json: Some(serde_json::json!({
                    "summary": format!(
                        "Resolved {} commute danger nudges in the last {} days.",
                        resolved_commute_danger.len(),
                        WINDOW_DAYS
                    ),
                    "trigger": "resolved_commute_danger",
                    "window_days": WINDOW_DAYS,
                    "count": resolved_commute_danger.len(),
                    "threshold": REPEAT_THRESHOLD,
                })),
            })
            .await?;
        for nudge in resolved_commute_danger.iter().take(REPEAT_THRESHOLD) {
            storage
                .insert_suggestion_evidence(vel_storage::SuggestionEvidenceInsert {
                    suggestion_id: suggestion_id.clone(),
                    evidence_type: "nudge".to_string(),
                    ref_id: nudge.nudge_id.clone(),
                    evidence_json: Some(serde_json::json!({
                        "nudge_type": nudge.nudge_type,
                        "level": nudge.level,
                        "resolved_at": nudge.resolved_at,
                    })),
                    weight: Some(1.0),
                })
                .await?;
        }
        created += 1;
    }

    let resolved_prep = nudges
        .iter()
        .filter(|n| {
            n.nudge_type == "meeting_prep_window"
                && n.state == "resolved"
                && n.resolved_at.map(|t| t >= window_start).unwrap_or(false)
        })
        .collect::<Vec<_>>();
    let existing_prep_suggestion = storage
        .find_recent_suggestion_by_dedupe_key("increase_prep_window")
        .await?
        .is_some_and(|suggestion| suggestion.state == "pending");
    if resolved_prep.len() >= REPEAT_THRESHOLD && !existing_prep_suggestion {
        let payload = serde_json::json!({
            "type": "increase_prep_window",
            "current_minutes": 30,
            "suggested_minutes": 45
        });
        let suggestion_id = storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "increase_prep_window".to_string(),
                state: "pending".to_string(),
                title: Some("Increase prep window".to_string()),
                summary: Some(
                    "Repeated prep nudges suggest the default meeting prep window is too small."
                        .to_string(),
                ),
                priority: 60,
                confidence: Some(SuggestionConfidence::Medium.to_string()),
                dedupe_key: Some("increase_prep_window".to_string()),
                payload_json: payload,
                decision_context_json: Some(serde_json::json!({
                    "summary": format!(
                        "Resolved {} prep-window nudges in the last {} days.",
                        resolved_prep.len(),
                        WINDOW_DAYS
                    ),
                    "trigger": "resolved_prep_window",
                    "window_days": WINDOW_DAYS,
                    "count": resolved_prep.len(),
                    "threshold": REPEAT_THRESHOLD,
                })),
            })
            .await?;
        for nudge in resolved_prep.iter().take(REPEAT_THRESHOLD) {
            storage
                .insert_suggestion_evidence(vel_storage::SuggestionEvidenceInsert {
                    suggestion_id: suggestion_id.clone(),
                    evidence_type: "nudge".to_string(),
                    ref_id: nudge.nudge_id.clone(),
                    evidence_json: Some(serde_json::json!({
                        "nudge_type": nudge.nudge_type,
                        "level": nudge.level,
                        "resolved_at": nudge.resolved_at,
                    })),
                    weight: Some(1.0),
                })
                .await?;
        }
        created += 1;
    }

    Ok(created)
}
