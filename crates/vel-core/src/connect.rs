use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectInstanceStatus {
    Ready,
    Degraded,
    Offline,
    Unknown,
}

impl Display for ConnectInstanceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Offline => "offline",
            Self::Unknown => "unknown",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectRuntimeCapability {
    pub runtime_id: String,
    pub display_name: String,
    pub supports_launch: bool,
    pub supports_interactive_followup: bool,
    pub supports_native_open: bool,
    pub supports_host_agent_control: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectInstanceCapabilityManifest {
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub launchable_runtimes: Vec<ConnectRuntimeCapability>,
    pub supports_agent_launch: bool,
    pub supports_interactive_followup: bool,
    pub supports_native_open: bool,
    pub supports_host_agent_control: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectInstance {
    pub id: String,
    pub node_id: String,
    pub display_name: String,
    pub connection_id: Option<String>,
    pub status: ConnectInstanceStatus,
    pub reachability: String,
    pub sync_base_url: Option<String>,
    pub sync_transport: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub worker_ids: Vec<String>,
    pub worker_classes: Vec<String>,
    pub last_seen_at: Option<OffsetDateTime>,
    pub manifest: ConnectInstanceCapabilityManifest,
    pub metadata_json: JsonValue,
}
