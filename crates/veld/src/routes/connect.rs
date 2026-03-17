use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ConnectInstanceCapabilityManifestData, ConnectInstanceData,
    ConnectRuntimeCapabilityData,
};
use vel_core::ConnectInstance;

use crate::{errors::AppError, state::AppState};

#[allow(dead_code)]
pub async fn list_instances(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ConnectInstanceData>>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::connect::list_connect_instances(&state)
        .await?
        .into_iter()
        .map(map_connect_instance_to_dto)
        .collect::<Vec<_>>();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[allow(dead_code)]
pub async fn get_instance(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
) -> Result<Json<ApiResponse<ConnectInstanceData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::connect::get_connect_instance(&state, instance_id.trim())
        .await?
        .map(map_connect_instance_to_dto)
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

fn map_connect_instance_to_dto(instance: ConnectInstance) -> ConnectInstanceData {
    ConnectInstanceData {
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
            .map(|timestamp| timestamp.unix_timestamp()),
        manifest: ConnectInstanceCapabilityManifestData {
            worker_classes: instance.manifest.worker_classes,
            capabilities: instance.manifest.capabilities,
            launchable_runtimes: instance
                .manifest
                .launchable_runtimes
                .into_iter()
                .map(|runtime| ConnectRuntimeCapabilityData {
                    runtime_id: runtime.runtime_id,
                    display_name: runtime.display_name,
                    supports_launch: runtime.supports_launch,
                    supports_interactive_followup: runtime.supports_interactive_followup,
                    supports_native_open: runtime.supports_native_open,
                    supports_host_agent_control: runtime.supports_host_agent_control,
                })
                .collect(),
            supports_agent_launch: instance.manifest.supports_agent_launch,
            supports_interactive_followup: instance.manifest.supports_interactive_followup,
            supports_native_open: instance.manifest.supports_native_open,
            supports_host_agent_control: instance.manifest.supports_host_agent_control,
        },
        metadata: instance.metadata_json,
    }
}
