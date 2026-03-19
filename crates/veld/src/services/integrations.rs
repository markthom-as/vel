use std::collections::HashSet;

use super::{
    integration_runtime::{
        canonical_integration_id, integration_log_limit, map_integration_log_event,
    },
    integrations_google, integrations_host, integrations_todoist,
};
use crate::{adapters, errors::AppError};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{IntegrationConnectionStatus, IntegrationFamily, IntegrationProvider};
use vel_storage::{
    IntegrationConnectionFilters, IntegrationConnectionInsert, SignalRecord, Storage,
};

pub use super::integration_runtime::IntegrationLogEvent;

const GOOGLE_SETTINGS_KEY: &str = "integration_google_calendar";
const GOOGLE_SECRETS_KEY: &str = "integration_google_calendar_secrets";
const ACTIVITY_SETTINGS_KEY: &str = "integration_activity";
const HEALTH_SETTINGS_KEY: &str = "integration_health";
const GIT_SETTINGS_KEY: &str = "integration_git";
const MESSAGING_SETTINGS_KEY: &str = "integration_messaging";
const REMINDERS_SETTINGS_KEY: &str = "integration_reminders";
const NOTES_SETTINGS_KEY: &str = "integration_notes";
const TRANSCRIPTS_SETTINGS_KEY: &str = "integration_transcripts";

#[derive(Debug, Clone)]
pub struct GoogleCalendarAuthStart {
    pub auth_url: String,
}

