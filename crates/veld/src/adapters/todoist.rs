//! Todoist adapter: read snapshot JSON from config path, reconcile commitments, and emit signals.

use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{Commitment, CommitmentStatus};
use vel_storage::{CommitmentInsert, SignalInsert, Storage};

/// Ingest tasks from Todoist snapshot; create/update commitments and emit signals. Returns signals count.
pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let path = match &config.todoist_snapshot_path {
        Some(p) => p,
        None => return Ok(0),
    };
    match tokio::fs::try_exists(path).await {
        Ok(true) => {}
        Ok(false) if vel_config::is_default_local_source_path("todoist", path) => return Ok(0),
        Ok(false) => {
            return Err(crate::errors::AppError::internal(format!(
                "read todoist snapshot {}: No such file or directory",
                path
            )));
        }
        Err(error) => {
            return Err(crate::errors::AppError::internal(format!(
                "stat todoist snapshot {}: {}",
                path, error
            )));
        }
    }
    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read todoist snapshot {}: {}", path, e))
    })?;
    let snapshot: TodoistSnapshot = serde_json::from_str(&content)
        .map_err(|e| crate::errors::AppError::internal(format!("parse todoist snapshot: {}", e)))?;

    let now = OffsetDateTime::now_utc().unix_timestamp();
    let existing_commitments = storage.list_commitments(None, None, None, 1000).await?;
    let mut signals_count = 0u32;

    for item in snapshot
        .items
        .into_iter()
        .filter(|i| !i.content.trim().is_empty())
    {
        let task_id = item.id.clone();
        let completed = item.checked.unwrap_or(false);
        let due_ts = item
            .due
            .as_ref()
            .and_then(|d| d.date.as_deref())
            .and_then(parse_iso_datetime);
        let commitment_kind = infer_kind(&item);
        let source_id = format!("todoist_{}", task_id);
    let existing_commitment = existing_commitments
        .iter()
        .find(|c| c.source_type == "todoist" && c.source_id.as_deref() == Some(source_id.as_str()))
        .or_else(|| {
            existing_commitments.iter().find(|c| {
                c.source_type == "todoist"
                        && todoist_id_from_metadata(&c.metadata_json).as_deref() == Some(task_id.as_str())
            })
        });

        reconcile_commitment(
            storage,
            existing_commitment,
            &item,
            &source_id,
            commitment_kind,
            completed,
            due_ts,
        )
        .await?;

        let payload = serde_json::json!({
            "task_id": task_id,
            "text": item.content,
            "completed": completed,
            "due_time": due_ts,
            "labels": item.labels,
            "project_id": item.project_id
        });
        let signal_id = storage
            .insert_signal(SignalInsert {
                signal_type: "external_task".to_string(),
                source: "todoist".to_string(),
                source_ref: Some(todoist_signal_source_ref(&item)),
                timestamp: now,
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

async fn reconcile_commitment(
    storage: &Storage,
    existing: Option<&Commitment>,
    item: &TodoistItem,
    source_id: &str,
    commitment_kind: &'static str,
    completed: bool,
    due_ts: Option<i64>,
) -> Result<(), crate::errors::AppError> {
    let due_at = due_ts.and_then(|t| time::OffsetDateTime::from_unix_timestamp(t).ok());
    let metadata = serde_json::json!({
        "todoist_id": item.id,
        "labels": item.labels,
        "priority": item.priority.unwrap_or(1),
        "updated_at": item
            .updated_at
            .as_deref()
            .and_then(parse_rfc3339_timestamp),
        "has_due_time": item
            .due
            .as_ref()
            .and_then(|due| due.date.as_deref())
            .map(has_explicit_due_time)
            .unwrap_or(false),
    });
    let project = item.project_id.as_deref();
    let status = if completed {
        CommitmentStatus::Done
    } else {
        CommitmentStatus::Open
    };

    if let Some(commitment) = existing {
        storage
            .update_commitment(
                commitment.id.as_ref(),
                Some(item.content.trim()),
                Some(status),
                Some(due_at),
                project,
                Some(commitment_kind),
                Some(&metadata),
            )
            .await
            .map_err(crate::errors::AppError::from)?;
    } else {
        storage
            .insert_commitment(CommitmentInsert {
                text: item.content.clone(),
                source_type: "todoist".to_string(),
                source_id: source_id.to_string(),
                status,
                due_at,
                project: item.project_id.clone(),
                commitment_kind: Some(commitment_kind.to_string()),
                metadata_json: Some(metadata),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
    }

    Ok(())
}

fn todoist_id_from_metadata(metadata_json: &serde_json::Value) -> Option<String> {
    metadata_json.get("todoist_id").and_then(|value| match value {
        serde_json::Value::String(value) => Some(value.clone()),
        serde_json::Value::Number(value) => Some(value.to_string()),
        _ => None,
    })
}

fn infer_kind(item: &TodoistItem) -> &'static str {
    let content_lower = item.content.to_lowercase();
    let labels: Vec<String> = item.labels.iter().map(|s| s.to_lowercase()).collect();
    if labels.contains(&"health".to_string())
        || content_lower.contains("meds")
        || content_lower.contains("medication")
    {
        "medication"
    } else {
        "todo"
    }
}

fn parse_iso_datetime(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.len() < 10 {
        return None;
    }
    let date_part = s.get(0..10)?;
    let time_part = s.get(11..).unwrap_or("00:00:00");
    let ymd: Vec<&str> = date_part.split('-').collect();
    if ymd.len() != 3 {
        return None;
    }
    let year: i32 = ymd[0].parse().ok()?;
    let month: u8 = ymd[1].parse().ok()?;
    let day: u8 = ymd[2].parse().ok()?;
    let (hour, min, sec) = if time_part.len() >= 8 {
        let t: Vec<&str> = time_part.split(':').collect();
        (
            t.first().and_then(|x| x.parse().ok()).unwrap_or(0),
            t.get(1).and_then(|x| x.parse().ok()).unwrap_or(0),
            t.get(2).and_then(|x| x.parse().ok()).unwrap_or(0),
        )
    } else {
        (0, 0, 0)
    };
    let month = time::Month::try_from(month).ok()?;
    let date = time::Date::from_calendar_date(year, month, day).ok()?;
    let t = time::Time::from_hms(hour, min, sec).ok()?;
    let dt = time::PrimitiveDateTime::new(date, t).assume_utc();
    Some(dt.unix_timestamp())
}

fn parse_rfc3339_timestamp(value: &str) -> Option<i64> {
    OffsetDateTime::parse(value, &Rfc3339)
        .ok()
        .map(|timestamp| timestamp.unix_timestamp())
}

fn has_explicit_due_time(value: &str) -> bool {
    value.contains('T')
}

fn todoist_signal_source_ref(item: &TodoistItem) -> String {
    let state = if item.checked.unwrap_or(false) {
        "done"
    } else {
        "open"
    };
    let due = item
        .due
        .as_ref()
        .and_then(|due| due.date.as_deref())
        .unwrap_or("-");
    format!(
        "todoist:{}:{}:{}:{}",
        item.id,
        state,
        item.content.trim(),
        due
    )
}

#[derive(Debug, serde::Deserialize)]
struct TodoistSnapshot {
    #[serde(default)]
    items: Vec<TodoistItem>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TodoistItem {
    id: String,
    content: String,
    #[serde(default)]
    checked: Option<bool>,
    #[serde(default)]
    priority: Option<u8>,
    #[serde(default)]
    updated_at: Option<String>,
    due: Option<TodoistDue>,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    project_id: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TodoistDue {
    date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_source_ref_changes_with_lifecycle_state() {
        let open = TodoistItem {
            id: "123".to_string(),
            content: "Ship feature".to_string(),
            checked: Some(false),
            priority: Some(1),
            updated_at: None,
            due: None,
            labels: Vec::new(),
            project_id: None,
        };
        let done = TodoistItem {
            checked: Some(true),
            ..open.clone()
        };
        assert_ne!(
            todoist_signal_source_ref(&open),
            todoist_signal_source_ref(&done)
        );
    }

    #[test]
    fn parses_rfc3339_updated_at_timestamp() {
        let timestamp = parse_rfc3339_timestamp("2026-03-16T15:04:05Z");
        assert_eq!(timestamp, Some(1_773_673_445));
    }

    #[test]
    fn detects_due_time_presence_from_snapshot_value() {
        assert!(has_explicit_due_time("2026-03-17T09:30:00"));
        assert!(!has_explicit_due_time("2026-03-17"));
    }
}
