use std::{
    env,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use vel_api_types::{IntegrationGuidanceData, LocalIntegrationData};
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::errors::AppError;

pub(crate) const ACTIVITY_SETTINGS_KEY: &str = "integration_activity";
pub(crate) const HEALTH_SETTINGS_KEY: &str = "integration_health";
pub(crate) const GIT_SETTINGS_KEY: &str = "integration_git";
pub(crate) const MESSAGING_SETTINGS_KEY: &str = "integration_messaging";
pub(crate) const NOTES_SETTINGS_KEY: &str = "integration_notes";
pub(crate) const TRANSCRIPTS_SETTINGS_KEY: &str = "integration_transcripts";

const LOCAL_INTEGRATION_IDS: &[&str] = &[
    "activity",
    "health",
    "git",
    "messaging",
    "notes",
    "transcripts",
];
const MACOS_SUPPORT_RELATIVE_DIRS: &[&str] = &[
    "Library/Application Support/Vel/integrations",
    "Library/Application Support/vel/integrations",
];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalIntegrationSettings {
    pub source_path: Option<String>,
    pub last_sync_at: Option<i64>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
}

pub(crate) fn effective_local_source_path(
    integration_id: &str,
    settings_source: Option<&str>,
    config_source: Option<&str>,
) -> Option<String> {
    config_source_path(settings_source, config_source)
        .or_else(|| auto_local_source_path(integration_id))
}

pub(crate) fn sanitize_missing_default_local_sources(config: &mut AppConfig) {
    sanitize_missing_default_local_path("activity", &mut config.activity_snapshot_path);
    sanitize_missing_default_local_path("health", &mut config.health_snapshot_path);
    sanitize_missing_default_local_path("git", &mut config.git_snapshot_path);
    sanitize_missing_default_local_path("messaging", &mut config.messaging_snapshot_path);
    sanitize_missing_default_local_path("notes", &mut config.notes_path);
    sanitize_missing_default_local_path("transcripts", &mut config.transcript_snapshot_path);
}

fn sanitize_missing_default_local_path(kind: &str, path: &mut Option<String>) {
    let should_clear = path.as_deref().is_some_and(|value| {
        vel_config::is_default_local_source_path(kind, value) && !Path::new(value).exists()
    });
    if should_clear {
        *path = None;
    }
}

fn config_source_path(primary: Option<&str>, secondary: Option<&str>) -> Option<String> {
    primary
        .or(secondary)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn auto_local_source_path(integration_id: &str) -> Option<String> {
    if !running_on_macos() {
        return None;
    }

    if !LOCAL_INTEGRATION_IDS.contains(&integration_id) {
        return None;
    }

    auto_local_source_path_for_integration(integration_id)
}

fn running_on_macos() -> bool {
    cfg!(target_os = "macos")
        || env::var("VEL_FORCE_MACOS_LOCAL_SOURCE_DISCOVERY")
            .ok()
            .as_deref()
            == Some("1")
}

fn auto_local_source_path_for_integration(integration_id: &str) -> Option<String> {
    let home = env::var_os("HOME").map(PathBuf::from)?;
    auto_local_source_path_from_home(integration_id, &home)
}

pub(crate) fn auto_local_source_path_from_home(
    integration_id: &str,
    home: &Path,
) -> Option<String> {
    let candidates = match integration_id {
        "activity" => vec![
            home.join("Library/Application Support/Vel/activity/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/activity/snapshot.json"),
        ],
        "health" => vec![
            home.join("Library/Application Support/Vel/health/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/health/snapshot.json"),
        ],
        "git" => vec![
            home.join("Library/Application Support/Vel/git/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/git/snapshot.json"),
        ],
        "messaging" => vec![
            home.join("Library/Application Support/Vel/messages/snapshot.json"),
            home.join("Library/Application Support/Vel/messaging/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/messages/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/messaging/snapshot.json"),
        ],
        "notes" => vec![
            home.join("Library/Application Support/Vel/notes"),
            home.join("Library/Application Support/Vel/integrations/notes"),
        ],
        "transcripts" => vec![
            home.join("Library/Application Support/Vel/transcripts/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/transcripts/snapshot.json"),
        ],
        _ => Vec::new(),
    };

    candidates
        .into_iter()
        .find(|candidate| candidate.exists())
        .map(|candidate| candidate.to_string_lossy().to_string())
        .or_else(|| auto_local_source_path_from_support_roots(integration_id, home))
}

fn auto_local_source_path_from_support_roots(integration_id: &str, home: &Path) -> Option<String> {
    let suffix = match integration_id {
        "activity" | "health" | "git" | "messaging" | "transcripts" => "snapshot.json",
        "notes" => "",
        _ => return None,
    };

    MACOS_SUPPORT_RELATIVE_DIRS
        .iter()
        .map(|relative| home.join(relative).join(integration_id))
        .map(|base| {
            if suffix.is_empty() {
                base
            } else {
                base.join(suffix)
            }
        })
        .find(|candidate| candidate.exists())
        .map(|candidate| candidate.to_string_lossy().to_string())
}

pub(crate) fn local_settings_key(source: &str) -> &'static str {
    match source {
        "activity" => ACTIVITY_SETTINGS_KEY,
        "health" => HEALTH_SETTINGS_KEY,
        "git" => GIT_SETTINGS_KEY,
        "messaging" => MESSAGING_SETTINGS_KEY,
        "notes" => NOTES_SETTINGS_KEY,
        "transcripts" => TRANSCRIPTS_SETTINGS_KEY,
        _ => "",
    }
}

pub(crate) fn local_integration_id(source: &str) -> &'static str {
    match source {
        "activity" => "activity",
        "health" => "health",
        "git" => "git",
        "messaging" => "messaging",
        "notes" => "notes",
        "transcripts" => "transcripts",
        _ => "",
    }
}

