use axum::{extract::State, Json};
use vel_api_types::{
    ApiResponse, BranchSyncRequestData, ClusterBootstrapData, ClusterWorkersData,
    QueuedWorkRoutingData, ValidationRequestData,
};

use crate::{errors::AppError, routes::response, state::AppState};

fn cluster_bootstrap_to_api(
    data: crate::services::client_sync::ClusterBootstrap,
    include_sensitive_state: bool,
) -> vel_api_types::ClusterBootstrapData {
    vel_api_types::ClusterBootstrapData {
        node_id: data.node_id,
        node_display_name: data.node_display_name,
        active_authority_node_id: data.active_authority_node_id,
        active_authority_epoch: data.active_authority_epoch,
        configured_base_url: data.configured_base_url,
        sync_base_url: data.sync_base_url,
        sync_transport: data.sync_transport,
        tailscale_base_url: data.tailscale_base_url,
        lan_base_url: data.lan_base_url,
        localhost_base_url: data.localhost_base_url,
        capabilities: data.capabilities,
        branch_sync: data
            .branch_sync
            .map(|branch_sync| vel_api_types::BranchSyncCapabilityData {
                repo_root: branch_sync.repo_root,
                default_remote: branch_sync.default_remote,
                supports_fetch: branch_sync.supports_fetch,
                supports_pull: branch_sync.supports_pull,
                supports_push: branch_sync.supports_push,
            }),
        validation_profiles: data
            .validation_profiles
            .into_iter()
            .map(|profile| vel_api_types::ValidationProfileData {
                profile_id: profile.profile_id,
                label: profile.label,
                command_hint: profile.command_hint,
                environment: profile.environment,
            })
            .collect(),
        linked_nodes: if include_sensitive_state {
            data.linked_nodes
                .into_iter()
                .map(vel_api_types::LinkedNodeData::from)
                .collect()
        } else {
            Vec::new()
        },
        projects: if include_sensitive_state {
            data.projects
                .into_iter()
                .map(vel_api_types::ProjectRecordData::from)
                .collect()
        } else {
            Vec::new()
        },
        action_items: if include_sensitive_state {
            data.action_items
                .into_iter()
                .map(vel_api_types::ActionItemData::from)
                .collect()
        } else {
            Vec::new()
        },
        pending_writebacks: if include_sensitive_state {
            data.pending_writebacks
                .into_iter()
                .map(vel_api_types::WritebackOperationData::from)
                .collect()
        } else {
            Vec::new()
        },
        conflicts: if include_sensitive_state {
            data.conflicts
                .into_iter()
                .map(vel_api_types::ConflictCaseData::from)
                .collect()
        } else {
            Vec::new()
        },
        people: if include_sensitive_state {
            data.people
                .into_iter()
                .map(vel_api_types::PersonRecordData::from)
                .collect()
        } else {
            Vec::new()
        },
    }
}

pub async fn bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterBootstrapData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::effective_cluster_bootstrap_data(&state).await?;
    let data = cluster_bootstrap_to_api(data, true);
    Ok(response::success(data))
}

pub async fn discovery_bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterBootstrapData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::effective_cluster_bootstrap_data(&state).await?;
    Ok(response::success(cluster_bootstrap_to_api(data, false)))
}

pub async fn workers(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterWorkersData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::cluster_workers_data(&state).await?;
    let workers = data
        .workers
        .into_iter()
        .map(|worker| vel_api_types::WorkerPresenceData {
            worker_id: worker.worker_id,
            node_id: worker.node_id,
            node_display_name: worker.node_display_name,
            client_kind: worker.client_kind,
            client_version: worker.client_version,
            protocol_version: worker.protocol_version,
            build_id: worker.build_id,
            worker_classes: worker.worker_classes,
            capabilities: worker.capabilities,
            status: worker.status,
            queue_depth: worker.queue_depth,
            reachability: worker.reachability,
            latency_class: worker.latency_class,
            compute_class: worker.compute_class,
            power_class: worker.power_class,
            recent_failure_rate: worker.recent_failure_rate,
            tailscale_preferred: worker.tailscale_preferred,
            last_heartbeat_at: worker.last_heartbeat_at,
            started_at: worker.started_at.unwrap_or_default(),
            sync_base_url: worker.sync_base_url,
            sync_transport: worker.sync_transport,
            tailscale_base_url: worker.tailscale_base_url,
            preferred_tailnet_endpoint: worker.preferred_tailnet_endpoint,
            tailscale_reachable: worker.tailscale_reachable,
            lan_base_url: worker.lan_base_url,
            localhost_base_url: worker.localhost_base_url,
            ping_ms: worker.ping_ms,
            sync_status: Some(worker.sync_status),
            last_upstream_sync_at: worker.last_upstream_sync_at,
            last_downstream_sync_at: worker.last_downstream_sync_at,
            last_sync_error: worker.last_sync_error,
            incoming_linking_prompt: worker.incoming_linking_prompt,
            capacity: vel_api_types::WorkerCapacityData {
                max_concurrency: worker.capacity.max_concurrency,
                current_load: worker.capacity.current_load,
                available_concurrency: worker.capacity.available_concurrency,
            },
        })
        .collect();

    Ok(response::success(vel_api_types::ClusterWorkersData {
        active_authority_node_id: data.active_authority_node_id,
        active_authority_epoch: data.active_authority_epoch,
        generated_at: data.generated_at,
        workers,
    }))
}

pub async fn branch_sync_request(
    State(state): State<AppState>,
    Json(payload): Json<BranchSyncRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    state.storage.healthcheck().await?;
    let request = crate::services::client_sync::BranchSyncRequest {
        repo_root: payload.repo_root,
        branch: payload.branch,
        remote: payload.remote,
        base_branch: payload.base_branch,
        mode: payload.mode,
        requested_by: payload.requested_by,
    };
    let data = crate::services::client_sync::queue_branch_sync_request(
        &state,
        request,
        "cluster_route",
        None,
    )
    .await?;

    Ok(response::success(
        crate::routes::sync::queued_work_routing_to_api(data),
    ))
}

pub async fn validation_request(
    State(state): State<AppState>,
    Json(payload): Json<ValidationRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    state.storage.healthcheck().await?;
    let request = crate::services::client_sync::ValidationRequest {
        repo_root: payload.repo_root,
        profile_id: payload.profile_id,
        branch: payload.branch,
        environment: payload.environment,
        requested_by: payload.requested_by,
    };
    let data = crate::services::client_sync::queue_validation_request(
        &state,
        request,
        "cluster_route",
        None,
    )
    .await?;

    Ok(response::success(
        crate::routes::sync::queued_work_routing_to_api(data),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn preferred_sync_target_prefers_tailscale() {
        let (url, transport) = crate::services::client_sync::preferred_sync_target(
            true,
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
            true,
            None,
            "http://127.0.0.1:4130",
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
        );
        assert_eq!(url, "http://127.0.0.1:4130");
        assert_eq!(transport, "localhost");
    }

    #[test]
    fn preferred_sync_target_can_disable_tailscale_preference() {
        let (url, transport) = crate::services::client_sync::preferred_sync_target(
            false,
            Some("http://vel.tailnet.ts.net:4130"),
            "http://127.0.0.1:4130",
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
        );
        assert_eq!(url, "http://127.0.0.1:4130");
        assert_eq!(transport, "localhost");
    }
}
