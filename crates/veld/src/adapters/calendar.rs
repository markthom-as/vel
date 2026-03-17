//! Calendar adapter: read .ics from config path or URL, normalize to calendar_event signals.
//! Minimal line-based ICS parse for VEVENT metadata used by context, risk, and nudges.

use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

const DEFAULT_PREP_MINUTES: i64 = 15;
const DEFAULT_TRAVEL_MINUTES: i64 = 0;

/// Ingest calendar events from config (ics_path or ics_url). Returns count of signals ingested.
pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let ics_content = if let Some(path) = &config.calendar_ics_path {
        tokio::fs::read_to_string(path).await.map_err(|e| {
            crate::errors::AppError::internal(format!("read ics path {}: {}", path, e))
        })?
    } else if let Some(url) = &config.calendar_ics_url {
        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| crate::errors::AppError::internal(format!("fetch ics url: {}", e)))?;
        resp.text()
            .await
            .map_err(|e| crate::errors::AppError::internal(format!("ics response body: {}", e)))?
    } else {
        return Ok(0);
    };

    let events = parse_ics_events(&ics_content);
    let mut count = 0u32;
    for ev in events {
        storage
            .insert_signal(SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: None,
                timestamp: ev.start_ts,
                payload_json: Some(ev.payload),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        count += 1;
    }
    Ok(count)
}

struct ParsedEvent {
    start_ts: i64,
    payload: serde_json::Value,
}

fn parse_ics_events(content: &str) -> Vec<ParsedEvent> {
    let mut events = Vec::new();
    let mut in_vevent = false;
    let mut uid = String::new();
    let mut summary = String::new();
    let mut start_ts: Option<i64> = None;
    let mut end_ts: Option<i64> = None;
    let mut location = String::new();
    let mut description = String::new();
    let mut status = String::new();
    let mut url = String::new();
    let mut attendees: Vec<String> = Vec::new();
    let mut prep_minutes: Option<i64> = None;
    let mut travel_minutes: Option<i64> = None;
    let mut travel_start_ts: Option<i64> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.eq_ignore_ascii_case("BEGIN:VEVENT") {
            in_vevent = true;
            uid.clear();
            summary.clear();
            start_ts = None;
            end_ts = None;
            location.clear();
            description.clear();
            status.clear();
            url.clear();
            attendees.clear();
            prep_minutes = None;
            travel_minutes = None;
            travel_start_ts = None;
            continue;
        }
        if line.eq_ignore_ascii_case("END:VEVENT") {
            in_vevent = false;
            if status.eq_ignore_ascii_case("CANCELLED") {
                continue;
            }
            if let Some(ts) = start_ts {
                let derived_travel_minutes = travel_minutes.or_else(|| {
                    travel_start_ts.and_then(|travel_start| {
                        let delta_seconds = ts - travel_start;
                        (delta_seconds > 0).then_some(delta_seconds / 60)
                    })
                });
                let payload = serde_json::json!({
                    "event_id": uid,
                    "title": summary,
                    "start": ts,
                    "end": end_ts,
                    "location": location,
                    "description": description,
                    "status": status,
                    "url": url,
                    "attendees": attendees,
                    "prep_minutes": prep_minutes.unwrap_or(DEFAULT_PREP_MINUTES),
                    "travel_minutes": derived_travel_minutes.unwrap_or(DEFAULT_TRAVEL_MINUTES)
                });
                events.push(ParsedEvent {
                    start_ts: ts,
                    payload,
                });
            }
            continue;
        }
        if !in_vevent {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            let raw_name = name.trim();
            let name = raw_name.to_uppercase();
            let base_name = name.split(';').next().unwrap_or(name.as_str());
            let value = value.trim();
            match base_name {
                "UID" => uid = value.to_string(),
                "SUMMARY" => summary = value.to_string(),
                "DTSTART" => start_ts = parse_ical_dt(raw_name, value),
                "DTEND" => end_ts = parse_ical_dt(raw_name, value),
                "LOCATION" => location = value.to_string(),
                "DESCRIPTION" => description = value.to_string(),
                "STATUS" => status = value.to_string(),
                "URL" => url = value.to_string(),
                "ATTENDEE" => attendees.push(parse_attendee(raw_name, value)),
                "X-VEL-PREP-MINUTES" => prep_minutes = value.parse::<i64>().ok(),
                "X-VEL-TRAVEL-MINUTES" => travel_minutes = value.parse::<i64>().ok(),
                "X-APPLE-TRAVEL-DURATION" => travel_minutes = parse_travel_duration_minutes(value),
                "X-APPLE-TRAVEL-START" => travel_start_ts = parse_ical_dt(raw_name, value),
                _ => {}
            }
        }
    }
    events
}