pub(crate) async fn load_local_settings(
    storage: &Storage,
    key: &str,
) -> Result<LocalIntegrationSettings, AppError> {
    let all = storage.get_all_settings().await?;
    Ok(all
        .get(key)
        .cloned()
        .map(|value| serde_json::from_value::<LocalIntegrationSettings>(value).unwrap_or_default())
        .unwrap_or_default())
}

pub(crate) async fn save_local_settings(
    storage: &Storage,
    key: &str,
    settings: &LocalIntegrationSettings,
) -> Result<(), AppError> {
    let value = serde_json::to_value(settings).map_err(|error| {
        AppError::internal(format!("serialize integration settings: {}", error))
    })?;
    storage.set_setting(key, &value).await?;
    Ok(())
}

pub(crate) async fn update_local_source_path(
    storage: &Storage,
    source: &str,
    source_path: Option<String>,
) -> Result<(), AppError> {
    let key = local_settings_key(source);
    if key.is_empty() {
        return Err(AppError::not_found("integration not found"));
    }
    let mut settings = load_local_settings(storage, key).await?;
    settings.source_path = normalize_optional(source_path.unwrap_or_default());
    save_local_settings(storage, key, &settings).await
}

pub(crate) async fn update_local_sync_settings(
    storage: &Storage,
    source: &str,
    status: &str,
    error: Option<String>,
    item_count: Option<u32>,
) -> Result<(), AppError> {
    let key = local_settings_key(source);
    if key.is_empty() {
        return Ok(());
    }
    let mut settings = load_local_settings(storage, key).await?;
    settings.last_sync_at = Some(now_ts());
    settings.last_sync_status = Some(status.to_string());
    settings.last_error = error;
    settings.last_item_count = item_count;
    save_local_settings(storage, key, &settings).await
}

pub(crate) fn local_status_data(
    integration_id: &str,
    settings: &LocalIntegrationSettings,
    config_source: Option<&str>,
) -> LocalIntegrationData {
    let source_path = effective_local_source_path(
        integration_id,
        settings.source_path.as_deref(),
        config_source,
    );
    LocalIntegrationData {
        configured: source_path.is_some(),
        guidance: local_guidance(integration_id, source_path.as_deref(), settings),
        source_path,
        last_sync_at: settings.last_sync_at,
        last_sync_status: settings.last_sync_status.clone(),
        last_error: settings.last_error.clone(),
        last_item_count: settings.last_item_count,
    }
}

pub(crate) async fn runtime_local_config(
    storage: &Storage,
    config: &AppConfig,
) -> Result<AppConfig, AppError> {
    let activity = load_local_settings(storage, ACTIVITY_SETTINGS_KEY).await?;
    let health = load_local_settings(storage, HEALTH_SETTINGS_KEY).await?;
    let git = load_local_settings(storage, GIT_SETTINGS_KEY).await?;
    let messaging = load_local_settings(storage, MESSAGING_SETTINGS_KEY).await?;
    let notes = load_local_settings(storage, NOTES_SETTINGS_KEY).await?;
    let transcripts = load_local_settings(storage, TRANSCRIPTS_SETTINGS_KEY).await?;

    let mut runtime = config.clone();
    runtime.activity_snapshot_path = effective_local_source_path(
        "activity",
        activity.source_path.as_deref(),
        config.activity_snapshot_path.as_deref(),
    );
    runtime.health_snapshot_path = effective_local_source_path(
        "health",
        health.source_path.as_deref(),
        config.health_snapshot_path.as_deref(),
    );
    runtime.git_snapshot_path = effective_local_source_path(
        "git",
        git.source_path.as_deref(),
        config.git_snapshot_path.as_deref(),
    );
    runtime.messaging_snapshot_path = effective_local_source_path(
        "messaging",
        messaging.source_path.as_deref(),
        config.messaging_snapshot_path.as_deref(),
    );
    runtime.notes_path = effective_local_source_path(
        "notes",
        notes.source_path.as_deref(),
        config.notes_path.as_deref(),
    );
    runtime.transcript_snapshot_path = effective_local_source_path(
        "transcripts",
        transcripts.source_path.as_deref(),
        config.transcript_snapshot_path.as_deref(),
    );
    sanitize_missing_default_local_sources(&mut runtime);
    Ok(runtime)
}

fn local_guidance(
    integration_id: &str,
    source_path: Option<&str>,
    settings: &LocalIntegrationSettings,
) -> Option<IntegrationGuidanceData> {
    if source_path.is_none() {
        let detail = if integration_id == "notes" {
            "Configure the Obsidian vault root or another local notes directory before syncing it."
                .to_string()
        } else {
            "Configure a source path for this local adapter before syncing it.".to_string()
        };
        return Some(guidance("Local source missing", detail, "Set source path"));
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
        let detail = if integration_id == "notes" {
            "Run sync now after Obsidian Sync or your local notes workflow has updated the vault."
                .to_string()
        } else {
            "Run sync now to ingest this local source into Vel.".to_string()
        };
        return Some(guidance(
            "Local source has not synced yet",
            detail,
            "Sync now",
        ));
    }
    None
}

fn guidance(title: &str, detail: String, action: &str) -> IntegrationGuidanceData {
    IntegrationGuidanceData {
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
    time::OffsetDateTime::now_utc().unix_timestamp()
}
