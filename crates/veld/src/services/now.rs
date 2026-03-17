use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_api_types::{
    NowAttentionData, NowData, NowDebugData, NowEventData, NowFreshnessData,
    NowFreshnessEntryData, NowLabelData, NowRiskSummaryData, NowScheduleData, NowSummaryData,
    NowTaskData, NowTasksData,
};
use vel_config::AppConfig;
use vel_core::{Commitment, CommitmentStatus};
use vel_storage::{SignalRecord, Storage};

use crate::{errors::AppError, services::integrations};

pub async fn get_now(storage: &Storage, config: &AppConfig) -> Result<NowData, AppError> {
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let Some((computed_at, context_json)) = storage.get_current_context().await? else {
        return Ok(empty_now(now_ts));
    };
    let context: JsonValue = serde_json::from_str(&context_json).unwrap_or_else(|_| json!({}));

    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;
    let next_commitment_id = string_field(&context, "next_commitment_id");
    let next_commitment = next_commitment_id
        .as_ref()
        .and_then(|id| commitments.iter().find(|commitment| commitment.id.as_ref() == id))
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
    let upcoming_events = events
        .into_iter()
        .filter(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .take(5)
        .collect();

    let integrations = integrations::get_integrations_with_config(storage, config).await?;
    let freshness = build_freshness(now_ts, computed_at, &integrations);
    let attention_reasons = string_array_field(&context, "attention_reasons");
    let reasons = build_reasons(&context, &attention_reasons);

    Ok(NowData {
        computed_at,
        summary: NowSummaryData {
            mode: label_for_mode(string_field(&context, "mode").as_deref().unwrap_or("unknown")),
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

fn empty_now(now_ts: i64) -> NowData {
    NowData {
        computed_at: now_ts,
        summary: NowSummaryData {
            mode: label("unknown", "Unknown"),
            phase: label("unknown", "Unknown"),
            meds: label("unknown", "Unknown"),
            risk: risk_summary("unknown", None),
        },
        schedule: NowScheduleData {
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
    commitments.sort_by(|left, right| {
        let left_due = left.due_at.map(|value| value.unix_timestamp()).unwrap_or(i64::MAX);
        let right_due = right.due_at.map(|value| value.unix_timestamp()).unwrap_or(i64::MAX);
        left_due
            .cmp(&right_due)
            .then_with(|| right.created_at.cmp(&left.created_at))
    });
    commitments
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
    let start_ts = payload.get("start")?.as_i64().or_else(|| payload.get("start_ts")?.as_i64())?;
    let title = payload
        .get("title")
        .and_then(|value| value.as_str())
        .unwrap_or("Untitled event")
        .to_string();
    let travel_minutes = payload.get("travel_minutes").and_then(|value| value.as_i64());
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
