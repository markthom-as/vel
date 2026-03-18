use reqwest::Url;
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use vel_core::{Commitment, CommitmentStatus};
use vel_storage::{CommitmentInsert, SignalInsert, Storage};

use crate::errors::AppError;

pub(crate) const TODOIST_SETTINGS_KEY: &str = "integration_todoist";
pub(crate) const TODOIST_SECRETS_KEY: &str = "integration_todoist_secrets";

#[derive(Debug, Clone)]
pub(crate) struct IntegrationGuidance {
    pub title: String,
    pub detail: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TodoistStatus {
    pub configured: bool,
    pub connected: bool,
    pub has_api_token: bool,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TodoistSettings {
    pub api_token: Option<String>,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TodoistPublicSettings {
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TodoistSecrets {
    pub api_token: Option<String>,
}

pub(crate) fn todoist_status(settings: &TodoistSettings) -> TodoistStatus {
    TodoistStatus {
        configured: settings.api_token.is_some(),
        connected: settings.api_token.is_some(),
        has_api_token: settings.api_token.is_some(),
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
        guidance: todoist_guidance(settings),
    }
}

pub(crate) async fn update_todoist_settings(
    storage: &Storage,
    api_token: Option<String>,
) -> Result<(), AppError> {
    let mut settings = load_todoist_settings(storage).await?;
    if let Some(value) = api_token {
        settings.api_token = normalize_optional(value);
    }
    save_todoist_settings(storage, &settings).await
}

pub(crate) async fn disconnect_todoist(storage: &Storage) -> Result<(), AppError> {
    let mut settings = load_todoist_settings(storage).await?;
    settings.api_token = None;
    settings.last_sync_status = Some("disconnected".to_string());
    settings.last_error = None;
    save_todoist_settings(storage, &settings).await
}

pub(crate) async fn sync_todoist(storage: &Storage) -> Result<Option<u32>, AppError> {
    let mut settings = load_todoist_settings(storage).await?;
    let Some(api_token) = settings.api_token.clone() else {
        return Ok(None);
    };

    let client = reqwest::Client::new();
    let tasks = todoist_request_list::<TodoistTask>(&client, &api_token, "/tasks").await?;
    let projects = todoist_request_list::<TodoistProject>(&client, &api_token, "/projects").await?;
    let project_names = projects
        .into_iter()
        .map(|project| (project.id, project.name))
        .collect::<std::collections::HashMap<_, _>>();

    let existing_commitments = storage.list_commitments(None, None, None, 2000).await?;
    let now = now_ts();
    let mut signals_count = 0u32;

    for item in tasks
        .into_iter()
        .filter(|task| !task.content.trim().is_empty())
    {
        let completed = item.checked.unwrap_or(false);
        let due_ts = item
            .due
            .as_ref()
            .and_then(|due| due.datetime.as_deref().or(due.date.as_deref()))
            .and_then(parse_iso_datetime);
        let project = item
            .project_id
            .as_ref()
            .and_then(|id| project_names.get(id))
            .cloned()
            .or(item.project_id.clone());
        let commitment_kind = infer_todoist_kind(&item);
        let source_id = format!("todoist_{}", item.id);
        reconcile_commitment(
            storage,
            existing_commitments.iter().find(|commitment| {
                commitment.source_type == "todoist"
                    && commitment.source_id.as_deref() == Some(source_id.as_str())
            }),
            &item,
            &source_id,
            commitment_kind,
            completed,
            due_ts,
            project.as_deref(),
        )
        .await?;

        let payload = serde_json::json!({
            "task_id": item.id,
            "text": item.content,
            "completed": completed,
            "due_time": due_ts,
            "labels": item.labels,
            "project": project,
            "priority": item.priority,
        });
        let signal_id = storage
            .insert_signal(SignalInsert {
                signal_type: "external_task".to_string(),
                source: "todoist".to_string(),
                source_ref: Some(todoist_signal_source_ref(&item, due_ts)),
                timestamp: now,
                payload_json: Some(payload),
            })
            .await?;
        if signal_id.starts_with("sig_") {
            signals_count += 1;
        }
    }

    settings.last_sync_at = Some(now);
    settings.last_sync_status = Some("ok".to_string());
    settings.last_error = None;
    settings.last_item_count = Some(signals_count);
    save_todoist_settings(storage, &settings).await?;
    Ok(Some(signals_count))
}

pub(crate) async fn record_sync_error(storage: &Storage, error: &str) -> Result<(), AppError> {
    let mut settings = load_todoist_settings(storage).await?;
    settings.last_sync_at = Some(now_ts());
    settings.last_sync_status = Some("error".to_string());
    settings.last_error = Some(error.to_string());
    save_todoist_settings(storage, &settings).await
}

pub(crate) async fn load_todoist_settings(storage: &Storage) -> Result<TodoistSettings, AppError> {
    let public_settings: TodoistPublicSettings =
        load_settings(storage, TODOIST_SETTINGS_KEY).await?;
    let secrets: TodoistSecrets = load_settings(storage, TODOIST_SECRETS_KEY).await?;
    Ok(TodoistSettings {
        api_token: secrets.api_token,
        last_sync_at: public_settings.last_sync_at,
        last_sync_status: public_settings.last_sync_status,
        last_error: public_settings.last_error,
        last_item_count: public_settings.last_item_count,
    })
}

async fn save_todoist_settings(
    storage: &Storage,
    settings: &TodoistSettings,
) -> Result<(), AppError> {
    let public_settings = TodoistPublicSettings {
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
    };
    let secrets = TodoistSecrets {
        api_token: settings.api_token.clone(),
    };
    save_settings(storage, TODOIST_SETTINGS_KEY, &public_settings).await?;
    save_settings(storage, TODOIST_SECRETS_KEY, &secrets).await
}

async fn load_settings<T>(storage: &Storage, key: &str) -> Result<T, AppError>
where
    T: for<'de> Deserialize<'de> + Default,
{
    let all = storage.get_all_settings().await?;
    Ok(all
        .get(key)
        .cloned()
        .map(|value| {
            serde_json::from_value::<T>(value).unwrap_or_else(|err| {
                tracing::warn!(
                    key = %key,
                    error = %err,
                    "integration settings deserialization failed, using defaults"
                );
                T::default()
            })
        })
        .unwrap_or_default())
}

async fn save_settings<T>(storage: &Storage, key: &str, value: &T) -> Result<(), AppError>
where
    T: Serialize,
{
    let value = serde_json::to_value(value).map_err(|error| {
        AppError::internal(format!("serialize integration settings: {}", error))
    })?;
    storage.set_setting(key, &value).await?;
    Ok(())
}

fn todoist_guidance(settings: &TodoistSettings) -> Option<IntegrationGuidance> {
    if settings.api_token.is_none() {
        return Some(guidance(
            "Todoist token missing",
            "Save a Todoist API token before attempting sync.".to_string(),
            "Save token",
        ));
    }
    if settings.last_sync_status.as_deref() == Some("error") {
        return Some(guidance(
            "Todoist sync failed",
            settings
                .last_error
                .clone()
                .unwrap_or_else(|| "Todoist sync last failed.".to_string()),
            "Inspect history and retry sync",
        ));
    }
    if settings.last_sync_at.is_none() {
        return Some(guidance(
            "Todoist has not synced yet",
            "Run a Todoist sync to load open commitments and due tasks.".to_string(),
            "Sync now",
        ));
    }
    None
}

fn guidance(title: &str, detail: String, action: &str) -> IntegrationGuidance {
    IntegrationGuidance {
        title: title.to_string(),
        detail,
        action: action.to_string(),
    }
}

fn normalize_optional(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn now_ts() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

fn parse_rfc3339(value: &str) -> Option<i64> {
    OffsetDateTime::parse(value, &Rfc3339)
        .ok()
        .map(|date_time| date_time.unix_timestamp())
}

fn parse_iso_datetime(value: &str) -> Option<i64> {
    parse_rfc3339(value).or_else(|| {
        let normalized = if value.ends_with('Z') {
            value.to_string()
        } else {
            format!("{}Z", value)
        };
        parse_rfc3339(&normalized)
    })
}

async fn todoist_request_json(
    client: &reqwest::Client,
    api_token: &str,
    endpoint: &str,
    cursor: Option<&str>,
) -> Result<serde_json::Value, AppError> {
    let mut url = Url::parse(&format!("https://api.todoist.com/api/v1{}", endpoint))
        .map_err(|error| AppError::internal(format!("todoist url: {}", error)))?;
    if let Some(cursor) = cursor {
        url.query_pairs_mut().append_pair("cursor", cursor);
    }

    client
        .get(url)
        .bearer_auth(api_token)
        .send()
        .await
        .map_err(|error| AppError::internal(format!("todoist request: {}", error)))?
        .error_for_status()
        .map_err(|error| AppError::internal(format!("todoist request: {}", error)))?
        .json()
        .await
        .map_err(|error| AppError::internal(format!("todoist decode: {}", error)))
}

async fn todoist_request_list<T>(
    client: &reqwest::Client,
    api_token: &str,
    endpoint: &str,
) -> Result<Vec<T>, AppError>
where
    T: for<'de> Deserialize<'de>,
{
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let value = todoist_request_json(client, api_token, endpoint, cursor.as_deref()).await?;
        if let Ok(items) = serde_json::from_value::<Vec<T>>(value.clone()) {
            all_items.extend(items);
            break;
        }

        let page: TodoistPage<T> = serde_json::from_value(value)
            .map_err(|error| AppError::internal(format!("todoist decode results: {}", error)))?;
        all_items.extend(page.results);

        match page.next_cursor {
            Some(next_cursor) if !next_cursor.is_empty() => {
                cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(all_items)
}

async fn reconcile_commitment(
    storage: &Storage,
    existing: Option<&Commitment>,
    item: &TodoistTask,
    source_id: &str,
    commitment_kind: &'static str,
    completed: bool,
    due_ts: Option<i64>,
    project: Option<&str>,
) -> Result<(), AppError> {
    let due_at = due_ts.and_then(|timestamp| OffsetDateTime::from_unix_timestamp(timestamp).ok());
    let metadata = serde_json::json!({
        "todoist_id": item.id,
        "labels": item.labels,
    });
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
            .await?;
    } else {
        storage
            .insert_commitment(CommitmentInsert {
                text: item.content.clone(),
                source_type: "todoist".to_string(),
                source_id: source_id.to_string(),
                status,
                due_at,
                project: project.map(|value| value.to_string()),
                commitment_kind: Some(commitment_kind.to_string()),
                metadata_json: Some(metadata),
            })
            .await?;
    }

    Ok(())
}

fn infer_todoist_kind(task: &TodoistTask) -> &'static str {
    let content_lower = task.content.to_lowercase();
    let labels: Vec<String> = task
        .labels
        .iter()
        .map(|label| label.to_lowercase())
        .collect();
    if labels.iter().any(|label| label == "health")
        || content_lower.contains("meds")
        || content_lower.contains("medication")
    {
        "medication"
    } else {
        "todo"
    }
}

fn todoist_signal_source_ref(task: &TodoistTask, due_ts: Option<i64>) -> String {
    let state = if task.checked.unwrap_or(false) {
        "done"
    } else {
        "open"
    };
    format!(
        "todoist:{}:{}:{}:{}",
        task.id,
        state,
        task.content.trim(),
        due_ts
            .map(|timestamp| timestamp.to_string())
            .unwrap_or_else(|| "-".to_string())
    )
}

#[derive(Debug, Deserialize)]
struct TodoistTask {
    id: String,
    content: String,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    project_id: Option<String>,
    #[serde(default)]
    priority: Option<u8>,
    #[serde(default)]
    checked: Option<bool>,
    #[serde(default)]
    due: Option<TodoistDue>,
}

#[derive(Debug, Deserialize)]
struct TodoistDue {
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    datetime: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TodoistProject {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct TodoistPage<T> {
    results: Vec<T>,
    #[serde(default)]
    next_cursor: Option<String>,
}
