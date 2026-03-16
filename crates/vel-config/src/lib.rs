mod models;

pub use models::{load_model_profiles, load_routing, ModelProfile, RoutingConfig};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::Path};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:4130";
const DEFAULT_DB_PATH: &str = "var/data/vel.sqlite";
const DEFAULT_ARTIFACT_ROOT: &str = "var/artifacts";
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4130";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub bind_addr: String,
    pub db_path: String,
    pub artifact_root: String,
    pub log_level: String,
    pub base_url: String,
    /// Calendar: .ics URL for pull-based sync (optional).
    pub calendar_ics_url: Option<String>,
    /// Calendar: local .ics file path if URL not set (optional).
    pub calendar_ics_path: Option<String>,
    /// Todoist: path to snapshot JSON file (e.g. data/todoist/snapshot.json).
    pub todoist_snapshot_path: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("config validation: {0}")]
    Validation(String),
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bind_addr: DEFAULT_BIND_ADDR.to_string(),
            db_path: DEFAULT_DB_PATH.to_string(),
            artifact_root: DEFAULT_ARTIFACT_ROOT.to_string(),
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            base_url: DEFAULT_BASE_URL.to_string(),
            calendar_ics_url: None,
            calendar_ics_path: None,
            todoist_snapshot_path: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FileConfig {
    bind_addr: Option<String>,
    db_path: Option<String>,
    artifact_root: Option<String>,
    log_level: Option<String>,
    base_url: Option<String>,
    calendar_ics_url: Option<String>,
    calendar_ics_path: Option<String>,
    todoist_snapshot_path: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from_path("vel.toml")
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let mut config = Self::default();
        let path = path.as_ref();

        if path.exists() {
            let content = fs::read_to_string(path)?;
            let file = toml::from_str::<FileConfig>(&content)?;
            config.apply_file(file);
        }

        let env_map = env::vars().collect::<HashMap<_, _>>();
        config.apply_env_map(&env_map);
        Ok(config)
    }

    fn apply_file(&mut self, file: FileConfig) {
        if let Some(value) = file.bind_addr {
            self.bind_addr = value;
        }
        if let Some(value) = file.db_path {
            self.db_path = value;
        }
        if let Some(value) = file.artifact_root {
            self.artifact_root = value;
        }
        if let Some(value) = file.log_level {
            self.log_level = value;
        }
        if let Some(value) = file.base_url {
            self.base_url = value;
        }
        if file.calendar_ics_url.is_some() {
            self.calendar_ics_url = file.calendar_ics_url;
        }
        if file.calendar_ics_path.is_some() {
            self.calendar_ics_path = file.calendar_ics_path;
        }
        if file.todoist_snapshot_path.is_some() {
            self.todoist_snapshot_path = file.todoist_snapshot_path;
        }
    }

    pub fn apply_env_map(&mut self, env_map: &HashMap<String, String>) {
        if let Some(value) = env_map.get("VEL_BIND_ADDR") {
            self.bind_addr = value.clone();
        }
        if let Some(value) = env_map.get("VEL_DB_PATH") {
            self.db_path = value.clone();
        }
        if let Some(value) = env_map.get("VEL_ARTIFACT_ROOT") {
            self.artifact_root = value.clone();
        }
        if let Some(value) = env_map.get("VEL_LOG_LEVEL") {
            self.log_level = value.clone();
        }
        if let Some(value) = env_map.get("VEL_BASE_URL") {
            self.base_url = value.clone();
        }
        if let Some(value) = env_map.get("VEL_CALENDAR_ICS_URL") {
            self.calendar_ics_url = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_CALENDAR_ICS_PATH") {
            self.calendar_ics_path = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_TODOIST_SNAPSHOT_PATH") {
            self.todoist_snapshot_path = Some(value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn defaults_load_without_file() {
        let config = AppConfig::default();
        assert_eq!(config.bind_addr, DEFAULT_BIND_ADDR);
        assert_eq!(config.db_path, DEFAULT_DB_PATH);
    }

    #[test]
    fn env_map_overrides_defaults() {
        let mut config = AppConfig::default();
        let env_map = HashMap::from([
            ("VEL_BIND_ADDR".to_string(), "0.0.0.0:9999".to_string()),
            ("VEL_DB_PATH".to_string(), "/tmp/vel.sqlite".to_string()),
        ]);

        config.apply_env_map(&env_map);

        assert_eq!(config.bind_addr, "0.0.0.0:9999");
        assert_eq!(config.db_path, "/tmp/vel.sqlite");
    }
}

