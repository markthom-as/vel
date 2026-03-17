use chrono::{DateTime, Datelike, LocalResult, TimeZone, Utc};
use chrono_tz::Tz;
use time::OffsetDateTime;
use vel_storage::Storage;

use crate::errors::AppError;

const TIMEZONE_SETTING_KEY: &str = "timezone";

#[derive(Clone, Debug)]
pub struct ResolvedTimeZone {
    pub name: String,
    tz: Tz,
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
        match timezone
            .tz
            .with_ymd_and_hms(local.year(), local.month(), local.day(), 0, 0, 0)
        {
            LocalResult::Single(value) => value,
            LocalResult::Ambiguous(earliest, _) => earliest,
            LocalResult::None => {
                return Err(AppError::internal(format!(
                    "unable to resolve local midnight for timezone {}",
                    timezone.name
                )));
            }
        };
    Ok(local_midnight.with_timezone(&Utc).timestamp())
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

fn utc_datetime(value: OffsetDateTime) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(value.unix_timestamp(), value.nanosecond())
        .expect("offset datetime should convert to chrono datetime")
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
}
