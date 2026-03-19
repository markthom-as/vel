use serde_json::Value as JsonValue;
use time::{Duration, OffsetDateTime};
use vel_config::AppConfig;
use vel_core::CommitmentStatus;
use vel_storage::{SignalRecord, Storage};

use crate::{errors::AppError, services::operator_queue};

#[derive(Debug, Clone)]
pub struct MorningInputSnapshot {
    pub summary: String,
    pub friction_callouts: Vec<vel_core::MorningFrictionCallout>,
}

pub async fn load_daily_loop_inputs(
    storage: &Storage,
    config: &AppConfig,
    session_date: &str,
) -> Result<MorningInputSnapshot, AppError> {
    let now = OffsetDateTime::now_utc();
    let calendar_window_end = now + Duration::hours(12);
    let open_commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 128)
        .await?;

    let today_todoist = open_commitments
        .iter()
        .filter(|commitment| commitment.source_type == "todoist")
        .filter(|commitment| {
            commitment
                .due_at
                .map(|due_at| due_at.date() <= now.date())
                .unwrap_or(false)
        })
        .take(3)
        .map(|commitment| commitment.text.clone())
        .collect::<Vec<_>>();

    let calendar_events = storage
        .list_signals(Some("calendar_event"), Some(now.unix_timestamp()), 64)
        .await?
        .into_iter()
        .filter_map(calendar_event_title)
        .filter(|event| event.start_ts <= calendar_window_end.unix_timestamp())
        .take(3)
        .map(|event| event.title)
        .collect::<Vec<_>>();

    let queue_snapshot = operator_queue::build_action_items(storage, config).await?;
    let mut seen = std::collections::HashSet::new();
    let friction_callouts = queue_snapshot
        .action_items
        .into_iter()
        .filter_map(|item| {
            let key = format!("{}:{}", item.kind, item.title);
            if !seen.insert(key) {
                return None;
            }
            Some(vel_core::MorningFrictionCallout {
                label: item.title,
                detail: item.summary,
            })
        })
        .take(2)
        .collect::<Vec<_>>();

    let schedule_sentence = if calendar_events.is_empty() {
        "No scheduled events are in the next 12 hours.".to_string()
    } else {
        format!("Next 12 hours: {}.", calendar_events.join(", "))
    };
    let task_sentence = if today_todoist.is_empty() {
        "Todoist today/overdue is clear.".to_string()
    } else {
        format!("Today and overdue Todoist: {}.", today_todoist.join(", "))
    };
    let friction_sentence = if friction_callouts.is_empty() {
        "No major friction callouts are active right now.".to_string()
    } else {
        format!(
            "Main friction: {}.",
            friction_callouts
                .iter()
                .map(|callout| callout.label.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let summary = format!(
        "Morning overview for {session_date}. {schedule_sentence} {task_sentence} {friction_sentence}"
    );

    Ok(MorningInputSnapshot {
        summary,
        friction_callouts,
    })
}

#[derive(Debug, Clone)]
struct CalendarEventTitle {
    title: String,
    start_ts: i64,
}

fn calendar_event_title(signal: SignalRecord) -> Option<CalendarEventTitle> {
    let payload = signal.payload_json;
    let title = payload.get("title")?.as_str()?.trim();
    let start_ts = json_i64(&payload, "start")?;
    if title.is_empty() {
        return None;
    }
    Some(CalendarEventTitle {
        title: title.to_string(),
        start_ts,
    })
}

fn json_i64(value: &JsonValue, key: &str) -> Option<i64> {
    value.get(key).and_then(|entry| {
        entry
            .as_i64()
            .or_else(|| entry.as_str().and_then(|text| text.parse::<i64>().ok()))
    })
}
