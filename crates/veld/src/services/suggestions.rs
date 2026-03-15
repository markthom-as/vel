//! Suggestion triggers: create suggestions from repeated evidence (e.g. repeated commute danger, prep warnings).
//! See vel-agent-next-implementation-steps.md — trigger only on repeated evidence.

use time::OffsetDateTime;
use vel_storage::Storage;

const REPEAT_THRESHOLD: usize = 2;
const WINDOW_DAYS: i64 = 7;

/// After nudges are evaluated, check for repeated patterns and insert suggestions if not already present.
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
        .count();
    let existing_commute_suggestion = storage
        .list_suggestions(Some("pending"), 20)
        .await?
        .into_iter()
        .any(|(_, stype, _, _, _, _)| stype == "increase_commute_buffer");
    if resolved_commute_danger >= REPEAT_THRESHOLD && !existing_commute_suggestion {
        let payload = serde_json::json!({
            "type": "increase_commute_buffer",
            "current_minutes": 20,
            "suggested_minutes": 30
        });
        let _ = storage.insert_suggestion("increase_commute_buffer", "pending", &payload.to_string()).await?;
        created += 1;
    }

    let resolved_prep = nudges
        .iter()
        .filter(|n| {
            n.nudge_type == "meeting_prep_window"
                && n.state == "resolved"
                && n.resolved_at.map(|t| t >= window_start).unwrap_or(false)
        })
        .count();
    let existing_prep_suggestion = storage
        .list_suggestions(Some("pending"), 20)
        .await?
        .into_iter()
        .any(|(_, stype, _, _, _, _)| stype == "increase_prep_window");
    if resolved_prep >= REPEAT_THRESHOLD && !existing_prep_suggestion {
        let payload = serde_json::json!({
            "type": "increase_prep_window",
            "current_minutes": 30,
            "suggested_minutes": 45
        });
        let _ = storage.insert_suggestion("increase_prep_window", "pending", &payload.to_string()).await?;
        created += 1;
    }

    Ok(created)
}
