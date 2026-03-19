use std::{fs, path::PathBuf, time::Duration};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::{ApiResponse, ConnectInstanceData};
use vel_config::AppConfig;
use vel_storage::Storage;
use veld::{app::build_app, policy_config::PolicyConfig, state::AppState};

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_connect_runtime_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

async fn test_state() -> AppState {
    let storage = Storage::connect(":memory:").await.expect("storage");
    storage.migrate().await.expect("migrations");
    let (broadcast_tx, _) = broadcast::channel(16);
    let config = AppConfig {
        artifact_root: unique_dir("artifacts").to_string_lossy().to_string(),
        node_id: Some("vel-authority".to_string()),
        node_display_name: Some("Vel Authority".to_string()),
        ..Default::default()
    };
    AppState::new(
        storage,
        config,
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

fn request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request")
}

async fn decode_json<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice::<T>(&body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for status {}: {} ({})",
            status,
            String::from_utf8_lossy(&body),
            error
        )
    })
}

#[tokio::test]
async fn connect_runtime_launches_lists_heartbeats_inspects_and_terminates() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("workspace");

    let launch_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "local_command",
                "actor_id": "codex_local",
                "display_name": "Codex Local",
                "command": ["/bin/sh", "-lc", "sleep 30"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "capability_allowlist": [
                    { "scope": "read:repo", "resource": null, "action": "read" }
                ],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_payload: ApiResponse<ConnectInstanceData> = decode_json(launch_response).await;
    let launched = launch_payload.data.expect("launch data");
    assert_eq!(launched.status, "ready");
    assert_eq!(launched.display_name, "Codex Local");
    assert!(launched
        .metadata
        .get("trace_id")
        .and_then(serde_json::Value::as_str)
        .is_some());
    let run_id = launched.id.clone();

    let connect_run = storage
        .get_connect_run(&run_id)
        .await
        .expect("connect run lookup")
        .expect("connect run should exist");
    assert_eq!(connect_run.status, "running");
    let initial_lease = connect_run.lease_expires_at;

    let run = storage
        .get_run_by_id(&run_id)
        .await
        .expect("run lookup")
        .expect("run should exist");
    assert_eq!(run.status.to_string(), "running");
    assert_eq!(
        run.input_json
            .get("trace_id")
            .and_then(serde_json::Value::as_str),
        launched
            .metadata
            .get("trace_id")
            .and_then(serde_json::Value::as_str)
    );

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/connect/instances")
                .body(Body::empty())
                .expect("list request"),
        )
        .await
        .expect("list response");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_payload: ApiResponse<Vec<ConnectInstanceData>> = decode_json(list_response).await;
    assert_eq!(list_payload.data.expect("list data").len(), 1);

    let heartbeat_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/heartbeat"),
            json!({ "status": "healthy" }),
        ))
        .await
        .expect("heartbeat response");
    assert_eq!(heartbeat_response.status(), StatusCode::OK);

    let heartbeat_connect_run = storage
        .get_connect_run(&run_id)
        .await
        .expect("connect run lookup")
        .expect("connect run should still exist");
    assert!(heartbeat_connect_run.lease_expires_at >= initial_lease);

    let inspect_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/connect/instances/{run_id}"))
                .body(Body::empty())
                .expect("inspect request"),
        )
        .await
        .expect("inspect response");
    assert_eq!(inspect_response.status(), StatusCode::OK);
    let inspect_payload: ApiResponse<ConnectInstanceData> = decode_json(inspect_response).await;
    let inspected = inspect_payload.data.expect("inspect data");
    assert_eq!(inspected.id, run_id);
    assert_eq!(inspected.status, "ready");
    assert!(inspected
        .metadata
        .get("trace_id")
        .and_then(serde_json::Value::as_str)
        .is_some());

    let terminate_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/terminate"),
            json!({ "reason": "operator_requested" }),
        ))
        .await
        .expect("terminate response");
    assert_eq!(terminate_response.status(), StatusCode::OK);

    let terminated_connect_run = storage
        .get_connect_run(&run_id)
        .await
        .expect("connect run lookup")
        .expect("terminated run should exist");
    assert_eq!(terminated_connect_run.status, "terminated");
    assert_eq!(
        terminated_connect_run.terminal_reason.as_deref(),
        Some("operator_requested")
    );

    let terminated_run = storage
        .get_run_by_id(&run_id)
        .await
        .expect("run lookup")
        .expect("terminated run should exist");
    assert_eq!(terminated_run.status.to_string(), "cancelled");
}

#[tokio::test]
async fn connect_runtime_expires_without_heartbeat() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("expiry_workspace");

    let launch_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "local_command",
                "actor_id": "codex_expiry",
                "display_name": "Expiry Probe",
                "command": ["/bin/sh", "-lc", "sleep 30"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "capability_allowlist": [],
                "lease_seconds": 1
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_payload: ApiResponse<ConnectInstanceData> = decode_json(launch_response).await;
    let run_id = launch_payload.data.expect("launch data").id;

    tokio::time::sleep(Duration::from_secs(2)).await;

    let inspect_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/connect/instances/{run_id}"))
                .body(Body::empty())
                .expect("inspect request"),
        )
        .await
        .expect("inspect response");
    assert_eq!(inspect_response.status(), StatusCode::OK);

    let connect_run = storage
        .get_connect_run(&run_id)
        .await
        .expect("connect run lookup")
        .expect("connect run should exist");
    assert_eq!(connect_run.status, "expired");

    let run = storage
        .get_run_by_id(&run_id)
        .await
        .expect("run lookup")
        .expect("run should exist");
    assert_eq!(run.status.to_string(), "expired");
}

#[tokio::test]
async fn connect_runtime_rejects_unsupported_runtime_kind() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("unsupported_workspace");

    let response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "wasm_guest",
                "actor_id": "codex_denied",
                "command": ["/bin/sh", "-lc", "sleep 30"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "capability_allowlist": [],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(storage
        .list_connect_runs(None)
        .await
        .expect("list connect runs")
        .is_empty());
}

#[tokio::test]
async fn connect_runtime_rejects_out_of_scope_writable_root() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("allowed_workspace");
    let outside = unique_dir("outside_workspace");

    let response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "local_command",
                "actor_id": "codex_denied",
                "command": ["/bin/sh", "-lc", "sleep 30"],
                "working_dir": workspace,
                "writable_roots": [outside],
                "capability_allowlist": [],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert!(storage
        .list_connect_runs(None)
        .await
        .expect("list connect runs")
        .is_empty());
}

fn build_app_with_state(state: AppState) -> axum::Router {
    build_app(
        state.storage.clone(),
        state.config.clone(),
        state.policy_config.clone(),
        state.llm_router.clone(),
        state.chat_profile_id.clone(),
    )
}
