//! Reminders adapter: ingest local reminders snapshots and emit replay-safe reminder_item signals.

use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

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

#[derive(Debug, serde::Deserialize)]
struct RemindersSnapshot {
    source: Option<String>,
    account_id: Option<String>,
    generated_at: Option<i64>,
    #[serde(default)]
    reminders: Vec<ReminderItem>,
}

#[derive(Debug, serde::Deserialize)]
struct ReminderItem {
    reminder_id: String,
    title: String,
    list_id: Option<String>,
    list_title: Option<String>,
    notes: Option<String>,
    due_at: Option<i64>,
    #[serde(default)]
    completed: bool,
    completed_at: Option<i64>,
    priority: Option<i64>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    metadata: Option<serde_json::Value>,
    updated_at: Option<i64>,
    source: Option<String>,
    source_ref: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
