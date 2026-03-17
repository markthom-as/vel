//! Messaging adapter: ingest local messaging thread snapshots and emit replay-safe message_thread signals.

use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let path = match &config.messaging_snapshot_path {
        Some(path) => path,
        None => return Ok(0),
    };

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read messaging snapshot {}: {}", path, e))
    })?;
    let snapshot: MessagingSnapshot = serde_json::from_str(&content).map_err(|e| {
        crate::errors::AppError::internal(format!("parse messaging snapshot: {}", e))
    })?;

    let mut signals_count = 0u32;
    let source = snapshot
        .source
        .clone()
        .unwrap_or_else(|| "messaging".to_string());
    let account_id = snapshot
        .account_id
        .clone()
        .unwrap_or_else(|| "default".to_string());

    for thread in snapshot.threads.into_iter() {
        let thread_id = thread.thread_id.trim();
        let platform = thread.platform.trim();
        if thread_id.is_empty() || platform.is_empty() {
            continue;
        }

        let timestamp = thread.latest_timestamp;
        let participant_ids = participant_ids(&thread);
        let participants = thread.participants;
        let payload = serde_json::json!({
            "thread_id": thread_id,
            "platform": platform,
            "account_id": account_id,
            "title": thread.title,
            "participants": participants,
            "participant_ids": participant_ids,
            "latest_timestamp": timestamp,
            "waiting_state": thread.waiting_state,
            "scheduling_related": thread.scheduling_related,
            "urgent": thread.urgent,
            "summary": thread.summary,
            "snippet": thread.snippet,
        });

        let signal_id = storage
            .insert_signal(SignalInsert {
                signal_type: "message_thread".to_string(),
                source: source.clone(),
                source_ref: Some(thread_source_ref(
                    platform,
                    &account_id,
                    thread_id,
                    timestamp,
                )),
                timestamp,
                payload_json: Some(payload),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        if signal_id.starts_with("sig_") {
            signals_count += 1;
        }
    }

    Ok(signals_count)
}

fn participant_ids(thread: &MessagingThread) -> Vec<String> {
    thread
        .participants
        .iter()
        .map(|participant| participant.id.clone())
        .collect()
}

fn thread_source_ref(
    platform: &str,
    account_id: &str,
    thread_id: &str,
    latest_timestamp: i64,
) -> String {
    format!(
        "messaging:{}:{}:{}:{}",
        platform, account_id, thread_id, latest_timestamp
    )
}

#[derive(Debug, serde::Deserialize)]
struct MessagingSnapshot {
    source: Option<String>,
    account_id: Option<String>,
    #[serde(default)]
    threads: Vec<MessagingThread>,
}

#[derive(Debug, serde::Deserialize)]
struct MessagingThread {
    thread_id: String,
    platform: String,
    title: Option<String>,
    #[serde(default)]
    participants: Vec<MessagingParticipant>,
    latest_timestamp: i64,
    waiting_state: String,
    #[serde(default)]
    scheduling_related: bool,
    #[serde(default)]
    urgent: bool,
    summary: Option<String>,
    snippet: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct MessagingParticipant {
    id: String,
    name: Option<String>,
    #[serde(default)]
    is_me: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_ref_tracks_thread_and_latest_timestamp() {
        let a = thread_source_ref("gmail", "acct", "thr_1", 100);
        let b = thread_source_ref("gmail", "acct", "thr_1", 100);
        let c = thread_source_ref("gmail", "acct", "thr_1", 101);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
