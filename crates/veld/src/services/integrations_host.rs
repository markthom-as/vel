use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
};

use serde_json::Value as JsonValue;
use vel_config::AppConfig;

const LOCAL_INTEGRATION_IDS: &[&str] = &[
    "activity",
    "health",
    "git",
    "messaging",
    "reminders",
    "notes",
    "transcripts",
];
const MACOS_SUPPORT_RELATIVE_DIRS: &[&str] = &[
    "Library/Application Support/Vel/integrations",
    "Library/Application Support/vel/integrations",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalSourcePathKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LocalSourcePathSuggestions {
    pub available_paths: Vec<String>,
    pub internal_paths: Vec<String>,
    pub suggested_paths: Vec<String>,
}

pub(crate) fn effective_local_source_path(
    integration_id: &str,
    settings_source: Option<&str>,
    config_source: Option<&str>,
) -> Option<String> {
    configured_source_path(integration_id, settings_source, config_source)
        .or_else(|| auto_local_source_path(integration_id))
}

pub(crate) fn local_source_path_kind(integration_id: &str) -> Option<LocalSourcePathKind> {
    match integration_id {
        "notes" => Some(LocalSourcePathKind::Directory),
        "activity" | "health" | "git" | "messaging" | "reminders" | "transcripts" => {
            Some(LocalSourcePathKind::File)
        }
        _ => None,
    }
}

pub(crate) fn suggested_local_source_paths(
    integration_id: &str,
    settings_source: Option<&str>,
    config_source: Option<&str>,
) -> LocalSourcePathSuggestions {
    let home = current_home_dir();
    let appdata = env::var_os("APPDATA").map(PathBuf::from);
    let user_profile = env::var_os("USERPROFILE").map(PathBuf::from);
    let mut available_paths = Vec::new();
    let mut internal_paths = Vec::new();

    for value in [settings_source, config_source].into_iter().flatten() {
        let candidate = value.trim();
        if candidate.is_empty() {
            continue;
        }
        if Path::new(candidate).exists() {
            available_paths.push(candidate.to_string());
        } else if vel_config::is_default_local_source_path(integration_id, candidate) {
            internal_paths.push(candidate.to_string());
        }
    }

    if let Some(home) = home.as_deref() {
        let platform_candidates = platform_candidate_paths(
            integration_id,
            home,
            appdata.as_deref(),
            user_profile.as_deref(),
        );
        let (existing_platform_paths, missing_platform_paths) =
            partition_existing_paths(platform_candidates);
        available_paths.extend(existing_platform_paths);
        internal_paths.extend(missing_platform_paths);

        if integration_id == "activity" {
            available_paths.extend(discover_activity_source_paths(
                home,
                appdata.as_deref(),
                user_profile.as_deref(),
            ));
        }

        if integration_id == "notes" {
            available_paths.extend(discover_obsidian_vault_paths(
                home,
                appdata.as_deref(),
                user_profile.as_deref(),
            ));
        }
    }

    let available_paths = dedupe_paths(available_paths);
    let internal_paths = dedupe_paths(
        internal_paths
            .into_iter()
            .filter(|path| !available_paths.contains(path))
            .collect(),
    );
    let mut suggested_paths = available_paths.clone();
    suggested_paths.extend(internal_paths.clone());
    LocalSourcePathSuggestions {
        available_paths,
        internal_paths,
        suggested_paths,
    }
}

pub(crate) async fn choose_local_source_path(
    integration_id: &str,
) -> Result<Option<String>, crate::errors::AppError> {
    let kind = local_source_path_kind(integration_id)
        .ok_or_else(|| crate::errors::AppError::bad_request("unsupported local integration"))?;
    let prompt = match integration_id {
        "notes" => "Choose Obsidian vault",
        "activity" => "Choose activity snapshot",
        "health" => "Choose health snapshot",
        "git" => "Choose git activity snapshot",
        "messaging" => "Choose messaging snapshot",
        "reminders" => "Choose reminders snapshot",
        "transcripts" => "Choose transcript snapshot",
        _ => "Choose local source path",
    };

    if cfg!(target_os = "macos") {
        return choose_with_osascript(kind, prompt).await;
    }
    if cfg!(target_os = "linux") {
        return choose_with_linux_dialog(kind, prompt).await;
    }
    if cfg!(target_os = "windows") {
        return choose_with_powershell(kind, prompt).await;
    }

    Err(crate::errors::AppError::bad_request(
        "path dialogs are not supported on this host platform",
    ))
}

pub(crate) fn sanitize_missing_default_local_sources(config: &mut AppConfig) {
    sanitize_missing_default_local_path("activity", &mut config.activity_snapshot_path);
    sanitize_missing_default_local_path("health", &mut config.health_snapshot_path);
    sanitize_missing_default_local_path("git", &mut config.git_snapshot_path);
    sanitize_missing_default_local_path("messaging", &mut config.messaging_snapshot_path);
    sanitize_missing_default_local_path("reminders", &mut config.reminders_snapshot_path);
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

fn current_home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
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
    let candidates = candidate_paths_for_home(integration_id, home);

    candidates
        .into_iter()
        .find(|candidate| candidate.exists())
        .map(|candidate| candidate.to_string_lossy().to_string())
        .or_else(|| auto_local_source_path_from_support_roots(integration_id, home))
}

fn candidate_paths_for_home(integration_id: &str, home: &Path) -> Vec<PathBuf> {
    match integration_id {
        "activity" => vec![
            home.join("Library/Application Support/Vel/activity/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/activity/snapshot.json"),
            home.join(".local/share/vel/activity/snapshot.json"),
            home.join(".local/share/vel/integrations/activity/snapshot.json"),
        ],
        "health" => vec![
            home.join("Library/Application Support/Vel/health/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/health/snapshot.json"),
            home.join(".local/share/vel/health/snapshot.json"),
            home.join(".local/share/vel/integrations/health/snapshot.json"),
        ],
        "git" => vec![
            home.join("Library/Application Support/Vel/git/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/git/snapshot.json"),
            home.join(".local/share/vel/git/snapshot.json"),
            home.join(".local/share/vel/integrations/git/snapshot.json"),
        ],
        "messaging" => vec![
            home.join("Library/Application Support/Vel/messages/snapshot.json"),
            home.join("Library/Application Support/Vel/messaging/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/messages/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/messaging/snapshot.json"),
            home.join(".local/share/vel/messages/snapshot.json"),
            home.join(".local/share/vel/messaging/snapshot.json"),
            home.join(".local/share/vel/integrations/messages/snapshot.json"),
            home.join(".local/share/vel/integrations/messaging/snapshot.json"),
        ],
        "notes" => vec![
            home.join("Library/Application Support/Vel/notes"),
            home.join("Library/Application Support/Vel/integrations/notes"),
            home.join(".local/share/vel/notes"),
            home.join(".local/share/vel/integrations/notes"),
        ],
        "reminders" => vec![
            home.join("Library/Application Support/Vel/reminders/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/reminders/snapshot.json"),
            home.join(".local/share/vel/reminders/snapshot.json"),
            home.join(".local/share/vel/integrations/reminders/snapshot.json"),
        ],
        "transcripts" => vec![
            home.join("Library/Application Support/Vel/transcripts/snapshot.json"),
            home.join("Library/Application Support/Vel/integrations/transcripts/snapshot.json"),
            home.join(".local/share/vel/transcripts/snapshot.json"),
            home.join(".local/share/vel/integrations/transcripts/snapshot.json"),
        ],
        _ => Vec::new(),
    }
}

fn auto_local_source_path_from_support_roots(integration_id: &str, home: &Path) -> Option<String> {
    let suffix = match integration_id {
        "activity" | "health" | "git" | "messaging" | "reminders" | "transcripts" => {
            "snapshot.json"
        }
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

fn platform_candidate_paths(
    integration_id: &str,
    home: &Path,
    appdata: Option<&Path>,
    user_profile: Option<&Path>,
) -> Vec<String> {
    let mut candidates = candidate_paths_for_home(integration_id, home);

    if let Some(appdata) = appdata {
        candidates.extend(match integration_id {
            "activity" => vec![
                appdata.join("Vel/activity/snapshot.json"),
                appdata.join("Vel/integrations/activity/snapshot.json"),
            ],
            "health" => vec![
                appdata.join("Vel/health/snapshot.json"),
                appdata.join("Vel/integrations/health/snapshot.json"),
            ],
            "git" => vec![
                appdata.join("Vel/git/snapshot.json"),
                appdata.join("Vel/integrations/git/snapshot.json"),
            ],
            "messaging" => vec![
                appdata.join("Vel/messages/snapshot.json"),
                appdata.join("Vel/messaging/snapshot.json"),
                appdata.join("Vel/integrations/messages/snapshot.json"),
                appdata.join("Vel/integrations/messaging/snapshot.json"),
            ],
            "notes" => vec![
                appdata.join("Vel/notes"),
                appdata.join("Vel/integrations/notes"),
            ],
            "reminders" => vec![
                appdata.join("Vel/reminders/snapshot.json"),
                appdata.join("Vel/integrations/reminders/snapshot.json"),
            ],
            "transcripts" => vec![
                appdata.join("Vel/transcripts/snapshot.json"),
                appdata.join("Vel/integrations/transcripts/snapshot.json"),
            ],
            _ => Vec::new(),
        });
    }

    if let Some(user_profile) = user_profile {
        candidates.extend(match integration_id {
            "activity" => vec![user_profile.join("AppData/Roaming/Vel/activity/snapshot.json")],
            "health" => vec![user_profile.join("AppData/Roaming/Vel/health/snapshot.json")],
            "git" => vec![user_profile.join("AppData/Roaming/Vel/git/snapshot.json")],
            "messaging" => vec![
                user_profile.join("AppData/Roaming/Vel/messages/snapshot.json"),
                user_profile.join("AppData/Roaming/Vel/messaging/snapshot.json"),
            ],
            "notes" => vec![user_profile.join("AppData/Roaming/Vel/notes")],
            "reminders" => vec![user_profile.join("AppData/Roaming/Vel/reminders/snapshot.json")],
            "transcripts" => {
                vec![user_profile.join("AppData/Roaming/Vel/transcripts/snapshot.json")]
            }
            _ => Vec::new(),
        });
    }

    candidates
        .into_iter()
        .map(|candidate| candidate.to_string_lossy().to_string())
        .collect()
}

fn partition_existing_paths(paths: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut existing = Vec::new();
    let mut missing = Vec::new();

    for path in dedupe_paths(paths) {
        if Path::new(&path).exists() {
            existing.push(path);
        } else {
            missing.push(path);
        }
    }

    (existing, missing)
}

fn discover_activity_source_paths(
    home: &Path,
    appdata: Option<&Path>,
    user_profile: Option<&Path>,
) -> Vec<String> {
    let mut candidates = vec![
        home.join(".histfile"),
        home.join(".zsh_history"),
        home.join(".local/share/zsh/history"),
        home.join(".local/share/zsh/zsh_history"),
        home.join("Library/Application Support/activitywatch"),
        home.join(".local/share/activitywatch"),
        home.join(".config/activitywatch"),
        home.join(".var/app/net.activitywatch.ActivityWatch/data/activitywatch"),
    ];

    if let Some(appdata) = appdata {
        candidates.push(appdata.join("activitywatch"));
        candidates.push(appdata.join("ActivityWatch"));
    }

    if let Some(user_profile) = user_profile {
        candidates.push(user_profile.join("AppData/Roaming/activitywatch"));
        candidates.push(user_profile.join("AppData/Local/activitywatch"));
        candidates.push(user_profile.join("AppData/Roaming/ActivityWatch"));
        candidates.push(user_profile.join("AppData/Local/ActivityWatch"));
    }

    dedupe_paths(
        candidates
            .into_iter()
            .filter(|candidate| candidate.exists())
            .map(|candidate| candidate.to_string_lossy().to_string())
            .collect(),
    )
}

fn discover_obsidian_vault_paths(
    home: &Path,
    appdata: Option<&Path>,
    user_profile: Option<&Path>,
) -> Vec<String> {
    let mut config_paths = vec![
        home.join("Library/Application Support/obsidian/obsidian.json"),
        home.join(".config/obsidian/obsidian.json"),
        home.join(".var/app/md.obsidian.Obsidian/config/obsidian/obsidian.json"),
        home.join("snap/obsidian/current/.config/obsidian/obsidian.json"),
    ];
    if let Some(appdata) = appdata {
        config_paths.push(appdata.join("obsidian/obsidian.json"));
    }
    if let Some(user_profile) = user_profile {
        config_paths.push(user_profile.join("AppData/Roaming/obsidian/obsidian.json"));
    }

    let mut suggestions = Vec::new();
    for config_path in dedupe_paths(
        config_paths
            .into_iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect(),
    ) {
        let path = PathBuf::from(&config_path);
        let Ok(contents) = std::fs::read_to_string(&path) else {
            continue;
        };
        suggestions.extend(parse_obsidian_vault_paths(&contents));
    }

    if let Ok(entries) =
        std::fs::read_dir(home.join("Library/Mobile Documents/iCloud~md~obsidian/Documents"))
    {
        suggestions.extend(
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .map(|path| path.to_string_lossy().to_string()),
        );
    }

    dedupe_paths(suggestions)
}

fn parse_obsidian_vault_paths(contents: &str) -> Vec<String> {
    let Ok(json) = serde_json::from_str::<JsonValue>(contents) else {
        return Vec::new();
    };
    let Some(vaults) = json.get("vaults").and_then(JsonValue::as_object) else {
        return Vec::new();
    };

    dedupe_paths(
        vaults
            .values()
            .filter_map(|value| value.get("path").and_then(JsonValue::as_str))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect(),
    )
}

fn dedupe_paths(values: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    let mut existing = Vec::new();
    let mut missing = Vec::new();

    for value in values {
        let normalized = value.trim();
        if normalized.is_empty() || !seen.insert(normalized.to_string()) {
            continue;
        }
        if Path::new(normalized).exists() {
            existing.push(normalized.to_string());
        } else {
            missing.push(normalized.to_string());
        }
    }

    existing.extend(missing);
    existing
}

async fn choose_with_osascript(
    kind: LocalSourcePathKind,
    prompt: &str,
) -> Result<Option<String>, crate::errors::AppError> {
    let chooser = match kind {
        LocalSourcePathKind::Directory => {
            "set chosenPath to POSIX path of (choose folder with prompt \""
        }
        LocalSourcePathKind::File => "set chosenPath to POSIX path of (choose file with prompt \"",
    };
    let script = format!(
        "{}{}\")\nreturn chosenPath",
        chooser,
        prompt.replace('"', "\\\""),
    );
    run_path_picker_command("osascript", [OsString::from("-e"), OsString::from(script)]).await
}

async fn choose_with_linux_dialog(
    kind: LocalSourcePathKind,
    prompt: &str,
) -> Result<Option<String>, crate::errors::AppError> {
    if let Ok(selection) = run_path_picker_command(
        "zenity",
        match kind {
            LocalSourcePathKind::Directory => vec![
                OsString::from("--file-selection"),
                OsString::from("--directory"),
                OsString::from("--title"),
                OsString::from(prompt),
            ],
            LocalSourcePathKind::File => vec![
                OsString::from("--file-selection"),
                OsString::from("--title"),
                OsString::from(prompt),
            ],
        },
    )
    .await
    {
        return Ok(selection);
    }

    let kdialog_args = match kind {
        LocalSourcePathKind::Directory => {
            vec![
                OsString::from("--getexistingdirectory"),
                OsString::from("."),
            ]
        }
        LocalSourcePathKind::File => vec![OsString::from("--getopenfilename"), OsString::from(".")],
    };
    run_path_picker_command("kdialog", kdialog_args).await
}

async fn choose_with_powershell(
    kind: LocalSourcePathKind,
    prompt: &str,
) -> Result<Option<String>, crate::errors::AppError> {
    let script = match kind {
        LocalSourcePathKind::Directory => format!(
            "Add-Type -AssemblyName System.Windows.Forms; \
$dialog = New-Object System.Windows.Forms.FolderBrowserDialog; \
$dialog.Description = '{}'; \
if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {{ Write-Output $dialog.SelectedPath }}",
            prompt.replace('\'', "''"),
        ),
        LocalSourcePathKind::File => format!(
            "Add-Type -AssemblyName System.Windows.Forms; \
$dialog = New-Object System.Windows.Forms.OpenFileDialog; \
$dialog.Title = '{}'; \
if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {{ Write-Output $dialog.FileName }}",
            prompt.replace('\'', "''"),
        ),
    };
    run_path_picker_command(
        "powershell",
        [
            OsString::from("-NoProfile"),
            OsString::from("-Command"),
            OsString::from(script),
        ],
    )
    .await
}

async fn run_path_picker_command<I>(
    program: &str,
    args: I,
) -> Result<Option<String>, crate::errors::AppError>
where
    I: IntoIterator<Item = OsString>,
{
    let output = tokio::process::Command::new(program)
        .args(args)
        .output()
        .await
        .map_err(|error| {
            crate::errors::AppError::bad_request(format!("path dialog unavailable: {}", error))
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        return Ok((!stdout.is_empty()).then_some(stdout));
    }

    if stderr.contains("User canceled")
        || stderr.contains("User cancelled")
        || stderr.contains("-128")
        || output.status.code() == Some(1)
    {
        return Ok(None);
    }

    Err(crate::errors::AppError::bad_request(format!(
        "path dialog failed: {}",
        if stderr.is_empty() { stdout } else { stderr }
    )))
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

    #[test]
    fn auto_discovers_macos_reminders_snapshot_from_home() {
        let home = unique_temp_dir("mac-reminders-home");
        let snapshot_path = home.join("Library/Application Support/Vel/reminders/snapshot.json");
        fs::create_dir_all(
            snapshot_path
                .parent()
                .expect("snapshot parent should exist"),
        )
        .expect("snapshot parent dir should be created");
        fs::write(&snapshot_path, "{}").expect("snapshot fixture should be written");

        let resolved = auto_local_source_path_from_home("reminders", &home)
            .expect("reminders snapshot should be discovered");
        assert_eq!(resolved, snapshot_path.to_string_lossy());

        let _ = fs::remove_dir_all(home);
    }

    #[test]
    fn parses_obsidian_vault_paths_from_config() {
        let suggestions = parse_obsidian_vault_paths(
            r#"{
              "vaults": {
                "work": { "path": "/Users/test/Notes/Work" },
                "personal": { "path": "/Users/test/Notes/Personal" }
              }
            }"#,
        );

        assert_eq!(
            suggestions,
            vec![
                "/Users/test/Notes/Personal".to_string(),
                "/Users/test/Notes/Work".to_string(),
            ]
        );
    }

    #[test]
    fn suggested_paths_include_existing_obsidian_vaults() {
        let home = unique_temp_dir("obsidian-home");
        let config_path = home.join("Library/Application Support/obsidian/obsidian.json");
        let work_vault = home.join("Notes/Work");
        let personal_vault = home.join("Notes/Personal");
        fs::create_dir_all(
            config_path
                .parent()
                .expect("obsidian config parent should exist"),
        )
        .expect("obsidian config dir should be created");
        fs::create_dir_all(&work_vault).expect("work vault should exist");
        fs::create_dir_all(&personal_vault).expect("personal vault should exist");
        fs::write(
            &config_path,
            format!(
                r#"{{
                  "vaults": {{
                    "work": {{ "path": "{}" }},
                    "personal": {{ "path": "{}" }}
                  }}
                }}"#,
                work_vault.to_string_lossy(),
                personal_vault.to_string_lossy(),
            ),
        )
        .expect("obsidian config should be written");

        let suggestions = discover_obsidian_vault_paths(&home, None, None);
        assert!(suggestions.contains(&work_vault.to_string_lossy().to_string()));
        assert!(suggestions.contains(&personal_vault.to_string_lossy().to_string()));

        let _ = fs::remove_dir_all(home);
    }

    #[test]
    fn suggested_activity_paths_prioritize_existing_host_sources() {
        let home = unique_temp_dir("activity-source-home");
        let zsh_history = home.join(".zsh_history");
        let internal_snapshot = home.join(".local/share/vel/activity/snapshot.json");
        fs::create_dir_all(
            internal_snapshot
                .parent()
                .expect("activity snapshot parent should exist"),
        )
        .expect("activity snapshot parent should be created");
        fs::write(&zsh_history, ": 1710000000:1;vel\n")
            .expect("zsh history fixture should be written");
        fs::write(&internal_snapshot, "{}").expect("activity snapshot fixture should be written");

        let _home_guard = EnvVarGuard::set("HOME", home.as_os_str().to_os_string());
        let suggestions = suggested_local_source_paths(
            "activity",
            None,
            Some("var/integrations/activity/snapshot.json"),
        );

        assert!(suggestions
            .available_paths
            .contains(&zsh_history.to_string_lossy().to_string()));
        assert!(suggestions
            .available_paths
            .contains(&internal_snapshot.to_string_lossy().to_string()));
        assert!(suggestions
            .internal_paths
            .contains(&"var/integrations/activity/snapshot.json".to_string()));

        let _ = fs::remove_dir_all(home);
    }
}
