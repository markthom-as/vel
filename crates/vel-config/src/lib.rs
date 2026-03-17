mod models;

pub use models::{load_model_profiles, load_routing, ModelProfile, RoutingConfig};

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:4130";
const DEFAULT_DB_PATH: &str = "var/data/vel.sqlite";
const DEFAULT_ARTIFACT_ROOT: &str = "var/artifacts";
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4130";
const DEFAULT_AGENT_SPEC_PATH: &str = "config/agent-specs.yaml";
const DEFAULT_LLM_MODEL_PATH: &str =
    "configs/models/weights/qwen3-coder-30b-a3b-instruct-q4_k_m.gguf";
const DEFAULT_LLM_FAST_MODEL_PATH: &str =
    "configs/models/weights/qwen2.5-coder-14b-instruct-q4_k_m.gguf";
const DEFAULT_CALENDAR_ICS_PATH: &str = "var/integrations/calendar/local.ics";
const DEFAULT_TODOIST_SNAPSHOT_PATH: &str = "var/integrations/todoist/snapshot.json";
const DEFAULT_ACTIVITY_SNAPSHOT_PATH: &str = "var/integrations/activity/snapshot.json";
const DEFAULT_HEALTH_SNAPSHOT_PATH: &str = "var/integrations/health/snapshot.json";
const DEFAULT_GIT_SNAPSHOT_PATH: &str = "var/integrations/git/snapshot.json";
const DEFAULT_MESSAGING_SNAPSHOT_PATH: &str = "var/integrations/messaging/snapshot.json";
const DEFAULT_NOTES_PATH: &str = "var/integrations/notes";
const DEFAULT_TRANSCRIPT_SNAPSHOT_PATH: &str = "var/integrations/transcripts/snapshot.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub bind_addr: String,
    pub db_path: String,
    pub artifact_root: String,
    pub log_level: String,
    pub base_url: String,
    pub node_id: Option<String>,
    pub node_display_name: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub agent_spec_path: Option<String>,
    pub llm_model_path: String,
    pub llm_fast_model_path: String,
    /// Calendar: .ics URL for pull-based sync (optional).
    pub calendar_ics_url: Option<String>,
    /// Calendar: local .ics file path if URL not set (optional).
    pub calendar_ics_path: Option<String>,
    /// Todoist: path to snapshot JSON file (e.g. data/todoist/snapshot.json).
    pub todoist_snapshot_path: Option<String>,
    /// Activity: path to workstation activity snapshot JSON.
    pub activity_snapshot_path: Option<String>,
    /// Health: path to local health/activity snapshot JSON.
    pub health_snapshot_path: Option<String>,
    /// Git: path to local git activity snapshot JSON.
    pub git_snapshot_path: Option<String>,
    /// Messaging: path to local messaging snapshot JSON.
    pub messaging_snapshot_path: Option<String>,
    /// Notes: file or directory path for markdown/plaintext note sync.
    pub notes_path: Option<String>,
    /// Transcripts: path to assistant/chat transcript snapshot JSON.
    pub transcript_snapshot_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub id: String,
    pub kind: AgentSpecKind,
    pub mission: String,
    pub ttl_seconds: u64,
    pub allowed_tools: Vec<String>,
    pub memory_scope: AgentMemoryScope,
    pub return_contract: String,
    #[serde(default)]
    pub budgets: Option<AgentBudgets>,
}