fn parse_ical_dt(raw_name: &str, s: &str) -> Option<i64> {
    let tzid = raw_name.split(';').skip(1).find_map(|param| {
        let (key, value) = param.split_once('=')?;
        key.eq_ignore_ascii_case("TZID").then_some(value.trim())
    });
    let has_utc_suffix = s.trim().ends_with('Z');
    let s = s.trim().trim_end_matches('Z');
    if s.len() == 8 {
        let year: i32 = s.get(0..4)?.parse().ok()?;
        let month: u8 = s.get(4..6)?.parse().ok()?;
        let day: u8 = s.get(6..8)?.parse().ok()?;
        let month = time::Month::try_from(month).ok()?;
        let date = time::Date::from_calendar_date(year, month, day).ok()?;
        return Some(date.midnight().assume_utc().unix_timestamp());
    }
    if s.len() < 15 || s.get(8..9) != Some("T") {
        return None;
    }
    let date_part = s.get(0..8)?;
    let time_part = s.get(9..15)?;
    let year: i32 = date_part.get(0..4)?.parse().ok()?;
    let month: u8 = date_part.get(4..6)?.parse().ok()?;
    let day: u8 = date_part.get(6..8)?.parse().ok()?;
    let hour: u8 = time_part.get(0..2)?.parse().ok()?;
    let min: u8 = time_part.get(2..4)?.parse().ok()?;
    let sec: u8 = time_part.get(4..6)?.parse().ok()?;
    let month = time::Month::try_from(month).ok()?;
    let date = time::Date::from_calendar_date(year, month, day).ok()?;
    let t = time::Time::from_hms(hour, min, sec).ok()?;
    let dt = time::PrimitiveDateTime::new(date, t);
    let dt = if has_utc_suffix {
        dt.assume_utc()
    } else if let Some(tzid) = tzid {
        dt.assume_offset(offset_for_tzid(tzid, date)?)
    } else {
        dt.assume_utc()
    };
    Some(dt.unix_timestamp())
}

fn offset_for_tzid(tzid: &str, date: time::Date) -> Option<time::UtcOffset> {
    let hours = match tzid {
        "UTC" | "Etc/UTC" => 0,
        "America/Phoenix" => -7,
        "America/Denver" => {
            if is_us_dst(date) {
                -6
            } else {
                -7
            }
        }
        "America/Chicago" => {
            if is_us_dst(date) {
                -5
            } else {
                -6
            }
        }
        "America/New_York" => {
            if is_us_dst(date) {
                -4
            } else {
                -5
            }
        }
        "America/Los_Angeles" => {
            if is_us_dst(date) {
                -7
            } else {
                -8
            }
        }
        _ => return None,
    };

    time::UtcOffset::from_hms(hours, 0, 0).ok()
}

fn is_us_dst(date: time::Date) -> bool {
    let year = date.year();
    let dst_start = nth_weekday_of_month(year, time::Month::March, time::Weekday::Sunday, 2);
    let dst_end = nth_weekday_of_month(year, time::Month::November, time::Weekday::Sunday, 1);
    date >= dst_start && date < dst_end
}

fn nth_weekday_of_month(
    year: i32,
    month: time::Month,
    weekday: time::Weekday,
    occurrence: u8,
) -> time::Date {
    let first = time::Date::from_calendar_date(year, month, 1).expect("valid month start");
    let days_until = (weekday.number_days_from_monday() as i16
        - first.weekday().number_days_from_monday() as i16)
        .rem_euclid(7) as u8;
    first + time::Duration::days(i64::from(days_until + (occurrence - 1) * 7))
}

fn parse_travel_duration_minutes(value: &str) -> Option<i64> {
    value.trim().parse::<i64>().ok().map(|seconds| seconds / 60)
}

fn parse_attendee(name: &str, value: &str) -> String {
    let params = name.split(';').skip(1);
    for param in params {
        if let Some((key, param_value)) = param.split_once('=') {
            if key.eq_ignore_ascii_case("CN") && !param_value.trim().is_empty() {
                return param_value.trim_matches('"').to_string();
            }
        }
    }

    value
        .trim()
        .strip_prefix("mailto:")
        .unwrap_or(value.trim())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ics_events_supports_parameterized_datetime_fields() {
        let content = "BEGIN:VEVENT\nUID:event-1\nSUMMARY:Meeting\nDTSTART;TZID=America/Denver:20260316T090000\nDTEND;TZID=America/Denver:20260316T100000\nLOCATION:Office\nEND:VEVENT\n";
        let events = parse_ics_events(content);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].payload["event_id"], "event-1");
        assert_eq!(events[0].payload["title"], "Meeting");
        assert_eq!(events[0].payload["location"], "Office");
    }

    #[test]
    fn parse_ics_events_preserves_event_specific_metadata() {
        let content = "BEGIN:VEVENT\nUID:event-2\nSUMMARY:Meeting with Dimitri\nDTSTART:20260316T110000Z\nDTEND:20260316T120000Z\nDESCRIPTION:Prep review\nSTATUS:CONFIRMED\nURL:https://calendar.example/events/2\nATTENDEE;CN=Dimitri:mailto:d@example.com\nATTENDEE:mailto:ops@example.com\nX-VEL-PREP-MINUTES:30\nX-VEL-TRAVEL-MINUTES:40\nEND:VEVENT\n";
        let events = parse_ics_events(content);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].payload["prep_minutes"], 30);
        assert_eq!(events[0].payload["travel_minutes"], 40);
        assert_eq!(events[0].payload["description"], "Prep review");
        assert_eq!(events[0].payload["status"], "CONFIRMED");
        assert_eq!(
            events[0].payload["url"],
            "https://calendar.example/events/2"
        );
        assert_eq!(events[0].payload["attendees"][0], "Dimitri");
        assert_eq!(events[0].payload["attendees"][1], "ops@example.com");
    }

    #[test]
    fn parse_ics_events_uses_apple_travel_duration() {
        let content = "BEGIN:VEVENT\nUID:event-3\nSUMMARY:Client Meeting\nDTSTART:20260316T110000Z\nX-APPLE-TRAVEL-DURATION:2400\nEND:VEVENT\n";
        let events = parse_ics_events(content);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].payload["travel_minutes"], 40);
    }

    #[test]
    fn parse_ics_events_skips_cancelled_events() {
        let content = "BEGIN:VEVENT\nUID:event-4\nSUMMARY:Cancelled Meeting\nDTSTART:20260316T110000Z\nSTATUS:CANCELLED\nEND:VEVENT\n";
        let events = parse_ics_events(content);
        assert!(events.is_empty());
    }
}
