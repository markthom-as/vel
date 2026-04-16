use chrono::{DateTime, Datelike, LocalResult, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use time::{Date, OffsetDateTime, Time};
use vel_storage::Storage;

use crate::errors::AppError;

const TIMEZONE_SETTING_KEY: &str = "timezone";
pub const CURRENT_DAY_ROLLOVER_HOUR: u32 = 4;

#[derive(Clone, Debug)]
pub struct ResolvedTimeZone {
    pub name: String,
    tz: Tz,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentDayWindow {
    pub start_ts: i64,
    pub end_ts: i64,
    pub session_date: String,
}

impl ResolvedTimeZone {
    pub fn utc() -> Self {
        Self {
            name: "UTC".to_string(),
            tz: chrono_tz::UTC,
        }
    }

    pub fn parse(name: &str) -> Result<Self, AppError> {
        let trimmed = name.trim();
        let tz = trimmed
            .parse::<Tz>()
            .map_err(|_| AppError::bad_request("timezone must be a valid IANA timezone"))?;
        Ok(Self {
            name: trimmed.to_string(),
            tz,
        })
    }
}

pub async fn resolve_timezone(storage: &Storage) -> Result<ResolvedTimeZone, AppError> {
    let settings = storage.get_all_settings().await?;
    if let Some(name) = settings
        .get(TIMEZONE_SETTING_KEY)
        .and_then(|value| value.as_str())
    {
        return ResolvedTimeZone::parse(name);
    }

    if let Ok(name) = iana_time_zone::get_timezone() {
        if let Ok(resolved) = ResolvedTimeZone::parse(&name) {
            return Ok(resolved);
        }
    }

    Ok(ResolvedTimeZone::utc())
}

pub fn start_of_local_day_timestamp(
    timezone: &ResolvedTimeZone,
    now_utc: OffsetDateTime,
) -> Result<i64, AppError> {
    let local = utc_datetime(now_utc).with_timezone(&timezone.tz);
    let local_midnight =
        resolve_local_datetime(timezone, local.year(), local.month(), local.day(), 0, 0, 0)?;
    Ok(local_midnight.with_timezone(&Utc).timestamp())
}

pub fn current_day_window(
    timezone: &ResolvedTimeZone,
    now_utc: OffsetDateTime,
) -> Result<CurrentDayWindow, AppError> {
    let local = utc_datetime(now_utc).with_timezone(&timezone.tz);
    let mut session_date = local.date_naive();
    if local.hour() < CURRENT_DAY_ROLLOVER_HOUR {
        session_date = session_date
            .pred_opt()
            .ok_or_else(|| AppError::internal("unable to compute previous local date"))?;
    }
    let next_date = session_date
        .succ_opt()
        .ok_or_else(|| AppError::internal("unable to compute next local date"))?;
    let start = resolve_local_datetime(
        timezone,
        session_date.year(),
        session_date.month(),
        session_date.day(),
        CURRENT_DAY_ROLLOVER_HOUR,
        0,
        0,
    )?;
    let end = resolve_local_datetime(
        timezone,
        next_date.year(),
        next_date.month(),
        next_date.day(),
        CURRENT_DAY_ROLLOVER_HOUR,
        0,
        0,
    )?;
    Ok(CurrentDayWindow {
        start_ts: start.with_timezone(&Utc).timestamp(),
        end_ts: end.with_timezone(&Utc).timestamp(),
        session_date: format!(
            "{:04}-{:02}-{:02}",
            session_date.year(),
            session_date.month(),
            session_date.day()
        ),
    })
}

pub fn current_day_date_string(
    timezone: &ResolvedTimeZone,
    value: OffsetDateTime,
) -> Result<String, AppError> {
    Ok(current_day_window(timezone, value)?.session_date)
}

pub fn same_local_day(
    timezone: &ResolvedTimeZone,
    left: OffsetDateTime,
    right: OffsetDateTime,
) -> bool {
    let left_local = utc_datetime(left).with_timezone(&timezone.tz);
    let right_local = utc_datetime(right).with_timezone(&timezone.tz);
    left_local.date_naive() == right_local.date_naive()
}

pub fn local_date_string(timezone: &ResolvedTimeZone, value: OffsetDateTime) -> String {
    let local = utc_datetime(value).with_timezone(&timezone.tz);
    format!(
        "{:04}-{:02}-{:02}",
        local.year(),
        local.month(),
        local.day()
    )
}

pub fn local_hour_minute(timezone: &ResolvedTimeZone, value: OffsetDateTime) -> (u32, u32) {
    let local = utc_datetime(value).with_timezone(&timezone.tz);
    (local.hour(), local.minute())
}

pub fn local_time_label(timezone: &ResolvedTimeZone, unix_ts: i64) -> String {
    let local = DateTime::<Utc>::from_timestamp(unix_ts, 0)
        .expect("unix timestamp should convert to chrono datetime")
        .with_timezone(&timezone.tz);
    if local.minute() == 0 {
        local.format("%-I %p").to_string()
    } else {
        local.format("%-I:%M %p").to_string()
    }
}

pub fn local_calendar_label(
    timezone: &ResolvedTimeZone,
    value: OffsetDateTime,
    prefix: &str,
) -> String {
    let local = utc_datetime(value).with_timezone(&timezone.tz);
    format!("{prefix} {}", local.format("%b %-d"))
}

pub(crate) fn local_datetime_timestamp(
    timezone: &ResolvedTimeZone,
    date: Date,
    time: Time,
) -> Result<i64, AppError> {
    let local = resolve_local_datetime(
        timezone,
        date.year(),
        u8::from(date.month()).into(),
        date.day().into(),
        time.hour().into(),
        time.minute().into(),
        time.second().into(),
    )?;
    Ok(local.with_timezone(&Utc).timestamp())
}

fn utc_datetime(value: OffsetDateTime) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(value.unix_timestamp(), value.nanosecond())
        .expect("offset datetime should convert to chrono datetime")
}

fn resolve_local_datetime(
    timezone: &ResolvedTimeZone,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Result<DateTime<Tz>, AppError> {
    match timezone
        .tz
        .with_ymd_and_hms(year, month, day, hour, minute, second)
    {
        LocalResult::Single(value) => Ok(value),
        LocalResult::Ambiguous(earliest, _) => Ok(earliest),
        LocalResult::None => Err(AppError::internal(format!(
            "unable to resolve local datetime for timezone {}",
            timezone.name
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn parses_valid_iana_timezone() {
        let timezone = ResolvedTimeZone::parse("America/Denver").expect("timezone should parse");
        assert_eq!(timezone.name, "America/Denver");
    }

    #[test]
    fn rejects_invalid_timezone() {
        assert!(ResolvedTimeZone::parse("Mars/Olympus").is_err());
    }

    #[test]
    fn computes_start_of_local_day_for_timezone() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let now_utc = datetime!(2026-03-16 06:30:00 UTC);

        let start = start_of_local_day_timestamp(&timezone, now_utc).unwrap();

        assert_eq!(start, datetime!(2026-03-16 06:00:00 UTC).unix_timestamp());
    }

    #[test]
    fn compares_days_using_local_timezone() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let left = datetime!(2026-03-16 00:30:00 UTC);
        let right = datetime!(2026-03-16 06:30:00 UTC);

        assert!(!same_local_day(&timezone, left, right));
    }

    #[test]
    fn formats_local_date_string_using_timezone() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let value = datetime!(2026-03-16 05:30:00 UTC);

        let date = local_date_string(&timezone, value);

        assert_eq!(date, "2026-03-15");
    }

    #[test]
    fn formats_local_time_label_using_timezone() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let on_hour = datetime!(2026-03-16 18:00:00 UTC).unix_timestamp();
        let with_minutes = datetime!(2026-03-16 18:30:00 UTC).unix_timestamp();

        assert_eq!(local_time_label(&timezone, on_hour), "12 PM");
        assert_eq!(local_time_label(&timezone, with_minutes), "12:30 PM");
    }

    #[test]
    fn formats_local_calendar_label_using_timezone() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let value = datetime!(2026-03-16 05:30:00 UTC);

        let label = local_calendar_label(&timezone, value, "Due");

        assert_eq!(label, "Due Mar 15");
    }

    #[test]
    fn current_day_window_stays_on_previous_day_before_rollover_hour() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let now_utc = datetime!(2026-03-16 08:30:00 UTC);

        let window = current_day_window(&timezone, now_utc).unwrap();

        assert_eq!(window.session_date, "2026-03-15");
        assert_eq!(
            window.start_ts,
            datetime!(2026-03-15 10:00:00 UTC).unix_timestamp()
        );
        assert_eq!(
            window.end_ts,
            datetime!(2026-03-16 10:00:00 UTC).unix_timestamp()
        );
    }

    #[test]
    fn current_day_window_advances_after_rollover_hour() {
        let timezone = ResolvedTimeZone::parse("America/Denver").unwrap();
        let now_utc = datetime!(2026-03-16 11:30:00 UTC);

        let window = current_day_window(&timezone, now_utc).unwrap();

        assert_eq!(window.session_date, "2026-03-16");
        assert_eq!(
            window.start_ts,
            datetime!(2026-03-16 10:00:00 UTC).unix_timestamp()
        );
        assert_eq!(
            window.end_ts,
            datetime!(2026-03-17 10:00:00 UTC).unix_timestamp()
        );
    }
}
