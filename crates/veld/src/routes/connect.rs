use axum::{
    extract::{Path, Query, State},
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, convert::Infallible, time::Duration};
use vel_api_types::{
    ApiResponse, ConnectAttachData, ConnectInstanceData, ConnectRunEventData,
    ConnectStdinWriteAckData,
};
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

#[derive(Debug, Deserialize)]
pub struct ConnectStdinRequest {
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectEventsQuery {
    #[serde(default)]
    pub after_id: Option<i64>,
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectEventStreamQuery {
    #[serde(default)]
    pub after_id: Option<i64>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub poll_ms: Option<u64>,
    #[serde(default)]
    pub max_events: Option<u32>,
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

pub async fn attach_connect_instance(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ConnectAttachData>>, AppError> {
    let id = id.trim();
    let instance = connect_runtime::get_connect_instance(&state, id).await?;
    let latest_event_id = connect_runtime::latest_connect_instance_event_id(&state, id).await?;
    let stream_path = if let Some(after_id) = latest_event_id {
        format!("/v1/connect/instances/{id}/events/stream?after_id={after_id}")
    } else {
        format!("/v1/connect/instances/{id}/events/stream")
    };
    Ok(response::success(ConnectAttachData {
        instance: ConnectInstanceData::from(instance),
        latest_event_id,
        stream_path,
    }))
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

pub async fn write_connect_instance_stdin(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ConnectStdinRequest>,
) -> Result<Json<ApiResponse<ConnectStdinWriteAckData>>, AppError> {
    let ack =
        connect_runtime::write_connect_instance_stdin(&state, id.trim(), &payload.input).await?;
    Ok(response::success(ConnectStdinWriteAckData {
        run_id: ack.run_id,
        accepted_bytes: ack.accepted_bytes,
        event_id: ack.event_id,
        trace_id: ack.trace_id,
    }))
}

pub async fn list_connect_instance_events(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<ConnectEventsQuery>,
) -> Result<Json<ApiResponse<Vec<ConnectRunEventData>>>, AppError> {
    let events = connect_runtime::list_connect_instance_events(
        &state,
        id.trim(),
        query.after_id,
        query.limit.unwrap_or(200),
    )
    .await?;
    Ok(response::success(
        events
            .into_iter()
            .map(|value| ConnectRunEventData {
                id: value.id,
                run_id: value.run_id,
                stream: value.stream,
                chunk: value.chunk,
                created_at: value.created_at,
            })
            .collect(),
    ))
}

pub async fn stream_connect_instance_events(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<ConnectEventStreamQuery>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, Infallible>>>, AppError> {
    let id = id.trim().to_string();
    connect_runtime::get_connect_instance(&state, &id).await?;

    let state_for_stream = state.clone();
    let poll_ms = query.poll_ms.unwrap_or(500).clamp(50, 5_000);
    let limit = query.limit.unwrap_or(200).clamp(1, 1_000);
    let stream = stream::unfold(
        ConnectEventStreamState {
            app_state: state_for_stream,
            instance_id: id,
            after_id: query.after_id,
            limit,
            poll_ms,
            max_events: query.max_events.map(|value| value.max(1)),
            emitted_events: 0,
            pending: VecDeque::new(),
            done: false,
        },
        |mut stream_state| async move {
            loop {
                if stream_state.done {
                    return None;
                }

                if let Some(event) = stream_state.pending.pop_front() {
                    stream_state.after_id = Some(event.id);
                    stream_state.emitted_events += 1;

                    let payload = serde_json::to_string(&ConnectRunEventData {
                        id: event.id,
                        run_id: event.run_id,
                        stream: event.stream,
                        chunk: event.chunk,
                        created_at: event.created_at,
                    })
                    .unwrap_or_else(|_| "{}".to_string());
                    let sse_event = Event::default().event("connect_event").data(payload);

                    if let Some(max_events) = stream_state.max_events {
                        if stream_state.emitted_events >= max_events {
                            stream_state.done = true;
                        }
                    }

                    return Some((Ok(sse_event), stream_state));
                }

                match connect_runtime::list_connect_instance_events(
                    &stream_state.app_state,
                    &stream_state.instance_id,
                    stream_state.after_id,
                    stream_state.limit,
                )
                .await
                {
                    Ok(events) if events.is_empty() => {
                        tokio::time::sleep(Duration::from_millis(stream_state.poll_ms)).await;
                    }
                    Ok(events) => {
                        stream_state.pending = events.into();
                    }
                    Err(error) => {
                        let sse_event = Event::default()
                            .event("connect_error")
                            .data(error.to_string());
                        stream_state.done = true;
                        return Some((Ok(sse_event), stream_state));
                    }
                }
            }
        },
    );

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    ))
}

#[derive(Clone)]
struct ConnectEventStreamState {
    app_state: AppState,
    instance_id: String,
    after_id: Option<i64>,
    limit: u32,
    poll_ms: u64,
    max_events: Option<u32>,
    emitted_events: u32,
    pending: VecDeque<vel_storage::ConnectRunEventRecord>,
    done: bool,
}

pub fn connect_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v1/connect/instances",
            get(list_connect_instances).post(launch_connect_runtime),
        )
        .route("/v1/connect/instances/:id", get(get_connect_instance))
        .route(
            "/v1/connect/instances/:id/attach",
            get(attach_connect_instance),
        )
        .route(
            "/v1/connect/instances/:id/events",
            get(list_connect_instance_events),
        )
        .route(
            "/v1/connect/instances/:id/events/stream",
            get(stream_connect_instance_events),
        )
        .route(
            "/v1/connect/instances/:id/heartbeat",
            post(heartbeat_connect_instance),
        )
        .route(
            "/v1/connect/instances/:id/stdin",
            post(write_connect_instance_stdin),
        )
        .route(
            "/v1/connect/instances/:id/terminate",
            post(terminate_connect_instance),
        )
}
