use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::{adapters, errors::AppError, policy_config::PolicyConfig};

const COMPONENT_SETTINGS_PREFIX: &str = "component_state:";
const COMPONENT_LOG_LIMIT_DEFAULT: u32 = 50;

#[derive(Debug, Clone)]
pub struct ComponentListItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub last_restarted_at: Option<i64>,
    pub last_error: Option<String>,
    pub restart_count: u32,
}

#[derive(Debug, Clone)]
pub struct ComponentLogEvent {
    pub id: String,
    pub component_id: String,
    pub event_name: String,
    pub status: String,
    pub message: String,
    pub payload: serde_json::Value,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct ComponentRestartResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub last_restarted_at: Option<i64>,
    pub last_error: Option<String>,
    pub restart_count: u32,
}

#[derive(Clone, Copy)]
pub(crate) struct ComponentSpec {
    id: &'static str,
    name: &'static str,
    description: &'static str,
}

const COMPONENT_SPECS: &[ComponentSpec] = &[
    ComponentSpec {
        id: "google-calendar",
        name: "Google Calendar",
        description: "Refresh calendar ingest and event-derived context signals.",
    },
    ComponentSpec {
        id: "todoist",
        name: "Todoist",
        description: "Refresh task/completion state and ingest task commitments/signals.",
    },
    ComponentSpec {
        id: "activity",
        name: "Activity",
        description: "Ingest local workstation activity snapshots into signals.",
    },
    ComponentSpec {
        id: "health",
        name: "Health",
        description: "Ingest local health/activity snapshots into signals.",
    },
    ComponentSpec {
        id: "git",
        name: "Git",
        description: "Ingest local git activity snapshots into signals.",
    },
    ComponentSpec {
        id: "messaging",
        name: "Messaging",
        description: "Ingest local messaging snapshots and waiting-state signals.",
    },
    ComponentSpec {
        id: "reminders",
        name: "Reminders",
        description: "Ingest local reminders snapshots into reminder signals.",
    },
    ComponentSpec {
        id: "notes",
        name: "Notes",
        description: "Ingest local notes into captures and signals.",
    },
    ComponentSpec {
        id: "transcripts",
        name: "Transcripts",
        description: "Ingest local transcript snapshots into transcripts and signals.",
    },
    ComponentSpec {
        id: "evaluate",
        name: "Evaluate",
        description: "Run full evaluation and persist updated states/risk/nudges/suggestions.",
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredComponentState {
    status: String,
    last_restarted_at: Option<i64>,
    last_error: Option<String>,
    restart_count: u32,
}

impl Default for StoredComponentState {
    fn default() -> Self {
        Self {
            status: "idle".to_string(),
            last_restarted_at: None,
            last_error: None,
            restart_count: 0,
        }
    }
}

fn component_state_key(component_id: &str) -> String {
    format!("{COMPONENT_SETTINGS_PREFIX}{component_id}")
}

#[allow(dead_code)]
pub(crate) fn component_specs() -> &'static [ComponentSpec] {
    COMPONENT_SPECS
}

pub fn component_exists(id: &str) -> bool {
    COMPONENT_SPECS.iter().any(|spec| spec.id == id)
}

fn component_spec(id: &str) -> Option<&'static ComponentSpec> {
    COMPONENT_SPECS.iter().find(|spec| spec.id == id)
}

fn now_ts() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

fn log_status_for_event(event_name: &str) -> &'static str {
    match event_name {
        "component.restart.requested" => "running",
        "component.restart.completed" => "success",
        "component.restart.failed" => "error",
        _ => "info",
    }
}

fn payload_status(payload: &serde_json::Value) -> String {
    payload
        .get("status")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .map(std::string::ToString::to_string)
        .unwrap_or_default()
}

fn payload_message(event_name: &str, payload: &serde_json::Value) -> String {
    if let Some(message) = payload.get("message").and_then(serde_json::Value::as_str) {
        if !message.is_empty() {
            return message.to_string();
        }
    }
    event_name.to_string()
}

fn payload_json(payload: serde_json::Value) -> String {
    payload.to_string()
}

