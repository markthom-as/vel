use std::collections::HashMap;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use uuid::Uuid;
use vel_api_types::{
    GoogleCalendarAuthStartData, GoogleCalendarIntegrationData, IntegrationCalendarData,
    IntegrationLogEventData, IntegrationsData, LocalIntegrationData, TodoistIntegrationData,
};
use vel_config::AppConfig;
use vel_core::{Commitment, CommitmentStatus};
use vel_storage::{CommitmentInsert, SignalInsert, Storage};

use crate::{adapters, errors::AppError};

const GOOGLE_SETTINGS_KEY: &str = "integration_google_calendar";
const GOOGLE_SECRETS_KEY: &str = "integration_google_calendar_secrets";
const TODOIST_SETTINGS_KEY: &str = "integration_todoist";
const TODOIST_SECRETS_KEY: &str = "integration_todoist_secrets";
const ACTIVITY_SETTINGS_KEY: &str = "integration_activity";
const GIT_SETTINGS_KEY: &str = "integration_git";
const MESSAGING_SETTINGS_KEY: &str = "integration_messaging";
const NOTES_SETTINGS_KEY: &str = "integration_notes";
const TRANSCRIPTS_SETTINGS_KEY: &str = "integration_transcripts";
const GOOGLE_AUTH_BASE: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_CALENDAR_BASE: &str = "https://www.googleapis.com/calendar/v3";
const GOOGLE_LOOKBACK_DAYS: i64 = 60;
const GOOGLE_LOOKAHEAD_DAYS: i64 = 180;
const INTEGRATION_LOG_LIMIT_DEFAULT: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoredCalendar {
    pub id: String,
    pub summary: String,
    #[serde(default)]
    pub primary: bool,
    #[serde(default = "default_true")]
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoistSettings {
    pub api_token: Option<String>,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoistPublicSettings {
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoogleCalendarSecrets {
    pub client_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoistSecrets {
    pub api_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalIntegrationSettings {
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

fn default_true() -> bool {
    true
}

fn integration_log_status(event_name: &str) -> &'static str {
    match event_name {
        "integration.sync.succeeded" => "ok",
        "integration.sync.failed" => "error",
        _ => "info",
    }
}

fn integration_log_message(event_name: &str, payload: &serde_json::Value) -> String {
    let item_count = payload
        .get("item_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let integration_id = payload
        .get("integration_id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("integration");
    match event_name {
        "integration.sync.succeeded" => {
            format!("{integration_id} sync completed with {item_count} items.")
        }
        "integration.sync.failed" => {
            let error = payload
                .get("error")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown error");
            format!("{integration_id} sync failed: {error}")
        }
        _ => event_name.to_string(),
    }
}

fn canonical_integration_id(integration_id: &str) -> Option<&'static str> {
    match integration_id {
        "google-calendar" | "calendar" | "google_calendar" => Some("google-calendar"),
        "todoist" => Some("todoist"),
        "activity" => Some("activity"),
        "git" => Some("git"),
        "messaging" => Some("messaging"),
        "notes" => Some("notes"),
        "transcripts" => Some("transcripts"),
        _ => None,
    }
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
    match adapters::activity::ingest(storage, config).await {
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

pub async fn run_messaging_sync(storage: &Storage, config: &AppConfig) -> Result<u32, AppError> {
    match adapters::messaging::ingest(storage, config).await {
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

pub async fn get_integrations(storage: &Storage) -> Result<IntegrationsData, AppError> {
    let google = load_google_settings(storage).await?;
    let todoist = load_todoist_settings(storage).await?;
    let activity = load_local_settings(storage, ACTIVITY_SETTINGS_KEY).await?;
    let git = load_local_settings(storage, GIT_SETTINGS_KEY).await?;
    let messaging = load_local_settings(storage, MESSAGING_SETTINGS_KEY).await?;
    let notes = load_local_settings(storage, NOTES_SETTINGS_KEY).await?;
    let transcripts = load_local_settings(storage, TRANSCRIPTS_SETTINGS_KEY).await?;
    Ok(IntegrationsData {
        google_calendar: google_status(&google),
        todoist: todoist_status(&todoist),
        activity: local_status(config_source_path(None, None), &activity),
        git: local_status(config_source_path(None, None), &git),
        messaging: local_status(config_source_path(None, None), &messaging),
        notes: local_status(config_source_path(None, None), &notes),
        transcripts: local_status(config_source_path(None, None), &transcripts),
    })
}

pub async fn get_integrations_with_config(
    storage: &Storage,
    config: &AppConfig,
) -> Result<IntegrationsData, AppError> {
    let google = load_google_settings(storage).await?;
    let todoist = load_todoist_settings(storage).await?;
    let activity = load_local_settings(storage, ACTIVITY_SETTINGS_KEY).await?;
    let git = load_local_settings(storage, GIT_SETTINGS_KEY).await?;
    let messaging = load_local_settings(storage, MESSAGING_SETTINGS_KEY).await?;
    let notes = load_local_settings(storage, NOTES_SETTINGS_KEY).await?;
    let transcripts = load_local_settings(storage, TRANSCRIPTS_SETTINGS_KEY).await?;
    Ok(IntegrationsData {
        google_calendar: google_status(&google),
        todoist: todoist_status(&todoist),
        activity: local_status(
            config_source_path(config.activity_snapshot_path.as_deref(), None),
            &activity,
        ),
        git: local_status(
            config_source_path(config.git_snapshot_path.as_deref(), None),
            &git,
        ),
        messaging: local_status(
            config_source_path(config.messaging_snapshot_path.as_deref(), None),
            &messaging,
        ),
        notes: local_status(
            config_source_path(config.notes_path.as_deref(), None),
            &notes,
        ),
        transcripts: local_status(
            config_source_path(config.transcript_snapshot_path.as_deref(), None),
            &transcripts,
        ),
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
        .list_events_by_aggregate(
            "integration",
            integration_id,
            limit.unwrap_or(INTEGRATION_LOG_LIMIT_DEFAULT),
        )
        .await?;

    Ok(events
        .into_iter()
        .map(|event| {
            let payload =
                serde_json::from_str(&event.payload_json).unwrap_or_else(|_| serde_json::json!({}));
            let event_name = event.event_name;
            let status = payload
                .get("status")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_else(|| integration_log_status(&event_name))
                .to_string();
            IntegrationLogEventData {
                id: event.id.to_string(),
                integration_id: event
                    .aggregate_id
                    .unwrap_or_else(|| integration_id.to_string()),
                event_name: event_name.clone(),
                status: status.clone(),
                message: integration_log_message(&event_name, &payload),
                payload,
                created_at: event.created_at,
            }
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
) -> Result<IntegrationsData, AppError> {
    let mut settings = load_todoist_settings(storage).await?;
    if let Some(value) = api_token {
        settings.api_token = normalize_optional(value);
    }
    save_todoist_settings(storage, &settings).await?;
    get_integrations(storage).await
}

pub async fn disconnect_google_calendar(storage: &Storage) -> Result<IntegrationsData, AppError> {
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

pub async fn disconnect_todoist(storage: &Storage) -> Result<IntegrationsData, AppError> {
    let mut settings = load_todoist_settings(storage).await?;
    settings.api_token = None;
    settings.last_sync_status = Some("disconnected".to_string());
    settings.last_error = None;
    save_todoist_settings(storage, &settings).await?;
    get_integrations(storage).await
}

pub async fn start_google_auth(
    storage: &Storage,
    config: &AppConfig,
) -> Result<GoogleCalendarAuthStartData, AppError> {
    let mut settings = load_google_settings(storage).await?;
    let client_id = settings
        .client_id
        .as_deref()
        .ok_or_else(|| AppError::bad_request("google calendar client_id is required"))?;

    if settings.client_secret.as_deref().unwrap_or("").is_empty() {
        return Err(AppError::bad_request(
            "google calendar client_secret is required",
        ));
    }

    let oauth_state = format!("gcal_{}", Uuid::new_v4().simple());
    settings.pending_oauth_state = Some(oauth_state.clone());
    save_google_settings(storage, &settings).await?;

    let redirect_uri = google_redirect_uri(config)?;
    let auth_url = Url::parse_with_params(
        GOOGLE_AUTH_BASE,
        [
            ("client_id", client_id),
            ("redirect_uri", redirect_uri.as_str()),
            ("response_type", "code"),
            ("access_type", "offline"),
            ("prompt", "consent"),
            ("scope", "https://www.googleapis.com/auth/calendar.readonly"),
            ("state", oauth_state.as_str()),
        ],
    )
    .map_err(|error| AppError::internal(format!("google auth url: {}", error)))?;

    Ok(GoogleCalendarAuthStartData {
        auth_url: auth_url.to_string(),
    })
}

pub async fn complete_google_auth(
    storage: &Storage,
    config: &AppConfig,
    state_param: &str,
    code: &str,
) -> Result<(), AppError> {
    let mut settings = load_google_settings(storage).await?;
    let pending_state = settings
        .pending_oauth_state
        .clone()
        .ok_or_else(|| AppError::bad_request("no pending google oauth flow"))?;
    if pending_state != state_param {
        return Err(AppError::bad_request("google oauth state mismatch"));
    }

    let client_id = settings
        .client_id
        .clone()
        .ok_or_else(|| AppError::bad_request("google calendar client_id is required"))?;
    let client_secret = settings
        .client_secret
        .clone()
        .ok_or_else(|| AppError::bad_request("google calendar client_secret is required"))?;
    let redirect_uri = google_redirect_uri(config)?;

    let token: GoogleTokenResponse = reqwest::Client::new()
        .post(GOOGLE_TOKEN_URL)
        .form(&[
            ("code", code),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|error| AppError::internal(format!("google token exchange: {}", error)))?
        .error_for_status()
        .map_err(|error| AppError::internal(format!("google token exchange: {}", error)))?
        .json()
        .await
        .map_err(|error| AppError::internal(format!("google token decode: {}", error)))?;

    settings.access_token = Some(token.access_token);
    settings.refresh_token = token.refresh_token.or(settings.refresh_token);
    settings.token_expires_at = Some(now_ts() + token.expires_in.unwrap_or(3600) - 60);
    settings.pending_oauth_state = None;
    settings.last_error = None;

    let calendars = list_google_calendars(&settings).await?;
    settings.calendars = merge_calendar_selection(settings.calendars, calendars);
    save_google_settings(storage, &settings).await?;
    Ok(())
}

pub async fn sync_google_calendar(
    storage: &Storage,
    _config: &AppConfig,
) -> Result<Option<u32>, AppError> {
    let mut settings = load_google_settings(storage).await?;
    let Some(client_id) = settings.client_id.clone() else {
        return Ok(None);
    };
    let Some(client_secret) = settings.client_secret.clone() else {
        return Ok(None);
    };
    if settings.refresh_token.as_deref().unwrap_or("").is_empty() {
        return Ok(None);
    }

    let access_token =
        ensure_google_access_token(storage, &mut settings, &client_id, &client_secret).await?;
    let calendars = list_google_calendars_with_token(&access_token).await?;
    settings.calendars = merge_calendar_selection(settings.calendars, calendars);
    let selected = selected_calendars(&settings);
    let time_min = OffsetDateTime::now_utc() - Duration::days(GOOGLE_LOOKBACK_DAYS);
    let time_max = OffsetDateTime::now_utc() + Duration::days(GOOGLE_LOOKAHEAD_DAYS);

    let mut inserted = 0u32;
    for calendar in &selected {
        let events =
            list_google_events_with_token(&access_token, &calendar.id, time_min, time_max).await?;
        for event in events {
            if event.status.as_deref() == Some("cancelled") {
                continue;
            }
            let Some(start_ts) = google_event_start_ts(&event) else {
                continue;
            };
            let source_ref = format!(
                "google_calendar:{}:{}:{}",
                calendar.id,
                event.id,
                event.updated.as_deref().unwrap_or("-"),
            );
            let payload = serde_json::json!({
                "event_id": event.id,
                "calendar_id": calendar.id,
                "calendar_name": calendar.summary,
                "title": event.summary.clone().unwrap_or_else(|| "(untitled event)".to_string()),
                "start": start_ts,
                "end": google_event_end_ts(&event),
                "location": event.location.unwrap_or_default(),
                "description": event.description.unwrap_or_default(),
                "status": event.status.unwrap_or_default(),
                "url": event.html_link.unwrap_or_default(),
                "attendees": event.attendees.unwrap_or_default().into_iter().filter_map(|item| item.email.or(item.display_name)).collect::<Vec<_>>(),
                "prep_minutes": 15,
                "travel_minutes": 0
            });
            let signal_id = storage
                .insert_signal(SignalInsert {
                    signal_type: "calendar_event".to_string(),
                    source: "google_calendar".to_string(),
                    source_ref: Some(source_ref),
                    timestamp: start_ts,
                    payload_json: Some(payload),
                })
                .await?;
            if signal_id.starts_with("sig_") {
                inserted += 1;
            }
        }
    }

    settings.last_sync_at = Some(now_ts());
    settings.last_sync_status = Some("ok".to_string());
    settings.last_error = None;
    settings.last_item_count = Some(inserted);
    save_google_settings(storage, &settings).await?;
    append_sync_event(storage, "google-calendar", "ok", inserted, None).await?;
    Ok(Some(inserted))
}

pub async fn sync_todoist(storage: &Storage) -> Result<Option<u32>, AppError> {
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
        .collect::<HashMap<_, _>>();

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
    append_sync_event(storage, "todoist", "ok", signals_count, None).await?;
    Ok(Some(signals_count))
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
            let mut settings = load_todoist_settings(storage).await?;
            settings.last_sync_at = Some(now_ts());
            settings.last_sync_status = Some("error".to_string());
            settings.last_error = Some(error.to_string());
            save_todoist_settings(storage, &settings).await?;
            append_sync_event(storage, "todoist", "error", 0, Some(error)).await?;
        }
        "activity" | "git" | "messaging" | "notes" | "transcripts" => {
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
        "activity" | "git" | "messaging" | "notes" | "transcripts" => {
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

async fn list_google_calendars(
    settings: &GoogleCalendarSettings,
) -> Result<Vec<StoredCalendar>, AppError> {
    let access_token = settings
        .access_token
        .as_deref()
        .ok_or_else(|| AppError::bad_request("google access token missing"))?;
    list_google_calendars_with_token(access_token).await
}

async fn list_google_calendars_with_token(
    access_token: &str,
) -> Result<Vec<StoredCalendar>, AppError> {
    let client = reqwest::Client::new();
    let mut calendars = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = Url::parse(&format!("{}/users/me/calendarList", GOOGLE_CALENDAR_BASE))
            .map_err(|error| AppError::internal(format!("google calendar list url: {}", error)))?;
        if let Some(token) = page_token.as_deref() {
            url.query_pairs_mut().append_pair("pageToken", token);
        }

        let response: GoogleCalendarListResponse = client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|error| AppError::internal(format!("google calendar list: {}", error)))?
            .error_for_status()
            .map_err(|error| AppError::internal(format!("google calendar list: {}", error)))?
            .json()
            .await
            .map_err(|error| AppError::internal(format!("google calendar decode: {}", error)))?;

        calendars.extend(response.items.into_iter().map(|item| StoredCalendar {
            id: item.id,
            summary: item.summary,
            primary: item.primary.unwrap_or(false),
            selected: true,
        }));

        match response.next_page_token {
            Some(token) if !token.is_empty() => page_token = Some(token),
            _ => break,
        }
    }

    Ok(calendars)
}

async fn list_google_events_with_token(
    access_token: &str,
    calendar_id: &str,
    time_min: OffsetDateTime,
    time_max: OffsetDateTime,
) -> Result<Vec<GoogleCalendarEvent>, AppError> {
    let client = reqwest::Client::new();
    let mut events = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut url = Url::parse(GOOGLE_CALENDAR_BASE)
            .map_err(|error| AppError::internal(format!("google events url: {}", error)))?;
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| AppError::internal("google events url path"))?;
            segments.push("calendars");
            segments.push(calendar_id);
            segments.push("events");
        }
        {
            let mut query = url.query_pairs_mut();
            query
                .append_pair("singleEvents", "true")
                .append_pair("orderBy", "startTime")
                .append_pair("timeMin", &format_rfc3339(time_min)?)
                .append_pair("timeMax", &format_rfc3339(time_max)?);
            if let Some(token) = page_token.as_deref() {
                query.append_pair("pageToken", token);
            }
        }

        let response: GoogleEventListResponse = client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|error| AppError::internal(format!("google event list: {}", error)))?
            .error_for_status()
            .map_err(|error| AppError::internal(format!("google event list: {}", error)))?
            .json()
            .await
            .map_err(|error| AppError::internal(format!("google event decode: {}", error)))?;
        events.extend(response.items);

        match response.next_page_token {
            Some(token) if !token.is_empty() => page_token = Some(token),
            _ => break,
        }
    }

    Ok(events)
}

async fn ensure_google_access_token(
    storage: &Storage,
    settings: &mut GoogleCalendarSettings,
    client_id: &str,
    client_secret: &str,
) -> Result<String, AppError> {
    if let (Some(access_token), Some(expires_at)) =
        (settings.access_token.clone(), settings.token_expires_at)
    {
        if now_ts() < expires_at {
            return Ok(access_token);
        }
    }

    let refresh_token = settings
        .refresh_token
        .clone()
        .ok_or_else(|| AppError::bad_request("google refresh token missing"))?;

    let token: GoogleTokenResponse = reqwest::Client::new()
        .post(GOOGLE_TOKEN_URL)
        .form(&[
            ("refresh_token", refresh_token.as_str()),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .await
        .map_err(|error| AppError::internal(format!("google token refresh: {}", error)))?
        .error_for_status()
        .map_err(|error| AppError::internal(format!("google token refresh: {}", error)))?
        .json()
        .await
        .map_err(|error| AppError::internal(format!("google token refresh decode: {}", error)))?;

    settings.access_token = Some(token.access_token.clone());
    settings.token_expires_at = Some(now_ts() + token.expires_in.unwrap_or(3600) - 60);
    save_google_settings(storage, settings).await?;
    Ok(token.access_token)
}

fn merge_calendar_selection(
    existing: Vec<StoredCalendar>,
    latest: Vec<StoredCalendar>,
) -> Vec<StoredCalendar> {
    let selected_by_id = existing
        .into_iter()
        .map(|calendar| (calendar.id, calendar.selected))
        .collect::<HashMap<_, _>>();
    latest
        .into_iter()
        .map(|mut calendar| {
            if let Some(selected) = selected_by_id.get(&calendar.id) {
                calendar.selected = *selected;
            }
            calendar
        })
        .collect()
}

fn selected_calendars(settings: &GoogleCalendarSettings) -> Vec<StoredCalendar> {
    if settings.all_calendars_selected {
        return settings.calendars.clone();
    }
    settings
        .calendars
        .iter()
        .filter(|calendar| calendar.selected)
        .cloned()
        .collect()
}

fn google_status(settings: &GoogleCalendarSettings) -> GoogleCalendarIntegrationData {
    GoogleCalendarIntegrationData {
        configured: settings.client_id.is_some() && settings.client_secret.is_some(),
        connected: settings.refresh_token.is_some(),
        has_client_id: settings.client_id.is_some(),
        has_client_secret: settings.client_secret.is_some(),
        calendars: settings
            .calendars
            .iter()
            .map(|calendar| IntegrationCalendarData {
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
    }
}

fn todoist_status(settings: &TodoistSettings) -> TodoistIntegrationData {
    TodoistIntegrationData {
        configured: settings.api_token.is_some(),
        connected: settings.api_token.is_some(),
        has_api_token: settings.api_token.is_some(),
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
    }
}

fn local_status(
    source_path: Option<String>,
    settings: &LocalIntegrationSettings,
) -> LocalIntegrationData {
    LocalIntegrationData {
        configured: source_path.is_some(),
        source_path,
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
    }
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

async fn load_todoist_settings(storage: &Storage) -> Result<TodoistSettings, AppError> {
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
        .map(|value| serde_json::from_value::<T>(value).unwrap_or_default())
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
        "git" => GIT_SETTINGS_KEY,
        "messaging" => MESSAGING_SETTINGS_KEY,
        "notes" => NOTES_SETTINGS_KEY,
        "transcripts" => TRANSCRIPTS_SETTINGS_KEY,
        _ => "",
    }
}

fn local_integration_id(source: &str) -> &'static str {
    match source {
        "activity" => "activity",
        "git" => "git",
        "messaging" => "messaging",
        "notes" => "notes",
        "transcripts" => "transcripts",
        _ => "",
    }
}

fn config_source_path(primary: Option<&str>, secondary: Option<&str>) -> Option<String> {
    primary
        .or(secondary)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn google_redirect_uri(config: &AppConfig) -> Result<String, AppError> {
    let base = config.base_url.trim_end_matches('/');
    if base.is_empty() {
        return Err(AppError::internal("base_url is required for google oauth"));
    }
    Ok(format!(
        "{}/api/integrations/google-calendar/oauth/callback",
        base
    ))
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

fn format_rfc3339(value: OffsetDateTime) -> Result<String, AppError> {
    value
        .format(&Rfc3339)
        .map_err(|error| AppError::internal(format!("format rfc3339: {}", error)))
}

fn google_event_start_ts(event: &GoogleCalendarEvent) -> Option<i64> {
    google_event_time_to_ts(event.start.as_ref())
}

fn google_event_end_ts(event: &GoogleCalendarEvent) -> Option<i64> {
    google_event_time_to_ts(event.end.as_ref())
}

fn google_event_time_to_ts(value: Option<&GoogleEventDateTime>) -> Option<i64> {
    let value = value?;
    if let Some(date_time) = value.date_time.as_deref() {
        return parse_rfc3339(date_time);
    }
    value
        .date
        .as_deref()
        .and_then(|date| parse_iso_datetime(&format!("{}T00:00:00Z", date)))
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
                source_id: Some(source_id.to_string()),
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
struct GoogleTokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarListResponse {
    #[serde(default)]
    items: Vec<GoogleCalendarListItem>,
    #[serde(default, rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarListItem {
    id: String,
    summary: String,
    #[serde(default)]
    primary: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GoogleEventListResponse {
    #[serde(default)]
    items: Vec<GoogleCalendarEvent>,
    #[serde(default, rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarEvent {
    id: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    location: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    updated: Option<String>,
    #[serde(default)]
    html_link: Option<String>,
    #[serde(default)]
    attendees: Option<Vec<GoogleEventAttendee>>,
    start: Option<GoogleEventDateTime>,
    end: Option<GoogleEventDateTime>,
}

#[derive(Debug, Deserialize)]
struct GoogleEventDateTime {
    #[serde(default, rename = "dateTime")]
    date_time: Option<String>,
    #[serde(default)]
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleEventAttendee {
    #[serde(default)]
    email: Option<String>,
    #[serde(default, rename = "displayName")]
    display_name: Option<String>,
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
