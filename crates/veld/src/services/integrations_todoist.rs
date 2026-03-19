use std::collections::HashMap;

use reqwest::{Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use vel_core::{
    Commitment, CommitmentStatus, ConflictCaseKind, IntegrationConnectionId,
    IntegrationConnectionStatus, IntegrationFamily, IntegrationProvider, IntegrationSourceRef,
    NodeIdentity, OrderingStamp, ProjectId, WritebackTargetRef,
};
use vel_storage::{
    CommitmentInsert, IntegrationConnectionFilters, IntegrationConnectionInsert, SignalInsert,
    Storage, UpstreamObjectRefRecord,
};

use crate::errors::AppError;

pub(crate) const TODOIST_SETTINGS_KEY: &str = "integration_todoist";
pub(crate) const TODOIST_SECRETS_KEY: &str = "integration_todoist_secrets";

const TODOIST_PROVIDER_KEY: &str = "todoist";
const TODOIST_SYNC_API_BASE_URL: &str = "https://api.todoist.com/api/v1";
const TODOIST_REST_API_BASE_URL: &str = "https://api.todoist.com/rest/v2";
const DEFAULT_ORDERING_NODE_ID: &str = "00000000-0000-0000-0000-000000000000";
const WAITING_ON_LABEL_PREFIX: &str = "waiting_on:";
const REVIEW_STATE_LABEL_PREFIX: &str = "review_state:";
const SCHEDULED_FOR_LABEL_PREFIX: &str = "scheduled_for:";
const PRIORITY_LABEL_PREFIX: &str = "priority:";

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TodoistTaskMutation {
    pub content: Option<String>,
    pub project_id: Option<String>,
    pub scheduled_for: Option<String>,
    pub priority: Option<u8>,
    pub waiting_on: Option<String>,
    pub review_state: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TodoistWritePlan {
    pub target: WritebackTargetRef,
    pub requested_payload: JsonValue,
    pub provenance: Vec<IntegrationSourceRef>,
    pub requested_by_node_id: String,
    execution: TodoistWriteExecutionPlan,
}

#[derive(Debug, Clone)]
enum TodoistWriteExecutionPlan {
    Create {
        connection_id: IntegrationConnectionId,
        mutation: TodoistTaskMutation,
        project: Option<ResolvedTodoistProject>,
    },
    Update {
        connection_id: IntegrationConnectionId,
        commitment: Commitment,
        upstream_ref: UpstreamObjectRefRecord,
        mutation: TodoistTaskMutation,
    },
    Complete {
        connection_id: IntegrationConnectionId,
        commitment: Commitment,
        upstream_ref: UpstreamObjectRefRecord,
    },
    Reopen {
        connection_id: IntegrationConnectionId,
        commitment: Commitment,
        upstream_ref: UpstreamObjectRefRecord,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum TodoistWriteExecutionResult {
    Applied {
        result_payload: JsonValue,
        target_external_id: Option<String>,
        provenance: Vec<IntegrationSourceRef>,
    },
    Conflict {
        kind: ConflictCaseKind,
        summary: String,
        upstream_payload: Option<JsonValue>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TodoistTaskFields {
    project_id: Option<ProjectId>,
    scheduled_for: Option<String>,
    priority: Option<u8>,
    waiting_on: Option<String>,
    review_state: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct TodoistLabelState {
    scheduled_for: Option<String>,
    priority: Option<u8>,
    waiting_on: Option<String>,
    review_state: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct ResolvedTodoistProject {
    project_id: Option<ProjectId>,
    project_slug: Option<String>,
    display_name: Option<String>,
    upstream_id: Option<String>,
}

#[derive(Debug, Clone)]
struct PersistedTodoistTask {
    commitment_id: String,
    source_ref: IntegrationSourceRef,
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
    let tasks = todoist_sync_request_list::<TodoistTask>(&client, &api_token, "/tasks").await?;
    let projects =
        todoist_sync_request_list::<TodoistProject>(&client, &api_token, "/projects").await?;
    let project_names = projects
        .into_iter()
        .map(|project| (project.id, project.name))
        .collect::<HashMap<_, _>>();
    let connection_id = ensure_todoist_connection(storage).await?;
    let existing_commitments = storage.list_commitments(None, None, None, 2000).await?;
    let mut signals_count = 0u32;

    for item in tasks
        .into_iter()
        .filter(|task| !task.content.trim().is_empty())
    {
        let source_id = format!("todoist_{}", item.id);
        let existing = existing_commitments.iter().find(|commitment| {
            commitment.source_type == TODOIST_PROVIDER_KEY
                && commitment.source_id.as_deref() == Some(source_id.as_str())
        });
        let persisted = persist_todoist_task(
            storage,
            existing,
            &item,
            &connection_id,
            project_names
                .get(item.project_id.as_deref().unwrap_or(""))
                .map(String::as_str),
            DEFAULT_ORDERING_NODE_ID,
            true,
        )
        .await?;
        if !persisted.commitment_id.is_empty() {
            signals_count += 1;
        }
    }

    settings.last_sync_at = Some(now_ts());
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

pub(crate) async fn plan_todoist_create_task(
    storage: &Storage,
    requested_by_node_id: &str,
    mutation: TodoistTaskMutation,
) -> Result<TodoistWritePlan, AppError> {
    let connection_id = ensure_todoist_connection(storage).await?;
    let project = resolve_local_project(storage, mutation.project_id.as_deref()).await?;
    let has_project = project.project_id.is_some();
    let target = WritebackTargetRef {
        family: IntegrationFamily::Tasks,
        provider_key: TODOIST_PROVIDER_KEY.to_string(),
        project_id: project.project_id.clone(),
        connection_id: Some(connection_id.clone()),
        external_id: None,
    };
    Ok(TodoistWritePlan {
        requested_payload: todoist_requested_payload(&mutation, None),
        provenance: vec![],
        requested_by_node_id: normalize_requested_by_node_id(requested_by_node_id),
        target,
        execution: TodoistWriteExecutionPlan::Create {
            connection_id,
            mutation,
            project: has_project.then_some(project),
        },
    })
}

pub(crate) async fn plan_todoist_update_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
    mutation: TodoistTaskMutation,
) -> Result<TodoistWritePlan, AppError> {
    if mutation.content.is_none()
        && mutation.project_id.is_none()
        && mutation.scheduled_for.is_none()
        && mutation.priority.is_none()
        && mutation.waiting_on.is_none()
        && mutation.review_state.is_none()
    {
        return Err(AppError::bad_request(
            "todoist_update_task requires at least one changed field",
        ));
    }
    let connection_id = ensure_todoist_connection(storage).await?;
    let commitment = storage
        .get_commitment_by_id(commitment_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("todoist commitment not found"))?;
    let upstream_ref = storage
        .get_upstream_object_ref("commitment", commitment.id.as_ref())
        .await?
        .ok_or_else(|| AppError::bad_request("todoist commitment has no upstream source ref"))?;
    let target = upstream_ref_target(&upstream_ref, Some(connection_id.clone()));
    let provenance = vec![todoist_source_ref(
        &connection_id,
        &upstream_ref.external_id,
    )];
    Ok(TodoistWritePlan {
        requested_payload: todoist_requested_payload(&mutation, Some(commitment.id.as_ref())),
        provenance,
        requested_by_node_id: normalize_requested_by_node_id(requested_by_node_id),
        target,
        execution: TodoistWriteExecutionPlan::Update {
            connection_id,
            commitment,
            upstream_ref,
            mutation,
        },
    })
}

pub(crate) async fn plan_todoist_complete_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
) -> Result<TodoistWritePlan, AppError> {
    plan_existing_todoist_task(
        storage,
        requested_by_node_id,
        commitment_id,
        TodoistExistingAction::Complete,
    )
    .await
}

pub(crate) async fn plan_todoist_reopen_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
) -> Result<TodoistWritePlan, AppError> {
    plan_existing_todoist_task(
        storage,
        requested_by_node_id,
        commitment_id,
        TodoistExistingAction::Reopen,
    )
    .await
}

pub(crate) async fn execute_todoist_write_plan(
    storage: &Storage,
    plan: &TodoistWritePlan,
) -> Result<TodoistWriteExecutionResult, AppError> {
    let settings = load_todoist_settings(storage).await?;
    let api_token = settings
        .api_token
        .ok_or_else(|| AppError::bad_request("todoist api token is required"))?;
    let client = reqwest::Client::new();
    execute_todoist_write_plan_with_base_url(
        storage,
        &client,
        &api_token,
        TODOIST_REST_API_BASE_URL,
        plan,
    )
    .await
}

async fn execute_todoist_write_plan_with_base_url(
    storage: &Storage,
    client: &reqwest::Client,
    api_token: &str,
    base_url: &str,
    plan: &TodoistWritePlan,
) -> Result<TodoistWriteExecutionResult, AppError> {
    match &plan.execution {
        TodoistWriteExecutionPlan::Create {
            connection_id,
            mutation,
            project,
        } => {
            let payload = todoist_upstream_payload(mutation, project.as_ref(), None)?;
            let created: TodoistTask = todoist_rest_request(
                client,
                api_token,
                base_url,
                Method::POST,
                "/tasks",
                Some(payload),
            )
            .await?;
            let persisted = persist_todoist_task(
                storage,
                None,
                &created,
                connection_id,
                project
                    .as_ref()
                    .and_then(|value| value.display_name.as_deref()),
                &plan.requested_by_node_id,
                true,
            )
            .await?;
            let source_ref = persisted.source_ref;
            Ok(TodoistWriteExecutionResult::Applied {
                result_payload: json!({
                    "task": todoist_task_payload(&created),
                    "commitment_id": persisted.commitment_id,
                    "operation": "todoist_create_task",
                }),
                target_external_id: Some(created.id.clone()),
                provenance: vec![source_ref],
            })
        }
        TodoistWriteExecutionPlan::Update {
            connection_id,
            commitment,
            upstream_ref,
            mutation,
        } => {
            let current =
                todoist_get_task(client, api_token, base_url, &upstream_ref.external_id).await?;
            if let Some(conflict) = detect_upstream_conflict(upstream_ref, &current) {
                return Ok(TodoistWriteExecutionResult::Conflict {
                    kind: conflict,
                    summary: format!(
                        "todoist_update_task blocked because upstream task {} changed since the last synced snapshot",
                        upstream_ref.external_id
                    ),
                    upstream_payload: Some(todoist_task_payload(&current)),
                });
            }
            let resolved_project = match mutation.project_id.as_deref() {
                Some(local_project_id) => {
                    Some(resolve_local_project(storage, Some(local_project_id)).await?)
                }
                None => None,
            };
            let payload =
                todoist_upstream_payload(mutation, resolved_project.as_ref(), Some(&current))?;
            let endpoint = format!("/tasks/{}", upstream_ref.external_id);
            let updated: TodoistTask = todoist_rest_request(
                client,
                api_token,
                base_url,
                Method::POST,
                &endpoint,
                Some(payload),
            )
            .await?;
            let persisted = persist_todoist_task(
                storage,
                Some(commitment),
                &updated,
                connection_id,
                resolved_project
                    .as_ref()
                    .and_then(|value| value.display_name.as_deref()),
                &plan.requested_by_node_id,
                true,
            )
            .await?;
            Ok(TodoistWriteExecutionResult::Applied {
                result_payload: json!({
                    "task": todoist_task_payload(&updated),
                    "commitment_id": persisted.commitment_id,
                    "operation": "todoist_update_task",
                }),
                target_external_id: Some(updated.id.clone()),
                provenance: vec![persisted.source_ref],
            })
        }
        TodoistWriteExecutionPlan::Complete {
            connection_id,
            commitment,
            upstream_ref,
        } => {
            let current =
                todoist_get_task(client, api_token, base_url, &upstream_ref.external_id).await?;
            if let Some(conflict) = detect_upstream_conflict(upstream_ref, &current) {
                return Ok(TodoistWriteExecutionResult::Conflict {
                    kind: conflict,
                    summary: format!(
                        "todoist_complete_task blocked because upstream task {} changed since the last synced snapshot",
                        upstream_ref.external_id
                    ),
                    upstream_payload: Some(todoist_task_payload(&current)),
                });
            }
            let endpoint = format!("/tasks/{}/close", upstream_ref.external_id);
            let _: JsonValue =
                todoist_rest_request(client, api_token, base_url, Method::POST, &endpoint, None)
                    .await?;
            let refreshed =
                todoist_get_task(client, api_token, base_url, &upstream_ref.external_id).await?;
            let persisted = persist_todoist_task(
                storage,
                Some(commitment),
                &refreshed,
                connection_id,
                None,
                &plan.requested_by_node_id,
                true,
            )
            .await?;
            Ok(TodoistWriteExecutionResult::Applied {
                result_payload: json!({
                    "task": todoist_task_payload(&refreshed),
                    "commitment_id": persisted.commitment_id,
                    "operation": "todoist_complete_task",
                }),
                target_external_id: Some(refreshed.id.clone()),
                provenance: vec![persisted.source_ref],
            })
        }
        TodoistWriteExecutionPlan::Reopen {
            connection_id,
            commitment,
            upstream_ref,
        } => {
            let current =
                todoist_get_task(client, api_token, base_url, &upstream_ref.external_id).await?;
            if let Some(conflict) = detect_upstream_conflict(upstream_ref, &current) {
                return Ok(TodoistWriteExecutionResult::Conflict {
                    kind: conflict,
                    summary: format!(
                        "todoist_reopen_task blocked because upstream task {} changed since the last synced snapshot",
                        upstream_ref.external_id
                    ),
                    upstream_payload: Some(todoist_task_payload(&current)),
                });
            }
            let endpoint = format!("/tasks/{}/reopen", upstream_ref.external_id);
            let _: JsonValue =
                todoist_rest_request(client, api_token, base_url, Method::POST, &endpoint, None)
                    .await?;
            let refreshed =
                todoist_get_task(client, api_token, base_url, &upstream_ref.external_id).await?;
            let persisted = persist_todoist_task(
                storage,
                Some(commitment),
                &refreshed,
                connection_id,
                None,
                &plan.requested_by_node_id,
                true,
            )
            .await?;
            Ok(TodoistWriteExecutionResult::Applied {
                result_payload: json!({
                    "task": todoist_task_payload(&refreshed),
                    "commitment_id": persisted.commitment_id,
                    "operation": "todoist_reopen_task",
                }),
                target_external_id: Some(refreshed.id.clone()),
                provenance: vec![persisted.source_ref],
            })
        }
    }
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

fn normalize_requested_by_node_id(node_id: &str) -> String {
    let trimmed = node_id.trim();
    if trimmed.is_empty() {
        DEFAULT_ORDERING_NODE_ID.to_string()
    } else {
        trimmed.to_string()
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

async fn ensure_todoist_connection(storage: &Storage) -> Result<IntegrationConnectionId, AppError> {
    let mut existing = storage
        .list_integration_connections(IntegrationConnectionFilters {
            family: Some(IntegrationFamily::Tasks),
            provider_key: Some(TODOIST_PROVIDER_KEY.to_string()),
            status: None,
            include_disabled: true,
        })
        .await?;
    if let Some(connection) = existing.pop() {
        return Ok(connection.id);
    }

    let provider = IntegrationProvider::new(IntegrationFamily::Tasks, TODOIST_PROVIDER_KEY)
        .map_err(|error| AppError::internal(format!("todoist provider: {error}")))?;
    storage
        .insert_integration_connection(IntegrationConnectionInsert {
            family: IntegrationFamily::Tasks,
            provider,
            status: IntegrationConnectionStatus::Pending,
            display_name: "Todoist".to_string(),
            account_ref: None,
            metadata_json: json!({ "foundation": true }),
        })
        .await
        .map_err(Into::into)
}

async fn todoist_sync_request_list<T>(
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
        let value =
            todoist_sync_request_json(client, api_token, endpoint, cursor.as_deref()).await?;
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

async fn todoist_sync_request_json(
    client: &reqwest::Client,
    api_token: &str,
    endpoint: &str,
    cursor: Option<&str>,
) -> Result<JsonValue, AppError> {
    let mut url = Url::parse(&format!("{}{}", TODOIST_SYNC_API_BASE_URL, endpoint))
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

async fn todoist_rest_request<T>(
    client: &reqwest::Client,
    api_token: &str,
    base_url: &str,
    method: Method,
    endpoint: &str,
    body: Option<JsonValue>,
) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let url = Url::parse(&format!("{}{}", base_url.trim_end_matches('/'), endpoint))
        .map_err(|error| AppError::internal(format!("todoist url: {}", error)))?;
    let mut request = client.request(method, url).bearer_auth(api_token);
    if let Some(body) = body {
        request = request.json(&body);
    }

    let response = request
        .send()
        .await
        .map_err(|error| AppError::internal(format!("todoist request: {}", error)))?
        .error_for_status()
        .map_err(|error| AppError::internal(format!("todoist request: {}", error)))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|error| AppError::internal(format!("todoist decode: {}", error)))?;

    if bytes.is_empty() {
        serde_json::from_value(json!({}))
            .map_err(|error| AppError::internal(format!("todoist decode: {}", error)))
    } else {
        serde_json::from_slice(&bytes)
            .map_err(|error| AppError::internal(format!("todoist decode: {}", error)))
    }
}

async fn todoist_get_task(
    client: &reqwest::Client,
    api_token: &str,
    base_url: &str,
    task_id: &str,
) -> Result<TodoistTask, AppError> {
    let endpoint = format!("/tasks/{}", task_id.trim());
    todoist_rest_request(client, api_token, base_url, Method::GET, &endpoint, None).await
}

async fn persist_todoist_task(
    storage: &Storage,
    existing: Option<&Commitment>,
    item: &TodoistTask,
    connection_id: &IntegrationConnectionId,
    project_name_hint: Option<&str>,
    requested_by_node_id: &str,
    emit_signal: bool,
) -> Result<PersistedTodoistTask, AppError> {
    let resolved_project =
        resolve_synced_todoist_project(storage, item.project_id.as_deref(), project_name_hint)
            .await?;
    let task_fields = typed_todoist_task_fields(item, &resolved_project);
    let source_ref = todoist_source_ref(connection_id, &item.id);
    let project_source_ref = item
        .project_id
        .as_ref()
        .map(|project_id| todoist_source_ref(connection_id, project_id));
    let due_ts = parse_due_timestamp(item);
    let due_at = due_ts.and_then(|timestamp| OffsetDateTime::from_unix_timestamp(timestamp).ok());
    let source_id = format!("todoist_{}", item.id);
    let metadata = json!({
        "todoist_id": item.id,
        "labels": item.labels,
        "source_ref": source_ref,
        "project_source_ref": project_source_ref,
        "project_id": task_fields.project_id.as_ref().map(|value| value.as_ref().to_string()),
        "scheduled_for": task_fields.scheduled_for,
        "priority": task_fields.priority,
        "waiting_on": task_fields.waiting_on,
        "review_state": task_fields.review_state,
        "updated_at": item.updated_at,
        "checked": item.checked.unwrap_or(false),
    });
    let commitment_kind = infer_todoist_kind(item);
    let project_label = resolved_project
        .project_slug
        .clone()
        .or_else(|| project_name_hint.map(ToOwned::to_owned))
        .or_else(|| item.project_id.clone());
    let commitment_id = if let Some(commitment) = existing {
        storage
            .update_commitment(
                commitment.id.as_ref(),
                Some(item.content.trim()),
                Some(if item.checked.unwrap_or(false) {
                    CommitmentStatus::Done
                } else {
                    CommitmentStatus::Open
                }),
                Some(due_at),
                project_label.as_deref(),
                Some(commitment_kind),
                Some(&metadata),
            )
            .await?;
        commitment.id.as_ref().to_string()
    } else {
        storage
            .insert_commitment(CommitmentInsert {
                text: item.content.clone(),
                source_type: TODOIST_PROVIDER_KEY.to_string(),
                source_id: source_id.clone(),
                status: if item.checked.unwrap_or(false) {
                    CommitmentStatus::Done
                } else {
                    CommitmentStatus::Open
                },
                due_at,
                project: project_label,
                commitment_kind: Some(commitment_kind.to_string()),
                metadata_json: Some(metadata.clone()),
            })
            .await?
            .as_ref()
            .to_string()
    };

    storage
        .upsert_upstream_object_ref(&UpstreamObjectRefRecord {
            id: format!("uor_{}_{}", TODOIST_PROVIDER_KEY, commitment_id),
            family: IntegrationFamily::Tasks,
            provider_key: TODOIST_PROVIDER_KEY.to_string(),
            project_id: task_fields.project_id.clone(),
            local_object_kind: "commitment".to_string(),
            local_object_id: commitment_id.clone(),
            external_id: item.id.clone(),
            external_parent_id: item.project_id.clone(),
            ordering_stamp: OrderingStamp::new(
                item.updated_at
                    .as_deref()
                    .and_then(parse_rfc3339)
                    .unwrap_or_else(now_ts),
                0,
                NodeIdentity::from(normalize_requested_by_node_id(requested_by_node_id)),
            ),
            last_seen_at: OffsetDateTime::now_utc(),
            metadata_json: json!({
                "content": item.content,
                "checked": item.checked.unwrap_or(false),
                "project_id": item.project_id,
                "scheduled_for": task_fields.scheduled_for,
                "priority": task_fields.priority,
                "waiting_on": task_fields.waiting_on,
                "review_state": task_fields.review_state,
                "updated_at": item.updated_at,
                "labels": item.labels,
            }),
        })
        .await?;

    if emit_signal {
        storage
            .insert_signal(SignalInsert {
                signal_type: "external_task".to_string(),
                source: TODOIST_PROVIDER_KEY.to_string(),
                source_ref: Some(todoist_signal_source_ref(item, due_ts)),
                timestamp: now_ts(),
                payload_json: Some(json!({
                    "task_id": item.id,
                    "text": item.content,
                    "completed": item.checked.unwrap_or(false),
                    "source_ref": source_ref,
                    "project_source_ref": project_source_ref,
                    "labels": item.labels,
                    "project_id": task_fields.project_id.as_ref().map(|value| value.as_ref().to_string()),
                    "scheduled_for": task_fields.scheduled_for,
                    "priority": task_fields.priority,
                    "waiting_on": task_fields.waiting_on,
                    "review_state": task_fields.review_state,
                    "upstream_project_id": item.project_id,
                })),
            })
            .await?;
    }

    Ok(PersistedTodoistTask {
        commitment_id,
        source_ref,
    })
}

async fn resolve_synced_todoist_project(
    storage: &Storage,
    upstream_project_id: Option<&str>,
    project_name_hint: Option<&str>,
) -> Result<ResolvedTodoistProject, AppError> {
    if let Some(upstream_project_id) = upstream_project_id {
        if let Some(project) = storage
            .get_project_by_upstream_id(TODOIST_PROVIDER_KEY, upstream_project_id)
            .await?
        {
            return Ok(ResolvedTodoistProject {
                project_id: Some(project.id),
                project_slug: Some(project.slug),
                display_name: Some(project.name),
                upstream_id: Some(upstream_project_id.to_string()),
            });
        }
    }

    if let Some(project_name_hint) = project_name_hint {
        let normalized_slug = normalize_project_slug(project_name_hint);
        if let Some(project) = storage.get_project_by_slug(&normalized_slug).await? {
            return Ok(ResolvedTodoistProject {
                project_id: Some(project.id),
                project_slug: Some(project.slug),
                display_name: Some(project.name),
                upstream_id: upstream_project_id.map(ToOwned::to_owned),
            });
        }
    }

    Ok(ResolvedTodoistProject {
        project_id: None,
        project_slug: project_name_hint.map(normalize_project_slug),
        display_name: project_name_hint.map(ToOwned::to_owned),
        upstream_id: upstream_project_id.map(ToOwned::to_owned),
    })
}

async fn resolve_local_project(
    storage: &Storage,
    local_project_id: Option<&str>,
) -> Result<ResolvedTodoistProject, AppError> {
    let Some(local_project_id) = local_project_id else {
        return Ok(ResolvedTodoistProject::default());
    };
    let project = storage
        .get_project(local_project_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("local project not found"))?;
    let upstream_id = project
        .upstream_ids
        .get(TODOIST_PROVIDER_KEY)
        .cloned()
        .ok_or_else(|| AppError::bad_request("project is missing todoist upstream_ids mapping"))?;
    Ok(ResolvedTodoistProject {
        project_id: Some(project.id),
        project_slug: Some(project.slug),
        display_name: Some(project.name),
        upstream_id: Some(upstream_id),
    })
}

fn typed_todoist_task_fields(
    task: &TodoistTask,
    resolved_project: &ResolvedTodoistProject,
) -> TodoistTaskFields {
    let label_state = parse_todoist_labels(&task.labels);
    TodoistTaskFields {
        project_id: resolved_project.project_id.clone(),
        scheduled_for: task
            .due
            .as_ref()
            .and_then(|due| due.datetime.as_ref().or(due.date.as_ref()))
            .cloned()
            .or(label_state.scheduled_for),
        priority: task.priority.or(label_state.priority),
        waiting_on: label_state.waiting_on,
        review_state: label_state.review_state,
    }
}

fn parse_todoist_labels(labels: &[String]) -> TodoistLabelState {
    let mut state = TodoistLabelState::default();
    for label in labels {
        let lower = label.trim().to_ascii_lowercase();
        if let Some(value) = lower.strip_prefix(WAITING_ON_LABEL_PREFIX) {
            state.waiting_on = normalize_optional(value.to_string());
        } else if let Some(value) = lower.strip_prefix(REVIEW_STATE_LABEL_PREFIX) {
            state.review_state = normalize_optional(value.to_string());
        } else if let Some(value) = label.trim().strip_prefix(SCHEDULED_FOR_LABEL_PREFIX) {
            state.scheduled_for = normalize_optional(value.to_string());
        } else if let Some(value) = lower.strip_prefix(PRIORITY_LABEL_PREFIX) {
            state.priority = value.parse::<u8>().ok();
        }
    }
    state
}

fn todoist_requested_payload(
    mutation: &TodoistTaskMutation,
    commitment_id: Option<&str>,
) -> JsonValue {
    json!({
        "commitment_id": commitment_id,
        "content": mutation.content,
        "project_id": mutation.project_id,
        "scheduled_for": mutation.scheduled_for,
        "priority": mutation.priority,
        "waiting_on": mutation.waiting_on,
        "review_state": mutation.review_state,
    })
}

fn todoist_upstream_payload(
    mutation: &TodoistTaskMutation,
    resolved_project: Option<&ResolvedTodoistProject>,
    current: Option<&TodoistTask>,
) -> Result<JsonValue, AppError> {
    let mut body = serde_json::Map::new();
    if let Some(content) = mutation.content.as_deref() {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err(AppError::bad_request(
                "todoist task content must not be empty",
            ));
        }
        body.insert(
            "content".to_string(),
            JsonValue::String(trimmed.to_string()),
        );
    }
    if mutation.project_id.is_some() {
        body.insert(
            "project_id".to_string(),
            resolved_project
                .and_then(|project| project.upstream_id.clone())
                .map(JsonValue::String)
                .unwrap_or(JsonValue::Null),
        );
    }
    if let Some(scheduled_for) = mutation.scheduled_for.as_deref() {
        body.insert(
            "due_string".to_string(),
            JsonValue::String(scheduled_for.trim().to_string()),
        );
    }
    if let Some(priority) = mutation.priority {
        body.insert("priority".to_string(), json!(priority));
    }
    let labels = todoist_compatibility_labels(
        current.map(|task| task.labels.as_slice()).unwrap_or(&[]),
        mutation.waiting_on.as_deref(),
        mutation.review_state.as_deref(),
    );
    if !labels.is_empty() || mutation.waiting_on.is_some() || mutation.review_state.is_some() {
        body.insert("labels".to_string(), json!(labels));
    }
    Ok(JsonValue::Object(body))
}

fn todoist_compatibility_labels(
    existing_labels: &[String],
    waiting_on: Option<&str>,
    review_state: Option<&str>,
) -> Vec<String> {
    let mut labels = existing_labels
        .iter()
        .filter(|label| {
            let lower = label.to_ascii_lowercase();
            !lower.starts_with(WAITING_ON_LABEL_PREFIX)
                && !lower.starts_with(REVIEW_STATE_LABEL_PREFIX)
                && !lower.starts_with(SCHEDULED_FOR_LABEL_PREFIX)
                && !lower.starts_with(PRIORITY_LABEL_PREFIX)
        })
        .cloned()
        .collect::<Vec<_>>();
    if let Some(waiting_on) = waiting_on.and_then(|value| normalize_optional(value.to_string())) {
        labels.push(format!("{WAITING_ON_LABEL_PREFIX}{waiting_on}"));
    }
    if let Some(review_state) = review_state.and_then(|value| normalize_optional(value.to_string()))
    {
        labels.push(format!("{REVIEW_STATE_LABEL_PREFIX}{review_state}"));
    }
    labels.sort();
    labels.dedup();
    labels
}

fn detect_upstream_conflict(
    upstream_ref: &UpstreamObjectRefRecord,
    current: &TodoistTask,
) -> Option<ConflictCaseKind> {
    // Conflict kinds stay explicit for operator review: `stale_write` on updated_at drift,
    // `upstream_vs_local` when the synced task payload no longer matches local state.
    let current_payload = todoist_task_payload(current);
    let previous_updated_at = upstream_ref
        .metadata_json
        .get("updated_at")
        .and_then(JsonValue::as_str);
    let current_updated_at = current.updated_at.as_deref();
    if previous_updated_at.is_some() && previous_updated_at != current_updated_at {
        return Some(ConflictCaseKind::StaleWrite);
    }

    for key in [
        "content",
        "checked",
        "project_id",
        "scheduled_for",
        "priority",
    ] {
        if upstream_ref.metadata_json.get(key) != current_payload.get(key) {
            return Some(ConflictCaseKind::UpstreamVsLocal);
        }
    }

    None
}

fn todoist_task_payload(task: &TodoistTask) -> JsonValue {
    json!({
        "id": task.id,
        "content": task.content,
        "checked": task.checked.unwrap_or(false),
        "project_id": task.project_id,
        "scheduled_for": task
            .due
            .as_ref()
            .and_then(|due| due.datetime.as_ref().or(due.date.as_ref())),
        "priority": task.priority,
        "labels": task.labels,
        "updated_at": task.updated_at,
    })
}

fn upstream_ref_target(
    upstream_ref: &UpstreamObjectRefRecord,
    connection_id: Option<IntegrationConnectionId>,
) -> WritebackTargetRef {
    WritebackTargetRef {
        family: upstream_ref.family,
        provider_key: upstream_ref.provider_key.clone(),
        project_id: upstream_ref.project_id.clone(),
        connection_id,
        external_id: Some(upstream_ref.external_id.clone()),
    }
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

fn parse_due_timestamp(task: &TodoistTask) -> Option<i64> {
    task.due
        .as_ref()
        .and_then(|due| due.datetime.as_deref().or(due.date.as_deref()))
        .and_then(parse_iso_datetime)
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

fn todoist_source_ref(
    connection_id: &IntegrationConnectionId,
    external_id: &str,
) -> IntegrationSourceRef {
    IntegrationSourceRef {
        family: IntegrationFamily::Tasks,
        provider_key: TODOIST_PROVIDER_KEY.to_string(),
        connection_id: connection_id.clone(),
        external_id: external_id.to_string(),
    }
}

fn normalize_project_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut last_dash = false;
    for ch in name.trim().chars() {
        let normalized = ch.to_ascii_lowercase();
        if normalized.is_ascii_alphanumeric() {
            slug.push(normalized);
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

#[derive(Debug, Clone, Copy)]
enum TodoistExistingAction {
    Complete,
    Reopen,
}

async fn plan_existing_todoist_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
    action: TodoistExistingAction,
) -> Result<TodoistWritePlan, AppError> {
    let connection_id = ensure_todoist_connection(storage).await?;
    let commitment = storage
        .get_commitment_by_id(commitment_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("todoist commitment not found"))?;
    let upstream_ref = storage
        .get_upstream_object_ref("commitment", commitment.id.as_ref())
        .await?
        .ok_or_else(|| AppError::bad_request("todoist commitment has no upstream source ref"))?;
    let target = upstream_ref_target(&upstream_ref, Some(connection_id.clone()));
    let provenance = vec![todoist_source_ref(
        &connection_id,
        &upstream_ref.external_id,
    )];
    let requested_payload = json!({
        "commitment_id": commitment.id,
        "external_id": upstream_ref.external_id,
        "action": match action {
            TodoistExistingAction::Complete => "todoist_complete_task",
            TodoistExistingAction::Reopen => "todoist_reopen_task",
        },
    });
    let execution = match action {
        TodoistExistingAction::Complete => TodoistWriteExecutionPlan::Complete {
            connection_id,
            commitment,
            upstream_ref,
        },
        TodoistExistingAction::Reopen => TodoistWriteExecutionPlan::Reopen {
            connection_id,
            commitment,
            upstream_ref,
        },
    };
    Ok(TodoistWritePlan {
        target,
        requested_payload,
        provenance,
        requested_by_node_id: normalize_requested_by_node_id(requested_by_node_id),
        execution,
    })
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
    #[serde(default)]
    updated_at: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        extract::{Path, State},
        routing::{get, post},
        Json, Router,
    };
    use serde_json::json;
    use std::{
        collections::HashMap,
        net::SocketAddr,
        sync::{Arc, Mutex},
    };
    use tokio::net::TcpListener;
    use vel_core::{ProjectFamily, ProjectProvisionRequest, ProjectRootRef, ProjectStatus};

    #[test]
    fn parses_typed_todoist_label_fields_at_adapter_boundary() {
        let parsed = parse_todoist_labels(&[
            "waiting_on:alex".to_string(),
            "review_state:needs_review".to_string(),
            "priority:4".to_string(),
            "scheduled_for:2026-03-20".to_string(),
        ]);

        assert_eq!(parsed.waiting_on.as_deref(), Some("alex"));
        assert_eq!(parsed.review_state.as_deref(), Some("needs_review"));
        assert_eq!(parsed.priority, Some(4));
        assert_eq!(parsed.scheduled_for.as_deref(), Some("2026-03-20"));
    }

    #[tokio::test]
    async fn resolves_synced_projects_by_todoist_upstream_id_before_slug_fallback() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();
        storage
            .create_project(vel_core::ProjectRecord {
                id: "proj_local".to_string().into(),
                slug: "legacy-runtime".to_string(),
                name: "Legacy Runtime".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/runtime".to_string(),
                    label: "runtime".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/runtime-notes".to_string(),
                    label: "runtime-notes".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: [("todoist".to_string(), "todo-proj-1".to_string())]
                    .into_iter()
                    .collect(),
                pending_provision: ProjectProvisionRequest::default(),
                created_at: now,
                updated_at: now,
                archived_at: None,
            })
            .await
            .unwrap();

        let resolved =
            resolve_synced_todoist_project(&storage, Some("todo-proj-1"), Some("legacy runtime"))
                .await
                .unwrap();

        assert_eq!(
            resolved.project_id.as_ref().map(|value| value.as_ref()),
            Some("proj_local")
        );
        assert_eq!(resolved.project_slug.as_deref(), Some("legacy-runtime"));
    }

    #[tokio::test]
    async fn todoist_write_plan_detects_stale_write_conflicts() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(TODOIST_SECRETS_KEY, &json!({ "api_token": "todo-token" }))
            .await
            .unwrap();
        let _connection_id = ensure_todoist_connection(&storage).await.unwrap();
        let commitment_id = storage
            .insert_commitment(CommitmentInsert {
                text: "Follow up".to_string(),
                source_type: TODOIST_PROVIDER_KEY.to_string(),
                source_id: "todoist_task_1".to_string(),
                status: CommitmentStatus::Open,
                due_at: None,
                project: Some("runtime".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(json!({ "todoist_id": "task_1" })),
            })
            .await
            .unwrap();
        storage
            .upsert_upstream_object_ref(&UpstreamObjectRefRecord {
                id: "uor_task_1".to_string(),
                family: IntegrationFamily::Tasks,
                provider_key: TODOIST_PROVIDER_KEY.to_string(),
                project_id: None,
                local_object_kind: "commitment".to_string(),
                local_object_id: commitment_id.as_ref().to_string(),
                external_id: "task_1".to_string(),
                external_parent_id: Some("todo-proj-1".to_string()),
                ordering_stamp: OrderingStamp::new(
                    1,
                    0,
                    NodeIdentity::from(DEFAULT_ORDERING_NODE_ID.to_string()),
                ),
                last_seen_at: OffsetDateTime::now_utc(),
                metadata_json: json!({
                    "content": "Follow up",
                    "checked": false,
                    "project_id": "todo-proj-1",
                    "scheduled_for": "2026-03-19",
                    "priority": 2,
                    "updated_at": "2026-03-18T08:00:00Z"
                }),
            })
            .await
            .unwrap();
        let plan = plan_todoist_update_task(
            &storage,
            "node-local",
            commitment_id.as_ref(),
            TodoistTaskMutation {
                content: Some("Follow up now".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        let server = spawn_mock_todoist_server(vec![MockTodoistTask {
            id: "task_1".to_string(),
            content: "Follow up".to_string(),
            project_id: Some("todo-proj-1".to_string()),
            priority: Some(2),
            checked: false,
            labels: vec![],
            due_string: Some("2026-03-19".to_string()),
            updated_at: Some("2026-03-18T09:00:00Z".to_string()),
        }])
        .await;

        let outcome = execute_todoist_write_plan_with_base_url(
            &storage,
            &reqwest::Client::new(),
            "todo-token",
            &server.base_url,
            &plan,
        )
        .await
        .unwrap();

        match outcome {
            TodoistWriteExecutionResult::Conflict { kind, .. } => {
                assert_eq!(kind, ConflictCaseKind::StaleWrite);
            }
            other => panic!("expected conflict, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn todoist_write_plan_applies_updates_and_preserves_commitment_identity() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(TODOIST_SECRETS_KEY, &json!({ "api_token": "todo-token" }))
            .await
            .unwrap();
        let commitment_id = storage
            .insert_commitment(CommitmentInsert {
                text: "Follow up".to_string(),
                source_type: TODOIST_PROVIDER_KEY.to_string(),
                source_id: "todoist_task_1".to_string(),
                status: CommitmentStatus::Open,
                due_at: None,
                project: Some("runtime".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(json!({ "todoist_id": "task_1" })),
            })
            .await
            .unwrap();
        storage
            .upsert_upstream_object_ref(&UpstreamObjectRefRecord {
                id: "uor_task_1".to_string(),
                family: IntegrationFamily::Tasks,
                provider_key: TODOIST_PROVIDER_KEY.to_string(),
                project_id: None,
                local_object_kind: "commitment".to_string(),
                local_object_id: commitment_id.as_ref().to_string(),
                external_id: "task_1".to_string(),
                external_parent_id: Some("todo-proj-1".to_string()),
                ordering_stamp: OrderingStamp::new(
                    1,
                    0,
                    NodeIdentity::from(DEFAULT_ORDERING_NODE_ID.to_string()),
                ),
                last_seen_at: OffsetDateTime::now_utc(),
                metadata_json: json!({
                    "content": "Follow up",
                    "checked": false,
                    "project_id": "todo-proj-1",
                    "scheduled_for": "2026-03-19",
                    "priority": 2,
                    "updated_at": "2026-03-18T08:00:00Z"
                }),
            })
            .await
            .unwrap();
        let plan = plan_todoist_update_task(
            &storage,
            "node-local",
            commitment_id.as_ref(),
            TodoistTaskMutation {
                content: Some("Follow up today".to_string()),
                waiting_on: Some("alex".to_string()),
                review_state: Some("needs_review".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        let server = spawn_mock_todoist_server(vec![MockTodoistTask {
            id: "task_1".to_string(),
            content: "Follow up".to_string(),
            project_id: Some("todo-proj-1".to_string()),
            priority: Some(2),
            checked: false,
            labels: vec![],
            due_string: Some("2026-03-19".to_string()),
            updated_at: Some("2026-03-18T08:00:00Z".to_string()),
        }])
        .await;

        let outcome = execute_todoist_write_plan_with_base_url(
            &storage,
            &reqwest::Client::new(),
            "todo-token",
            &server.base_url,
            &plan,
        )
        .await
        .unwrap();

        match outcome {
            TodoistWriteExecutionResult::Applied { result_payload, .. } => {
                assert_eq!(result_payload["operation"], "todoist_update_task");
                assert_eq!(result_payload["commitment_id"], commitment_id.as_ref());
            }
            other => panic!("expected applied result, got {other:?}"),
        }

        let updated = storage
            .get_commitment_by_id(commitment_id.as_ref())
            .await
            .unwrap()
            .expect("updated commitment should exist");
        assert_eq!(updated.text, "Follow up today");
        assert_eq!(updated.metadata_json["waiting_on"], "alex");
        assert_eq!(updated.metadata_json["review_state"], "needs_review");
    }

    #[derive(Clone)]
    struct MockTodoistServer {
        base_url: String,
        _handle: Arc<tokio::task::JoinHandle<()>>,
    }

    #[derive(Debug, Clone)]
    struct MockTodoistTask {
        id: String,
        content: String,
        project_id: Option<String>,
        priority: Option<u8>,
        checked: bool,
        labels: Vec<String>,
        due_string: Option<String>,
        updated_at: Option<String>,
    }

    #[derive(Clone)]
    struct MockTodoistState {
        tasks: Arc<Mutex<HashMap<String, MockTodoistTask>>>,
    }

    #[derive(Debug, Deserialize)]
    struct MockTodoistMutation {
        #[serde(default)]
        content: Option<String>,
        #[serde(default)]
        project_id: Option<String>,
        #[serde(default)]
        due_string: Option<String>,
        #[serde(default)]
        priority: Option<u8>,
        #[serde(default)]
        labels: Option<Vec<String>>,
    }

    async fn spawn_mock_todoist_server(initial_tasks: Vec<MockTodoistTask>) -> MockTodoistServer {
        let state = MockTodoistState {
            tasks: Arc::new(Mutex::new(
                initial_tasks
                    .into_iter()
                    .map(|task| (task.id.clone(), task))
                    .collect(),
            )),
        };
        let app = Router::new()
            .route("/tasks", post(mock_create_task))
            .route("/tasks/:id", get(mock_get_task).post(mock_update_task))
            .route("/tasks/:id/close", post(mock_close_task))
            .route("/tasks/:id/reopen", post(mock_reopen_task))
            .with_state(state);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr: SocketAddr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        MockTodoistServer {
            base_url: format!("http://{}", addr),
            _handle: Arc::new(handle),
        }
    }

    async fn mock_get_task(
        Path(id): Path<String>,
        State(state): State<MockTodoistState>,
    ) -> Json<JsonValue> {
        let tasks = state.tasks.lock().unwrap();
        let task = tasks.get(&id).unwrap();
        Json(mock_task_json(task))
    }

    async fn mock_create_task(
        State(state): State<MockTodoistState>,
        Json(payload): Json<MockTodoistMutation>,
    ) -> Json<JsonValue> {
        let mut tasks = state.tasks.lock().unwrap();
        let task = MockTodoistTask {
            id: "task_created".to_string(),
            content: payload.content.unwrap_or_else(|| "new task".to_string()),
            project_id: payload.project_id,
            priority: payload.priority,
            checked: false,
            labels: payload.labels.unwrap_or_default(),
            due_string: payload.due_string,
            updated_at: Some("2026-03-18T10:00:00Z".to_string()),
        };
        tasks.insert(task.id.clone(), task.clone());
        Json(mock_task_json(&task))
    }

    async fn mock_update_task(
        Path(id): Path<String>,
        State(state): State<MockTodoistState>,
        Json(payload): Json<MockTodoistMutation>,
    ) -> Json<JsonValue> {
        let mut tasks = state.tasks.lock().unwrap();
        let task = tasks.get_mut(&id).unwrap();
        if let Some(content) = payload.content {
            task.content = content;
        }
        if payload.project_id.is_some() {
            task.project_id = payload.project_id;
        }
        if payload.priority.is_some() {
            task.priority = payload.priority;
        }
        if payload.due_string.is_some() {
            task.due_string = payload.due_string;
        }
        if let Some(labels) = payload.labels {
            task.labels = labels;
        }
        task.updated_at = Some("2026-03-18T08:00:00Z".to_string());
        Json(mock_task_json(task))
    }

    async fn mock_close_task(
        Path(id): Path<String>,
        State(state): State<MockTodoistState>,
    ) -> Json<JsonValue> {
        let mut tasks = state.tasks.lock().unwrap();
        let task = tasks.get_mut(&id).unwrap();
        task.checked = true;
        task.updated_at = Some("2026-03-18T10:01:00Z".to_string());
        Json(json!({}))
    }

    async fn mock_reopen_task(
        Path(id): Path<String>,
        State(state): State<MockTodoistState>,
    ) -> Json<JsonValue> {
        let mut tasks = state.tasks.lock().unwrap();
        let task = tasks.get_mut(&id).unwrap();
        task.checked = false;
        task.updated_at = Some("2026-03-18T10:02:00Z".to_string());
        Json(json!({}))
    }

    fn mock_task_json(task: &MockTodoistTask) -> JsonValue {
        json!({
            "id": task.id,
            "content": task.content,
            "project_id": task.project_id,
            "priority": task.priority,
            "checked": task.checked,
            "labels": task.labels,
            "updated_at": task.updated_at,
            "due": task.due_string.as_ref().map(|value| json!({ "date": value })),
        })
    }
}