async fn append_component_event(
    storage: &Storage,
    component_id: &str,
    event_name: &str,
    status: &str,
    message: &str,
    details: serde_json::Value,
) -> Result<(), AppError> {
    storage
        .append_event(vel_storage::EventLogInsert {
            id: None,
            event_name: event_name.to_string(),
            aggregate_type: Some("component".to_string()),
            aggregate_id: Some(component_id.to_string()),
            payload_json: payload_json(serde_json::json!({
                "component_id": component_id,
                "status": status,
                "message": message,
                "details": details,
            })),
        })
        .await?;
    Ok(())
}

async fn load_component_state(
    storage: &Storage,
    component_id: &str,
) -> Result<StoredComponentState, AppError> {
    let all_settings = storage.get_all_settings().await?;
    let state = all_settings
        .get(&component_state_key(component_id))
        .cloned()
        .unwrap_or_default();
    let state = serde_json::from_value(state).unwrap_or_default();
    Ok(state)
}

async fn save_component_state(
    storage: &Storage,
    component_id: &str,
    state: &StoredComponentState,
) -> Result<(), AppError> {
    storage
        .set_setting(
            &component_state_key(component_id),
            &serde_json::to_value(state).map_err(|error| {
                AppError::internal(format!("serialize component state {component_id}: {error}"))
            })?,
        )
        .await?;
    Ok(())
}

fn to_list_item(id: &str, spec: &ComponentSpec, state: StoredComponentState) -> ComponentListItem {
    ComponentListItem {
        id: id.to_string(),
        name: spec.name.to_string(),
        description: spec.description.to_string(),
        status: state.status,
        last_restarted_at: state.last_restarted_at,
        last_error: state.last_error,
        restart_count: state.restart_count,
    }
}

pub async fn list_components(storage: &Storage) -> Result<Vec<ComponentListItem>, AppError> {
    let mut output = Vec::new();
    for spec in COMPONENT_SPECS {
        let state = load_component_state(storage, spec.id).await?;
        output.push(to_list_item(spec.id, spec, state));
    }
    Ok(output)
}

pub async fn list_component_logs(
    storage: &Storage,
    component_id: &str,
    limit: Option<u32>,
) -> Result<Vec<ComponentLogEvent>, AppError> {
    if !component_exists(component_id) {
        return Err(AppError::not_found("component not found"));
    }
    let limit = limit.unwrap_or(COMPONENT_LOG_LIMIT_DEFAULT);
    let events = storage
        .list_events_by_aggregate("component", component_id, limit)
        .await?;

    Ok(events
        .into_iter()
        .map(|event| {
            let payload =
                serde_json::from_str(&event.payload_json).unwrap_or_else(|_| serde_json::json!({}));
            let event_name = event.event_name;
            let status = payload_status(&payload);
            let status = if status.is_empty() {
                log_status_for_event(&event_name).to_string()
            } else {
                status
            };
            ComponentLogEvent {
                id: event.id.to_string(),
                component_id: event
                    .aggregate_id
                    .unwrap_or_else(|| component_id.to_string()),
                event_name: event_name.clone(),
                status,
                message: payload_message(&event_name, &payload),
                payload,
                created_at: event.created_at,
            }
        })
        .collect())
}

async fn restart_google(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = match crate::services::integrations::sync_google_calendar(storage, config).await?
    {
        Some(count) => count,
        None => adapters::calendar::ingest(storage, config).await?,
    };
    Ok(format!(
        "Google Calendar ingest complete: {} signals",
        signals
    ))
}

async fn restart_todoist(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = match crate::services::integrations::sync_todoist(storage).await? {
        Some(count) => count,
        None => adapters::todoist::ingest(storage, config).await?,
    };
    Ok(format!("Todoist ingest complete: {} signals", signals))
}

async fn restart_activity(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::activity::ingest(storage, config).await?;
    Ok(format!("Activity ingest complete: {} signals", signals))
}

async fn restart_health(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::health::ingest(storage, config).await?;
    Ok(format!("Health ingest complete: {} signals", signals))
}

