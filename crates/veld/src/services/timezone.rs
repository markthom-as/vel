use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};
use time_tz::{timezones, OffsetError, PrimitiveDateTimeExt, TimeZone, ToTimezone, Tz};
use vel_storage::Storage;

use crate::errors::AppError;

const TIMEZONE_SETTING_KEY: &str = "timezone";
pub const CURRENT_DAY_ROLLOVER_HOUR: u32 = 4;

#[derive(Clone, Debug)]
pub struct ResolvedTimeZone {
    pub name: String,
    tz: &'static Tz,
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
            tz: timezones::db::UTC,
        }
    }

    pub fn parse(name: &str) -> Result<Self, AppError> {
        let trimmed = name.trim();
        let tz = timezones::get_by_name(trimmed)
            .filter(|timezone| timezone.name() == trimmed)
            .ok_or_else(|| AppError::bad_request("timezone must be a valid IANA timezone"))?;
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
    let local = local_offset_datetime(timezone, now_utc);
    let local_midnight = resolve_local_datetime(
        timezone,
        local.date(),
        Time::MIDNIGHT,
        LocalResolutionErrorKind::Internal,
    )?;
    Ok(local_midnight.unix_timestamp())
}

pub fn current_day_window(
    timezone: &ResolvedTimeZone,
    now_utc: OffsetDateTime,
) -> Result<CurrentDayWindow, AppError> {
    let local = local_offset_datetime(timezone, now_utc);
    let mut session_date = local.date();
    if local.time().hour() < CURRENT_DAY_ROLLOVER_HOUR as u8 {
        session_date = session_date
            .previous_day()
            .ok_or_else(|| AppError::internal("unable to compute previous local date"))?;
    }
    let next_date = session_date
        .next_day()
        .ok_or_else(|| AppError::internal("unable to compute next local date"))?;
    let rollover_time = Time::from_hms(CURRENT_DAY_ROLLOVER_HOUR as u8, 0, 0)
        .expect("current day rollover hour should be valid");
    let start = resolve_local_datetime(
        timezone,
        session_date,
        rollover_time,
        LocalResolutionErrorKind::Internal,
    )?;
    let end = resolve_local_datetime(
        timezone,
        next_date,
        rollover_time,
        LocalResolutionErrorKind::Internal,
    )?;
    Ok(CurrentDayWindow {
        start_ts: start.unix_timestamp(),
        end_ts: end.unix_timestamp(),
        session_date: format_date(session_date),
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
    let left_local = local_offset_datetime(timezone, left);
    let right_local = local_offset_datetime(timezone, right);
    left_local.date() == right_local.date()
}

pub fn local_date_string(timezone: &ResolvedTimeZone, value: OffsetDateTime) -> String {
    let local = local_offset_datetime(timezone, value);
    format_date(local.date())
}

pub fn local_hour_minute(timezone: &ResolvedTimeZone, value: OffsetDateTime) -> (u8, u8) {
    let local = local_offset_datetime(timezone, value);
    (local.time().hour(), local.time().minute())
}

pub fn local_time_label(timezone: &ResolvedTimeZone, unix_ts: i64) -> String {
    let local = OffsetDateTime::from_unix_timestamp(unix_ts)
        .expect("unix timestamp should convert to offset datetime")
        .to_timezone(timezone.tz);
    let hour = local.time().hour();
    let minute = local.time().minute();
    let suffix = if hour < 12 { "AM" } else { "PM" };
    let hour_12 = match hour % 12 {
        0 => 12,
        value => value,
    };
    if minute == 0 {
        format!("{hour_12} {suffix}")
    } else {
        format!("{hour_12}:{minute:02} {suffix}")
    }
}

pub fn local_calendar_label(
    timezone: &ResolvedTimeZone,
    value: OffsetDateTime,
    prefix: &str,
) -> String {
    let local = local_offset_datetime(timezone, value);
    let date = local.date();
    format!(
        "{prefix} {} {}",
        month_abbreviation(date.month()),
        date.day()
    )
}

pub(crate) fn local_datetime_timestamp(
    timezone: &ResolvedTimeZone,
    date: Date,
    time: Time,
) -> Result<i64, AppError> {
    let local = resolve_local_datetime(timezone, date, time, LocalResolutionErrorKind::BadRequest)?;
    Ok(local.unix_timestamp())
}

fn resolve_local_datetime(
    timezone: &ResolvedTimeZone,
    date: Date,
    time: Time,
    error_kind: LocalResolutionErrorKind,
) -> Result<OffsetDateTime, AppError> {
    let local = PrimitiveDateTime::new(date, time);
    match local.assume_timezone(timezone.tz) {
        Ok(value) => Ok(value),
        Err(OffsetError::Ambiguous(left, right)) => Ok(if left <= right { left } else { right }),
        Err(OffsetError::Undefined) => Err(local_resolution_error(timezone, error_kind)),
    }
}

fn local_offset_datetime(timezone: &ResolvedTimeZone, value: OffsetDateTime) -> OffsetDateTime {
    value.to_timezone(timezone.tz)
}

fn local_resolution_error(
    timezone: &ResolvedTimeZone,
    error_kind: LocalResolutionErrorKind,
) -> AppError {
    match error_kind {
        LocalResolutionErrorKind::Internal => AppError::internal(format!(
            "unable to resolve local datetime for timezone {}",
            timezone.name
        )),
        LocalResolutionErrorKind::BadRequest => AppError::bad_request(format!(
            "unable to resolve routine block local time for timezone {}",
            timezone.name
        )),
    }
}

#[derive(Clone, Copy)]
enum LocalResolutionErrorKind {
    Internal,
    BadRequest,
}

fn format_date(date: Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        u8::from(date.month()),
        date.day()
    )
}

fn month_abbreviation(month: Month) -> &'static str {
    match month {
        Month::January => "Jan",
        Month::February => "Feb",
        Month::March => "Mar",
        Month::April => "Apr",
        Month::May => "May",
        Month::June => "Jun",
        Month::July => "Jul",
        Month::August => "Aug",
        Month::September => "Sep",
        Month::October => "Oct",
        Month::November => "Nov",
        Month::December => "Dec",
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
    fn rejects_non_iana_timezone_alias() {
        assert!(ResolvedTimeZone::parse("Mountain Standard Time").is_err());
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