#[derive(Debug, Clone)]
pub struct IntegrationGuidanceOutput {
    pub title: String,
    pub detail: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub struct IntegrationCalendarOutput {
    pub id: String,
    pub summary: String,
    pub primary: bool,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct GoogleCalendarIntegrationOutput {
    pub configured: bool,
    pub connected: bool,
    pub has_client_id: bool,
    pub has_client_secret: bool,
    pub calendars: Vec<IntegrationCalendarOutput>,
    pub all_calendars_selected: bool,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceOutput>,
}

#[derive(Debug, Clone)]
pub struct TodoistIntegrationOutput {
    pub configured: bool,
    pub connected: bool,
    pub has_api_token: bool,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceOutput>,
}

#[derive(Debug, Clone)]
pub struct LocalIntegrationOutput {
    pub configured: bool,
    pub guidance: Option<IntegrationGuidanceOutput>,
    pub source_path: Option<String>,
    pub selected_paths: Vec<String>,
    pub available_paths: Vec<String>,
    pub internal_paths: Vec<String>,
    pub suggested_paths: Vec<String>,
    pub source_kind: String,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct IntegrationsOutput {
    pub google_calendar: GoogleCalendarIntegrationOutput,
    pub todoist: TodoistIntegrationOutput,
    pub activity: LocalIntegrationOutput,
    pub health: LocalIntegrationOutput,
    pub git: LocalIntegrationOutput,
    pub messaging: LocalIntegrationOutput,
    pub reminders: LocalIntegrationOutput,
    pub notes: LocalIntegrationOutput,
    pub transcripts: LocalIntegrationOutput,
}

#[derive(Debug, Clone)]
pub struct IntegrationConnectionSettingRefOutput {
    pub setting_key: String,
    pub setting_value: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct IntegrationConnectionOutput {
    pub id: String,
    pub family: String,
    pub provider_key: String,
    pub status: String,
    pub display_name: String,
    pub account_ref: Option<String>,
    pub metadata: JsonValue,
    pub created_at: i64,
    pub updated_at: i64,
    pub setting_refs: Vec<IntegrationConnectionSettingRefOutput>,
}

#[derive(Debug, Clone)]
pub struct IntegrationConnectionEventOutput {
    pub id: String,
    pub connection_id: String,
    pub event_type: String,
    pub payload: JsonValue,
    pub timestamp: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarSettings {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<i64>,
    #[serde(default)]
    pub calendars: Vec<StoredCalendar>,
    #[serde(default = "default_true")]
    pub all_calendars_selected: bool,
    pub pending_oauth_state: Option<String>,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

impl Default for GoogleCalendarSettings {
    fn default() -> Self {
        Self {
            client_id: None,
            client_secret: None,
            access_token: None,
            refresh_token: None,
            token_expires_at: None,
            calendars: Vec::new(),
            all_calendars_selected: true,
            pending_oauth_state: None,
            last_sync_at: None,
            last_sync_status: None,
            last_error: None,
            last_item_count: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarPublicSettings {
    pub client_id: Option<String>,
    #[serde(default)]
    pub calendars: Vec<StoredCalendar>,
    #[serde(default = "default_true")]
    pub all_calendars_selected: bool,
    pub pending_oauth_state: Option<String>,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

impl Default for GoogleCalendarPublicSettings {
    fn default() -> Self {
        Self {
            client_id: None,
            calendars: Vec::new(),
            all_calendars_selected: true,
            pending_oauth_state: None,
            last_sync_at: None,
            last_sync_status: None,
            last_error: None,
            last_item_count: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoredCalendar {
    pub id: String,
    pub summary: String,
    #[serde(default)]
    pub primary: bool,
    #[serde(default = "default_true")]
    pub selected: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GoogleCalendarSelectionFilter {
    all_calendars_selected: bool,
    selected_calendar_ids: HashSet<String>,
}

impl GoogleCalendarSelectionFilter {
    pub fn includes_signal(&self, signal: &SignalRecord) -> bool {
        if signal.signal_type != "calendar_event" || signal.source != "google_calendar" {
            return true;
        }
        if self.all_calendars_selected {
            return true;
        }
        signal
            .payload_json
            .get("calendar_id")
            .and_then(serde_json::Value::as_str)
            .map(|calendar_id| self.selected_calendar_ids.contains(calendar_id))
            .unwrap_or(false)
    }

    pub fn has_any_selected(&self) -> bool {
        self.all_calendars_selected || !self.selected_calendar_ids.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoogleCalendarSecrets {
    pub client_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalIntegrationSettings {
    pub source_path: Option<String>,
    #[serde(default)]
    pub selected_paths: Vec<String>,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

fn default_true() -> bool {
    true
}

pub async fn run_calendar_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    match sync_google_calendar(storage, config).await {
        Ok(Some(count)) => Ok(count),
        Ok(None) => adapters::calendar::ingest(storage, config).await,
        Err(error) => {
            let _ = record_sync_error(storage, "google_calendar", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_todoist_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    match sync_todoist(storage).await {
        Ok(Some(count)) => Ok(count),
        Ok(None) => adapters::todoist::ingest(storage, config).await,
        Err(error) => {
            let _ = record_sync_error(storage, "todoist", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_activity_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::activity::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "activity", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "activity", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_health_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::health::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "health", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "health", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_git_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::git::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "git", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "git", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_messaging_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::messaging::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "messaging", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "messaging", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_reminders_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::reminders::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "reminders", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "reminders", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_notes_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::notes::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "notes", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "notes", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn run_transcripts_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    match adapters::transcripts::ingest(storage, &runtime_config).await {
        Ok(count) => {
            let _ = record_sync_success(storage, "transcripts", count).await;
            Ok(count)
        }
        Err(error) => {
            let _ = record_sync_error(storage, "transcripts", &error.to_string()).await;
            Err(error)
        }
    }
}

pub async fn bootstrap_local_context_sources(
    storage: &Storage,
    config: &AppConfig,
) -> Result<u32, AppError> {
    let runtime_config = runtime_local_config(storage, config).await?;
    let mut ingested = 0u32;

    if runtime_config.activity_snapshot_path.is_some() {
        ingested += run_activity_sync(storage, config).await?;
    }
    if runtime_config.health_snapshot_path.is_some() {
        ingested += run_health_sync(storage, config).await?;
    }
    if runtime_config.git_snapshot_path.is_some() {
        ingested += run_git_sync(storage, config).await?;
    }
    if runtime_config.messaging_snapshot_path.is_some() {
        ingested += run_messaging_sync(storage, config).await?;
    }
    if runtime_config.reminders_snapshot_path.is_some() {
        ingested += run_reminders_sync(storage, config).await?;
    }
    if runtime_config.notes_path.is_some() {
        ingested += run_notes_sync(storage, config).await?;
    }
    if runtime_config.transcript_snapshot_path.is_some() {
        ingested += run_transcripts_sync(storage, config).await?;
    }

    Ok(ingested)
}

pub async fn get_integrations(storage: &Storage) -> Result<IntegrationsOutput, AppError> {
    let google = load_google_settings(storage).await?;
    let todoist = integrations_todoist::load_todoist_settings(storage).await?;
    let activity = load_local_settings(storage, ACTIVITY_SETTINGS_KEY).await?;
    let health = load_local_settings(storage, HEALTH_SETTINGS_KEY).await?;
    let git = load_local_settings(storage, GIT_SETTINGS_KEY).await?;
    let messaging = load_local_settings(storage, MESSAGING_SETTINGS_KEY).await?;
    let reminders = load_local_settings(storage, REMINDERS_SETTINGS_KEY).await?;
    let notes = load_local_settings(storage, NOTES_SETTINGS_KEY).await?;
    let transcripts = load_local_settings(storage, TRANSCRIPTS_SETTINGS_KEY).await?;
    Ok(IntegrationsOutput {
        google_calendar: google_status(&google),
        todoist: map_todoist_status(integrations_todoist::todoist_status(&todoist)),
        activity: local_status(
            "activity",
            integrations_host::effective_local_source_path(
                "activity",
                activity.source_path.as_deref(),
                None,
            ),
            integrations_host::suggested_local_source_paths(
                "activity",
                activity.source_path.as_deref(),
                None,
            ),
            &activity,
        ),
        health: local_status(
            "health",
            integrations_host::effective_local_source_path(
                "health",
                health.source_path.as_deref(),
                None,
            ),
            integrations_host::suggested_local_source_paths(
                "health",
                health.source_path.as_deref(),
                None,
            ),
            &health,
        ),
        git: local_status(
            "git",
            integrations_host::effective_local_source_path("git", git.source_path.as_deref(), None),
            integrations_host::suggested_local_source_paths(
                "git",
                git.source_path.as_deref(),
                None,
            ),
            &git,
        ),
        messaging: local_status(
            "messaging",
            integrations_host::effective_local_source_path(
                "messaging",
                messaging.source_path.as_deref(),
                None,
            ),
            integrations_host::suggested_local_source_paths(
                "messaging",
                messaging.source_path.as_deref(),
                None,
            ),
            &messaging,
        ),
        reminders: local_status(
            "reminders",
            integrations_host::effective_local_source_path(
                "reminders",
                reminders.source_path.as_deref(),
                None,
            ),
            integrations_host::suggested_local_source_paths(
                "reminders",
                reminders.source_path.as_deref(),
                None,
            ),
            &reminders,
        ),
        notes: local_status(
            "notes",
            integrations_host::effective_local_source_path(
                "notes",
                notes.source_path.as_deref(),
                None,
            ),
            integrations_host::suggested_local_source_paths(
                "notes",
                notes.source_path.as_deref(),
                None,
            ),
            &notes,
        ),
        transcripts: local_status(
            "transcripts",
            integrations_host::effective_local_source_path(
                "transcripts",
                transcripts.source_path.as_deref(),
                None,
            ),
            integrations_host::suggested_local_source_paths(
                "transcripts",
                transcripts.source_path.as_deref(),
                None,
            ),
            &transcripts,
        ),
    })
}

pub async fn get_integrations_with_config(
    storage: &Storage,
    config: &AppConfig,
) -> Result<IntegrationsOutput, AppError> {
    let google = load_google_settings(storage).await?;
    let todoist = integrations_todoist::load_todoist_settings(storage).await?;
    let activity = load_local_settings(storage, ACTIVITY_SETTINGS_KEY).await?;
    let health = load_local_settings(storage, HEALTH_SETTINGS_KEY).await?;
    let git = load_local_settings(storage, GIT_SETTINGS_KEY).await?;
    let messaging = load_local_settings(storage, MESSAGING_SETTINGS_KEY).await?;
    let reminders = load_local_settings(storage, REMINDERS_SETTINGS_KEY).await?;
    let notes = load_local_settings(storage, NOTES_SETTINGS_KEY).await?;
    let transcripts = load_local_settings(storage, TRANSCRIPTS_SETTINGS_KEY).await?;
    Ok(IntegrationsOutput {
        google_calendar: google_status(&google),
        todoist: map_todoist_status(integrations_todoist::todoist_status(&todoist)),
        activity: local_status(
            "activity",
            integrations_host::effective_local_source_path(
                "activity",
                activity.source_path.as_deref(),
                config.activity_snapshot_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "activity",
                activity.source_path.as_deref(),
                config.activity_snapshot_path.as_deref(),
            ),
            &activity,
        ),
        health: local_status(
            "health",
            integrations_host::effective_local_source_path(
                "health",
                health.source_path.as_deref(),
                config.health_snapshot_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "health",
                health.source_path.as_deref(),
                config.health_snapshot_path.as_deref(),
            ),
            &health,
        ),
        git: local_status(
            "git",
            integrations_host::effective_local_source_path(
                "git",
                git.source_path.as_deref(),
                config.git_snapshot_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "git",
                git.source_path.as_deref(),
                config.git_snapshot_path.as_deref(),
            ),
            &git,
        ),
        messaging: local_status(
            "messaging",
            integrations_host::effective_local_source_path(
                "messaging",
                messaging.source_path.as_deref(),
                config.messaging_snapshot_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "messaging",
                messaging.source_path.as_deref(),
                config.messaging_snapshot_path.as_deref(),
            ),
            &messaging,
        ),
        reminders: local_status(
            "reminders",
            integrations_host::effective_local_source_path(
                "reminders",
                reminders.source_path.as_deref(),
                config.reminders_snapshot_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "reminders",
                reminders.source_path.as_deref(),
                config.reminders_snapshot_path.as_deref(),
            ),
            &reminders,
        ),
        notes: local_status(
            "notes",
            integrations_host::effective_local_source_path(
                "notes",
                notes.source_path.as_deref(),
                config.notes_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "notes",
                notes.source_path.as_deref(),
                config.notes_path.as_deref(),
            ),
            &notes,
        ),
        transcripts: local_status(
            "transcripts",
            integrations_host::effective_local_source_path(
                "transcripts",
                transcripts.source_path.as_deref(),
                config.transcript_snapshot_path.as_deref(),
            ),
            integrations_host::suggested_local_source_paths(
                "transcripts",
                transcripts.source_path.as_deref(),
                config.transcript_snapshot_path.as_deref(),
            ),
            &transcripts,
        ),
    })
}

pub async fn google_calendar_selection_filter(
    storage: &Storage,
) -> Result<GoogleCalendarSelectionFilter, AppError> {
    let settings = load_google_settings(storage).await?;
    Ok(GoogleCalendarSelectionFilter {
        all_calendars_selected: settings.all_calendars_selected,
        selected_calendar_ids: settings
            .calendars
            .into_iter()
            .filter(|calendar| calendar.selected)
            .map(|calendar| calendar.id)
            .collect(),
    })
}

pub async fn list_integration_logs(
    storage: &Storage,
    integration_id: &str,
    limit: Option<u32>,
) -> Result<Vec<IntegrationLogEvent>, AppError> {
    let integration_id = canonical_integration_id(integration_id)
        .ok_or_else(|| AppError::not_found("integration not found"))?;
    let events = storage
        .list_events_by_aggregate("integration", integration_id, integration_log_limit(limit))
        .await?;

    Ok(events
        .into_iter()
        .map(|event| map_integration_log_event(event, integration_id))
        .collect())
}

struct FoundationConnectionSeed {
    family: IntegrationFamily,
    provider_key: &'static str,
    display_name: &'static str,
    status: IntegrationConnectionStatus,
}

fn foundation_connection_seeds() -> &'static [FoundationConnectionSeed] {
    &[
        FoundationConnectionSeed {
            family: IntegrationFamily::Calendar,
            provider_key: "google_calendar",
            display_name: "Google Calendar",
            status: IntegrationConnectionStatus::Pending,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Tasks,
            provider_key: "todoist",
            display_name: "Todoist",
            status: IntegrationConnectionStatus::Pending,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Activity,
            provider_key: "activity",
            display_name: "Computer Activity",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Health,
            provider_key: "health",
            display_name: "Apple Health",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Git,
            provider_key: "git",
            display_name: "Git Activity",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Git,
            provider_key: "gh",
            display_name: "GitHub (gh)",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Messaging,
            provider_key: "messaging",
            display_name: "Messaging",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Tasks,
            provider_key: "reminders",
            display_name: "Apple Reminders",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Notes,
            provider_key: "notes",
            display_name: "Notes",
            status: IntegrationConnectionStatus::Connected,
        },
        FoundationConnectionSeed {
            family: IntegrationFamily::Transcripts,
            provider_key: "transcripts",
            display_name: "Transcripts",
            status: IntegrationConnectionStatus::Connected,
        },
    ]
}

async fn seed_foundation_integration_connections_if_empty(
    storage: &Storage,
) -> Result<(), AppError> {
    let existing = storage
        .list_integration_connections(IntegrationConnectionFilters {
            family: None,
            provider_key: None,
            status: None,
            include_disabled: true,
        })
        .await?;
    let mut existing_providers: std::collections::HashSet<(IntegrationFamily, String)> = existing
        .into_iter()
        .map(|connection| (connection.provider.family, connection.provider.key))
        .collect();

    for seed in foundation_connection_seeds() {
        let provider_key = seed.provider_key.to_string();
        if existing_providers.contains(&(seed.family, provider_key.clone())) {
            continue;
        }
        let provider =
            IntegrationProvider::new(seed.family, seed.provider_key).map_err(|error| {
                AppError::internal(format!("foundation integration provider: {error}"))
            })?;
        storage
            .insert_integration_connection(IntegrationConnectionInsert {
                family: seed.family,
                provider,
                status: seed.status,
                display_name: seed.display_name.to_string(),
                account_ref: None,
                metadata_json: serde_json::json!({ "foundation": true }),
            })
            .await?;
        existing_providers.insert((seed.family, provider_key));
    }

    Ok(())
}

pub async fn list_integration_connections(
    storage: &Storage,
    family: Option<&str>,
    provider_key: Option<&str>,
    include_disabled: bool,
) -> Result<Vec<IntegrationConnectionOutput>, AppError> {
    seed_foundation_integration_connections_if_empty(storage).await?;
    let family = family
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::parse::<IntegrationFamily>)
        .transpose()
        .map_err(|error| AppError::bad_request(error.to_string()))?;
    let provider_key = provider_key
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);

    let connections = storage
        .list_integration_connections(IntegrationConnectionFilters {
            family,
            provider_key,
            status: None,
            include_disabled,
        })
        .await?;

    let mut data = Vec::with_capacity(connections.len());
    for connection in connections {
        let setting_refs = storage
            .list_integration_connection_setting_refs(connection.id.as_ref())
            .await?;
        data.push(IntegrationConnectionOutput {
            id: connection.id.to_string(),
            family: connection.provider.family.to_string(),
            provider_key: connection.provider.key,
            status: connection.status.to_string(),
            display_name: connection.display_name,
            account_ref: connection.account_ref,
            metadata: connection.metadata_json,
            created_at: connection.created_at.unix_timestamp(),
            updated_at: connection.updated_at.unix_timestamp(),
            setting_refs: setting_refs
                .into_iter()
                .map(|setting_ref| IntegrationConnectionSettingRefOutput {
                    setting_key: setting_ref.setting_key,
                    setting_value: setting_ref.setting_value,
                    created_at: setting_ref.created_at.unix_timestamp(),
                })
                .collect(),
        });
    }

    Ok(data)
}

pub async fn get_integration_connection(
    storage: &Storage,
    connection_id: &str,
) -> Result<IntegrationConnectionOutput, AppError> {
    seed_foundation_integration_connections_if_empty(storage).await?;
    let connection = storage
        .get_integration_connection(connection_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("integration connection not found"))?;
    let setting_refs = storage
        .list_integration_connection_setting_refs(connection.id.as_ref())
        .await?;

    Ok(IntegrationConnectionOutput {
        id: connection.id.to_string(),
        family: connection.provider.family.to_string(),
        provider_key: connection.provider.key,
        status: connection.status.to_string(),
        display_name: connection.display_name,
        account_ref: connection.account_ref,
        metadata: connection.metadata_json,
        created_at: connection.created_at.unix_timestamp(),
        updated_at: connection.updated_at.unix_timestamp(),
        setting_refs: setting_refs
            .into_iter()
            .map(|setting_ref| IntegrationConnectionSettingRefOutput {
                setting_key: setting_ref.setting_key,
                setting_value: setting_ref.setting_value,
                created_at: setting_ref.created_at.unix_timestamp(),
            })
            .collect(),
    })
}

pub async fn list_integration_connection_events(
    storage: &Storage,
    connection_id: &str,
    limit: Option<u32>,
) -> Result<Vec<IntegrationConnectionEventOutput>, AppError> {
    seed_foundation_integration_connections_if_empty(storage).await?;
    let connection_id = connection_id.trim();
    let _ = storage
        .get_integration_connection(connection_id)
        .await?
        .ok_or_else(|| AppError::not_found("integration connection not found"))?;
    let events = storage
        .list_integration_connection_events(connection_id, limit.unwrap_or(20))
        .await?;

    Ok(events
        .into_iter()
        .map(|event| IntegrationConnectionEventOutput {
            id: event.id,
            connection_id: event.connection_id.to_string(),
            event_type: event.event_type.to_string(),
            payload: event.payload_json,
            timestamp: event.timestamp.unix_timestamp(),
            created_at: event.created_at.unix_timestamp(),
        })
        .collect())
}

pub async fn update_google_settings(
    storage: &Storage,
    client_id: Option<String>,
    client_secret: Option<String>,
    selected_calendar_ids: Option<Vec<String>>,
    all_calendars_selected: Option<bool>,
) -> Result<IntegrationsOutput, AppError> {
    let mut settings = load_google_settings(storage).await?;

    if let Some(value) = client_id {
        settings.client_id = normalize_optional(value);
    }
    if let Some(value) = client_secret {
        settings.client_secret = normalize_optional(value);
    }
    if let Some(all_selected) = all_calendars_selected {
        settings.all_calendars_selected = all_selected;
        if all_selected {
            for calendar in &mut settings.calendars {
                calendar.selected = true;
            }
        }
    }
    if let Some(ids) = selected_calendar_ids {
        let selected = ids.into_iter().collect::<std::collections::HashSet<_>>();
        for calendar in &mut settings.calendars {
            calendar.selected = selected.contains(&calendar.id);
        }
        settings.all_calendars_selected = false;
    }

    save_google_settings(storage, &settings).await?;
    get_integrations(storage).await
}

pub async fn update_todoist_settings(
    storage: &Storage,
    api_token: Option<String>,
) -> Result<IntegrationsOutput, AppError> {
    integrations_todoist::update_todoist_settings(storage, api_token).await?;
    get_integrations(storage).await
}

pub async fn update_local_source_path(
    storage: &Storage,
    source: &str,
    source_path: Option<String>,
    selected_paths: Option<Vec<String>>,
) -> Result<IntegrationsOutput, AppError> {
    let key = local_settings_key(source);
    if key.is_empty() {
        return Err(AppError::not_found("integration not found"));
    }

    let mut settings = load_local_settings(storage, key).await?;
    settings.source_path = normalize_optional(source_path.unwrap_or_default());
    if let Some(selected_paths) = selected_paths {
        settings.selected_paths = selected_paths
            .into_iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect();
    }
    save_settings(storage, key, &settings).await?;
    get_integrations(storage).await
}

pub async fn choose_local_source_path(source: &str) -> Result<Option<String>, AppError> {
    if local_settings_key(source).is_empty() {
        return Err(AppError::not_found("integration not found"));
    }
    integrations_host::choose_local_source_path(source).await
}

pub async fn disconnect_google_calendar(storage: &Storage) -> Result<IntegrationsOutput, AppError> {
    let mut settings = load_google_settings(storage).await?;
    settings.access_token = None;
    settings.refresh_token = None;
    settings.token_expires_at = None;
    settings.pending_oauth_state = None;
    settings.last_sync_status = Some("disconnected".to_string());
    settings.last_error = None;
    save_google_settings(storage, &settings).await?;
    get_integrations(storage).await
}

pub async fn disconnect_todoist(storage: &Storage) -> Result<IntegrationsOutput, AppError> {
    integrations_todoist::disconnect_todoist(storage).await?;
    get_integrations(storage).await
}

pub async fn start_google_auth(
    storage: &Storage,
    config: &AppConfig,
) -> Result<GoogleCalendarAuthStart, AppError> {
    let mut settings = load_google_settings(storage).await?;
    let auth_start = integrations_google::start_google_auth_local(&mut settings, config).await?;
    save_google_settings(storage, &settings).await?;
    Ok(GoogleCalendarAuthStart {
        auth_url: auth_start.auth_url,
    })
}

pub async fn complete_google_auth(
    storage: &Storage,
    config: &AppConfig,
    state_param: &str,
    code: &str,
) -> Result<(), AppError> {
    let mut settings = load_google_settings(storage).await?;
    integrations_google::complete_google_auth(&mut settings, config, state_param, code).await?;
    save_google_settings(storage, &settings).await?;
    Ok(())
}

pub async fn sync_google_calendar(
    storage: &Storage,
    _config: &AppConfig,
) -> Result<Option<u32>, AppError> {
    let mut settings = load_google_settings(storage).await?;
    let result = integrations_google::sync_google_calendar(storage, &mut settings).await?;
    if result.is_some() {
        save_google_settings(storage, &settings).await?;
    }
    if let Some(inserted) = result {
        append_sync_event(storage, "google-calendar", "ok", inserted, None).await?;
    }
    Ok(result)
}

pub async fn sync_todoist(storage: &Storage) -> Result<Option<u32>, AppError> {
    let result = integrations_todoist::sync_todoist(storage).await?;
    if let Some(signals_count) = result {
        append_sync_event(storage, "todoist", "ok", signals_count, None).await?;
    }
    Ok(result)
}

pub async fn record_sync_error(
    storage: &Storage,
    source: &str,
    error: &str,
) -> Result<(), AppError> {
    match source {
        "google_calendar" => {
            let mut settings = load_google_settings(storage).await?;
            settings.last_sync_at = Some(now_ts());
            settings.last_sync_status = Some("error".to_string());
            settings.last_error = Some(error.to_string());
            save_google_settings(storage, &settings).await?;
            append_sync_event(storage, "google-calendar", "error", 0, Some(error)).await?;
        }
        "todoist" => {
            integrations_todoist::record_sync_error(storage, error).await?;
            append_sync_event(storage, "todoist", "error", 0, Some(error)).await?;
        }
        "activity" | "health" | "git" | "messaging" | "reminders" | "notes" | "transcripts" => {
            update_local_sync_settings(
                storage,
                local_settings_key(source),
                "error",
                Some(error.to_string()),
                None,
            )
            .await?;
            append_sync_event(
                storage,
                local_integration_id(source),
                "error",
                0,
                Some(error),
            )
            .await?;
        }
        _ => {}
    }
    Ok(())
}

pub async fn record_sync_success(
    storage: &Storage,
    source: &str,
    item_count: u32,
) -> Result<(), AppError> {
    match source {
        "activity" | "health" | "git" | "messaging" | "reminders" | "notes" | "transcripts" => {
            update_local_sync_settings(
                storage,
                local_settings_key(source),
                "ok",
                None,
                Some(item_count),
            )
            .await?;
            append_sync_event(
                storage,
                local_integration_id(source),
                "ok",
                item_count,
                None,
            )
            .await?;
        }
        _ => {}
    }
    Ok(())
}

fn google_status(settings: &GoogleCalendarSettings) -> GoogleCalendarIntegrationOutput {
    GoogleCalendarIntegrationOutput {
        configured: settings.client_id.is_some() && settings.client_secret.is_some(),
        connected: settings.refresh_token.is_some(),
        has_client_id: settings.client_id.is_some(),
        has_client_secret: settings.client_secret.is_some(),
        calendars: settings
            .calendars
            .iter()
            .map(|calendar| IntegrationCalendarOutput {
                id: calendar.id.clone(),
                summary: calendar.summary.clone(),
                primary: calendar.primary,
                selected: settings.all_calendars_selected || calendar.selected,
            })
            .collect(),
        all_calendars_selected: settings.all_calendars_selected,
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
        guidance: google_guidance(settings),
    }
}

fn local_status(
    integration_id: &str,
    source_path: Option<String>,
    path_suggestions: integrations_host::LocalSourcePathSuggestions,
    settings: &LocalIntegrationSettings,
) -> LocalIntegrationOutput {
    LocalIntegrationOutput {
        configured: source_path.is_some(),
        guidance: local_guidance(integration_id, source_path.as_deref(), settings),
        source_path,
        selected_paths: settings.selected_paths.clone(),
        available_paths: path_suggestions.available_paths,
        internal_paths: path_suggestions.internal_paths,
        suggested_paths: path_suggestions.suggested_paths,
        source_kind: match integrations_host::local_source_path_kind(integration_id) {
            Some(integrations_host::LocalSourcePathKind::Directory) => "directory".to_string(),
            Some(integrations_host::LocalSourcePathKind::File) => "file".to_string(),
            None => "path".to_string(),
        },
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
    }
}

fn map_todoist_status(status: integrations_todoist::TodoistStatus) -> TodoistIntegrationOutput {
    TodoistIntegrationOutput {
        configured: status.configured,
        connected: status.connected,
        has_api_token: status.has_api_token,
        last_sync_at: status.last_sync_at,
        last_sync_status: status.last_sync_status,
        last_error: status.last_error,
        last_item_count: status.last_item_count,
        guidance: status.guidance.map(|guidance| IntegrationGuidanceOutput {
            title: guidance.title,
            detail: guidance.detail,
            action: guidance.action,
        }),
    }
}

fn guidance(title: &str, detail: String, action: &str) -> IntegrationGuidanceOutput {
    IntegrationGuidanceOutput {
        title: title.to_string(),
        detail,
        action: action.to_string(),
    }
}

fn google_guidance(settings: &GoogleCalendarSettings) -> Option<IntegrationGuidanceOutput> {
    if settings.client_id.is_none() || settings.client_secret.is_none() {
        return Some(guidance(
            "Calendar credentials missing",
            "Add a Google client ID and client secret in Settings before attempting sync."
                .to_string(),
            "Save credentials",
        ));
    }
    if settings.refresh_token.is_none() {
        return Some(guidance(
            "Calendar not connected",
            "Start the Google OAuth flow from Settings, then run sync.".to_string(),
            "Connect Google",
        ));
    }
    if settings.last_sync_status.as_deref() == Some("error") {
        return Some(guidance(
            "Calendar sync failed",
            settings
                .last_error
                .clone()
                .unwrap_or_else(|| "Google Calendar sync last failed.".to_string()),
            "Inspect history and retry sync",
        ));
    }
    if settings.last_sync_at.is_none() {
        return Some(guidance(
            "Calendar has not synced yet",
            "Run a calendar sync to populate upcoming events and prep/commute context.".to_string(),
            "Sync now",
        ));
    }
    None
}

fn local_guidance(
    integration_id: &str,
    source_path: Option<&str>,
    settings: &LocalIntegrationSettings,
) -> Option<IntegrationGuidanceOutput> {
    if integration_id == "git" && !settings.selected_paths.is_empty() {
        return None;
    }
    if source_path.is_none() {
        return Some(guidance(
            "Local source missing",
            "Configure a source path for this local adapter before syncing it.".to_string(),
            "Set source path",
        ));
    }
    if settings.last_sync_status.as_deref() == Some("error") {
        return Some(guidance(
            "Local sync failed",
            settings
                .last_error
                .clone()
                .unwrap_or_else(|| "The last local sync failed.".to_string()),
            "Fix the source and retry sync",
        ));
    }
    if settings.last_sync_at.is_none() {
        return Some(guidance(
            "Local source has not synced yet",
            "Run sync now to ingest this local source into Vel.".to_string(),
            "Sync now",
        ));
    }
    None
}

async fn load_google_settings(storage: &Storage) -> Result<GoogleCalendarSettings, AppError> {
    let public_settings: GoogleCalendarPublicSettings =
        load_settings(storage, GOOGLE_SETTINGS_KEY).await?;
    let secrets: GoogleCalendarSecrets = load_settings(storage, GOOGLE_SECRETS_KEY).await?;
    Ok(GoogleCalendarSettings {
        client_id: public_settings.client_id,
        client_secret: secrets.client_secret,
        access_token: secrets.access_token,
        refresh_token: secrets.refresh_token,
        token_expires_at: secrets.token_expires_at,
        calendars: public_settings.calendars,
        all_calendars_selected: public_settings.all_calendars_selected,
        pending_oauth_state: public_settings.pending_oauth_state,
        last_sync_at: public_settings.last_sync_at,
        last_sync_status: public_settings.last_sync_status,
        last_error: public_settings.last_error,
        last_item_count: public_settings.last_item_count,
    })
}

async fn save_google_settings(
    storage: &Storage,
    settings: &GoogleCalendarSettings,
) -> Result<(), AppError> {
    let public_settings = GoogleCalendarPublicSettings {
        client_id: settings.client_id.clone(),
        calendars: settings.calendars.clone(),
        all_calendars_selected: settings.all_calendars_selected,
        pending_oauth_state: settings.pending_oauth_state.clone(),
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
    };
    let secrets = GoogleCalendarSecrets {
        client_secret: settings.client_secret.clone(),
        access_token: settings.access_token.clone(),
        refresh_token: settings.refresh_token.clone(),
        token_expires_at: settings.token_expires_at,
    };
    save_settings(storage, GOOGLE_SETTINGS_KEY, &public_settings).await?;
    save_settings(storage, GOOGLE_SECRETS_KEY, &secrets).await
}

async fn load_local_settings(
    storage: &Storage,
    key: &str,
) -> Result<LocalIntegrationSettings, AppError> {
    load_settings(storage, key).await
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

async fn update_local_sync_settings(
    storage: &Storage,
    key: &str,
    status: &str,
    error: Option<String>,
    item_count: Option<u32>,
) -> Result<(), AppError> {
    let mut settings = load_local_settings(storage, key).await?;
    settings.last_sync_at = Some(now_ts());
    settings.last_sync_status = Some(status.to_string());
    settings.last_error = error;
    settings.last_item_count = item_count;
    save_settings(storage, key, &settings).await
}

async fn append_sync_event(
    storage: &Storage,
    integration_id: &str,
    status: &str,
    item_count: u32,
    error: Option<&str>,
) -> Result<(), AppError> {
    storage
        .append_event(vel_storage::EventLogInsert {
            id: None,
            event_name: match status {
                "ok" => "integration.sync.succeeded".to_string(),
                "error" => "integration.sync.failed".to_string(),
                other => format!("integration.sync.{other}"),
            },
            aggregate_type: Some("integration".to_string()),
            aggregate_id: Some(integration_id.to_string()),
            payload_json: serde_json::json!({
                "integration_id": integration_id,
                "status": status,
                "item_count": item_count,
                "error": error,
            })
            .to_string(),
        })
        .await?;
    Ok(())
}

fn local_settings_key(source: &str) -> &'static str {
    match source {
        "activity" => ACTIVITY_SETTINGS_KEY,
        "health" => HEALTH_SETTINGS_KEY,
        "git" => GIT_SETTINGS_KEY,
        "messaging" => MESSAGING_SETTINGS_KEY,
        "reminders" => REMINDERS_SETTINGS_KEY,
        "notes" => NOTES_SETTINGS_KEY,
        "transcripts" => TRANSCRIPTS_SETTINGS_KEY,
        _ => "",
    }
}

fn local_integration_id(source: &str) -> &'static str {
    match source {
        "activity" => "activity",
        "health" => "health",
        "git" => "git",
        "messaging" => "messaging",
        "reminders" => "reminders",
        "notes" => "notes",
        "transcripts" => "transcripts",
        _ => "",
    }
}

async fn runtime_local_config(
    storage: &Storage,
    config: &AppConfig,
) -> Result<AppConfig, AppError> {
    let activity = load_local_settings(storage, ACTIVITY_SETTINGS_KEY).await?;
    let health = load_local_settings(storage, HEALTH_SETTINGS_KEY).await?;
    let git = load_local_settings(storage, GIT_SETTINGS_KEY).await?;
    let messaging = load_local_settings(storage, MESSAGING_SETTINGS_KEY).await?;
    let reminders = load_local_settings(storage, REMINDERS_SETTINGS_KEY).await?;
    let notes = load_local_settings(storage, NOTES_SETTINGS_KEY).await?;
    let transcripts = load_local_settings(storage, TRANSCRIPTS_SETTINGS_KEY).await?;

    let mut runtime = config.clone();
    runtime.activity_snapshot_path = integrations_host::effective_local_source_path(
        "activity",
        activity.source_path.as_deref(),
        config.activity_snapshot_path.as_deref(),
    );
    runtime.health_snapshot_path = integrations_host::effective_local_source_path(
        "health",
        health.source_path.as_deref(),
        config.health_snapshot_path.as_deref(),
    );
    runtime.git_snapshot_path = integrations_host::effective_local_source_path(
        "git",
        git.source_path.as_deref(),
        config.git_snapshot_path.as_deref(),
    );
    runtime.messaging_snapshot_path = integrations_host::effective_local_source_path(
        "messaging",
        messaging.source_path.as_deref(),
        config.messaging_snapshot_path.as_deref(),
    );
    runtime.reminders_snapshot_path = integrations_host::effective_local_source_path(
        "reminders",
        reminders.source_path.as_deref(),
        config.reminders_snapshot_path.as_deref(),
    );
    runtime.notes_path = integrations_host::effective_local_source_path(
        "notes",
        notes.source_path.as_deref(),
        config.notes_path.as_deref(),
    );
    runtime.transcript_snapshot_path = integrations_host::effective_local_source_path(
        "transcripts",
        transcripts.source_path.as_deref(),
        config.transcript_snapshot_path.as_deref(),
    );
    integrations_host::sanitize_missing_default_local_sources(&mut runtime);
    Ok(runtime)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_sqlite_path(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("vel-{prefix}-{nanos}.sqlite"))
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("vel-{prefix}-{nanos}"));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[tokio::test]
    async fn google_credentials_persist_across_storage_reopen() {
        let db_path = unique_sqlite_path("google-creds");
        let db_path_string = db_path.to_string_lossy().to_string();

        {
            let storage = Storage::connect(&db_path_string).await.unwrap();
            storage.migrate().await.unwrap();

            update_google_settings(
                &storage,
                Some("google-client-id".to_string()),
                Some("google-client-secret".to_string()),
                None,
                None,
            )
            .await
            .unwrap();
        }

        {
            let storage = Storage::connect(&db_path_string).await.unwrap();
            storage.migrate().await.unwrap();

            let integrations = get_integrations(&storage).await.unwrap();
            assert!(integrations.google_calendar.has_client_id);
            assert!(integrations.google_calendar.has_client_secret);

            let all_settings = storage.get_all_settings().await.unwrap();
            let public_settings = all_settings
                .get(GOOGLE_SETTINGS_KEY)
                .expect("google public settings should exist")
                .clone();
            let secret_settings = all_settings
                .get(GOOGLE_SECRETS_KEY)
                .expect("google secret settings should exist")
                .clone();

            assert_eq!(
                public_settings.get("client_id").and_then(|v| v.as_str()),
                Some("google-client-id")
            );
            assert_eq!(
                public_settings.get("client_secret"),
                None,
                "public google settings must not contain the client secret"
            );
            assert_eq!(
                secret_settings
                    .get("client_secret")
                    .and_then(|v| v.as_str()),
                Some("google-client-secret")
            );
        }

        let _ = std::fs::remove_file(&db_path);
    }

    #[tokio::test]
    async fn google_calendar_selection_defaults_to_all_selected_when_unconfigured() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let settings = load_google_settings(&storage).await.unwrap();
        assert!(
            settings.all_calendars_selected,
            "unconfigured google calendar settings should default to all calendars selected"
        );

        let filter = google_calendar_selection_filter(&storage).await.unwrap();
        let signal = SignalRecord {
            signal_id: "sig_test".to_string(),
            signal_type: "calendar_event".to_string(),
            source: "google_calendar".to_string(),
            source_ref: None,
            timestamp: 1_700_000_000,
            payload_json: json!({
                "title": "Design review",
                "start": 1_700_003_600
            }),
            created_at: 1_700_000_000,
        };
        assert!(
            filter.includes_signal(&signal),
            "google calendar signals should not be hidden before selection state exists"
        );
    }

    #[tokio::test]
    async fn todoist_token_persists_across_storage_reopen() {
        let db_path = unique_sqlite_path("todoist-creds");
        let db_path_string = db_path.to_string_lossy().to_string();

        {
            let storage = Storage::connect(&db_path_string).await.unwrap();
            storage.migrate().await.unwrap();

            update_todoist_settings(&storage, Some("todoist-secret-token".to_string()))
                .await
                .unwrap();
        }

        {
            let storage = Storage::connect(&db_path_string).await.unwrap();
            storage.migrate().await.unwrap();

            let integrations = get_integrations(&storage).await.unwrap();
            assert!(integrations.todoist.has_api_token);

            let all_settings = storage.get_all_settings().await.unwrap();
            let public_settings = all_settings
                .get(integrations_todoist::TODOIST_SETTINGS_KEY)
                .expect("todoist public settings should exist")
                .clone();
            let secret_settings = all_settings
                .get(integrations_todoist::TODOIST_SECRETS_KEY)
                .expect("todoist secret settings should exist")
                .clone();

            assert_eq!(
                public_settings.get("api_token"),
                None,
                "public todoist settings must not contain the API token"
            );
            assert_eq!(
                secret_settings.get("api_token").and_then(|v| v.as_str()),
                Some("todoist-secret-token")
            );
        }

        let _ = std::fs::remove_file(&db_path);
    }

    #[test]
    fn auto_discovers_macos_messaging_snapshot_from_home_dir() {
        let home = unique_temp_dir("mac-home");
        let snapshot_path = home.join("Library/Application Support/Vel/messages/snapshot.json");
        std::fs::create_dir_all(snapshot_path.parent().unwrap()).unwrap();
        std::fs::write(&snapshot_path, "{}").unwrap();

        let discovered = integrations_host::auto_local_source_path_from_home("messaging", &home)
            .expect("messaging snapshot should be discovered");
        assert_eq!(discovered, snapshot_path.to_string_lossy());
    }

    #[tokio::test]
    async fn bootstrap_local_context_sources_ingests_and_updates_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        let snapshot_path = unique_temp_dir("bootstrap-messaging").join("messaging.json");
        let snapshot = serde_json::json!({
            "source": "messages",
            "account_id": "local-default",
            "threads": [
                {
                    "thread_id": "thr_bootstrap",
                    "platform": "imessage",
                    "title": "Dinner plan",
                    "participants": [
                        { "id": "me", "name": "Me", "is_me": true },
                        { "id": "+15551234567", "name": "Sam", "is_me": false }
                    ],
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": false,
                    "summary": "Need to answer about dinner.",
                    "snippet": "Can we do 7?"
                }
            ]
        });
        std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = AppConfig {
            activity_snapshot_path: None,
            health_snapshot_path: None,
            git_snapshot_path: None,
            messaging_snapshot_path: Some(snapshot_path.to_string_lossy().to_string()),
            reminders_snapshot_path: None,
            notes_path: None,
            transcript_snapshot_path: None,
            ..Default::default()
        };

        let ingested = bootstrap_local_context_sources(&storage, &config)
            .await
            .expect("bootstrap should ingest configured local sources");
        // Auto-discovery may surface additional local sources present on the host.
        // This test only requires that the configured messaging source is ingested.
        assert!(ingested >= 1);

        let policy_config = crate::policy_config::PolicyConfig::default();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state =
            crate::state::AppState::new(storage, config, policy_config, broadcast_tx, None, None);
        crate::services::evaluate::run_and_broadcast(&state)
            .await
            .expect("evaluate should succeed after bootstrap");

        let (_, context) = state
            .storage
            .get_current_context()
            .await
            .unwrap()
            .expect("bootstrap + evaluate should persist current context");
        assert_eq!(context.message_waiting_on_me_count, Some(1));
    }

    #[tokio::test]
    async fn list_integration_connections_seeds_foundation_connectors_when_empty() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let connections = list_integration_connections(&storage, None, None, true)
            .await
            .expect("listing connections should succeed");
        assert!(!connections.is_empty());
        let provider_keys: Vec<&str> = connections
            .iter()
            .map(|connection| connection.provider_key.as_str())
            .collect();
        assert!(provider_keys.contains(&"activity"));
        assert!(provider_keys.contains(&"git"));
        assert!(provider_keys.contains(&"gh"));
        assert!(provider_keys.contains(&"health"));
        assert!(provider_keys.contains(&"messaging"));
        assert!(provider_keys.contains(&"reminders"));
        assert!(provider_keys.contains(&"notes"));
        assert!(provider_keys.contains(&"transcripts"));
    }

    #[tokio::test]
    async fn list_integration_connections_backfills_missing_foundation_connectors() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        storage
            .insert_integration_connection(IntegrationConnectionInsert {
                family: IntegrationFamily::Messaging,
                provider: IntegrationProvider::new(IntegrationFamily::Messaging, "signal").unwrap(),
                status: IntegrationConnectionStatus::Connected,
                display_name: "Signal".to_string(),
                account_ref: None,
                metadata_json: serde_json::json!({}),
            })
            .await
            .expect("custom connector insert should succeed");

        let connections = list_integration_connections(&storage, None, None, true)
            .await
            .expect("listing connections should succeed");
        assert!(!connections.is_empty());

        let provider_keys: Vec<&str> = connections
            .iter()
            .map(|connection| connection.provider_key.as_str())
            .collect();
        assert!(provider_keys.contains(&"signal"));
        assert!(provider_keys.contains(&"git"));
        assert!(provider_keys.contains(&"gh"));
    }

    #[tokio::test]
    async fn load_todoist_settings_returns_default_when_key_missing() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        // No key written — load_todoist_settings must return Default, no panic
        let settings = integrations_todoist::load_todoist_settings(&storage)
            .await
            .expect("load should succeed even with no settings written");
        assert!(
            settings.api_token.is_none(),
            "unconfigured storage should have no api_token"
        );
        assert!(
            settings.last_sync_at.is_none(),
            "unconfigured storage should have no last_sync_at"
        );
    }

    #[tokio::test]
    async fn load_todoist_settings_returns_default_when_json_corrupt() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        // Write a JSON value that cannot deserialize into TodoistPublicSettings (wrong shape)
        let corrupt = serde_json::json!("this-is-a-string-not-an-object");
        storage
            .set_setting(integrations_todoist::TODOIST_SETTINGS_KEY, &corrupt)
            .await
            .unwrap();

        // Must succeed (Ok) and return Default — no panic
        let result = integrations_todoist::load_todoist_settings(&storage).await;
        assert!(
            result.is_ok(),
            "corrupt settings must not propagate an error: {:?}",
            result.err()
        );
        let settings = result.unwrap();
        assert!(
            settings.api_token.is_none(),
            "corrupt settings must fall back to api_token=None"
        );
    }
}
