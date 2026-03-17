//! Git activity adapter: ingest local git activity snapshot JSON and emit replay-safe git_activity signals.

use serde_json::Value as JsonValue;
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

/// Ingest git activity events from a local JSON snapshot.
///
pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let Some(path) = &config.git_snapshot_path else {
        return Ok(0);
    };

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read git snapshot {}: {}", path, e))
    })?;
    let snapshot: GitSnapshot = serde_json::from_str(&content)
        .map_err(|e| crate::errors::AppError::internal(format!("parse git snapshot: {}", e)))?;

    let default_source = snapshot.source.unwrap_or_else(|| "git".to_string());
    let mut count = 0u32;
    for event in snapshot.events {
        let payload = build_payload(&event);
        let source = event
            .source
            .clone()
            .unwrap_or_else(|| default_source.clone());
        let signal_id = storage
            .insert_signal(SignalInsert {
                signal_type: "git_activity".to_string(),
                source,
                source_ref: Some(source_ref(&event)),
                timestamp: event.timestamp,
                payload_json: Some(payload),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        if signal_id.starts_with("sig_") {
            count += 1;
        }
    }

    Ok(count)
}

fn build_payload(event: &GitEvent) -> JsonValue {
    serde_json::json!({
        "dedupe_key": dedupe_key(event),
        "repo": event.repo,
        "repo_name": event.repo_name,
        "branch": event.branch,
        "operation": event.operation,
        "commit_oid": event.commit_oid,
        "head_oid": event.head_oid,
        "author": event.author,
        "message": event.message,
        "files_changed": event.files_changed,
        "insertions": event.insertions,
        "deletions": event.deletions,
        "host": event.host,
        "cwd": event.cwd,
        "details": event.details.clone().unwrap_or_else(|| serde_json::json!({})),
    })
}

fn dedupe_key(event: &GitEvent) -> String {
    let repo = event.repo.as_deref().unwrap_or("-");
    let branch = event.branch.as_deref().unwrap_or("-");
    let operation = event.operation.as_deref().unwrap_or("activity");
    let commit = event
        .commit_oid
        .as_deref()
        .or(event.head_oid.as_deref())
        .unwrap_or("-");
    format!(
        "{}|{}|{}|{}|{}",
        repo, branch, operation, commit, event.timestamp
    )
}

fn source_ref(event: &GitEvent) -> String {
    format!("git:{}", dedupe_key(event))
}

#[derive(Debug, serde::Deserialize)]
struct GitSnapshot {
    source: Option<String>,
    #[serde(default)]
    events: Vec<GitEvent>,
}

#[derive(Debug, serde::Deserialize)]
struct GitEvent {
    timestamp: i64,
    source: Option<String>,
    repo: Option<String>,
    repo_name: Option<String>,
    branch: Option<String>,
    operation: Option<String>,
    commit_oid: Option<String>,
    head_oid: Option<String>,
    author: Option<String>,
    message: Option<String>,
    files_changed: Option<u32>,
    insertions: Option<u32>,
    deletions: Option<u32>,
    host: Option<String>,
    cwd: Option<String>,
    details: Option<JsonValue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedupe_key_prefers_commit_oid() {
        let event = GitEvent {
            timestamp: 123,
            source: Some("git".to_string()),
            repo: Some("/tmp/repo".to_string()),
            repo_name: Some("repo".to_string()),
            branch: Some("main".to_string()),
            operation: Some("commit".to_string()),
            commit_oid: Some("abc123".to_string()),
            head_oid: Some("def456".to_string()),
            author: None,
            message: None,
            files_changed: None,
            insertions: None,
            deletions: None,
            host: None,
            cwd: None,
            details: None,
        };
        assert_eq!(dedupe_key(&event), "/tmp/repo|main|commit|abc123|123");
    }

    #[test]
    fn dedupe_key_falls_back_to_head_oid() {
        let event = GitEvent {
            timestamp: 123,
            source: None,
            repo: Some("/tmp/repo".to_string()),
            repo_name: None,
            branch: Some("main".to_string()),
            operation: Some("checkout".to_string()),
            commit_oid: None,
            head_oid: Some("def456".to_string()),
            author: None,
            message: None,
            files_changed: None,
            insertions: None,
            deletions: None,
            host: None,
            cwd: None,
            details: None,
        };
        assert_eq!(dedupe_key(&event), "/tmp/repo|main|checkout|def456|123");
    }

    #[test]
    fn payload_carries_dedupe_key() {
        let event = GitEvent {
            timestamp: 123,
            source: None,
            repo: Some("/tmp/repo".to_string()),
            repo_name: None,
            branch: Some("main".to_string()),
            operation: Some("commit".to_string()),
            commit_oid: Some("abc123".to_string()),
            head_oid: None,
            author: None,
            message: None,
            files_changed: None,
            insertions: None,
            deletions: None,
            host: None,
            cwd: None,
            details: None,
        };
        let payload = build_payload(&event);
        assert_eq!(
            payload.get("dedupe_key").and_then(JsonValue::as_str),
            Some("/tmp/repo|main|commit|abc123|123")
        );
    }

    #[test]
    fn source_ref_wraps_dedupe_key() {
        let event = GitEvent {
            timestamp: 123,
            source: None,
            repo: Some("/tmp/repo".to_string()),
            repo_name: None,
            branch: Some("main".to_string()),
            operation: Some("commit".to_string()),
            commit_oid: Some("abc123".to_string()),
            head_oid: None,
            author: None,
            message: None,
            files_changed: None,
            insertions: None,
            deletions: None,
            host: None,
            cwd: None,
            details: None,
        };
        assert_eq!(source_ref(&event), "git:/tmp/repo|main|commit|abc123|123");
    }
}
