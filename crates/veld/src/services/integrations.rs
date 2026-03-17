use std::collections::HashSet;

use super::{
    integration_runtime::{
        canonical_integration_id, integration_log_limit, map_integration_log_event,
    },
    integrations_google, integrations_host, integrations_todoist,
};
use crate::{adapters, errors::AppError};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_api_types::{
    GoogleCalendarAuthStartData, IntegrationConnectionData, IntegrationConnectionEventData,
    IntegrationConnectionSettingRefData, IntegrationLogEventData, IntegrationsData,
};
use vel_config::AppConfig;
use vel_core::IntegrationFamily;
use vel_storage::{IntegrationConnectionFilters, SignalRecord, Storage};

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
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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

pub async fn run_notes_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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
    let runtime_config = integrations_host::runtime_local_config(storage, config).await?;
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
    if runtime_config.notes_path.is_some() {
        ingested += run_notes_sync(storage, config).await?;
    }
    if runtime_config.transcript_snapshot_path.is_some() {
        ingested += run_transcripts_sync(storage, config).await?;
    }

    Ok(ingested)
}

pub async fn get_integrations(storage: &Storage) -> Result<IntegrationsData, AppError> {
    let google = integrations_google::load_google_settings(storage).await?;
    let todoist = integrations_todoist::load_todoist_settings(storage).await?;
    let activity =
        integrations_host::load_local_settings(storage, integrations_host::ACTIVITY_SETTINGS_KEY)
            .await?;
    let health =
        integrations_host::load_local_settings(storage, integrations_host::HEALTH_SETTINGS_KEY)
            .await?;
    let git = integrations_host::load_local_settings(storage, integrations_host::GIT_SETTINGS_KEY)
        .await?;
    let messaging =
        integrations_host::load_local_settings(storage, integrations_host::MESSAGING_SETTINGS_KEY)
            .await?;
    let notes =
        integrations_host::load_local_settings(storage, integrations_host::NOTES_SETTINGS_KEY)
            .await?;
    let transcripts = integrations_host::load_local_settings(
        storage,
        integrations_host::TRANSCRIPTS_SETTINGS_KEY,
    )
    .await?;
    Ok(IntegrationsData {
        google_calendar: integrations_google::google_status(&google),
        todoist: integrations_todoist::todoist_status(&todoist),
        activity: integrations_host::local_status_data("activity", &activity, None),
        health: integrations_host::local_status_data("health", &health, None),
        git: integrations_host::local_status_data("git", &git, None),
        messaging: integrations_host::local_status_data("messaging", &messaging, None),
        notes: integrations_host::local_status_data("notes", &notes, None),
        transcripts: integrations_host::local_status_data("transcripts", &transcripts, None),
    })
}

pub async fn get_integrations_with_config(
    storage: &Storage,
    config: &AppConfig,
) -> Result<IntegrationsData, AppError> {
    let google = integrations_google::load_google_settings(storage).await?;
    let todoist = integrations_todoist::load_todoist_settings(storage).await?;
    let activity =
        integrations_host::load_local_settings(storage, integrations_host::ACTIVITY_SETTINGS_KEY)
            .await?;
    let health =
        integrations_host::load_local_settings(storage, integrations_host::HEALTH_SETTINGS_KEY)
            .await?;
    let git = integrations_host::load_local_settings(storage, integrations_host::GIT_SETTINGS_KEY)
        .await?;
    let messaging =
        integrations_host::load_local_settings(storage, integrations_host::MESSAGING_SETTINGS_KEY)
            .await?;
    let notes =
        integrations_host::load_local_settings(storage, integrations_host::NOTES_SETTINGS_KEY)
            .await?;
    let transcripts = integrations_host::load_local_settings(
        storage,
        integrations_host::TRANSCRIPTS_SETTINGS_KEY,
    )
    .await?;
    Ok(IntegrationsData {
        google_calendar: integrations_google::google_status(&google),
        todoist: integrations_todoist::todoist_status(&todoist),
        activity: integrations_host::local_status_data(
            "activity",
            &activity,
            config.activity_snapshot_path.as_deref(),
        ),
        health: integrations_host::local_status_data(
            "health",
            &health,
            config.health_snapshot_path.as_deref(),
        ),
        git: integrations_host::local_status_data("git", &git, config.git_snapshot_path.as_deref()),
        messaging: integrations_host::local_status_data(
            "messaging",
            &messaging,
            config.messaging_snapshot_path.as_deref(),
        ),
        notes: integrations_host::local_status_data("notes", &notes, config.notes_path.as_deref()),
        transcripts: integrations_host::local_status_data(
            "transcripts",
            &transcripts,
            config.transcript_snapshot_path.as_deref(),
        ),
    })
}

