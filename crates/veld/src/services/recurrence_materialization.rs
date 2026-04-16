use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime, UtcOffset, Weekday};
use vel_core::{
    Event, EventMoment, EventMomentKind, EventTransparency, ExceptionStatus, Occurrence,
    RecurrenceFrequency, RecurrenceWeekday, Series,
};

use crate::{errors::AppError, services::time_format::format_utc_rfc3339};

pub struct RecurrenceMaterializationService;

impl RecurrenceMaterializationService {
    pub fn materialize(
        event: &Event,
        series: &Series,
        window_start: OffsetDateTime,
        window_end: OffsetDateTime,
    ) -> Result<Vec<Occurrence>, AppError> {
        series.validate().map_err(AppError::bad_request)?;
        event.validate().map_err(AppError::bad_request)?;

        if event.start.kind != EventMomentKind::ZonedDateTime
            || event.end.kind != EventMomentKind::ZonedDateTime
        {
            return Err(AppError::bad_request(
                "recurrence materialization currently requires zoned event moments",
            ));
        }

        let anchor_start = parse_rfc3339(&event.start.value)?;
        let anchor_end = parse_rfc3339(&event.end.value)?;
        let duration = anchor_end - anchor_start;
        let until = series
            .rule
            .until
            .as_deref()
            .map(parse_rfc3339)
            .transpose()?;

        let mut current = anchor_start;
        let mut index = 0u32;
        let mut results = Vec::new();

        loop {
            if let Some(count) = series.rule.count {
                if index >= count {
                    break;
                }
            }
            if let Some(until) = until {
                if current > until {
                    break;
                }
            }

            if current >= window_start && current <= window_end {
                let occurrence_key = format!(
                    "{}:{}",
                    series.anchor_event_id,
                    format_utc_rfc3339(current)?
                );

                if let Some(exception) = series
                    .exceptions
                    .iter()
                    .find(|exception| exception.occurrence_key == occurrence_key)
                {
                    match exception.status {
                        ExceptionStatus::Cancelled => {
                            results.push(Occurrence {
                                occurrence_key,
                                series_id: series.series_id.clone(),
                                anchor_event_id: series.anchor_event_id.clone(),
                                start: event.start.clone(),
                                end: event.end.clone(),
                                materialized: true,
                                exception_status: Some(ExceptionStatus::Cancelled),
                            });
                        }
                        ExceptionStatus::Modified => {
                            let replacement_start =
                                exception.replacement_start.clone().ok_or_else(|| {
                                    AppError::bad_request("modified exception missing start")
                                })?;
                            let replacement_end =
                                exception.replacement_end.clone().ok_or_else(|| {
                                    AppError::bad_request("modified exception missing end")
                                })?;

                            results.push(Occurrence {
                                occurrence_key,
                                series_id: series.series_id.clone(),
                                anchor_event_id: series.anchor_event_id.clone(),
                                start: replacement_start,
                                end: replacement_end,
                                materialized: true,
                                exception_status: Some(ExceptionStatus::Modified),
                            });
                        }
                    }
                } else {
                    results.push(Occurrence {
                        occurrence_key,
                        series_id: series.series_id.clone(),
                        anchor_event_id: series.anchor_event_id.clone(),
                        start: EventMoment {
                            kind: EventMomentKind::ZonedDateTime,
                            value: format_utc_rfc3339(current)?,
                            timezone: event.start.timezone.clone(),
                        },
                        end: EventMoment {
                            kind: EventMomentKind::ZonedDateTime,
                            value: format_utc_rfc3339(current + duration)?,
                            timezone: event.end.timezone.clone(),
                        },
                        materialized: true,
                        exception_status: None,
                    });
                }
            }

            current = next_occurrence(current, &series.rule)?;
            index += 1;

            if current > window_end + Duration::days(366) {
                return Err(AppError::bad_request(
                    "recurrence materialization exceeded bounded safety window",
                ));
            }
        }

        Ok(results)
    }
}

fn parse_rfc3339(value: &str) -> Result<OffsetDateTime, AppError> {
    OffsetDateTime::parse(value, &Rfc3339)
        .map(|value| value.to_offset(UtcOffset::UTC))
        .map_err(|error| AppError::bad_request(format!("invalid zoned event moment: {error}")))
}

fn next_occurrence(
    current: OffsetDateTime,
    rule: &vel_core::SeriesRule,
) -> Result<OffsetDateTime, AppError> {
    match rule.frequency {
        RecurrenceFrequency::Daily => Ok(current + Duration::days(i64::from(rule.interval))),
        RecurrenceFrequency::Weekly => next_weekly_occurrence(current, rule),
        RecurrenceFrequency::Monthly | RecurrenceFrequency::Yearly => Err(AppError::bad_request(
            "monthly and yearly recurrence materialization are deferred in 0.5 proof service",
        )),
    }
}

fn next_weekly_occurrence(
    current: OffsetDateTime,
    rule: &vel_core::SeriesRule,
) -> Result<OffsetDateTime, AppError> {
    let current_weekday = current.weekday();
    let weekdays = normalize_weekdays(&rule.by_weekdays);
    let current_index = weekdays
        .iter()
        .position(|weekday| *weekday == current_weekday);

    if let Some(index) = current_index {
        if let Some(next_same_week) = weekdays.get(index + 1) {
            let days = days_until(current_weekday, *next_same_week);
            return Ok(current + Duration::days(days));
        }
    }

    let first = weekdays
        .first()
        .copied()
        .ok_or_else(|| AppError::bad_request("weekly recurrence requires weekdays"))?;
    let wrap_days =
        days_until(current_weekday, first) + (i64::from(rule.interval).saturating_sub(1) * 7);

    Ok(current + Duration::days(wrap_days))
}

fn normalize_weekdays(values: &[RecurrenceWeekday]) -> Vec<Weekday> {
    values
        .iter()
        .map(|value| match value {
            RecurrenceWeekday::Monday => Weekday::Monday,
            RecurrenceWeekday::Tuesday => Weekday::Tuesday,
            RecurrenceWeekday::Wednesday => Weekday::Wednesday,
            RecurrenceWeekday::Thursday => Weekday::Thursday,
            RecurrenceWeekday::Friday => Weekday::Friday,
            RecurrenceWeekday::Saturday => Weekday::Saturday,
            RecurrenceWeekday::Sunday => Weekday::Sunday,
        })
        .collect()
}

fn days_until(from: Weekday, to: Weekday) -> i64 {
    let from = from.number_days_from_monday() as i64;
    let to = to.number_days_from_monday() as i64;
    let delta = to - from;
    if delta <= 0 {
        delta + 7
    } else {
        delta
    }
}

#[allow(dead_code)]
fn _keep_event_transparency_imported(_: EventTransparency) {}
