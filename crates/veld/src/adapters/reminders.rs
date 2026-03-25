//! Reminders adapter: ingest local reminders snapshots and emit replay-safe reminder_item signals.
#![allow(dead_code)] // Reminder writeback remains staged while the operator path stays read-first.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ReminderWriteMutation {
    pub title: Option<String>,
    pub list_id: Option<String>,
    pub list_title: Option<String>,
    pub notes: Option<String>,
    pub due_at: Option<i64>,
    pub priority: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReminderWriteKind {
    Create,
    Update,
    Complete,
}

#[derive(Debug, Clone)]
pub(crate) struct ReminderWriteResult {
    pub snapshot_path: String,
    pub reminder: ReminderItem,
}

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let path = match &config.reminders_snapshot_path {
        Some(path) => path,
        None => return Ok(0),
    };
    match tokio::fs::try_exists(path).await {
        Ok(true) => {}
        Ok(false) if vel_config::is_default_local_source_path("reminders", path) => return Ok(0),
        Ok(false) => {
            return Err(crate::errors::AppError::internal(format!(
                "read reminders snapshot {}: No such file or directory",
                path
            )));
        }
        Err(error) => {
            return Err(crate::errors::AppError::internal(format!(
                "stat reminders snapshot {}: {}",
                path, error
            )));
        }
    }

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read reminders snapshot {}: {}", path, e))
    })?;
    let snapshot: RemindersSnapshot = serde_json::from_str(&content).map_err(|e| {
        crate::errors::AppError::internal(format!("parse reminders snapshot: {}", e))
    })?;

    let mut signals_count = 0u32;
    let default_source = snapshot
        .source
        .clone()
        .unwrap_or_else(|| "reminders".to_string());
    let account_id = snapshot
        .account_id
        .clone()
        .unwrap_or_else(|| "local-default".to_string());
    let fallback_ts = snapshot
        .generated_at
        .unwrap_or_else(|| OffsetDateTime::now_utc().unix_timestamp());

    for reminder in snapshot.reminders {
        let reminder_id = reminder.reminder_id.trim();
        let title = reminder.title.trim();
        if reminder_id.is_empty() || title.is_empty() {
            continue;
        }

        let timestamp = reminder
            .updated_at
            .or(reminder.completed_at)
            .or(reminder.due_at)
            .unwrap_or(fallback_ts);
        let source_ref = reminder_source_ref(&account_id, &reminder, timestamp);
        let payload = serde_json::json!({
            "reminder_id": reminder_id,
            "list_id": reminder.list_id,
            "list_title": reminder.list_title,
            "title": title,
            "notes": reminder.notes,
            "due_at": reminder.due_at,
            "completed": reminder.completed,
            "completed_at": reminder.completed_at,
            "priority": reminder.priority,
            "tags": reminder.tags.unwrap_or_default(),
            "metadata": reminder.metadata.unwrap_or_else(|| serde_json::json!({})),
        });

        let source = reminder
            .source
            .clone()
            .unwrap_or_else(|| default_source.clone());
        let signal_id = storage
            .insert_signal(SignalInsert {
                signal_type: "reminder_item".to_string(),
                source,
                source_ref: Some(source_ref),
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

fn reminder_source_ref(account_id: &str, reminder: &ReminderItem, timestamp: i64) -> String {
    if let Some(source_ref) = reminder
        .source_ref
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        return source_ref.to_string();
    }
    let list_id = reminder.list_id.as_deref().unwrap_or("default");
    let status = if reminder.completed { "done" } else { "open" };
    format!(
        "reminders:{}:{}:{}:{}:{}",
        account_id,
        list_id,
        reminder.reminder_id.trim(),
        timestamp,
        status
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RemindersSnapshot {
    source: Option<String>,
    account_id: Option<String>,
    generated_at: Option<i64>,
    #[serde(default)]
    reminders: Vec<ReminderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ReminderItem {
    pub(crate) reminder_id: String,
    pub(crate) title: String,
    pub(crate) list_id: Option<String>,
    pub(crate) list_title: Option<String>,
    pub(crate) notes: Option<String>,
    pub(crate) due_at: Option<i64>,
    #[serde(default)]
    pub(crate) completed: bool,
    pub(crate) completed_at: Option<i64>,
    pub(crate) priority: Option<i64>,
    #[serde(default)]
    pub(crate) tags: Option<Vec<String>>,
    pub(crate) metadata: Option<serde_json::Value>,
    pub(crate) updated_at: Option<i64>,
    pub(crate) source: Option<String>,
    pub(crate) source_ref: Option<String>,
}

pub(crate) async fn apply_write_intent(
    config: &AppConfig,
    kind: ReminderWriteKind,
    reminder_id: Option<&str>,
    mutation: ReminderWriteMutation,
) -> Result<ReminderWriteResult, crate::errors::AppError> {
    let path = config
        .reminders_snapshot_path
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            crate::errors::AppError::bad_request(
                "reminders writeback requires a local reminders snapshot path",
            )
        })?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let mut snapshot = load_snapshot_for_write(path, now).await?;
    let account_id = snapshot
        .account_id
        .clone()
        .unwrap_or_else(|| "local-default".to_string());
    let reminder = match kind {
        ReminderWriteKind::Create => {
            let title = mutation
                .title
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    crate::errors::AppError::bad_request("reminders_create requires a title")
                })?;
            let reminder_id = reminder_id
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| format!("rem_local_{}", uuid::Uuid::new_v4().simple()));
            let reminder = ReminderItem {
                reminder_id: reminder_id.clone(),
                title: title.to_string(),
                list_id: mutation.list_id,
                list_title: mutation.list_title,
                notes: mutation.notes,
                due_at: mutation.due_at,
                completed: false,
                completed_at: None,
                priority: mutation.priority,
                tags: mutation.tags,
                metadata: mutation.metadata,
                updated_at: Some(now),
                source: Some("reminders".to_string()),
                source_ref: Some(format!("vel-reminder:{}:{}", reminder_id, now)),
            };
            snapshot.reminders.push(reminder.clone());
            reminder
        }
        ReminderWriteKind::Update => {
            let reminder_id = reminder_id
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    crate::errors::AppError::bad_request("reminders_update requires reminder_id")
                })?;
            let reminder = snapshot
                .reminders
                .iter_mut()
                .find(|item| item.reminder_id == reminder_id)
                .ok_or_else(|| crate::errors::AppError::not_found("reminder not found"))?;
            apply_update(reminder, mutation, now);
            reminder.clone()
        }
        ReminderWriteKind::Complete => {
            let reminder_id = reminder_id
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    crate::errors::AppError::bad_request("reminders_complete requires reminder_id")
                })?;
            let reminder = snapshot
                .reminders
                .iter_mut()
                .find(|item| item.reminder_id == reminder_id)
                .ok_or_else(|| crate::errors::AppError::not_found("reminder not found"))?;
            reminder.completed = true;
            reminder.completed_at = Some(now);
            reminder.updated_at = Some(now);
            reminder.source = Some("reminders".to_string());
            reminder.source_ref = Some(format!(
                "reminders:{}:{}:{}:{}:{}",
                account_id,
                reminder.list_id.as_deref().unwrap_or("default"),
                reminder.reminder_id,
                now,
                "done"
            ));
            reminder.clone()
        }
    };
    snapshot.generated_at = Some(now);
    write_snapshot(path, &snapshot).await?;
    Ok(ReminderWriteResult {
        snapshot_path: path.to_string(),
        reminder,
    })
}

