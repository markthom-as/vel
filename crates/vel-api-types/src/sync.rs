use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{ClusterBootstrapData, LinkingPromptData, UnixSeconds};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchSyncCapabilityData {
    pub repo_root: String,
    pub default_remote: String,
    pub supports_fetch: bool,
    pub supports_pull: bool,
    pub supports_push: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationProfileData {
    pub profile_id: String,
    pub label: String,
    pub command_hint: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchSyncRequestData {
    pub repo_root: String,
    pub branch: String,
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub base_branch: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequestData {
    pub repo_root: String,
    pub profile_id: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueuedWorkRoutingKindData {
    BranchSync,
    Validation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedWorkRoutingData {
    pub work_request_id: String,
    pub request_type: QueuedWorkRoutingKindData,
    pub status: String,
    pub queued_signal_id: String,
    pub queued_signal_type: String,
    pub queued_at: UnixSeconds,
    pub queued_via: String,
    pub authority_node_id: String,
    pub authority_epoch: i64,
    pub target_node_id: String,
    pub target_worker_class: String,
    pub requested_capability: String,
    #[serde(default)]
    pub request_payload: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementRecommendationData {
    pub worker_id: String,
    pub node_id: String,
    pub capability: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHeartbeatResponseData {
    pub accepted: bool,
    pub worker_id: String,
    pub expires_at: UnixSeconds,
    pub cluster_view_version: UnixSeconds,
    #[serde(default)]
    pub placement_hints: Vec<PlacementRecommendationData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkAssignmentStatusData {
    Assigned,
    Started,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentReceiptData {
    pub receipt_id: String,
    pub work_request_id: String,
    pub worker_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    pub status: WorkAssignmentStatusData,
    pub assigned_at: UnixSeconds,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    #[serde(default)]
    pub completed_at: Option<UnixSeconds>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
    pub last_updated: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimRequestData {
    pub work_request_id: String,
    pub worker_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimNextRequestData {
    pub node_id: String,
    pub worker_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentUpdateRequest {
    pub receipt_id: String,
    pub status: WorkAssignmentStatusData,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    #[serde(default)]
    pub completed_at: Option<UnixSeconds>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedWorkItemData {
    pub work_request_id: String,
    pub request_type: QueuedWorkRoutingKindData,
    pub queued_signal_id: String,
    pub queued_signal_type: String,
    pub queued_at: UnixSeconds,
    pub target_node_id: String,
    pub target_worker_class: String,
    pub requested_capability: String,
    pub request_payload: JsonValue,
    #[serde(default)]
    pub latest_receipt: Option<WorkAssignmentReceiptData>,
    pub is_stale: bool,
    pub attempt_count: u32,
    pub claimable_now: bool,
    #[serde(default)]
    pub claim_reason: Option<String>,
    #[serde(default)]
    pub next_retry_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimedWorkData {
    pub queue_item: QueuedWorkItemData,
    pub receipt: WorkAssignmentReceiptData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignmentClaimNextResponseData {
    #[serde(default)]
    pub claim: Option<WorkAssignmentClaimedWorkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHeartbeatRequestData {
    pub node_id: String,
    #[serde(default)]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub client_kind: Option<String>,
    #[serde(default)]
    pub client_version: Option<String>,
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub build_id: Option<String>,
    pub worker_id: String,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub max_concurrency: Option<u32>,
    #[serde(default)]
    pub current_load: Option<u32>,
    #[serde(default)]
    pub queue_depth: Option<u32>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub latency_class: Option<String>,
    #[serde(default)]
    pub compute_class: Option<String>,
    #[serde(default)]
    pub power_class: Option<String>,
    #[serde(default)]
    pub recent_failure_rate: Option<f64>,
    #[serde(default)]
    pub tailscale_preferred: Option<bool>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub tailscale_base_url: Option<String>,
    #[serde(default)]
    pub preferred_tailnet_endpoint: Option<String>,
    #[serde(default)]
    pub tailscale_reachable: Option<bool>,
    #[serde(default)]
    pub lan_base_url: Option<String>,
    #[serde(default)]
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub ping_ms: Option<u32>,
    #[serde(default)]
    pub sync_status: Option<String>,
    #[serde(default)]
    pub last_upstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_downstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub last_heartbeat_at: Option<UnixSeconds>,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeStateData {
    pub node_id: String,
    #[serde(default, alias = "display_name")]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub node_class: Option<String>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub tailscale_base_url: Option<String>,
    #[serde(default)]
    pub lan_base_url: Option<String>,
    #[serde(default)]
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub last_seen_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmClientActiveWorkData {
    pub receipt_id: String,
    pub work_request_id: String,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    pub status: String,
    pub assigned_at: UnixSeconds,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    pub last_updated: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorkerStateData {
    #[serde(alias = "id")]
    pub worker_id: String,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub client_kind: Option<String>,
    #[serde(default)]
    pub client_version: Option<String>,
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub build_id: Option<String>,
    #[serde(default)]
    pub worker_class: Option<String>,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub max_concurrency: Option<u32>,
    #[serde(default)]
    pub current_load: Option<u32>,
    #[serde(default)]
    pub queue_depth: Option<u32>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub latency_class: Option<String>,
    #[serde(default)]
    pub compute_class: Option<String>,
    #[serde(default)]
    pub power_class: Option<String>,
    #[serde(default)]
    pub recent_failure_rate: Option<f64>,
    #[serde(default)]
    pub tailscale_preferred: Option<bool>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub tailscale_base_url: Option<String>,
    #[serde(default)]
    pub preferred_tailnet_endpoint: Option<String>,
    #[serde(default)]
    pub tailscale_reachable: Option<bool>,
    #[serde(default)]
    pub lan_base_url: Option<String>,
    #[serde(default)]
    pub localhost_base_url: Option<String>,
    #[serde(default)]
    pub ping_ms: Option<u32>,
    #[serde(default)]
    pub heartbeat_age_seconds: Option<UnixSeconds>,
    #[serde(default)]
    pub sync_status: Option<String>,
    #[serde(default)]
    pub last_upstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_downstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub last_heartbeat_at: Option<UnixSeconds>,
    #[serde(default)]
    pub started_at: Option<UnixSeconds>,
    #[serde(default)]
    pub available_concurrency: Option<u32>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub active_work: Vec<SwarmClientActiveWorkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmClientData {
    pub client_id: String,
    pub node_id: String,
    #[serde(default)]
    pub node_display_name: Option<String>,
    #[serde(default)]
    pub client_kind: Option<String>,
    #[serde(default)]
    pub client_version: Option<String>,
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub build_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub reachability: Option<String>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub sync_base_url: Option<String>,
    #[serde(default)]
    pub ping_ms: Option<u32>,
    #[serde(default)]
    pub heartbeat_age_seconds: Option<UnixSeconds>,
    #[serde(default)]
    pub last_heartbeat_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_upstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub last_downstream_sync_at: Option<UnixSeconds>,
    #[serde(default)]
    pub sync_status: Option<String>,
    #[serde(default)]
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub worker_classes: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub max_concurrency: Option<u32>,
    #[serde(default)]
    pub current_load: Option<u32>,
    #[serde(default)]
    pub queue_depth: Option<u32>,
    #[serde(default)]
    pub active_work: Vec<SwarmClientActiveWorkData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncClusterStateData {
    #[serde(default)]
    pub cluster_view_version: Option<i64>,
    #[serde(default)]
    pub authority_node_id: Option<String>,
    #[serde(default)]
    pub authority_epoch: Option<i64>,
    #[serde(default)]
    pub sync_transport: Option<String>,
    #[serde(default)]
    pub cluster: Option<ClusterBootstrapData>,
    #[serde(default)]
    pub nodes: Vec<ClusterNodeStateData>,
    #[serde(default)]
    pub workers: Vec<ClusterWorkerStateData>,
    #[serde(default)]
    pub clients: Vec<SwarmClientData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapacityData {
    pub max_concurrency: u32,
    pub current_load: u32,
    pub available_concurrency: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResultData {
    pub source: String,
    pub signals_ingested: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerPresenceData {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: String,
    pub client_kind: Option<String>,
    pub client_version: Option<String>,
    pub protocol_version: Option<String>,
    pub build_id: Option<String>,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub status: String,
    pub queue_depth: u32,
    pub reachability: String,
    pub latency_class: String,
    pub compute_class: String,
    pub power_class: String,
    pub recent_failure_rate: f64,
    pub tailscale_preferred: bool,
    pub last_heartbeat_at: UnixSeconds,
    pub started_at: UnixSeconds,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub ping_ms: Option<u32>,
    pub sync_status: Option<String>,
    pub last_upstream_sync_at: Option<UnixSeconds>,
    pub last_downstream_sync_at: Option<UnixSeconds>,
    pub last_sync_error: Option<String>,
    #[serde(default)]
    pub incoming_linking_prompt: Option<LinkingPromptData>,
    pub capacity: WorkerCapacityData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorkersData {
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub generated_at: UnixSeconds,
    pub workers: Vec<WorkerPresenceData>,
}
