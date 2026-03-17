use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, BranchSyncRequestData, ClusterBootstrapData, QueuedWorkRoutingData,
    SwarmClientsData, ValidationRequestData,
};

use crate::{errors::AppError, state::AppState};

pub async fn bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ClusterBootstrapData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::effective_cluster_bootstrap_data(&state).await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn workers(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<crate::services::client_sync::ClusterWorkersData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::cluster_workers_data(&state).await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn clients(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SwarmClientsData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::swarm_clients_data(&state).await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn branch_sync_request(
    State(state): State<AppState>,
    Json(payload): Json<BranchSyncRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::queue_branch_sync_request(
        &state,
        payload,
        "cluster_route",
        None,
    )
    .await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn validation_request(
    State(state): State<AppState>,
    Json(payload): Json<ValidationRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::client_sync::queue_validation_request(
        &state,
        payload,
        "cluster_route",
        None,
    )
    .await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
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
