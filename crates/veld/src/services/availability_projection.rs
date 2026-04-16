use serde_json::json;
use sqlx::SqlitePool;
use time::{format_description::well_known::Rfc3339, Date, OffsetDateTime, Time, UtcOffset};
use vel_core::{
    AllDayHandlingRule, AvailabilityBasis, AvailabilityConfidence, AvailabilityPolicyConfig,
    AvailabilityResult, AvailabilityWindow, BlockingInterval, Calendar, Event, EventMomentKind,
    EventTransparency, ParticipationResponseStatus,
};
use vel_storage::{upsert_projection, ProjectionRecord};

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct AvailabilityEventInput {
    pub calendar: Calendar,
    pub event: Event,
    pub source_account_id: Option<String>,
    pub response_status: Option<ParticipationResponseStatus>,
    pub cancelled: bool,
}

pub struct AvailabilityProjectionService;

impl AvailabilityProjectionService {
    pub fn project(
        window_start: OffsetDateTime,
        window_end: OffsetDateTime,
        config: &AvailabilityPolicyConfig,
        events: &[AvailabilityEventInput],
    ) -> Result<AvailabilityWindow, AppError> {
        let mut blocking_intervals = Vec::new();
        let mut sources_consulted = Vec::new();
        let mut filters_applied = vec![
            "included_calendars".to_string(),
            "transparency".to_string(),
            "declined_policy".to_string(),
            "cancelled_exclusion".to_string(),
            "all_day_rule".to_string(),
            "source_account_policy".to_string(),
        ];

        for input in events {
            sources_consulted.push(input.calendar.id.to_string());

            if !config.included_calendar_ids.is_empty()
                && !config
                    .included_calendar_ids
                    .iter()
                    .any(|id| id == &input.calendar.id)
            {
                continue;
            }

            if !config.source_account_ids.is_empty() {
                let Some(account_id) = &input.source_account_id else {
                    continue;
                };
                if !config.source_account_ids.iter().any(|id| id == account_id) {
                    continue;
                }
            }

            if config.exclude_cancelled_events && input.cancelled {
                continue;
            }

            if matches!(
                (config.declined_response_policy, input.response_status),
                (
                    vel_core::DeclinedResponsePolicy::IgnoreDeclined,
                    Some(ParticipationResponseStatus::Declined)
                )
            ) {
                continue;
            }

            if input.event.transparency == EventTransparency::Transparent {
                continue;
            }

            let Some((event_start, event_end)) =
                blocking_bounds(&input.event, config.all_day_handling_rule)?
            else {
                continue;
            };

            if event_end <= window_start || event_start >= window_end {
                continue;
            }

            blocking_intervals.push(BlockingInterval {
                event_id: input.event.id.to_string(),
                calendar_id: input.calendar.id.to_string(),
                start: format_utc_rfc3339(event_start)?,
                end: format_utc_rfc3339(event_end)?,
                reason: "opaque_calendar_event".to_string(),
            });
        }

        blocking_intervals.sort_by(|a, b| a.start.cmp(&b.start));
        sources_consulted.sort();
        sources_consulted.dedup();
        filters_applied.sort();

        Ok(AvailabilityWindow {
            window_start: format_utc_rfc3339(window_start)?,
            window_end: format_utc_rfc3339(window_end)?,
            result: if blocking_intervals.is_empty() {
                AvailabilityResult::Free
            } else {
                AvailabilityResult::Busy
            },
            blocking_intervals,
            sources_consulted,
            filters_applied,
            basis: AvailabilityBasis::Exact,
            confidence: Some(AvailabilityConfidence::High),
        })
    }

    pub async fn materialize(
        pool: &SqlitePool,
        projection_id: &str,
        window: &AvailabilityWindow,
    ) -> Result<ProjectionRecord, AppError> {
        let now = OffsetDateTime::now_utc();
        let projection = ProjectionRecord {
            id: projection_id.to_string(),
            projection_type: "availability".to_string(),
            object_id: None,
            source_summary_json: Some(json!({
                "basis": "derived",
                "result": window.result,
            })),
            projection_json: serde_json::to_value(window)
                .map_err(|error| AppError::internal(error.to_string()))?,
            rebuild_token: Some(format!("availability:{}", now.unix_timestamp())),
            created_at: now,
            updated_at: now,
        };

        upsert_projection(pool, &projection)
            .await
            .map_err(AppError::from)?;

        Ok(projection)
    }
}

fn blocking_bounds(
    event: &Event,
    all_day_rule: AllDayHandlingRule,
) -> Result<Option<(OffsetDateTime, OffsetDateTime)>, AppError> {
    match (&event.start.kind, &event.end.kind) {
        (EventMomentKind::ZonedDateTime, EventMomentKind::ZonedDateTime) => {
            let start = OffsetDateTime::parse(&event.start.value, &Rfc3339)
                .map_err(|error| AppError::bad_request(format!("invalid event start: {error}")))?
                .to_offset(UtcOffset::UTC);
            let end = OffsetDateTime::parse(&event.end.value, &Rfc3339)
                .map_err(|error| AppError::bad_request(format!("invalid event end: {error}")))?
                .to_offset(UtcOffset::UTC);
            Ok(Some((start, end)))
        }
        (EventMomentKind::AllDay, EventMomentKind::AllDay) => match all_day_rule {
            AllDayHandlingRule::RespectTransparency => {
                let date_format = time::format_description::parse("[year]-[month]-[day]")
                    .expect("hardcoded date format should parse");
                let start = Date::parse(&event.start.value, &date_format)
                    .map_err(|error| {
                        AppError::bad_request(format!("invalid all-day start: {error}"))
                    })?
                    .with_time(Time::MIDNIGHT)
                    .assume_utc();
                let end = Date::parse(&event.end.value, &date_format)
                    .map_err(|error| {
                        AppError::bad_request(format!("invalid all-day end: {error}"))
                    })?
                    .with_time(
                        Time::from_hms(23, 59, 59)
                            .expect("hardcoded all-day end time should parse"),
                    )
                    .assume_utc();
                Ok(Some((start, end)))
            }
        },
        _ => Ok(None),
    }
}

fn format_utc_rfc3339(value: OffsetDateTime) -> Result<String, AppError> {
    let formatted = value
        .to_offset(UtcOffset::UTC)
        .format(&Rfc3339)
        .map_err(|error| AppError::internal(error.to_string()))?;
    Ok(formatted
        .strip_suffix('Z')
        .map(|prefix| format!("{prefix}+00:00"))
        .unwrap_or(formatted))
}
