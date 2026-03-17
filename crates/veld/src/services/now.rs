use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_api_types::{
    NowAttentionData, NowData, NowDebugData, NowEventData, NowFreshnessData, NowFreshnessEntryData,
    NowLabelData, NowRiskSummaryData, NowScheduleData, NowSummaryData, NowTaskData, NowTasksData,
};
use vel_config::AppConfig;
use vel_core::{Commitment, CommitmentStatus};
use vel_storage::{SignalRecord, Storage};

use crate::{errors::AppError, services::integrations};

pub async fn get_now(storage: &Storage, config: &AppConfig) -> Result<NowData, AppError> {
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let Some((computed_at, context_json)) = storage.get_current_context().await? else {
        return Ok(empty_now(now_ts, &timezone.name));
    };
    let context: JsonValue = serde_json::from_str(&context_json).unwrap_or_else(|_| json!({}));

    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;
    let next_commitment_id = string_field(&context, "next_commitment_id");
    let next_commitment = next_commitment_id
        .as_ref()
        .and_then(|id| {
            commitments
                .iter()
                .find(|commitment| commitment.id.as_ref() == id)
        })
        .map(now_task);

    let mut todoist = Vec::new();
    let mut other_open = Vec::new();
    for commitment in sort_commitments(commitments) {
        if commitment.source_type == "todoist" {
            if todoist.len() < 6 {
                todoist.push(now_task(&commitment));
            }
        } else if other_open.len() < 4 {
            other_open.push(now_task(&commitment));
        }
    }

    let signal_ids = string_array_field(&context, "signals_used");
    let mut events = storage
        .list_signals_by_ids(&signal_ids)
        .await?
        .into_iter()
        .filter_map(calendar_event_from_signal)
        .collect::<Vec<_>>();
    if events.is_empty() {
        events = storage
            .list_signals(Some("calendar_event"), Some(now_ts - 12 * 60 * 60), 32)
            .await?
            .into_iter()
            .filter_map(calendar_event_from_signal)
            .collect();
    }
    events.sort_by_key(|event| event.start_ts);
    let next_event = events
        .iter()
        .find(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .cloned();
    let upcoming_events: Vec<NowEventData> = events
        .into_iter()
        .filter(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .take(5)
        .collect();

    let integrations = integrations::get_integrations_with_config(storage, config).await?;
    let freshness = build_freshness(now_ts, computed_at, &integrations);
    let schedule_empty_message = schedule_empty_message(&integrations, upcoming_events.is_empty());
    let attention_reasons = string_array_field(&context, "attention_reasons");
    let reasons = build_reasons(&context, &attention_reasons);

    Ok(NowData {
        computed_at,
        timezone: timezone.name,
        summary: NowSummaryData {
            mode: label_for_mode(
                string_field(&context, "mode")
                    .as_deref()
                    .unwrap_or("unknown"),
            ),
            phase: label_for_phase(
                string_field(&context, "morning_state")
                    .as_deref()
                    .unwrap_or("unknown"),
            ),
            meds: label_for_meds(
                string_field(&context, "meds_status")
                    .as_deref()
                    .unwrap_or("unknown"),
            ),
            risk: risk_summary(
                string_field(&context, "global_risk_level")
                    .as_deref()
                    .unwrap_or("unknown"),
                number_field(&context, "global_risk_score"),
            ),
        },
        schedule: NowScheduleData {
            empty_message: schedule_empty_message,
            next_event,
            upcoming_events,
        },
        tasks: NowTasksData {
            todoist,
            other_open,
            next_commitment,
        },
        attention: NowAttentionData {
            state: label_for_attention(
                string_field(&context, "attention_state")
                    .as_deref()
                    .unwrap_or("unknown"),
            ),
            drift: label_for_drift(
                string_field(&context, "drift_type")
                    .as_deref()
                    .unwrap_or("none"),
            ),
            severity: label_for_severity(
                string_field(&context, "drift_severity")
                    .as_deref()
                    .unwrap_or("none"),
            ),
            confidence: number_field(&context, "attention_confidence"),
            reasons: attention_reasons,
        },
        freshness,
        reasons,
        debug: NowDebugData {
            raw_context: context.clone(),
            signals_used: signal_ids,
            commitments_used: string_array_field(&context, "commitments_used"),
            risk_used: string_array_field(&context, "risk_used"),
        },
    })
}

fn empty_now(now_ts: i64, timezone: &str) -> NowData {
    NowData {
        computed_at: now_ts,
        timezone: timezone.to_string(),
        summary: NowSummaryData {
            mode: label("unknown", "Unknown"),
            phase: label("unknown", "Unknown"),
            meds: label("unknown", "Unknown"),
            risk: risk_summary("unknown", None),
        },
        schedule: NowScheduleData {
            empty_message: Some("No current context yet. Sync calendar sources or run evaluate.".to_string()),
            next_event: None,
            upcoming_events: Vec::new(),
        },
        tasks: NowTasksData {
            todoist: Vec::new(),
            other_open: Vec::new(),
            next_commitment: None,
        },
        attention: NowAttentionData {
            state: label("unknown", "Unknown"),
            drift: label("none", "None"),
            severity: label("none", "None"),
            confidence: None,
            reasons: Vec::new(),
        },
        freshness: NowFreshnessData {
            overall_status: "stale".to_string(),
            sources: vec![NowFreshnessEntryData {
                key: "context".to_string(),
                label: "Context".to_string(),
                status: "missing".to_string(),
                last_sync_at: None,
                age_seconds: None,
            }],
        },
        reasons: vec!["No current context yet. Sync integrations or run evaluate.".to_string()],
        debug: NowDebugData {
            raw_context: json!({}),
            signals_used: Vec::new(),
            commitments_used: Vec::new(),
            risk_used: Vec::new(),
        },
    }
}

fn schedule_empty_message(
    integrations: &vel_api_types::IntegrationsData,
    no_upcoming_events: bool,
) -> Option<String> {
    if !no_upcoming_events {
        return None;
    }

    let calendar = &integrations.google_calendar;
    if !calendar.connected {
        return Some("Google Calendar is disconnected. Reconnect it in Settings.".to_string());
    }
    if !calendar.all_calendars_selected && calendar.calendars.iter().all(|calendar| !calendar.selected) {
        return Some("No calendars are selected in Settings.".to_string());
    }

    Some("No upcoming calendar events in the current stream.".to_string())
}

fn build_reasons(context: &JsonValue, attention_reasons: &[String]) -> Vec<String> {
    let mut reasons = Vec::new();
    if let Some(mode) = string_field(context, "mode") {
        reasons.push(format!("Mode: {}", label_for_mode(&mode).label));
    }
    if bool_field(context, "prep_window_active") == Some(true) {
        reasons.push("Prep window active".to_string());
    }
    if bool_field(context, "commute_window_active") == Some(true) {
        reasons.push("Commute window active".to_string());
    }
    if string_field(context, "meds_status").as_deref() == Some("pending") {
        reasons.push("Medication task is still pending".to_string());
    }
    reasons.extend(attention_reasons.iter().cloned());
    reasons.truncate(8);
    reasons
}

fn build_freshness(
    now_ts: i64,
    computed_at: i64,
    integrations: &vel_api_types::IntegrationsData,
) -> NowFreshnessData {
    let mut sources = vec![NowFreshnessEntryData {
        key: "context".to_string(),
        label: "Context".to_string(),
        status: age_status(now_ts - computed_at).to_string(),
        last_sync_at: Some(computed_at),
        age_seconds: Some(now_ts - computed_at),
    }];
    sources.push(sync_freshness(
        now_ts,
        "calendar",
        "Calendar",
        integrations.google_calendar.last_sync_at,
        integrations.google_calendar.last_sync_status.as_deref(),
    ));
    sources.push(sync_freshness(
        now_ts,
        "todoist",
        "Todoist",
        integrations.todoist.last_sync_at,
        integrations.todoist.last_sync_status.as_deref(),
    ));
    sources.push(sync_freshness(
        now_ts,
        "activity",
        "Activity",
        integrations.activity.last_sync_at,
        integrations.activity.last_sync_status.as_deref(),
    ));
    sources.push(sync_freshness(
        now_ts,
        "messaging",
        "Messaging",
        integrations.messaging.last_sync_at,
        integrations.messaging.last_sync_status.as_deref(),
    ));
    let overall_status = if sources
        .iter()
        .any(|entry| matches!(entry.status.as_str(), "error" | "stale" | "missing"))
    {
        "stale"
    } else if sources.iter().any(|entry| entry.status == "aging") {
        "aging"
    } else {
        "fresh"
    };
    NowFreshnessData {
        overall_status: overall_status.to_string(),
        sources,
    }
}

fn sync_freshness(
    now_ts: i64,
    key: &str,
    label_text: &str,
    last_sync_at: Option<i64>,
    last_sync_status: Option<&str>,
) -> NowFreshnessEntryData {
    let age_seconds = last_sync_at.map(|timestamp| now_ts - timestamp);
    let status = match last_sync_status {
        Some("error") => "error",
        Some("disconnected") => "disconnected",
        _ => age_seconds.map(age_status).unwrap_or("missing"),
    };
    NowFreshnessEntryData {
        key: key.to_string(),
        label: label_text.to_string(),
        status: status.to_string(),
        last_sync_at,
        age_seconds,
    }
}

fn age_status(age_seconds: i64) -> &'static str {
    if age_seconds <= 120 {
        "fresh"
    } else if age_seconds <= 600 {
        "aging"
    } else {
        "stale"
    }
}

fn sort_commitments(mut commitments: Vec<Commitment>) -> Vec<Commitment> {
    let now = OffsetDateTime::now_utc();
    commitments.sort_by(|left, right| compare_commitments(left, right, now));
    commitments
}

fn compare_commitments(
    left: &Commitment,
    right: &Commitment,
    now: OffsetDateTime,
) -> std::cmp::Ordering {
    let left_priority = commitment_priority_key(left, now);
    let right_priority = commitment_priority_key(right, now);
    left_priority
        .cmp(&right_priority)
        .then_with(|| {
            left.due_at
                .map(|value| value.unix_timestamp())
                .unwrap_or(i64::MAX)
                .cmp(
                    &right
                        .due_at
                        .map(|value| value.unix_timestamp())
                        .unwrap_or(i64::MAX),
                )
        })
        .then_with(|| todoist_priority(right).cmp(&todoist_priority(left)))
        .then_with(|| todoist_updated_at(right).cmp(&todoist_updated_at(left)))
        .then_with(|| right.created_at.cmp(&left.created_at))
}

fn commitment_priority_key(commitment: &Commitment, now: OffsetDateTime) -> (u8, i64) {
    let due_ts = commitment.due_at.map(|value| value.unix_timestamp());
    let due_at = commitment.due_at;
    let due_date = due_at.map(|value| value.date());
    let today = now.date();
    let has_due_time = commitment
        .metadata_json
        .get("has_due_time")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let priority_bucket =
        if commitment.commitment_kind.as_deref() == Some("medication") && due_date == Some(today) {
            0
        } else if due_ts.is_some_and(|value| value < now.unix_timestamp()) {
            1
        } else if due_date == Some(today) && has_due_time {
            2
        } else if due_date == Some(today) {
            3
        } else if todoist_priority(commitment) >= 3 || was_recently_updated(commitment, now) {
            4
        } else {
            5
        };
    (priority_bucket, due_ts.unwrap_or(i64::MAX))
}

fn todoist_priority(commitment: &Commitment) -> i64 {
    commitment
        .metadata_json
        .get("priority")
        .and_then(|value| value.as_i64())
        .unwrap_or(1)
}

fn todoist_updated_at(commitment: &Commitment) -> i64 {
    commitment
        .metadata_json
        .get("updated_at")
        .and_then(|value| value.as_i64())
        .unwrap_or(i64::MIN)
}

fn was_recently_updated(commitment: &Commitment, now: OffsetDateTime) -> bool {
    todoist_updated_at(commitment) >= now.unix_timestamp() - (24 * 60 * 60)
}

fn now_task(commitment: &Commitment) -> NowTaskData {
    NowTaskData {
        id: commitment.id.as_ref().to_string(),
        text: commitment.text.clone(),
        source_type: commitment.source_type.clone(),
        due_at: commitment.due_at,
        project: commitment.project.clone(),
        commitment_kind: commitment.commitment_kind.clone(),
    }
}

fn calendar_event_from_signal(signal: SignalRecord) -> Option<NowEventData> {
    if signal.signal_type != "calendar_event" {
        return None;
    }
    let payload = signal.payload_json;
    let start_ts = payload
        .get("start")?
        .as_i64()
        .or_else(|| payload.get("start_ts")?.as_i64())?;
    let title = payload
        .get("title")
        .and_then(|value| value.as_str())
        .unwrap_or("Untitled event")
        .to_string();
    let travel_minutes = payload
        .get("travel_minutes")
        .and_then(|value| value.as_i64());
    Some(NowEventData {
        title,
        start_ts,
        end_ts: payload.get("end").and_then(|value| value.as_i64()),
        location: payload
            .get("location")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        prep_minutes: payload.get("prep_minutes").and_then(|value| value.as_i64()),
        travel_minutes,
        leave_by_ts: travel_minutes.map(|minutes| start_ts - (minutes * 60)),
    })
}

fn label(key: &str, text: &str) -> NowLabelData {
    NowLabelData {
        key: key.to_string(),
        label: text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Duration, Month};
    use vel_core::{CommitmentId, CommitmentStatus};

    #[test]
    fn pending_medication_today_outranks_ordinary_task() {
        let now = fixed_now();
        let med = commitment_fixture(
            "Take meds",
            Some(now.replace_hour(9).unwrap()),
            Some("medication"),
            json!({ "has_due_time": true }),
        );
        let ordinary = commitment_fixture("Reply to email", None, Some("todo"), json!({}));

        assert_eq!(
            compare_commitments(&med, &ordinary, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn overdue_task_outranks_no_due_task() {
        let now = fixed_now();
        let overdue = commitment_fixture(
            "Ship report",
            Some(now - Duration::hours(2)),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let no_due = commitment_fixture("Clean inbox", None, Some("todo"), json!({}));

        assert_eq!(
            compare_commitments(&overdue, &no_due, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn due_today_with_time_outranks_no_due_task() {
        let now = fixed_now();
        let due_soon = commitment_fixture(
            "Prepare notes",
            Some(now + Duration::hours(3)),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let no_due = commitment_fixture("Someday idea", None, Some("todo"), json!({}));

        assert_eq!(
            compare_commitments(&due_soon, &no_due, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn high_priority_beats_plain_backlog_task() {
        let now = fixed_now();
        let high_priority = commitment_fixture(
            "Urgent follow-up",
            None,
            Some("todo"),
            json!({ "priority": 4 }),
        );
        let backlog = commitment_fixture(
            "Backlog cleanup",
            None,
            Some("todo"),
            json!({ "priority": 1 }),
        );

        assert_eq!(
            compare_commitments(&high_priority, &backlog, now),
            std::cmp::Ordering::Less
        );
    }

    fn fixed_now() -> OffsetDateTime {
        time::Date::from_calendar_date(2026, Month::March, 16)
            .unwrap()
            .with_hms(8, 0, 0)
            .unwrap()
            .assume_utc()
    }

    fn commitment_fixture(
        text: &str,
        due_at: Option<OffsetDateTime>,
        kind: Option<&str>,
        metadata_json: JsonValue,
    ) -> Commitment {
        Commitment {
            id: CommitmentId::from(format!("com_{}", text.replace(' ', "_").to_lowercase())),
            text: text.to_string(),
            source_type: "todoist".to_string(),
            source_id: None,
            status: CommitmentStatus::Open,
            due_at,
            project: None,
            commitment_kind: kind.map(str::to_string),
            created_at: fixed_now() - Duration::hours(1),
            resolved_at: None,
            metadata_json,
        }
    }
}

fn label_for_mode(value: &str) -> NowLabelData {
    match value {
        "meeting_mode" => label(value, "Meeting prep"),
        "commute_mode" => label(value, "Commute"),
        "morning_mode" => label(value, "Morning"),
        "day_mode" => label(value, "Day"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_phase(value: &str) -> NowLabelData {
    match value {
        "awake_unstarted" => label(value, "Not started"),
        "underway" => label(value, "Underway"),
        "engaged" => label(value, "Engaged"),
        "at_risk" => label(value, "At risk"),
        "inactive" => label(value, "Inactive"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_meds(value: &str) -> NowLabelData {
    match value {
        "pending" => label(value, "Pending"),
        "done" => label(value, "Done"),
        "none" => label(value, "None"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_attention(value: &str) -> NowLabelData {
    match value {
        "on_task" => label(value, "On task"),
        "distracted" => label(value, "Distracted"),
        "unknown" => label(value, "Unknown"),
        _ => label(value, value),
    }
}

fn label_for_drift(value: &str) -> NowLabelData {
    match value {
        "morning_drift" => label(value, "Morning drift"),
        "prep_drift" => label(value, "Prep drift"),
        "none" => label(value, "None"),
        _ => label(value, value),
    }
}

fn label_for_severity(value: &str) -> NowLabelData {
    match value {
        "gentle" => label(value, "Gentle"),
        "warning" => label(value, "Warning"),
        "danger" => label(value, "Danger"),
        "none" => label(value, "None"),
        _ => label(value, value),
    }
}

fn risk_summary(level: &str, score: Option<f64>) -> NowRiskSummaryData {
    let label = match score {
        Some(score) => format!("{level} · {}%", (score * 100.0).round() as i64),
        None => level.to_string(),
    };
    NowRiskSummaryData {
        level: level.to_string(),
        score,
        label,
    }
}

fn string_field(value: &JsonValue, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(str::to_string)
}

fn number_field(value: &JsonValue, key: &str) -> Option<f64> {
    value.get(key)?.as_f64()
}

fn bool_field(value: &JsonValue, key: &str) -> Option<bool> {
    value.get(key)?.as_bool()
}

fn string_array_field(value: &JsonValue, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(|entry| entry.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}
