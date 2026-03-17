use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{
    normalize_risk_level, Commitment, CommitmentStatus, CurrentContextV1,
};
use vel_storage::{SignalRecord, Storage};

use crate::{errors::AppError, services::integrations};

#[derive(Debug, Clone)]
pub struct NowOutput {
    pub computed_at: i64,
    pub timezone: String,
    pub summary: NowSummaryOutput,
    pub schedule: NowScheduleOutput,
    pub tasks: NowTasksOutput,
    pub attention: NowAttentionOutput,
    pub sources: NowSourcesOutput,
    pub freshness: NowFreshnessOutput,
    pub reasons: Vec<String>,
    pub debug: NowDebugOutput,
}

#[derive(Debug, Clone)]
pub struct NowLabelOutput {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct NowRiskSummaryOutput {
    pub level: String,
    pub score: Option<f64>,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct NowSummaryOutput {
    pub mode: NowLabelOutput,
    pub phase: NowLabelOutput,
    pub meds: NowLabelOutput,
    pub risk: NowRiskSummaryOutput,
}

#[derive(Debug, Clone)]
pub struct NowEventOutput {
    pub title: String,
    pub start_ts: i64,
    pub end_ts: Option<i64>,
    pub location: Option<String>,
    pub prep_minutes: Option<i64>,
    pub travel_minutes: Option<i64>,
    pub leave_by_ts: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NowTaskOutput {
    pub id: String,
    pub text: String,
    pub source_type: String,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowScheduleOutput {
    pub empty_message: Option<String>,
    pub next_event: Option<NowEventOutput>,
    pub upcoming_events: Vec<NowEventOutput>,
}

#[derive(Debug, Clone)]
pub struct NowTasksOutput {
    pub todoist: Vec<NowTaskOutput>,
    pub other_open: Vec<NowTaskOutput>,
    pub next_commitment: Option<NowTaskOutput>,
}

#[derive(Debug, Clone)]
pub struct NowAttentionOutput {
    pub state: NowLabelOutput,
    pub drift: NowLabelOutput,
    pub severity: NowLabelOutput,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NowSourceActivityOutput {
    pub label: String,
    pub timestamp: i64,
    pub summary: JsonValue,
}

#[derive(Debug, Clone)]
pub struct NowSourcesOutput {
    pub git_activity: Option<NowSourceActivityOutput>,
    pub health: Option<NowSourceActivityOutput>,
    pub mood: Option<NowSourceActivityOutput>,
    pub pain: Option<NowSourceActivityOutput>,
    pub note_document: Option<NowSourceActivityOutput>,
    pub assistant_message: Option<NowSourceActivityOutput>,
}

#[derive(Debug, Clone)]
pub struct NowFreshnessEntryOutput {
    pub key: String,
    pub label: String,
    pub status: String,
    pub last_sync_at: Option<i64>,
    pub age_seconds: Option<i64>,
    pub guidance: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowFreshnessOutput {
    pub overall_status: String,
    pub sources: Vec<NowFreshnessEntryOutput>,
}

#[derive(Debug, Clone)]
pub struct NowDebugOutput {
    pub raw_context: JsonValue,
    pub signals_used: Vec<String>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
}

#[derive(Debug, Clone)]
struct IntegrationGuidanceOutput {
    title: String,
    detail: String,
}

#[derive(Debug, Clone)]
struct CalendarStatusOutput {
    connected: bool,
    all_calendars_selected: bool,
    any_selected: bool,
    last_sync_at: Option<i64>,
    last_sync_status: Option<String>,
    guidance: Option<IntegrationGuidanceOutput>,
}

#[derive(Debug, Clone)]
struct SyncStatusOutput {
    last_sync_at: Option<i64>,
    last_sync_status: Option<String>,
    guidance: Option<IntegrationGuidanceOutput>,
}

#[derive(Debug, Clone)]
struct IntegrationSnapshotOutput {
    google_calendar: CalendarStatusOutput,
    todoist: SyncStatusOutput,
    activity: SyncStatusOutput,
    messaging: SyncStatusOutput,
}

pub async fn get_now(storage: &Storage, config: &AppConfig) -> Result<NowOutput, AppError> {
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let Some((computed_at, context)) = storage.get_current_context().await? else {
        return Ok(empty_now(now_ts, &timezone.name));
    };

    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;
    let next_commitment_id = context.next_commitment_id.clone();
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

    let signal_ids = context.signals_used.clone();
    let calendar_selection = integrations::google_calendar_selection_filter(storage).await?;
    let mut events = storage
        .list_signals_by_ids(&signal_ids)
        .await?
        .into_iter()
        .filter(|signal| calendar_selection.includes_signal(signal))
        .filter_map(calendar_event_from_signal)
        .collect::<Vec<_>>();
    if events.is_empty() {
        events = storage
            .list_signals(Some("calendar_event"), Some(now_ts - 12 * 60 * 60), 32)
            .await?
            .into_iter()
            .filter(|signal| calendar_selection.includes_signal(signal))
            .filter_map(calendar_event_from_signal)
            .collect();
    }
    events.sort_by_key(|event| event.start_ts);
    let next_event = events
        .iter()
        .find(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .cloned();
    let upcoming_events: Vec<NowEventOutput> = events
        .into_iter()
        .filter(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .take(5)
        .collect();

    let integrations = integrations::get_integrations_with_config(storage, config).await?;
    let integrations = IntegrationSnapshotOutput {
        google_calendar: CalendarStatusOutput {
            connected: integrations.google_calendar.connected,
            all_calendars_selected: integrations.google_calendar.all_calendars_selected,
            any_selected: integrations
                .google_calendar
                .calendars
                .iter()
                .any(|calendar| calendar.selected),
            last_sync_at: integrations.google_calendar.last_sync_at,
            last_sync_status: integrations.google_calendar.last_sync_status.clone(),
            guidance: integrations
                .google_calendar
                .guidance
                .as_ref()
                .map(|guidance| IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }),
        },
        todoist: SyncStatusOutput {
            last_sync_at: integrations.todoist.last_sync_at,
            last_sync_status: integrations.todoist.last_sync_status.clone(),
            guidance: integrations.todoist.guidance.as_ref().map(|guidance| {
                IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }
            }),
        },
        activity: SyncStatusOutput {
            last_sync_at: integrations.activity.last_sync_at,
            last_sync_status: integrations.activity.last_sync_status.clone(),
            guidance: integrations.activity.guidance.as_ref().map(|guidance| {
                IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }
            }),
        },
        messaging: SyncStatusOutput {
            last_sync_at: integrations.messaging.last_sync_at,
            last_sync_status: integrations.messaging.last_sync_status.clone(),
            guidance: integrations.messaging.guidance.as_ref().map(|guidance| {
                IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }
            }),
        },
    };
    let freshness = build_freshness(now_ts, computed_at, &integrations, &calendar_selection);
    let schedule_empty_message = schedule_empty_message(&integrations, upcoming_events.is_empty());
    let attention_reasons = context.attention_reasons.clone();
    let reasons = build_reasons_typed(&context, &attention_reasons);

    Ok(NowOutput {
        computed_at,
        timezone: timezone.name,
        summary: NowSummaryOutput {
            mode: label_for_mode(context.mode.as_str()),
            phase: label_for_phase(context.morning_state.as_str()),
            meds: label_for_meds(context.meds_status.as_str()),
            risk: risk_summary(
                context.global_risk_level.as_str(),
                context.global_risk_score,
            ),
        },
        schedule: NowScheduleOutput {
            empty_message: schedule_empty_message,
            next_event,
            upcoming_events,
        },
        tasks: NowTasksOutput {
            todoist,
            other_open,
            next_commitment,
        },
        attention: NowAttentionOutput {
            state: label_for_attention(context.attention_state.as_str()),
            drift: label_for_drift(context.drift_type.as_deref().unwrap_or("none")),
            severity: label_for_severity(context.drift_severity.as_deref().unwrap_or("none")),
            confidence: context.attention_confidence,
            reasons: attention_reasons,
        },
        sources: NowSourcesOutput {
            git_activity: context_source_activity_typed(
                &context,
                "git_activity_summary",
                "Git activity",
                context.git_activity_summary.clone(),
            ),
            health: context_source_activity_typed(
                &context,
                "health_summary",
                "Health",
                context.health_summary.clone(),
            ),
            mood: context_source_activity_typed(
                &context,
                "mood_summary",
                "Mood",
                context.mood_summary.clone(),
            ),
            pain: context_source_activity_typed(
                &context,
                "pain_summary",
                "Pain",
                context.pain_summary.clone(),
            ),
            note_document: context_source_activity_typed(
                &context,
                "note_document_summary",
                "Recent note",
                context.note_document_summary.clone(),
            ),
            assistant_message: context_source_activity_typed(
                &context,
                "assistant_message_summary",
                "Recent transcript",
                context.assistant_message_summary.clone(),
            ),
        },
        freshness,
        reasons,
        debug: NowDebugOutput {
            raw_context: context.clone().into_json(),
            signals_used: context.signals_used.clone(),
            commitments_used: context.commitments_used.clone(),
            risk_used: context.risk_used.clone(),
        },
    })
}

fn build_reasons_typed(context: &CurrentContextV1, attention_reasons: &[String]) -> Vec<String> {
    let mut reasons = Vec::new();
    if !context.mode.is_empty() {
        reasons.push(format!("Mode: {}", label_for_mode(&context.mode).label));
    }
    if context.prep_window_active {
        reasons.push("Prep window active".to_string());
    }
    if context.commute_window_active {
        reasons.push("Commute window active".to_string());
    }
    if context.meds_status == "pending" {
        reasons.push("Medication task is still pending".to_string());
    }
    reasons.extend(attention_reasons.iter().cloned());
    reasons.truncate(8);
    reasons
}

fn context_source_activity_typed(
    context: &CurrentContextV1,
    key: &str,
    label: &str,
    typed_summary: Option<JsonValue>,
) -> Option<NowSourceActivityOutput> {
    let summary = typed_summary.or_else(|| context.extra.get(key).cloned())?;
    let timestamp = summary.get("timestamp").and_then(JsonValue::as_i64).unwrap_or(context.computed_at);
    Some(NowSourceActivityOutput {
        label: label.to_string(),
        timestamp,
        summary,
    })
}

fn empty_now(now_ts: i64, timezone: &str) -> NowOutput {
    NowOutput {
        computed_at: now_ts,
        timezone: timezone.to_string(),
        summary: NowSummaryOutput {
            mode: label("unknown", "Unknown"),
            phase: label("unknown", "Unknown"),
            meds: label("unknown", "Unknown"),
            risk: risk_summary("unknown", None),
        },
        schedule: NowScheduleOutput {
            empty_message: Some(
                "No current context yet. Sync calendar sources or run evaluate.".to_string(),
            ),
            next_event: None,
            upcoming_events: Vec::new(),
        },
        tasks: NowTasksOutput {
            todoist: Vec::new(),
            other_open: Vec::new(),
            next_commitment: None,
        },
        attention: NowAttentionOutput {
            state: label("unknown", "Unknown"),
            drift: label("none", "None"),
            severity: label("none", "None"),
            confidence: None,
            reasons: Vec::new(),
        },
        sources: NowSourcesOutput {
            git_activity: None,
            health: None,
            mood: None,
            pain: None,
            note_document: None,
            assistant_message: None,
        },
        freshness: NowFreshnessOutput {
            overall_status: "stale".to_string(),
            sources: vec![NowFreshnessEntryOutput {
                key: "context".to_string(),
                label: "Context".to_string(),
                status: "missing".to_string(),
                last_sync_at: None,
                age_seconds: None,
                guidance: None,
            }],
        },
        reasons: vec!["No current context yet. Sync integrations or run evaluate.".to_string()],
        debug: NowDebugOutput {
            raw_context: json!({}),
            signals_used: Vec::new(),
            commitments_used: Vec::new(),
            risk_used: Vec::new(),
        },
    }
}

fn schedule_empty_message(
    integrations: &IntegrationSnapshotOutput,
    no_upcoming_events: bool,
) -> Option<String> {
    if !no_upcoming_events {
        return None;
    }

    let calendar = &integrations.google_calendar;
    if !calendar.connected {
        return Some("Google Calendar is disconnected. Reconnect it in Settings.".to_string());
    }
    if !calendar.all_calendars_selected && !calendar.any_selected {
        return Some("No calendars are selected in Settings.".to_string());
    }

    Some("No upcoming calendar events in the current stream.".to_string())
}

fn build_freshness(
    now_ts: i64,
    computed_at: i64,
    integrations: &IntegrationSnapshotOutput,
    calendar_selection: &integrations::GoogleCalendarSelectionFilter,
) -> NowFreshnessOutput {
    let mut sources = vec![NowFreshnessEntryOutput {
        key: "context".to_string(),
        label: "Context".to_string(),
        status: age_status(now_ts - computed_at).to_string(),
        last_sync_at: Some(computed_at),
        age_seconds: Some(now_ts - computed_at),
        guidance: None,
    }];
    sources.push(sync_freshness(
        now_ts,
        "calendar",
        "Calendar",
        integrations.google_calendar.last_sync_at,
        integrations.google_calendar.last_sync_status.as_deref(),
        integrations
            .google_calendar
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        (!calendar_selection.has_any_selected()).then_some(
            "No calendars are selected in Settings. Unchecked calendars are excluded from Vel context by default."
                .to_string(),
        ),
    ));
    sources.push(sync_freshness(
        now_ts,
        "todoist",
        "Todoist",
        integrations.todoist.last_sync_at,
        integrations.todoist.last_sync_status.as_deref(),
        integrations
            .todoist
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        None,
    ));
    sources.push(sync_freshness(
        now_ts,
        "activity",
        "Activity",
        integrations.activity.last_sync_at,
        integrations.activity.last_sync_status.as_deref(),
        integrations
            .activity
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        None,
    ));
    sources.push(sync_freshness(
        now_ts,
        "messaging",
        "Messaging",
        integrations.messaging.last_sync_at,
        integrations.messaging.last_sync_status.as_deref(),
        integrations
            .messaging
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        None,
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
    NowFreshnessOutput {
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
    guidance: Option<String>,
    override_guidance: Option<String>,
) -> NowFreshnessEntryOutput {
    let age_seconds = last_sync_at.map(|timestamp| now_ts - timestamp);
    let status = if override_guidance.is_some() {
        "unchecked"
    } else {
        match last_sync_status {
            Some("error") => "error",
            Some("disconnected") => "disconnected",
            _ => age_seconds.map(age_status).unwrap_or("missing"),
        }
    };
    NowFreshnessEntryOutput {
        key: key.to_string(),
        label: label_text.to_string(),
        status: status.to_string(),
        last_sync_at,
        age_seconds,
        guidance: override_guidance.or(guidance),
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

fn now_task(commitment: &Commitment) -> NowTaskOutput {
    NowTaskOutput {
        id: commitment.id.as_ref().to_string(),
        text: commitment.text.clone(),
        source_type: commitment.source_type.clone(),
        due_at: commitment.due_at,
        project: commitment.project.clone(),
        commitment_kind: commitment.commitment_kind.clone(),
    }
}

fn calendar_event_from_signal(signal: SignalRecord) -> Option<NowEventOutput> {
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
    Some(NowEventOutput {
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

fn label(key: &str, text: &str) -> NowLabelOutput {
    NowLabelOutput {
        key: key.to_string(),
        label: text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Duration, Month};
    use vel_core::{CommitmentId, CommitmentStatus, ContextMigrator};

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

    #[test]
    fn risk_summary_normalizes_unrecognized_levels_to_unknown() {
        let summary = risk_summary("danger", Some(0.4));

        assert_eq!(summary.level, "unknown");
        assert_eq!(summary.label, "unknown · 40%");
    }

    #[test]
    fn now_context_shape_parses_with_typed_context_migrator() {
        let context = json!({
            "computed_at": 1_700_000_000i64,
            "mode": "morning_mode",
            "morning_state": "underway",
            "meds_status": "pending",
            "attention_confidence": 0.9,
            "signals_used": ["sig_1"],
            "commitments_used": ["com_1"],
            "risk_used": ["risk_1"]
        });

        let typed = ContextMigrator::from_json_value(context).expect("context should parse");
        assert_eq!(typed.mode, "morning_mode");
        assert_eq!(typed.attention_confidence, Some(0.9));
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

fn label_for_mode(value: &str) -> NowLabelOutput {
    match value {
        "meeting_mode" => label(value, "Meeting prep"),
        "commute_mode" => label(value, "Commute"),
        "morning_mode" => label(value, "Morning"),
        "day_mode" => label(value, "Day"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_phase(value: &str) -> NowLabelOutput {
    match value {
        "awake_unstarted" => label(value, "Not started"),
        "underway" => label(value, "Underway"),
        "engaged" => label(value, "Engaged"),
        "at_risk" => label(value, "At risk"),
        "inactive" => label(value, "Inactive"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_meds(value: &str) -> NowLabelOutput {
    match value {
        "pending" => label(value, "Pending"),
        "done" => label(value, "Done"),
        "none" => label(value, "None"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_attention(value: &str) -> NowLabelOutput {
    match value {
        "on_task" => label(value, "On task"),
        "distracted" => label(value, "Distracted"),
        "unknown" => label(value, "Unknown"),
        _ => label(value, value),
    }
}

fn label_for_drift(value: &str) -> NowLabelOutput {
    match value {
        "morning_drift" => label(value, "Morning drift"),
        "prep_drift" => label(value, "Prep drift"),
        "none" => label(value, "None"),
        _ => label(value, value),
    }
}

fn label_for_severity(value: &str) -> NowLabelOutput {
    match value {
        "gentle" => label(value, "Gentle"),
        "warning" => label(value, "Warning"),
        "danger" => label(value, "Danger"),
        "none" => label(value, "None"),
        _ => label(value, value),
    }
}

fn risk_summary(level: &str, score: Option<f64>) -> NowRiskSummaryOutput {
    let normalized_level = normalize_risk_level(level);
    let label = match score {
        Some(score) => format!("{normalized_level} · {}%", (score * 100.0).round() as i64),
        None => normalized_level.to_string(),
    };
    NowRiskSummaryOutput {
        level: normalized_level.to_string(),
        score,
        label,
    }
}

