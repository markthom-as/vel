use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use vel_api_types::{ApiResponse, ConnectInstanceData};
use vel_core::CapabilityDescriptor;

use crate::{errors::AppError, routes::response, services::connect_runtime, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ConnectLaunchRequest {
    pub runtime_kind: String,
    pub actor_id: String,
    #[serde(default)]
    pub display_name: Option<String>,
    pub command: Vec<String>,
    #[serde(default)]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub writable_roots: Vec<String>,
    #[serde(default)]
    pub capability_allowlist: Vec<CapabilityDescriptor>,
    #[serde(default)]
    pub lease_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectHeartbeatRequest {
    #[serde(default = "default_heartbeat_status")]
    pub status: String,
}

fn default_heartbeat_status() -> String {
    "healthy".to_string()
}

#[derive(Debug, Deserialize)]
pub struct ConnectTerminateRequest {
    #[serde(default = "default_terminate_reason")]
    pub reason: String,
}

fn default_terminate_reason() -> String {
    "operator_requested".to_string()
}

#[derive(Debug, Serialize)]
pub struct ConnectHeartbeatResponse {
    pub id: String,
    pub status: String,
    pub lease_expires_at: i64,
    pub trace_id: String,
}

pub async fn launch_connect_runtime(
    State(state): State<AppState>,
    Json(payload): Json<ConnectLaunchRequest>,
) -> Result<Json<ApiResponse<ConnectInstanceData>>, AppError> {
    let launched = connect_runtime::launch_connect_runtime(
        &state,
        connect_runtime::LaunchConnectRuntimeRequest {
            runtime_kind: payload.runtime_kind,
            actor_id: payload.actor_id,
            display_name: payload.display_name,
            command: payload.command,
            working_dir: payload.working_dir,
            writable_roots: payload.writable_roots,
            capability_allowlist: payload.capability_allowlist,
            lease_seconds: payload.lease_seconds,
        },
    )
    .await?;
    Ok(response::success(ConnectInstanceData::from(launched)))
}

pub async fn list_connect_instances(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ConnectInstanceData>>>, AppError> {
    let instances = connect_runtime::list_connect_instances(&state).await?;
    Ok(response::success(
        instances
            .into_iter()
            .map(ConnectInstanceData::from)
            .collect(),
    ))
}

pub async fn get_connect_instance(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ConnectInstanceData>>, AppError> {
    let instance = connect_runtime::get_connect_instance(&state, id.trim()).await?;
    Ok(response::success(ConnectInstanceData::from(instance)))
}

pub async fn heartbeat_connect_instance(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ConnectHeartbeatRequest>,
) -> Result<Json<ApiResponse<ConnectHeartbeatResponse>>, AppError> {
    let ack =
        connect_runtime::heartbeat_connect_instance(&state, id.trim(), &payload.status).await?;
    Ok(response::success(ConnectHeartbeatResponse {
        id: ack.id,
        status: ack.status,
        lease_expires_at: ack.lease_expires_at,
        trace_id: ack.trace_id,
    }))
}

pub async fn terminate_connect_instance(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ConnectTerminateRequest>,
) -> Result<Json<ApiResponse<ConnectInstanceData>>, AppError> {
    let instance =
        connect_runtime::terminate_connect_instance(&state, id.trim(), &payload.reason).await?;
    Ok(response::success(ConnectInstanceData::from(instance)))
}

pub fn connect_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v1/connect/instances",
            get(list_connect_instances).post(launch_connect_runtime),
        )
        .route("/v1/connect/instances/:id", get(get_connect_instance))
        .route(
            "/v1/connect/instances/:id/heartbeat",
            post(heartbeat_connect_instance),
        )
        .route(
            "/v1/connect/instances/:id/terminate",
            post(terminate_connect_instance),
        )
}
