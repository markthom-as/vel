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
    configured_source_path(integration_id, settings_source, config_source)
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

fn configured_source_path(
    integration_id: &str,
    primary: Option<&str>,
    secondary: Option<&str>,
) -> Option<String> {
    let candidate = config_source_path(primary, secondary)?;
    if vel_config::is_default_local_source_path(integration_id, &candidate)
        && !Path::new(&candidate).exists()
    {
        return None;
    }
    Some(candidate)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::OsString,
        fs,
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("vel-{prefix}-{nanos}"));
        fs::create_dir_all(&path).expect("temp dir should be created");
        path
    }

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: impl Into<OsString>) -> Self {
            let previous = env::var_os(key);
            env::set_var(key, value.into());
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                env::set_var(self.key, value);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn default_missing_config_path_does_not_block_auto_discovery() {
        let _env_guard = ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock should be acquired");

        let home = unique_temp_dir("mac-home");
        let snapshot_path = home.join("Library/Application Support/Vel/messages/snapshot.json");
        fs::create_dir_all(
            snapshot_path
                .parent()
                .expect("snapshot parent should exist"),
        )
        .expect("snapshot parent dir should be created");
        fs::write(&snapshot_path, "{}").expect("snapshot fixture should be written");

        let _home_guard = EnvVarGuard::set("HOME", home.as_os_str().to_os_string());
        let _mac_force_guard = EnvVarGuard::set("VEL_FORCE_MACOS_LOCAL_SOURCE_DISCOVERY", "1");

        let resolved = effective_local_source_path(
            "messaging",
            None,
            Some("var/integrations/messaging/snapshot.json"),
        );
        assert_eq!(
            resolved.as_deref(),
            Some(snapshot_path.to_string_lossy().as_ref())
        );

        let _ = fs::remove_dir_all(home);
    }

    #[test]
    fn explicit_source_override_wins_over_auto_discovery() {
        let configured = "/tmp/explicit-messaging.json";
        let resolved = effective_local_source_path(
            "messaging",
            Some(configured),
            Some("var/integrations/messaging/snapshot.json"),
        );
        assert_eq!(resolved.as_deref(), Some(configured));
    }
}
