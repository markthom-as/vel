mod models;

pub use models::{load_model_profiles, load_routing, ModelProfile, RoutingConfig};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:4130";
const DEFAULT_DB_PATH: &str = "var/data/vel.sqlite";
const DEFAULT_ARTIFACT_ROOT: &str = "var/artifacts";
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4130";
const DEFAULT_AGENT_SPEC_PATH: &str = "config/agent-specs.yaml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub bind_addr: String,
    pub db_path: String,
    pub artifact_root: String,
    pub log_level: String,
    pub base_url: String,
    /// Optional path to the agent spec bundle (YAML).
    pub agent_spec_path: Option<String>,
    /// Calendar: .ics URL for pull-based sync (optional).
    pub calendar_ics_url: Option<String>,
    /// Calendar: local .ics file path if URL not set (optional).
    pub calendar_ics_path: Option<String>,
    /// Todoist: path to snapshot JSON file (e.g. data/todoist/snapshot.json).
    pub todoist_snapshot_path: Option<String>,
    /// Activity: path to workstation activity snapshot JSON.
    pub activity_snapshot_path: Option<String>,
    /// Git: path to local git activity snapshot JSON.
    pub git_snapshot_path: Option<String>,
    /// Messaging: path to local messaging snapshot JSON.
    pub messaging_snapshot_path: Option<String>,
    /// Notes: file or directory path for markdown/plaintext note sync.
    pub notes_path: Option<String>,
    /// Transcripts: path to assistant/chat transcript snapshot JSON.
    pub transcript_snapshot_path: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("failed to parse agent spec file: {0}")]
    AgentSpecParse(#[from] serde_yaml::Error),
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
            agent_spec_path: Some(DEFAULT_AGENT_SPEC_PATH.to_string()),
            calendar_ics_url: None,
            calendar_ics_path: None,
            todoist_snapshot_path: None,
            activity_snapshot_path: None,
            git_snapshot_path: None,
            messaging_snapshot_path: None,
            notes_path: None,
            transcript_snapshot_path: None,
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
    agent_spec_path: Option<String>,
    calendar_ics_url: Option<String>,
    calendar_ics_path: Option<String>,
    todoist_snapshot_path: Option<String>,
    activity_snapshot_path: Option<String>,
    git_snapshot_path: Option<String>,
    messaging_snapshot_path: Option<String>,
    notes_path: Option<String>,
    transcript_snapshot_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub id: String,
    pub mission: String,
    #[serde(default)]
    pub kind: AgentSpecKind,
    pub allowed_tools: Vec<String>,
    pub memory_scope: AgentMemoryScope,
    pub return_contract: String,
    pub ttl_seconds: u64,
    #[serde(default)]
    pub budgets: AgentBudgets,
    #[serde(default)]
    pub mission_input_schema: Option<JsonValue>,
    #[serde(default)]
    pub side_effect_policy: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentSpecKind {
    Subagent,
    Supervisor,
    Specialist,
}

impl Default for AgentSpecKind {
    fn default() -> Self {
        Self::Subagent
    }
}

impl std::fmt::Display for AgentSpecKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Subagent => "subagent",
            Self::Supervisor => "supervisor",
            Self::Specialist => "specialist",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryScope {
    #[serde(default)]
    pub constitution: bool,
    #[serde(default)]
    pub topic_pads: Vec<String>,
    #[serde(default = "default_event_query")]
    pub event_query: String,
}

impl Default for AgentMemoryScope {
    fn default() -> Self {
        Self {
            constitution: false,
            topic_pads: Vec::new(),
            event_query: default_event_query(),
        }
    }
}

fn default_event_query() -> String {
    "limited".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBudgets {
    #[serde(default = "default_max_tool_calls")]
    pub max_tool_calls: u32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub max_memory_queries: Option<u32>,
    #[serde(default)]
    pub max_side_effects: Option<u32>,
}

impl Default for AgentBudgets {
    fn default() -> Self {
        Self {
            max_tool_calls: default_max_tool_calls(),
            max_tokens: default_max_tokens(),
            max_memory_queries: None,
            max_side_effects: None,
        }
    }
}

fn default_max_tool_calls() -> u32 {
    12
}

fn default_max_tokens() -> u32 {
    24_000
}

impl AgentSpec {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.id.trim().is_empty() {
            return Err(ConfigError::Validation(
                "agent spec id must be set".to_string(),
            ));
        }
        if self.mission.trim().is_empty() {
            return Err(ConfigError::Validation(
                "agent spec mission must be set".to_string(),
            ));
        }
        if self.allowed_tools.is_empty() {
            return Err(ConfigError::Validation(
                "agent spec allowed_tools must be non-empty".to_string(),
            ));
        }
        if self.return_contract.trim().is_empty() {
            return Err(ConfigError::Validation(
                "agent spec return_contract must be set".to_string(),
            ));
        }
        if self.ttl_seconds == 0 {
            return Err(ConfigError::Validation(
                "agent spec ttl_seconds must be > 0".to_string(),
            ));
        }
        if self.memory_scope.event_query.trim().is_empty() {
            return Err(ConfigError::Validation(
                "agent spec memory_scope.event_query must be set".to_string(),
            ));
        }
        if !self.budgets.validate() {
            return Err(ConfigError::Validation(
                "agent spec budgets are invalid".to_string(),
            ));
        }
        Ok(())
    }
}

impl AgentBudgets {
    pub fn validate(&self) -> bool {
        self.max_tool_calls > 0
            && self.max_tokens > 0
            && self.max_memory_queries.unwrap_or(1) > 0
            && self.max_side_effects.unwrap_or(1) > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum AgentSpecFile {
    Bare(Vec<AgentSpec>),
    Wrapped { specs: Vec<AgentSpec> },
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

    pub fn load_agent_specs(&self) -> Result<Vec<AgentSpec>, ConfigError> {
        let Some(path) = self.agent_spec_path.as_deref() else {
            return Ok(Vec::new());
        };

        Self::load_agent_specs_from_path(path)
    }

    pub fn load_agent_specs_from_path(
        path: impl AsRef<Path>,
    ) -> Result<Vec<AgentSpec>, ConfigError> {
        let resolved_path = resolve_agent_spec_path(path.as_ref());
        let content = fs::read_to_string(&resolved_path)?;
        let file: AgentSpecFile = serde_yaml::from_str(&content)?;
        let specs = match file {
            AgentSpecFile::Bare(specs) => specs,
            AgentSpecFile::Wrapped { specs } => specs,
        };

        let mut seen = HashSet::new();
        for spec in &specs {
            spec.validate()?;
            if !seen.insert(spec.id.clone()) {
                return Err(ConfigError::Validation(format!(
                    "duplicate agent spec id: {}",
                    spec.id
                )));
            }
        }
        Ok(specs)
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
        if file.agent_spec_path.is_some() {
            self.agent_spec_path = file.agent_spec_path;
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
        if file.activity_snapshot_path.is_some() {
            self.activity_snapshot_path = file.activity_snapshot_path;
        }
        if file.git_snapshot_path.is_some() {
            self.git_snapshot_path = file.git_snapshot_path;
        }
        if file.messaging_snapshot_path.is_some() {
            self.messaging_snapshot_path = file.messaging_snapshot_path;
        }
        if file.notes_path.is_some() {
            self.notes_path = file.notes_path;
        }
        if file.transcript_snapshot_path.is_some() {
            self.transcript_snapshot_path = file.transcript_snapshot_path;
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
        if let Some(value) = env_map.get("VEL_AGENT_SPEC_PATH") {
            self.agent_spec_path = Some(value.clone());
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
        if let Some(value) = env_map.get("VEL_ACTIVITY_SNAPSHOT_PATH") {
            self.activity_snapshot_path = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_GIT_SNAPSHOT_PATH") {
            self.git_snapshot_path = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_MESSAGING_SNAPSHOT_PATH") {
            self.messaging_snapshot_path = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_NOTES_PATH") {
            self.notes_path = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_TRANSCRIPT_SNAPSHOT_PATH") {
            self.transcript_snapshot_path = Some(value.clone());
        }
    }
}

fn resolve_agent_spec_path(path: &Path) -> PathBuf {
    if path.is_absolute() || path.exists() {
        return path.to_path_buf();
    }

    let workspace_relative = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path);
    if workspace_relative.exists() {
        return workspace_relative;
    }

    path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn tmp_path(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        std::env::temp_dir().join(format!("vel_config_{}_{}.yaml", name, unique))
    }

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
            (
                "VEL_MESSAGING_SNAPSHOT_PATH".to_string(),
                "/tmp/messaging.json".to_string(),
            ),
            (
                "VEL_AGENT_SPEC_PATH".to_string(),
                "/tmp/agent-specs.yaml".to_string(),
            ),
        ]);

        config.apply_env_map(&env_map);

        assert_eq!(config.bind_addr, "0.0.0.0:9999");
        assert_eq!(config.db_path, "/tmp/vel.sqlite");
        assert_eq!(
            config.messaging_snapshot_path.as_deref(),
            Some("/tmp/messaging.json")
        );
        assert_eq!(
            config.agent_spec_path.as_deref(),
            Some("/tmp/agent-specs.yaml")
        );
    }

    #[test]
    fn load_agent_specs_from_path_validates_file() {
        let path = tmp_path("valid");
        let fixture = r#"
specs:
  - id: research_agent
    mission: gather relevant information
    kind: subagent
    allowed_tools:
      - web.search
      - web.fetch
    memory_scope:
      constitution: true
      topic_pads:
        - project_vel
      event_query: limited
    return_contract: research_summary_v1
    ttl_seconds: 180
    budgets:
      max_tool_calls: 12
      max_tokens: 24000
      max_memory_queries: 5
      max_side_effects: 1
"#;
        std::fs::write(&path, fixture).unwrap();

        let specs = AppConfig::load_agent_specs_from_path(&path).unwrap();

        let _ = std::fs::remove_file(&path);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].id, "research_agent");
        assert_eq!(specs[0].kind.to_string(), "subagent");
        assert_eq!(specs[0].allowed_tools.len(), 2);
    }

    #[test]
    fn load_agent_specs_without_path_uses_bundled_defaults() {
        let config = AppConfig::default();
        let specs = config.load_agent_specs().unwrap();
        assert!(!specs.is_empty());
        assert!(specs.iter().any(|spec| spec.id == "research_agent"));
    }
}
