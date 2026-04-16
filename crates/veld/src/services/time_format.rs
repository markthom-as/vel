use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};

use crate::errors::AppError;

pub(crate) fn format_utc_rfc3339(value: OffsetDateTime) -> Result<String, AppError> {
    let formatted = value
        .to_offset(UtcOffset::UTC)
        .format(&Rfc3339)
        .map_err(|error| AppError::internal(error.to_string()))?;
    Ok(formatted
        .strip_suffix('Z')
        .map(|prefix| format!("{prefix}+00:00"))
        .unwrap_or(formatted))
}