fn apply_update(reminder: &mut ReminderItem, mutation: ReminderWriteMutation, now: i64) {
    if let Some(title) = mutation
        .title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        reminder.title = title.to_string();
    }
    if let Some(list_id) = mutation.list_id {
        reminder.list_id = Some(list_id);
    }
    if let Some(list_title) = mutation.list_title {
        reminder.list_title = Some(list_title);
    }
    if let Some(notes) = mutation.notes {
        reminder.notes = Some(notes);
    }
    if let Some(due_at) = mutation.due_at {
        reminder.due_at = Some(due_at);
    }
    if let Some(priority) = mutation.priority {
        reminder.priority = Some(priority);
    }
    if let Some(tags) = mutation.tags {
        reminder.tags = Some(tags);
    }
    if let Some(metadata) = mutation.metadata {
        reminder.metadata = Some(metadata);
    }
    reminder.completed = false;
    reminder.completed_at = None;
    reminder.updated_at = Some(now);
    reminder.source = Some("reminders".to_string());
    reminder.source_ref = Some(format!("vel-reminder:{}:{}", reminder.reminder_id, now));
}

async fn load_snapshot_for_write(
    path: &str,
    now: i64,
) -> Result<RemindersSnapshot, crate::errors::AppError> {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => serde_json::from_str(&content).map_err(|error| {
            crate::errors::AppError::internal(format!("parse reminders snapshot: {}", error))
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(RemindersSnapshot {
            source: Some("reminders".to_string()),
            account_id: Some("local-default".to_string()),
            generated_at: Some(now),
            reminders: Vec::new(),
        }),
        Err(error) => Err(crate::errors::AppError::internal(format!(
            "read reminders snapshot {}: {}",
            path, error
        ))),
    }
}

async fn write_snapshot(
    path: &str,
    snapshot: &RemindersSnapshot,
) -> Result<(), crate::errors::AppError> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|error| {
            crate::errors::AppError::internal(format!(
                "create reminders snapshot directory {}: {}",
                parent.display(),
                error
            ))
        })?;
    }
    let content = serde_json::to_vec_pretty(snapshot).map_err(|error| {
        crate::errors::AppError::internal(format!("serialize reminders snapshot: {}", error))
    })?;
    tokio::fs::write(path, content).await.map_err(|error| {
        crate::errors::AppError::internal(format!("write reminders snapshot {}: {}", path, error))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_snapshot_path(label: &str) -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir()
            .join(format!("vel_{label}_{nanos}.json"))
            .display()
            .to_string()
    }

    #[test]
    fn source_ref_prefers_explicit_identity() {
        let reminder = ReminderItem {
            reminder_id: "rem_1".to_string(),
            title: "Follow up".to_string(),
            list_id: Some("inbox".to_string()),
            list_title: None,
            notes: None,
            due_at: None,
            completed: false,
            completed_at: None,
            priority: None,
            tags: None,
            metadata: None,
            updated_at: Some(123),
            source: None,
            source_ref: Some("apple-reminder:abc".to_string()),
        };

        assert_eq!(
            reminder_source_ref("local-default", &reminder, 123),
            "apple-reminder:abc"
        );
    }

    #[tokio::test]
    async fn apply_write_intent_creates_updates_and_completes_reminders() {
        let snapshot_path = unique_snapshot_path("reminders_write");
        let config = AppConfig {
            reminders_snapshot_path: Some(snapshot_path.clone()),
            ..Default::default()
        };

        let created = apply_write_intent(
            &config,
            ReminderWriteKind::Create,
            Some("rem_local_test"),
            ReminderWriteMutation {
                title: Some("Follow up".to_string()),
                list_id: Some("inbox".to_string()),
                list_title: None,
                notes: Some("Call back".to_string()),
                due_at: None,
                priority: Some(3),
                tags: Some(vec!["work".to_string()]),
                metadata: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(created.reminder.title, "Follow up");

        let updated = apply_write_intent(
            &config,
            ReminderWriteKind::Update,
            Some("rem_local_test"),
            ReminderWriteMutation {
                title: Some("Updated".to_string()),
                list_id: None,
                list_title: None,
                notes: None,
                due_at: Some(1700000000),
                priority: None,
                tags: None,
                metadata: Some(serde_json::json!({"source": "vel"})),
            },
        )
        .await
        .unwrap();
        assert_eq!(updated.reminder.title, "Updated");
        assert_eq!(updated.reminder.due_at, Some(1700000000));

        let completed = apply_write_intent(
            &config,
            ReminderWriteKind::Complete,
            Some("rem_local_test"),
            ReminderWriteMutation {
                title: None,
                list_id: None,
                list_title: None,
                notes: None,
                due_at: None,
                priority: None,
                tags: None,
                metadata: None,
            },
        )
        .await
        .unwrap();
        assert!(completed.reminder.completed);

        let _ = std::fs::remove_file(snapshot_path);
    }
}
