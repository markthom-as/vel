//! Calendar adapter: read .ics from config path or URL, normalize to calendar_event signals.
//! Minimal line-based ICS parse for VEVENT (UID, DTSTART, DTEND, SUMMARY, LOCATION).

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
        let resp = client.get(url).send().await.map_err(|e| {
            crate::errors::AppError::internal(format!("fetch ics url: {}", e))
        })?;
        resp.text().await.map_err(|e| {
            crate::errors::AppError::internal(format!("ics response body: {}", e))
        })?
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

    for line in content.lines() {
        let line = line.trim();
        if line.eq_ignore_ascii_case("BEGIN:VEVENT") {
            in_vevent = true;
            uid.clear();
            summary.clear();
            start_ts = None;
            end_ts = None;
            location.clear();
            continue;
        }
        if line.eq_ignore_ascii_case("END:VEVENT") {
            in_vevent = false;
            if let Some(ts) = start_ts {
                let payload = serde_json::json!({
                    "event_id": uid,
                    "title": summary,
                    "start": ts,
                    "end": end_ts,
                    "location": location,
                    "prep_minutes": DEFAULT_PREP_MINUTES,
                    "travel_minutes": DEFAULT_TRAVEL_MINUTES
                });
                events.push(ParsedEvent { start_ts: ts, payload });
            }
            continue;
        }
        if !in_vevent {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            let name = name.trim().to_uppercase();
            let value = value.trim();
            match name.as_str() {
                "UID" => uid = value.to_string(),
                "SUMMARY" => summary = value.to_string(),
                "DTSTART" => start_ts = parse_ical_dt(value),
                "DTEND" => end_ts = parse_ical_dt(value),
                "LOCATION" => location = value.to_string(),
                _ => {}
            }
        }
    }
    events
}

fn parse_ical_dt(s: &str) -> Option<i64> {
    let s = s.trim().trim_end_matches('Z');
    if s.len() < 15 {
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
    let dt = time::PrimitiveDateTime::new(date, t).assume_utc();
    Some(dt.unix_timestamp())
}
