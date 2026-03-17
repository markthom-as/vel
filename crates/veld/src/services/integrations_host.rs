use std::{
    env,
    path::{Path, PathBuf},
};

use vel_config::AppConfig;

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
