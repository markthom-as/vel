//! Computer activity adapter: ingest workstation activity snapshots or fall back to local sources.

use std::{collections::HashMap, env, path::PathBuf, time::Duration as StdDuration};

use reqwest::Url;
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    if let Some(path) = &config.activity_snapshot_path {
        match tokio::fs::try_exists(path).await {
            Ok(true) => return ingest_snapshot_path(storage, path).await,
            Ok(false) if vel_config::is_default_local_source_path("activity", path) => {}
            Ok(false) => {
                return Err(crate::errors::AppError::internal(format!(
                    "read activity snapshot {}: No such file or directory",
                    path
                )));
            }
            Err(error) => {
                return Err(crate::errors::AppError::internal(format!(
                    "stat activity snapshot {}: {}",
                    path, error
                )));
            }
        }
    }

    let local_count = ingest_local_sources(storage).await?;
    if local_count > 0 {
        return Ok(local_count);
    }

    ingest_vel_invocation(storage).await
}

async fn ingest_snapshot_path(
    storage: &Storage,
    path: &str,
) -> Result<u32, crate::errors::AppError> {
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
                source: event
                    .source
                    .clone()
                    .unwrap_or_else(|| default_source.clone()),
                source_ref: Some(activity_source_ref(&event, signal_type)),
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

async fn ingest_local_sources(storage: &Storage) -> Result<u32, crate::errors::AppError> {
    let mut count = ingest_zsh_extended_history(storage).await?;
    count += ingest_activitywatch(storage, &default_activitywatch_base_url()).await?;
    Ok(count)
}

async fn ingest_zsh_extended_history(storage: &Storage) -> Result<u32, crate::errors::AppError> {
    let host = local_host_name();
    ingest_zsh_extended_history_from_paths(storage, &host, &candidate_zsh_history_paths()).await
}

async fn ingest_zsh_extended_history_from_paths(
    storage: &Storage,
    host: &str,
    paths: &[PathBuf],
) -> Result<u32, crate::errors::AppError> {
    let mut count = 0u32;

    for path in paths {
        let exists = tokio::fs::try_exists(&path).await.map_err(|error| {
            crate::errors::AppError::internal(format!(
                "stat zsh history {}: {}",
                path.display(),
                error
            ))
        })?;
        if !exists {
            continue;
        }

        let content = match tokio::fs::read(&path).await {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(error) => {
                tracing::debug!("skip unreadable zsh history {}: {}", path.display(), error);
                continue;
            }
        };

        for entry in parse_zsh_extended_history(&content) {
            storage
                .insert_signal(SignalInsert {
                    signal_type: "computer_activity".to_string(),
                    source: "zsh".to_string(),
                    source_ref: Some(zsh_history_source_ref(host, &entry)),
                    timestamp: entry.timestamp,
                    payload_json: Some(serde_json::json!({
                        "host": host,
                        "activity": "computer_activity",
                        "details": {
                            "origin": "zsh_extended_history",
                            "command": entry.command,
                            "duration_seconds": entry.duration_seconds,
                            "history_path": path.display().to_string(),
                        },
                    })),
                })
                .await
                .map_err(crate::errors::AppError::from)?;
            count += 1;
        }
    }

    Ok(count)
}

async fn ingest_activitywatch(
    storage: &Storage,
    base_url: &str,
) -> Result<u32, crate::errors::AppError> {
    match try_ingest_activitywatch(storage, base_url).await {
        Ok(count) => Ok(count),
        Err(error) => {
            tracing::debug!("activitywatch ingest skipped: {}", error);
            Ok(0)
        }
    }
}

async fn try_ingest_activitywatch(
    storage: &Storage,
    base_url: &str,
) -> Result<u32, crate::errors::AppError> {
    let base = activitywatch_base_url(base_url)?;
    let client = reqwest::Client::builder()
        .timeout(StdDuration::from_secs(2))
        .build()
        .map_err(|error| {
            crate::errors::AppError::internal(format!("activitywatch client: {}", error))
        })?;

    let buckets: HashMap<String, serde_json::Value> = client
        .get(base.join("api/0/buckets").map_err(|error| {
            crate::errors::AppError::internal(format!("activitywatch buckets url: {}", error))
        })?)
        .send()
        .await
        .map_err(|error| {
            crate::errors::AppError::internal(format!("activitywatch buckets request: {}", error))
        })?
        .error_for_status()
        .map_err(|error| {
            crate::errors::AppError::internal(format!("activitywatch buckets status: {}", error))
        })?
        .json()
        .await
        .map_err(|error| {
            crate::errors::AppError::internal(format!("activitywatch buckets parse: {}", error))
        })?;

    let now = OffsetDateTime::now_utc();
    let start = (now - Duration::hours(36))
        .format(&Rfc3339)
        .map_err(|error| {
            crate::errors::AppError::internal(format!("activitywatch start format: {}", error))
        })?;
    let end = now.format(&Rfc3339).map_err(|error| {
        crate::errors::AppError::internal(format!("activitywatch end format: {}", error))
    })?;

    let mut count = 0u32;
    for bucket_id in buckets.keys() {
        let signal_type = match activitywatch_bucket_signal_type(bucket_id) {
            Some(signal_type) => signal_type,
            None => continue,
        };
        let mut url = base
            .join(&format!("api/0/buckets/{bucket_id}/events"))
            .map_err(|error| {
                crate::errors::AppError::internal(format!(
                    "activitywatch events url for {}: {}",
                    bucket_id, error
                ))
            })?;
        url.query_pairs_mut()
            .append_pair("start", &start)
            .append_pair("end", &end);

        let events: Vec<ActivityWatchEvent> = client
            .get(url)
            .send()
            .await
            .map_err(|error| {
                crate::errors::AppError::internal(format!(
                    "activitywatch events request for {}: {}",
                    bucket_id, error
                ))
            })?
            .error_for_status()
            .map_err(|error| {
                crate::errors::AppError::internal(format!(
                    "activitywatch events status for {}: {}",
                    bucket_id, error
                ))
            })?
            .json()
            .await
            .map_err(|error| {
                crate::errors::AppError::internal(format!(
                    "activitywatch events parse for {}: {}",
                    bucket_id, error
                ))
            })?;

        count += ingest_activitywatch_events(storage, bucket_id, signal_type, &events).await?;
    }

    Ok(count)
}

async fn ingest_activitywatch_events(
    storage: &Storage,
    bucket_id: &str,
    signal_type: &str,
    events: &[ActivityWatchEvent],
) -> Result<u32, crate::errors::AppError> {
    let mut count = 0u32;
    for event in events {
        let Some(timestamp) = parse_activitywatch_timestamp(&event.timestamp) else {
            continue;
        };

        storage
            .insert_signal(SignalInsert {
                signal_type: signal_type.to_string(),
                source: "activitywatch".to_string(),
                source_ref: Some(activitywatch_source_ref(bucket_id, signal_type, event)),
                timestamp,
                payload_json: Some(serde_json::json!({
                    "host": local_host_name(),
                    "activity": signal_type,
                    "details": activitywatch_details(signal_type, bucket_id, event),
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

fn default_activitywatch_base_url() -> String {
    env::var("VEL_ACTIVITYWATCH_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:5600/".to_string())
}

fn activitywatch_base_url(base_url: &str) -> Result<Url, crate::errors::AppError> {
    let normalized = if base_url.ends_with('/') {
        base_url.to_string()
    } else {
        format!("{base_url}/")
    };
    Url::parse(&normalized).map_err(|error| {
        crate::errors::AppError::internal(format!(
            "invalid activitywatch base url {}: {}",
            base_url, error
        ))
    })
}

fn candidate_zsh_history_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);
        paths.push(home.join(".histfile"));
        paths.push(home.join(".zsh_history"));
        paths.push(home.join(".local/share/zsh/history"));
        paths.push(home.join(".local/share/zsh/zsh_history"));
    }
    paths
}

fn local_host_name() -> String {
    env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}

fn parse_zsh_extended_history(content: &str) -> Vec<ZshHistoryEntry> {
    content
        .lines()
        .filter_map(parse_zsh_extended_history_line)
        .collect()
}

fn parse_zsh_extended_history_line(line: &str) -> Option<ZshHistoryEntry> {
    let metadata = line.strip_prefix(": ")?;
    let (timing, command) = metadata.split_once(';')?;
    let (timestamp, duration_seconds) = timing.split_once(':')?;
    Some(ZshHistoryEntry {
        timestamp: timestamp.trim().parse().ok()?,
        duration_seconds: duration_seconds.trim().parse().ok()?,
        command: command.to_string(),
    })
}

fn zsh_history_source_ref(host: &str, entry: &ZshHistoryEntry) -> String {
    let digest = short_hash(&format!(
        "{}:{}:{}",
        entry.timestamp, entry.duration_seconds, entry.command
    ));
    format!("activity:zsh:{host}:{}:{digest}", entry.timestamp)
}

fn activity_source_ref(event: &ActivityEvent, signal_type: &str) -> String {
    let host = event.host.as_deref().unwrap_or("unknown");
    format!("activity:{signal_type}:{host}:{}", event.timestamp)
}

fn activitywatch_bucket_signal_type(bucket_id: &str) -> Option<&'static str> {
    if bucket_id.starts_with("aw-watcher-window_") {
        Some("computer_activity")
    } else if bucket_id.starts_with("aw-watcher-afk_") {
        Some("idle_state")
    } else {
        None
    }
}

fn parse_activitywatch_timestamp(timestamp: &str) -> Option<i64> {
    OffsetDateTime::parse(timestamp, &Rfc3339)
        .ok()
        .map(|value| value.unix_timestamp())
}

fn activitywatch_source_ref(
    bucket_id: &str,
    signal_type: &str,
    event: &ActivityWatchEvent,
) -> String {
    let digest = short_hash(&serde_json::to_string(&event.data).unwrap_or_default());
    format!(
        "activity:activitywatch:{bucket_id}:{signal_type}:{}:{digest}",
        event.timestamp
    )
}

fn activitywatch_details(
    signal_type: &str,
    bucket_id: &str,
    event: &ActivityWatchEvent,
) -> serde_json::Value {
    let mut details = serde_json::Map::new();
    details.insert(
        "origin".to_string(),
        serde_json::Value::String("activitywatch".to_string()),
    );
    details.insert(
        "bucket_id".to_string(),
        serde_json::Value::String(bucket_id.to_string()),
    );
    if let Some(duration_seconds) = event.duration {
        if let Some(value) = serde_json::Number::from_f64(duration_seconds) {
            details.insert(
                "duration_seconds".to_string(),
                serde_json::Value::Number(value),
            );
        }
    }
    if signal_type == "computer_activity" {
        if let Some(app) = event.data.get("app").and_then(serde_json::Value::as_str) {
            details.insert(
                "app".to_string(),
                serde_json::Value::String(app.to_string()),
            );
        }
        if let Some(title) = event.data.get("title").and_then(serde_json::Value::as_str) {
            details.insert(
                "title".to_string(),
                serde_json::Value::String(title.to_string()),
            );
        }
    }
    if signal_type == "idle_state" {
        if let Some(status) = event.data.get("status").and_then(serde_json::Value::as_str) {
            details.insert(
                "status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
        }
    }
    serde_json::Value::Object(details)
}

fn short_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let digest = hasher.finalize();
    hex::encode(&digest[..6])
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct ZshHistoryEntry {
    timestamp: i64,
    duration_seconds: i64,
    command: String,
}

#[derive(Debug, serde::Deserialize)]
struct ActivityWatchEvent {
    timestamp: String,
    duration: Option<f64>,
    #[serde(default)]
    data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_ref_is_stable_for_same_event() {
        let event = ActivityEvent {
            signal_type: "computer_activity".to_string(),
            timestamp: 123,
            source: None,
            host: Some("mbp".to_string()),
            details: None,
        };

        assert_eq!(
            activity_source_ref(&event, "computer_activity"),
            "activity:computer_activity:mbp:123"
        );
    }

    #[test]
    fn parses_zsh_extended_history_lines() {
        let entry = parse_zsh_extended_history_line(": 1710000000:12;cargo test -p veld").unwrap();
        assert_eq!(
            entry,
            ZshHistoryEntry {
                timestamp: 1710000000,
                duration_seconds: 12,
                command: "cargo test -p veld".to_string(),
            }
        );
    }

    #[tokio::test]
    async fn ingest_uses_zsh_extended_history_when_snapshot_missing() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir =
            std::env::temp_dir().join(format!("vel_zsh_history_{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).unwrap();
        let history_path = dir.join(".histfile");
        std::fs::write(
            &history_path,
            ": 1710000000:4;cargo fmt\n: 1710000100:9;cargo test -p veld\n",
        )
        .unwrap();

        let count = ingest_zsh_extended_history_from_paths(
            &storage,
            "testhost",
            std::slice::from_ref(&history_path),
        )
        .await
        .unwrap();

        assert_eq!(count, 2);
        let signals = storage
            .list_signals(Some("computer_activity"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 2);
        assert_eq!(signals[0].source, "zsh");
        assert_eq!(
            signals[0].payload_json["details"]["origin"],
            "zsh_extended_history"
        );

        let _ = std::fs::remove_file(history_path);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[tokio::test]
    async fn ingest_uses_activitywatch_when_available() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let window_events = vec![ActivityWatchEvent {
            timestamp: "2026-03-17T12:00:00Z".to_string(),
            duration: Some(30.0),
            data: serde_json::json!({ "app": "Zed", "title": "activity.rs" }),
        }];
        let afk_events = vec![ActivityWatchEvent {
            timestamp: "2026-03-17T12:05:00Z".to_string(),
            duration: Some(120.0),
            data: serde_json::json!({ "status": "afk" }),
        }];

        let count = ingest_activitywatch_events(
            &storage,
            "aw-watcher-window_testhost",
            "computer_activity",
            &window_events,
        )
        .await
        .unwrap()
            + ingest_activitywatch_events(
                &storage,
                "aw-watcher-afk_testhost",
                "idle_state",
                &afk_events,
            )
            .await
            .unwrap();
        assert_eq!(count, 2);

        let activity_signals = storage
            .list_signals(Some("computer_activity"), None, 10)
            .await
            .unwrap();
        let idle_signals = storage
            .list_signals(Some("idle_state"), None, 10)
            .await
            .unwrap();
        assert_eq!(activity_signals.len(), 1);
        assert_eq!(idle_signals.len(), 1);
        assert_eq!(activity_signals[0].source, "activitywatch");
        assert_eq!(idle_signals[0].payload_json["details"]["status"], "afk");
    }
}
