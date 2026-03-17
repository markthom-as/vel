use serde_json::Value as JsonValue;
use time::OffsetDateTime;

use crate::db::StorageError;

pub(crate) fn parse_json_value(s: &str) -> Result<JsonValue, StorageError> {
    serde_json::from_str(s).map_err(|error| StorageError::Validation(error.to_string()))
}

pub(crate) fn timestamp_to_datetime(timestamp: i64) -> Result<OffsetDateTime, StorageError> {
    OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|error| StorageError::InvalidTimestamp(error.to_string()))
}
