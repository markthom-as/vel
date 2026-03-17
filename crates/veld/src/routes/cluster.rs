use axum::{extract::State, Json};
use vel_api_types::{
    ApiResponse, BranchSyncRequestData, ClusterBootstrapData, QueuedWorkRoutingData,
    SwarmClientsData, ValidationRequestData,
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
) -> Result<Json<ApiResponse<crate::services::client_sync::ClusterWorkersData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::cluster_workers_data(&state).await?;

    Ok(response::success(data))
}

#[allow(dead_code)]
pub async fn clients(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SwarmClientsData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = response::map_response(
        crate::services::client_sync::swarm_clients_data(&state).await?,
        "cluster clients response",
    )?;

    Ok(response::success(data))
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
