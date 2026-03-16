//! Computer activity adapter: ingest workstation activity snapshots or fall back to vel_invocation.

use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let Some(path) = &config.activity_snapshot_path else {
        return ingest_vel_invocation(storage).await;
    };

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read activity snapshot {}: {}", path, e))
    })?;
    let snapshot: ActivitySnapshot = serde_json::from_str(&content).map_err(|e| {
        crate::errors::AppError::internal(format!("parse activity snapshot: {}", e))
    })?;

    let default_source = snapshot
        .source
        .clone()
        .unwrap_or_else(|| "activity".to_string());
    let mut count = 0u32;
    for event in snapshot.events {
        let signal_type = normalize_signal_type(&event.signal_type).ok_or_else(|| {
            crate::errors::AppError::bad_request(format!(
                "unsupported activity signal_type: {}",
                event.signal_type
            ))
        })?;
        storage
            .insert_signal(SignalInsert {
                signal_type: signal_type.to_string(),
                source: event.source.unwrap_or_else(|| default_source.clone()),
                source_ref: None,
                timestamp: event.timestamp,
                payload_json: Some(serde_json::json!({
                    "host": event.host,
                    "activity": signal_type,
                    "details": event.details.unwrap_or_else(|| serde_json::json!({})),
                })),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        count += 1;
    }

    Ok(count)
}

/// Record a single "vel invoked" signal (e.g. when CLI runs). Returns number of signals ingested (0 or 1).
pub async fn ingest_vel_invocation(storage: &Storage) -> Result<u32, crate::errors::AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    storage
        .insert_signal(SignalInsert {
            signal_type: "vel_invocation".to_string(),
            source: "cli".to_string(),
            source_ref: None,
            timestamp: now,
            payload_json: Some(serde_json::json!({ "type": "vel_invocation", "timestamp": now })),
        })
        .await
        .map_err(crate::errors::AppError::from)?;
    Ok(1)
}

fn normalize_signal_type(signal_type: &str) -> Option<&'static str> {
    match signal_type {
        "shell_login" => Some("shell_login"),
        "shell_exit" => Some("shell_exit"),
        "computer_activity" => Some("computer_activity"),
        "idle_state" => Some("idle_state"),
        "git_activity" => Some("computer_activity"),
        _ => None,
    }
}

#[derive(Debug, serde::Deserialize)]
struct ActivitySnapshot {
    source: Option<String>,
    #[serde(default)]
    events: Vec<ActivityEvent>,
}

#[derive(Debug, serde::Deserialize)]
struct ActivityEvent {
    signal_type: String,
    timestamp: i64,
    source: Option<String>,
    host: Option<String>,
    details: Option<serde_json::Value>,
}