async fn restart_git(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::git::ingest(storage, config).await?;
    Ok(format!("Git ingest complete: {} signals", signals))
}

async fn restart_messaging(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::messaging::ingest(storage, config).await?;
    Ok(format!("Messaging ingest complete: {} signals", signals))
}

async fn restart_reminders(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::reminders::ingest(storage, config).await?;
    Ok(format!("Reminders ingest complete: {} signals", signals))
}

async fn restart_notes(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::notes::ingest(storage, config).await?;
    Ok(format!("Notes ingest complete: {} captures", signals))
}

async fn restart_transcripts(storage: &Storage, config: &AppConfig) -> Result<String, AppError> {
    let signals = adapters::transcripts::ingest(storage, config).await?;
    Ok(format!("Transcript ingest complete: {} signals", signals))
}

async fn restart_evaluate(
    storage: &Storage,
    policy_config: &PolicyConfig,
) -> Result<String, AppError> {
    let result = crate::services::evaluate::run(storage, policy_config).await?;
    Ok(format!(
        "Evaluate complete: {} states, {} nudges",
        result.inferred_states, result.nudges_created_or_updated
    ))
}

async fn restart_component_by_id(
    storage: &Storage,
    config: &AppConfig,
    policy_config: &PolicyConfig,
    component_id: &str,
) -> Result<String, AppError> {
    match component_id {
        "google-calendar" => restart_google(storage, config).await,
        "todoist" => restart_todoist(storage, config).await,
        "activity" => restart_activity(storage, config).await,
        "health" => restart_health(storage, config).await,
        "git" => restart_git(storage, config).await,
        "messaging" => restart_messaging(storage, config).await,
        "reminders" => restart_reminders(storage, config).await,
        "notes" => restart_notes(storage, config).await,
        "transcripts" => restart_transcripts(storage, config).await,
        "evaluate" => restart_evaluate(storage, policy_config).await,
        _ => Err(AppError::not_found("component not found")),
    }
}

pub async fn restart_component(
    storage: &Storage,
    config: &AppConfig,
    policy_config: &PolicyConfig,
    component_id: &str,
) -> Result<ComponentRestartResult, AppError> {
    let component_id = component_id.trim();
    let spec =
        component_spec(component_id).ok_or_else(|| AppError::not_found("component not found"))?;

    let requested_at = now_ts();
    let mut state = load_component_state(storage, component_id).await?;
    state.status = "running".to_string();
    state.last_restarted_at = Some(requested_at);
    state.last_error = None;
    state.restart_count = state.restart_count.saturating_add(1);
    save_component_state(storage, component_id, &state).await?;
    append_component_event(
        storage,
        component_id,
        "component.restart.requested",
        "running",
        "component restart requested",
        serde_json::json!({
            "requested_at": requested_at,
            "component_name": spec.name,
        }),
    )
    .await?;

    match restart_component_by_id(storage, config, policy_config, component_id).await {
        Ok(message) => {
            state.status = "ok".to_string();
            state.last_error = None;
            append_component_event(
                storage,
                component_id,
                "component.restart.completed",
                "success",
                &message,
                serde_json::json!({
                    "requested_at": requested_at,
                    "completed_at": now_ts(),
                }),
            )
            .await?;
        }
        Err(error) => {
            state.status = "error".to_string();
            state.last_error = Some(error.to_string());
            append_component_event(
                storage,
                component_id,
                "component.restart.failed",
                "error",
                &error.to_string(),
                serde_json::json!({
                    "requested_at": requested_at,
                    "failed_at": now_ts(),
                    "error": error.to_string(),
                }),
            )
            .await?;
            save_component_state(storage, component_id, &state).await?;
            return Err(error);
        }
    };

    save_component_state(storage, component_id, &state).await?;
    Ok(ComponentRestartResult {
        id: component_id.to_string(),
        name: spec.name.to_string(),
        description: spec.description.to_string(),
        status: state.status,
        last_restarted_at: state.last_restarted_at,
        last_error: state.last_error,
        restart_count: state.restart_count,
    })
}
