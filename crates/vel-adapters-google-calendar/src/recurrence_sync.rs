use serde_json::{Value as JsonValue, json};
use vel_core::{
    Event, EventMoment, Exception, ExceptionStatus, Occurrence, RecurrenceFrequency,
    RecurrenceWeekday, Series, SeriesRule,
};

use crate::{
    event_mapping::GoogleEventMomentPayload,
    google_ids::{GOOGLE_CALENDAR_MODULE_ID, google_provider_object_ref},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleRecurrencePayload {
    pub remote_id: String,
    pub recurring_event_remote_id: Option<String>,
    pub original_start: Option<GoogleEventMomentPayload>,
    pub rrule: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoogleRecurrenceMapping {
    pub series: Option<Series>,
    pub occurrence: Option<Occurrence>,
    pub provider_facets: JsonValue,
}

pub fn map_google_recurrence(
    anchor_event: &Event,
    payload: &GoogleRecurrencePayload,
) -> Result<GoogleRecurrenceMapping, String> {
    let series = payload
        .rrule
        .as_deref()
        .map(|rrule| build_series(anchor_event, payload, rrule))
        .transpose()?;

    let occurrence = payload
        .recurring_event_remote_id
        .as_ref()
        .zip(payload.original_start.as_ref())
        .map(|(series_remote_id, original_start)| {
            build_occurrence(
                anchor_event,
                series_remote_id,
                original_start,
                payload.status.as_deref(),
            )
        });

    Ok(GoogleRecurrenceMapping {
        series,
        occurrence,
        provider_facets: json!({
            "google_calendar": {
                "source_ref": google_provider_object_ref("event", &payload.remote_id),
                "recurring_event_id": payload.recurring_event_remote_id,
                "raw_rrule": payload.rrule,
                "write_scope_support": {
                    "single_occurrence": true,
                    "entire_series": true,
                    "this_and_following": false,
                },
                "module_id": GOOGLE_CALENDAR_MODULE_ID,
            }
        }),
    })
}

fn build_series(
    anchor_event: &Event,
    payload: &GoogleRecurrencePayload,
    rrule: &str,
) -> Result<Series, String> {
    let rule = parse_rrule(rrule)?;
    let series = Series {
        series_id: format!("series:{}", anchor_event.id),
        anchor_event_id: anchor_event.id.clone(),
        timezone: anchor_event.start.timezone.clone(),
        rule,
        exceptions: if let Some(original_start) = payload.original_start.as_ref() {
            vec![build_exception(
                anchor_event,
                original_start,
                payload.status.as_deref(),
            )]
        } else {
            vec![]
        },
    };
    series.validate()?;
    Ok(series)
}

fn build_occurrence(
    anchor_event: &Event,
    series_remote_id: &str,
    original_start: &GoogleEventMomentPayload,
    status: Option<&str>,
) -> Occurrence {
    Occurrence {
        occurrence_key: occurrence_key(anchor_event, original_start),
        series_id: format!("google-calendar:series:{series_remote_id}"),
        anchor_event_id: anchor_event.id.clone(),
        start: to_moment(original_start),
        end: match status {
            Some("cancelled") => to_moment(original_start),
            _ => anchor_event.end.clone(),
        },
        materialized: true,
        exception_status: Some(exception_status(status)),
    }
}

fn build_exception(
    anchor_event: &Event,
    original_start: &GoogleEventMomentPayload,
    status: Option<&str>,
) -> Exception {
    let exception_status = exception_status(status);
    Exception {
        occurrence_key: occurrence_key(anchor_event, original_start),
        status: exception_status,
        replacement_start: match exception_status {
            ExceptionStatus::Modified => Some(anchor_event.start.clone()),
            ExceptionStatus::Cancelled => None,
        },
        replacement_end: match exception_status {
            ExceptionStatus::Modified => Some(anchor_event.end.clone()),
            ExceptionStatus::Cancelled => None,
        },
    }
}

fn exception_status(status: Option<&str>) -> ExceptionStatus {
    match status {
        Some("cancelled") => ExceptionStatus::Cancelled,
        _ => ExceptionStatus::Modified,
    }
}

fn occurrence_key(anchor_event: &Event, original_start: &GoogleEventMomentPayload) -> String {
    format!("{}:{}", anchor_event.id, original_start.value)
}

fn to_moment(payload: &GoogleEventMomentPayload) -> EventMoment {
    EventMoment {
        kind: payload.kind.clone(),
        value: payload.value.clone(),
        timezone: payload.timezone.clone(),
    }
}

fn parse_rrule(rrule: &str) -> Result<SeriesRule, String> {
    let mut frequency = None;
    let mut interval = 1u16;
    let mut by_weekdays = Vec::new();
    let mut count = None;
    let mut until = None;

    for part in rrule.split(';') {
        let mut pieces = part.splitn(2, '=');
        let key = pieces.next().unwrap_or_default();
        let value = pieces.next().unwrap_or_default();

        match key {
            "FREQ" => {
                frequency = Some(match value {
                    "DAILY" => RecurrenceFrequency::Daily,
                    "WEEKLY" => RecurrenceFrequency::Weekly,
                    "MONTHLY" => RecurrenceFrequency::Monthly,
                    "YEARLY" => RecurrenceFrequency::Yearly,
                    other => {
                        return Err(format!("unsupported Google recurrence frequency {other}"));
                    }
                });
            }
            "INTERVAL" => {
                interval = value
                    .parse::<u16>()
                    .map_err(|error| format!("invalid recurrence interval {value}: {error}"))?;
            }
            "COUNT" => {
                count = Some(
                    value
                        .parse::<u32>()
                        .map_err(|error| format!("invalid recurrence count {value}: {error}"))?,
                );
            }
            "UNTIL" => {
                until = Some(value.to_string());
            }
            "BYDAY" => {
                by_weekdays = value
                    .split(',')
                    .filter(|token| !token.trim().is_empty())
                    .map(parse_weekday)
                    .collect::<Result<Vec<_>, _>>()?;
            }
            _ => {}
        }
    }

    let rule = SeriesRule {
        frequency: frequency.ok_or_else(|| "RRULE missing FREQ".to_string())?,
        interval,
        by_weekdays,
        count,
        until,
        raw_rrule: Some(rrule.to_string()),
    };
    rule.validate()?;
    Ok(rule)
}

fn parse_weekday(token: &str) -> Result<RecurrenceWeekday, String> {
    match token {
        "MO" => Ok(RecurrenceWeekday::Monday),
        "TU" => Ok(RecurrenceWeekday::Tuesday),
        "WE" => Ok(RecurrenceWeekday::Wednesday),
        "TH" => Ok(RecurrenceWeekday::Thursday),
        "FR" => Ok(RecurrenceWeekday::Friday),
        "SA" => Ok(RecurrenceWeekday::Saturday),
        "SU" => Ok(RecurrenceWeekday::Sunday),
        other => Err(format!("unsupported Google recurrence weekday {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::{GoogleRecurrencePayload, map_google_recurrence};
    use crate::event_mapping::GoogleEventMomentPayload;
    use serde_json::Value as JsonValue;
    use vel_core::{
        Event, EventId, EventMoment, EventMomentKind, EventTransparency, ExceptionStatus,
        RecurrenceFrequency, RecurrenceWeekday,
    };

    fn anchor_event() -> Event {
        Event {
            id: EventId::from("event_standup".to_string()),
            title: "Standup".to_string(),
            description: None,
            start: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-24T09:00:00-06:00".to_string(),
                timezone: Some("America/Denver".to_string()),
            },
            end: EventMoment {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-24T09:30:00-06:00".to_string(),
                timezone: Some("America/Denver".to_string()),
            },
            transparency: EventTransparency::Opaque,
            location: None,
        }
    }

    #[test]
    fn recurrence_sync_builds_series_and_single_occurrence_exceptions() {
        let mapped = map_google_recurrence(
            &anchor_event(),
            &GoogleRecurrencePayload {
                remote_id: "evt_override".to_string(),
                recurring_event_remote_id: Some("evt_series".to_string()),
                original_start: Some(GoogleEventMomentPayload {
                    kind: EventMomentKind::ZonedDateTime,
                    value: "2026-03-25T09:00:00-06:00".to_string(),
                    timezone: Some("America/Denver".to_string()),
                }),
                rrule: Some("FREQ=WEEKLY;INTERVAL=1;BYDAY=WE;COUNT=4".to_string()),
                status: Some("cancelled".to_string()),
            },
        )
        .unwrap();

        let series = mapped.series.expect("series should be mapped");
        assert_eq!(series.rule.frequency, RecurrenceFrequency::Weekly);
        assert_eq!(series.rule.by_weekdays, vec![RecurrenceWeekday::Wednesday]);
        assert_eq!(series.exceptions.len(), 1);
        assert_eq!(series.exceptions[0].status, ExceptionStatus::Cancelled);

        let occurrence = mapped
            .occurrence
            .expect("occurrence should be materialized");
        assert_eq!(
            occurrence.exception_status,
            Some(ExceptionStatus::Cancelled)
        );
        assert_eq!(occurrence.series_id, "google-calendar:series:evt_series");
        assert_eq!(
            mapped.provider_facets["google_calendar"]["write_scope_support"]["this_and_following"],
            JsonValue::Bool(false)
        );
    }
}
