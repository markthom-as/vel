use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::UnixSeconds;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRuntimeCapabilityData {
    pub runtime_id: String,
    pub display_name: String,
    pub supports_launch: bool,
    pub supports_interactive_followup: bool,
    pub supports_native_open: bool,
    pub supports_host_agent_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectInstanceCapabilityManifestData {
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub launchable_runtimes: Vec<ConnectRuntimeCapabilityData>,
    pub supports_agent_launch: bool,
    pub supports_interactive_followup: bool,
    pub supports_native_open: bool,
    pub supports_host_agent_control: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectInstanceData {
    pub id: String,
    pub node_id: String,
    pub display_name: String,
    pub connection_id: Option<String>,
    pub status: String,
    pub reachability: String,
    pub sync_base_url: Option<String>,
    pub sync_transport: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub worker_ids: Vec<String>,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    pub last_seen_at: Option<UnixSeconds>,
    pub manifest: ConnectInstanceCapabilityManifestData,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRunEventData {
    pub id: i64,
    pub run_id: String,
    pub stream: String,
    pub chunk: String,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectStdinWriteAckData {
    pub run_id: String,
    pub accepted_bytes: u32,
    pub event_id: i64,
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectAttachData {
    pub instance: ConnectInstanceData,
    pub latest_event_id: Option<i64>,
    pub stream_path: String,
}

impl From<vel_core::ConnectRuntimeCapability> for ConnectRuntimeCapabilityData {
    fn from(capability: vel_core::ConnectRuntimeCapability) -> Self {
        Self {
            runtime_id: capability.runtime_id,
            display_name: capability.display_name,
            supports_launch: capability.supports_launch,
            supports_interactive_followup: capability.supports_interactive_followup,
            supports_native_open: capability.supports_native_open,
            supports_host_agent_control: capability.supports_host_agent_control,
        }
    }
}

impl From<vel_core::ConnectInstanceCapabilityManifest> for ConnectInstanceCapabilityManifestData {
    fn from(manifest: vel_core::ConnectInstanceCapabilityManifest) -> Self {
        Self {
            worker_classes: manifest.worker_classes,
            capabilities: manifest.capabilities,
            launchable_runtimes: manifest
                .launchable_runtimes
                .into_iter()
                .map(ConnectRuntimeCapabilityData::from)
                .collect(),
            supports_agent_launch: manifest.supports_agent_launch,
            supports_interactive_followup: manifest.supports_interactive_followup,
            supports_native_open: manifest.supports_native_open,
            supports_host_agent_control: manifest.supports_host_agent_control,
        }
    }
}

impl From<vel_core::ConnectInstance> for ConnectInstanceData {
    fn from(instance: vel_core::ConnectInstance) -> Self {
        Self {
            id: instance.id,
            node_id: instance.node_id,
            display_name: instance.display_name,
            connection_id: instance.connection_id,
            status: instance.status.to_string(),
            reachability: instance.reachability,
            sync_base_url: instance.sync_base_url,
            sync_transport: instance.sync_transport,
            tailscale_base_url: instance.tailscale_base_url,
            lan_base_url: instance.lan_base_url,
            localhost_base_url: instance.localhost_base_url,
            worker_ids: instance.worker_ids,
            worker_classes: instance.worker_classes,
            last_seen_at: instance
                .last_seen_at
                .map(|seen_at| seen_at.unix_timestamp()),
            manifest: instance.manifest.into(),
            metadata: instance.metadata_json,
        }
    }
}
