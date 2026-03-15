//! Computer activity adapter: record vel_invocation (and later machine_login, shell_start).

use time::OffsetDateTime;
use vel_storage::{SignalInsert, Storage};

/// Record a single "vel invoked" signal (e.g. when CLI runs). Returns number of signals ingested (0 or 1).
pub async fn ingest_vel_invocation(storage: &Storage) -> Result<u32, crate::errors::AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    storage
        .insert_signal(SignalInsert {
            signal_type: "vel_invocation".to_string(),
            source: "cli".to_string(),
            timestamp: now,
            payload_json: Some(serde_json::json!({ "type": "vel_invocation", "timestamp": now })),
        })
        .await
        .map_err(crate::errors::AppError::from)?;
    Ok(1)
}