pub async fn google_calendar_selection_filter(
    storage: &Storage,
) -> Result<GoogleCalendarSelectionFilter, AppError> {
    let settings = integrations_google::load_google_settings(storage).await?;
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
) -> Result<Vec<IntegrationLogEventData>, AppError> {
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

pub async fn list_integration_connections(
    storage: &Storage,
    family: Option<&str>,
    provider_key: Option<&str>,
    include_disabled: bool,
) -> Result<Vec<IntegrationConnectionData>, AppError> {
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
            include_disabled,
        })
        .await?;

    let mut data = Vec::with_capacity(connections.len());
    for connection in connections {
        let setting_refs = storage
            .list_integration_connection_setting_refs(connection.id.as_ref())
            .await?;
        data.push(IntegrationConnectionData {
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
                .map(|setting_ref| IntegrationConnectionSettingRefData {
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
) -> Result<IntegrationConnectionData, AppError> {
    let connection = storage
        .get_integration_connection(connection_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("integration connection not found"))?;
    let setting_refs = storage
        .list_integration_connection_setting_refs(connection.id.as_ref())
        .await?;

    Ok(IntegrationConnectionData {
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
            .map(|setting_ref| IntegrationConnectionSettingRefData {
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
) -> Result<Vec<IntegrationConnectionEventData>, AppError> {
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
        .map(|event| IntegrationConnectionEventData {
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
) -> Result<IntegrationsData, AppError> {
    let mut settings = integrations_google::load_google_settings(storage).await?;
    integrations_google::apply_google_settings_update(
        &mut settings,
        client_id,
        client_secret,
        selected_calendar_ids,
        all_calendars_selected,
    );
    integrations_google::save_google_settings(storage, &settings).await?;
    get_integrations(storage).await
}

pub async fn update_todoist_settings(
    storage: &Storage,
    api_token: Option<String>,
) -> Result<IntegrationsData, AppError> {
    integrations_todoist::update_todoist_settings(storage, api_token).await?;
    get_integrations(storage).await
}

pub async fn update_local_source_path(
    storage: &Storage,
    source: &str,
    source_path: Option<String>,
) -> Result<IntegrationsData, AppError> {
    integrations_host::update_local_source_path(storage, source, source_path).await?;
    get_integrations(storage).await
}

pub async fn disconnect_google_calendar(storage: &Storage) -> Result<IntegrationsData, AppError> {
    let mut settings = integrations_google::load_google_settings(storage).await?;
    integrations_google::disconnect_google_calendar(&mut settings);
    integrations_google::save_google_settings(storage, &settings).await?;
    get_integrations(storage).await
}

pub async fn disconnect_todoist(storage: &Storage) -> Result<IntegrationsData, AppError> {
    integrations_todoist::disconnect_todoist(storage).await?;
    get_integrations(storage).await
}

pub async fn start_google_auth(
    storage: &Storage,
    config: &AppConfig,
) -> Result<GoogleCalendarAuthStartData, AppError> {
    let mut settings = integrations_google::load_google_settings(storage).await?;
    let auth_start = integrations_google::start_google_auth(&mut settings, config).await?;
    integrations_google::save_google_settings(storage, &settings).await?;
    Ok(auth_start)
}

pub async fn complete_google_auth(
    storage: &Storage,
    config: &AppConfig,
    state_param: &str,
    code: &str,
) -> Result<(), AppError> {
    let mut settings = integrations_google::load_google_settings(storage).await?;
    integrations_google::complete_google_auth(&mut settings, config, state_param, code).await?;
    integrations_google::save_google_settings(storage, &settings).await?;
    Ok(())
}

pub async fn sync_google_calendar(
    storage: &Storage,
    _config: &AppConfig,
) -> Result<Option<u32>, AppError> {
    let mut settings = integrations_google::load_google_settings(storage).await?;
    let result = integrations_google::sync_google_calendar(storage, &mut settings).await?;
    if result.is_some() {
        integrations_google::save_google_settings(storage, &settings).await?;
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
            let mut settings = integrations_google::load_google_settings(storage).await?;
            settings.last_sync_at = Some(now_ts());
            settings.last_sync_status = Some("error".to_string());
            settings.last_error = Some(error.to_string());
            integrations_google::save_google_settings(storage, &settings).await?;
            append_sync_event(storage, "google-calendar", "error", 0, Some(error)).await?;
        }
        "todoist" => {
            integrations_todoist::record_sync_error(storage, error).await?;
            append_sync_event(storage, "todoist", "error", 0, Some(error)).await?;
        }
        "activity" | "health" | "git" | "messaging" | "notes" | "transcripts" => {
            integrations_host::update_local_sync_settings(
                storage,
                source,
                "error",
                Some(error.to_string()),
                None,
            )
            .await?;
            append_sync_event(
                storage,
                integrations_host::local_integration_id(source),
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
        "activity" | "health" | "git" | "messaging" | "notes" | "transcripts" => {
            integrations_host::update_local_sync_settings(
                storage,
                source,
                "ok",
                None,
                Some(item_count),
            )
            .await?;
            append_sync_event(
                storage,
                integrations_host::local_integration_id(source),
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
                .get(integrations_google::GOOGLE_SETTINGS_KEY)
                .expect("google public settings should exist")
                .clone();
            let secret_settings = all_settings
                .get(integrations_google::GOOGLE_SECRETS_KEY)
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

        let settings = integrations_google::load_google_settings(&storage)
            .await
            .unwrap();
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
            messaging_snapshot_path: Some(snapshot_path.to_string_lossy().to_string()),
            ..Default::default()
        };

        let ingested = bootstrap_local_context_sources(&storage, &config)
            .await
            .expect("bootstrap should ingest configured local sources");
        assert_eq!(ingested, 1);

        let policy_config = crate::policy_config::PolicyConfig::default();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state =
            crate::state::AppState::new(storage, config, policy_config, broadcast_tx, None, None);
        crate::services::evaluate::run_and_broadcast(&state)
            .await
            .expect("evaluate should succeed after bootstrap");

        let (_, context_json) = state
            .storage
            .get_current_context()
            .await
            .unwrap()
            .expect("bootstrap + evaluate should persist current context");
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["message_waiting_on_me_count"], 1);
    }
}
