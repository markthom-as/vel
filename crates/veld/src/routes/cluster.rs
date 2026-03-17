use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use vel_api_types::{
    ApiResponse, BranchSyncRequestData, ClusterBootstrapData, QueuedWorkRoutingData,
    ValidationRequestData,
};

use crate::{errors::AppError, routes::response, state::AppState};

pub async fn bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterBootstrapData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = response::map_response(
        crate::services::client_sync::effective_cluster_bootstrap_data(&state).await?,
        "cluster bootstrap response",
    )?;

    Ok(response::success(data))
}

pub async fn workers(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterWorkersData>>, AppError> {
    state.storage.healthcheck().await?;
    let data: ClusterWorkersData = response::map_response(
        crate::services::client_sync::cluster_workers_data(&state).await?,
        "cluster workers response",
    )?;

    Ok(response::success(data))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorkersData {
    pub active_authority_node_id: String,
    pub active_authority_epoch: i64,
    pub generated_at: i64,
    pub workers: Vec<ClusterWorkerPresenceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorkerPresenceData {
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
    pub last_heartbeat_at: i64,
    pub started_at: i64,
    pub sync_base_url: String,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub ping_ms: Option<u32>,
    pub sync_status: Option<String>,
    pub last_upstream_sync_at: Option<i64>,
    pub last_downstream_sync_at: Option<i64>,
    pub last_sync_error: Option<String>,
    pub capacity: WorkerCapacityData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapacityData {
    pub max_concurrency: u32,
    pub current_load: u32,
    pub available_concurrency: u32,
}

impl From<crate::services::client_sync::ClusterWorkersData> for ClusterWorkersData {
    fn from(data: crate::services::client_sync::ClusterWorkersData) -> Self {
        Self {
            active_authority_node_id: data.active_authority_node_id,
            active_authority_epoch: data.active_authority_epoch,
            generated_at: data.generated_at,
            workers: data
                .workers
                .into_iter()
                .map(ClusterWorkerPresenceData::from)
                .collect(),
        }
    }
}

impl From<crate::services::client_sync::WorkerPresenceData> for ClusterWorkerPresenceData {
    fn from(data: crate::services::client_sync::WorkerPresenceData) -> Self {
        Self {
            worker_id: data.worker_id,
            node_id: data.node_id,
            node_display_name: data.node_display_name,
            client_kind: data.client_kind,
            client_version: data.client_version,
            protocol_version: data.protocol_version,
            build_id: data.build_id,
            worker_classes: data.worker_classes,
            capabilities: data.capabilities,
            status: data.status,
            queue_depth: data.queue_depth,
            reachability: data.reachability,
            latency_class: data.latency_class,
            compute_class: data.compute_class,
            power_class: data.power_class,
            recent_failure_rate: data.recent_failure_rate,
            tailscale_preferred: data.tailscale_preferred,
            last_heartbeat_at: data.last_heartbeat_at,
            started_at: data.started_at,
            sync_base_url: data.sync_base_url,
            sync_transport: data.sync_transport,
            tailscale_base_url: data.tailscale_base_url,
            preferred_tailnet_endpoint: data.preferred_tailnet_endpoint,
            tailscale_reachable: data.tailscale_reachable,
            lan_base_url: data.lan_base_url,
            localhost_base_url: data.localhost_base_url,
            ping_ms: data.ping_ms,
            sync_status: data.sync_status,
            last_upstream_sync_at: data.last_upstream_sync_at,
            last_downstream_sync_at: data.last_downstream_sync_at,
            last_sync_error: data.last_sync_error,
            capacity: data.capacity.into(),
        }
    }
}

impl From<crate::services::client_sync::WorkerCapacityData> for WorkerCapacityData {
    fn from(data: crate::services::client_sync::WorkerCapacityData) -> Self {
        Self {
            max_concurrency: data.max_concurrency,
            current_load: data.current_load,
            available_concurrency: data.available_concurrency,
        }
    }
}

pub async fn branch_sync_request(
    State(state): State<AppState>,
    Json(payload): Json<BranchSyncRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    state.storage.healthcheck().await?;
    let request: crate::services::client_sync::BranchSyncRequestData =
        response::map_request(payload, "branch sync request")?;
    let data = response::map_response(
        crate::services::client_sync::queue_branch_sync_request(
            &state,
            request,
            "cluster_route",
            None,
        )
        .await?,
        "branch sync routing response",
    )?;

    Ok(response::success(data))
}

pub async fn validation_request(
    State(state): State<AppState>,
    Json(payload): Json<ValidationRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    state.storage.healthcheck().await?;
    let request: crate::services::client_sync::ValidationRequestData =
        response::map_request(payload, "validation request")?;
    let data = response::map_response(
        crate::services::client_sync::queue_validation_request(
            &state,
            request,
            "cluster_route",
            None,
        )
        .await?,
        "validation routing response",
    )?;

    Ok(response::success(data))
}

#[cfg(test)]
mod tests {
    #[test]
    fn preferred_sync_target_prefers_tailscale() {
        let (url, transport) = crate::services::client_sync::preferred_sync_target(
            Some("http://vel.tailnet.ts.net:4130"),
            "http://127.0.0.1:4130",
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
        );
        assert_eq!(url, "http://vel.tailnet.ts.net:4130");
        assert_eq!(transport, "tailscale");
    }

    #[test]
    fn preferred_sync_target_prefers_localhost_when_no_tailscale() {
        let (url, transport) = crate::services::client_sync::preferred_sync_target(
            None,
            "http://127.0.0.1:4130",
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
        );
        assert_eq!(url, "http://127.0.0.1:4130");
        assert_eq!(transport, "localhost");
    }
}