impl AgentSpec {
    fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("agent spec id is required".to_string());
        }
        if self.mission.trim().is_empty() {
            return Err(format!("agent spec {} mission is required", self.id));
        }
        if self.allowed_tools.is_empty() {
            return Err(format!(
                "agent spec {} requires at least one allowed tool",
                self.id
            ));
        }
        if self.return_contract.trim().is_empty() {
            return Err(format!(
                "agent spec {} return_contract is required",
                self.id
            ));
        }
        if self.ttl_seconds == 0 {
            return Err(format!(
                "agent spec {} ttl_seconds must be greater than zero",
                self.id
            ));
        }
        if self.allowed_tools.iter().any(|tool| tool.trim().is_empty()) {
            return Err(format!(
                "agent spec {} has empty allowed_tool entry",
                self.id
            ));
        }
        if self
            .memory_scope
            .topic_pads
            .iter()
            .any(|topic| topic.trim().is_empty())
        {
            return Err(format!("agent spec {} has empty topic_pad", self.id));
        }
        if let Some(budgets) = &self.budgets {
            budgets.validate(&self.id)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentSpecKind {
    Subagent,
    Supervisor,
    Specialist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemoryScope {
    pub constitution: bool,
    #[serde(default)]
    pub topic_pads: Vec<String>,
    #[serde(default)]
    pub event_query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBudgets {
    #[serde(default)]
    pub max_tool_calls: Option<u32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub max_memory_queries: Option<u32>,
    #[serde(default)]
    pub max_side_effects: Option<u32>,
}

impl AgentBudgets {
    fn validate(&self, spec_id: &str) -> Result<(), String> {
        if let Some(max_tool_calls) = self.max_tool_calls {
            if max_tool_calls == 0 {
                return Err(format!(
                    "agent spec {} max_tool_calls must be greater than zero",
                    spec_id
                ));
            }
        }
        if let Some(max_tokens) = self.max_tokens {
            if max_tokens == 0 {
                return Err(format!(
                    "agent spec {} max_tokens must be greater than zero",
                    spec_id
                ));
            }
        }
        if let Some(max_memory_queries) = self.max_memory_queries {
            if max_memory_queries == 0 {
                return Err(format!(
                    "agent spec {} max_memory_queries must be greater than zero",
                    spec_id
                ));
            }
        }
        if let Some(max_side_effects) = self.max_side_effects {
            if max_side_effects == 0 {
                return Err(format!(
                    "agent spec {} max_side_effects must be greater than zero",
                    spec_id
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum AgentSpecDocument {
    Single(AgentSpec),
    List(Vec<AgentSpec>),
    Wrapped { specs: Vec<AgentSpec> },
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
            node_id: None,
            node_display_name: None,
            tailscale_base_url: None,
            lan_base_url: None,
            agent_spec_path: Some(default_agent_spec_path()),
            llm_model_path: DEFAULT_LLM_MODEL_PATH.to_string(),
            llm_fast_model_path: DEFAULT_LLM_FAST_MODEL_PATH.to_string(),
            calendar_ics_url: None,
            calendar_ics_path: Some(DEFAULT_CALENDAR_ICS_PATH.to_string()),
            todoist_snapshot_path: Some(DEFAULT_TODOIST_SNAPSHOT_PATH.to_string()),
            activity_snapshot_path: Some(DEFAULT_ACTIVITY_SNAPSHOT_PATH.to_string()),
            health_snapshot_path: Some(DEFAULT_HEALTH_SNAPSHOT_PATH.to_string()),
            git_snapshot_path: Some(DEFAULT_GIT_SNAPSHOT_PATH.to_string()),
            messaging_snapshot_path: Some(DEFAULT_MESSAGING_SNAPSHOT_PATH.to_string()),
            notes_path: Some(DEFAULT_NOTES_PATH.to_string()),
            transcript_snapshot_path: Some(DEFAULT_TRANSCRIPT_SNAPSHOT_PATH.to_string()),
        }
    }
}

pub fn is_default_local_source_path(kind: &str, path: &str) -> bool {
    match kind {
        "calendar" => path == DEFAULT_CALENDAR_ICS_PATH,
        "todoist" => path == DEFAULT_TODOIST_SNAPSHOT_PATH,
        "activity" => path == DEFAULT_ACTIVITY_SNAPSHOT_PATH,
        "health" => path == DEFAULT_HEALTH_SNAPSHOT_PATH,
        "git" => path == DEFAULT_GIT_SNAPSHOT_PATH,
        "messaging" => path == DEFAULT_MESSAGING_SNAPSHOT_PATH,
        "notes" => path == DEFAULT_NOTES_PATH,
        "transcripts" => path == DEFAULT_TRANSCRIPT_SNAPSHOT_PATH,
        _ => false,
    }
}

fn default_agent_spec_path() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(DEFAULT_AGENT_SPEC_PATH)
        .to_string_lossy()
        .to_string()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FileConfig {
    bind_addr: Option<String>,
    db_path: Option<String>,
    artifact_root: Option<String>,
    log_level: Option<String>,
    base_url: Option<String>,
    node_id: Option<String>,
    node_display_name: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    agent_spec_path: Option<String>,
    llm_model_path: Option<String>,
    llm_fast_model_path: Option<String>,
    calendar_ics_url: Option<String>,
    calendar_ics_path: Option<String>,
    todoist_snapshot_path: Option<String>,
    activity_snapshot_path: Option<String>,
    health_snapshot_path: Option<String>,
    git_snapshot_path: Option<String>,
    messaging_snapshot_path: Option<String>,
    notes_path: Option<String>,
    transcript_snapshot_path: Option<String>,
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
        if let Some(path) = &self.agent_spec_path {
            Self::load_agent_specs_from_path(path)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn load_agent_specs_from_path(
        path: impl AsRef<Path>,
    ) -> Result<Vec<AgentSpec>, ConfigError> {
        let content = fs::read_to_string(path)?;
        let spec_doc = serde_yaml::from_str::<AgentSpecDocument>(&content)?;
        let specs = match spec_doc {
            AgentSpecDocument::Single(spec) => vec![spec],
            AgentSpecDocument::List(specs) => specs,
            AgentSpecDocument::Wrapped { specs } => specs,
        };

        let mut seen_ids = HashSet::new();
        for spec in &specs {
            spec.validate().map_err(ConfigError::Validation)?;
            if !seen_ids.insert(spec.id.clone()) {
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
        if file.node_id.is_some() {
            self.node_id = file.node_id;
        }
        if file.node_display_name.is_some() {
            self.node_display_name = file.node_display_name;
        }
        if file.tailscale_base_url.is_some() {
            self.tailscale_base_url = file.tailscale_base_url;
        }
        if file.lan_base_url.is_some() {
            self.lan_base_url = file.lan_base_url;
        }
        if file.agent_spec_path.is_some() {
            self.agent_spec_path = file.agent_spec_path;
        }
        if let Some(value) = file.llm_model_path {
            self.llm_model_path = value;
        }
        if let Some(value) = file.llm_fast_model_path {
            self.llm_fast_model_path = value;
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
        if file.health_snapshot_path.is_some() {
            self.health_snapshot_path = file.health_snapshot_path;
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
        if let Some(value) = env_map.get("VEL_NODE_ID") {
            self.node_id = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_NODE_DISPLAY_NAME") {
            self.node_display_name = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_TAILSCALE_BASE_URL") {
            self.tailscale_base_url = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_LAN_BASE_URL") {
            self.lan_base_url = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_AGENT_SPEC_PATH") {
            self.agent_spec_path = Some(value.clone());
        }
        if let Some(value) = env_map.get("VEL_LLM_MODEL") {
            self.llm_model_path = value.clone();
        }
        if let Some(value) = env_map.get("VEL_LLM_FAST_MODEL") {
            self.llm_fast_model_path = value.clone();
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
        if let Some(value) = env_map.get("VEL_HEALTH_SNAPSHOT_PATH") {
            self.health_snapshot_path = Some(value.clone());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn defaults_load_without_file() {
        let config = AppConfig::default();
        assert_eq!(config.bind_addr, DEFAULT_BIND_ADDR);
        assert_eq!(config.db_path, DEFAULT_DB_PATH);
        assert_eq!(config.llm_model_path, DEFAULT_LLM_MODEL_PATH);
        assert_eq!(config.llm_fast_model_path, DEFAULT_LLM_FAST_MODEL_PATH);
        assert_eq!(
            config.agent_spec_path.as_deref(),
            Some(default_agent_spec_path().as_str())
        );
        assert_eq!(
            config.calendar_ics_path.as_deref(),
            Some(DEFAULT_CALENDAR_ICS_PATH)
        );
        assert_eq!(
            config.todoist_snapshot_path.as_deref(),
            Some(DEFAULT_TODOIST_SNAPSHOT_PATH)
        );
        assert_eq!(
            config.activity_snapshot_path.as_deref(),
            Some(DEFAULT_ACTIVITY_SNAPSHOT_PATH)
        );
        assert_eq!(
            config.health_snapshot_path.as_deref(),
            Some(DEFAULT_HEALTH_SNAPSHOT_PATH)
        );
        assert_eq!(
            config.git_snapshot_path.as_deref(),
            Some(DEFAULT_GIT_SNAPSHOT_PATH)
        );
        assert_eq!(
            config.messaging_snapshot_path.as_deref(),
            Some(DEFAULT_MESSAGING_SNAPSHOT_PATH)
        );
        assert_eq!(config.notes_path.as_deref(), Some(DEFAULT_NOTES_PATH));
        assert_eq!(
            config.transcript_snapshot_path.as_deref(),
            Some(DEFAULT_TRANSCRIPT_SNAPSHOT_PATH)
        );
    }

    #[test]
    fn env_map_overrides_defaults() {
        let mut config = AppConfig::default();
        let env_map = HashMap::from([
            ("VEL_BIND_ADDR".to_string(), "0.0.0.0:9999".to_string()),
            ("VEL_DB_PATH".to_string(), "/tmp/vel.sqlite".to_string()),
            ("VEL_NODE_ID".to_string(), "vel-desktop".to_string()),
            (
                "VEL_TAILSCALE_BASE_URL".to_string(),
                "http://vel-desktop.tailnet.ts.net:4130".to_string(),
            ),
            (
                "VEL_LLM_MODEL".to_string(),
                "/tmp/qwen3-coder-30b.gguf".to_string(),
            ),
            (
                "VEL_LLM_FAST_MODEL".to_string(),
                "/tmp/qwen25-fast-14b.gguf".to_string(),
            ),
            (
                "VEL_MESSAGING_SNAPSHOT_PATH".to_string(),
                "/tmp/messaging.json".to_string(),
            ),
            (
                "VEL_HEALTH_SNAPSHOT_PATH".to_string(),
                "/tmp/health.json".to_string(),
            ),
            (
                "VEL_AGENT_SPEC_PATH".to_string(),
                "/tmp/agent-specs.yaml".to_string(),
            ),
        ]);

        config.apply_env_map(&env_map);

        assert_eq!(config.bind_addr, "0.0.0.0:9999");
        assert_eq!(config.db_path, "/tmp/vel.sqlite");
        assert_eq!(config.node_id.as_deref(), Some("vel-desktop"));
        assert_eq!(
            config.tailscale_base_url.as_deref(),
            Some("http://vel-desktop.tailnet.ts.net:4130")
        );
        assert_eq!(config.llm_model_path, "/tmp/qwen3-coder-30b.gguf");
        assert_eq!(config.llm_fast_model_path, "/tmp/qwen25-fast-14b.gguf");
        assert_eq!(
            config.health_snapshot_path.as_deref(),
            Some("/tmp/health.json")
        );
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
    fn load_agent_specs_reads_yaml_file() {
        let temp = Path::new("/tmp").join("vel-agent-specs-runtime-skeleton.yaml");
        let yaml = r#"
id: research_agent
kind: subagent
mission: gather relevant information about a topic and return structured findings
ttl_seconds: 180
allowed_tools:
  - web.search
  - web.fetch
memory_scope:
  constitution: true
  topic_pads:
    - project_vel
  event_query: limited
return_contract: research_summary_v1
budgets:
  max_tool_calls: 12
  max_tokens: 24000
"#;
        fs::write(&temp, yaml).unwrap();
        let specs = AppConfig::load_agent_specs_from_path(&temp).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].id, "research_agent");
        let _ = fs::remove_file(&temp);
    }

    #[test]
    fn load_agent_specs_uses_default_repo_path() {
        let config = AppConfig::default();
        let specs = config.load_agent_specs().unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].id, "research_agent");
    }

    #[test]
    fn load_agent_specs_rejects_duplicate_ids() {
        let temp = Path::new("/tmp").join("vel-agent-specs-runtime-skeleton-dup.yaml");
        let yaml = r#"
- id: research_agent
  kind: subagent
  mission: one
  ttl_seconds: 180
  allowed_tools:
    - web.search
  memory_scope:
    constitution: true
  return_contract: research_summary_v1
- id: research_agent
  kind: subagent
  mission: two
  ttl_seconds: 120
  allowed_tools:
    - web.search
  memory_scope:
    constitution: false
    topic_pads:
      - project_vel
  return_contract: research_summary_v1
"#;
        fs::write(&temp, yaml).unwrap();
        let result = AppConfig::load_agent_specs_from_path(&temp);
        assert!(result.is_err());
        let _ = fs::remove_file(&temp);
    }
}
