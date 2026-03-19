use std::{fs, path::PathBuf};

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
use veld::{policy_config::PolicyConfig, state::AppState};

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_wasm_guest_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn write_guest_spec(dir: &PathBuf, name: &str, spec: serde_json::Value) -> String {
    let path = dir.join(name);
    fs::write(&path, serde_json::to_vec_pretty(&spec).expect("spec json"))
        .expect("guest spec should write");
    path.to_string_lossy().to_string()
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

fn request(uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request")
}

async fn decode_json<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice::<T>(&body).expect("json body")
}

#[tokio::test]
async fn wasm_guest_launches_with_approved_bounded_host_call() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("workspace");
    let write_root = workspace.join("write_root");
    fs::create_dir_all(&write_root).expect("write root");
    let spec_path = write_guest_spec(
        &workspace,
        "guest_ok.json",
        json!({
            "module_id": "guest_ok",
            "requested_writable_roots": [write_root],
            "requested_hosts": [],
            "host_calls": [
                { "kind": "read_context", "query": "now" }
            ]
        }),
    );

    let response = app
        .clone()
        .oneshot(request(
            "/v1/connect/instances",
            json!({
                "runtime_kind": "wasm_guest",
                "actor_id": "guest_runner",
                "display_name": "Guest Runner",
                "command": [spec_path],
                "working_dir": workspace,
                "writable_roots": [write_root],
                "capability_allowlist": [
                    { "scope": "read:context", "resource": null, "action": "read" }
                ],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let payload: ApiResponse<ConnectInstanceData> = decode_json(response).await;
    let instance = payload.data.expect("connect instance");
    assert_eq!(instance.status, "offline");
    assert_eq!(
        instance
            .metadata
            .get("terminal_reason")
            .and_then(serde_json::Value::as_str),
        Some("guest_completed")
    );

    let run = storage
        .get_run_by_id(&instance.id)
        .await
        .expect("run lookup")
        .expect("run should exist");
    assert_eq!(run.status.to_string(), "succeeded");
}

#[tokio::test]
async fn wasm_guest_denies_out_of_scope_capability_request() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("workspace");
    let write_root = workspace.join("write_root");
    fs::create_dir_all(&write_root).expect("write root");
    let spec_path = write_guest_spec(
        &workspace,
        "guest_denied.json",
        json!({
            "module_id": "guest_denied",
            "requested_writable_roots": [write_root],
            "requested_hosts": [],
            "host_calls": [
                { "kind": "request_capability", "capability": "execute:todoist", "reason": "complete upstream task" }
            ]
        }),
    );

    let response = app
        .clone()
        .oneshot(request(
            "/v1/connect/instances",
            json!({
                "runtime_kind": "wasm_guest",
                "actor_id": "guest_runner",
                "command": [spec_path],
                "working_dir": workspace,
                "writable_roots": [write_root],
                "capability_allowlist": [
                    { "scope": "read:context", "resource": null, "action": "read" }
                ]
            }),
        ))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let connect_runs = storage.list_connect_runs(None).await.expect("connect runs");
    let run_id = connect_runs
        .first()
        .expect("connect run should be persisted")
        .id
        .clone();
    let run = storage
        .get_run_by_id(&run_id)
        .await
        .expect("run lookup")
        .expect("run should exist");
    assert_eq!(run.status.to_string(), "failed");
}

#[tokio::test]
async fn wasm_guest_rejects_write_scope_or_network_expansion() {
    let state = test_state().await;
    let app = build_app_with_state(state);
    let workspace = unique_dir("workspace");
    let declared_root = workspace.join("declared_root");
    fs::create_dir_all(&declared_root).expect("declared root");
    let other_root = unique_dir("other_root");
    let spec_path = write_guest_spec(
        &workspace,
        "guest_scope_violation.json",
        json!({
            "module_id": "guest_scope_violation",
            "requested_writable_roots": [other_root],
            "requested_hosts": ["api.example.com"],
            "host_calls": [
                { "kind": "read_context", "query": "now" }
            ]
        }),
    );

    let response = app
        .oneshot(request(
            "/v1/connect/instances",
            json!({
                "runtime_kind": "wasm_guest",
                "actor_id": "guest_runner",
                "command": [spec_path],
                "working_dir": workspace,
                "writable_roots": [declared_root],
                "capability_allowlist": [
                    { "scope": "read:context", "resource": null, "action": "read" }
                ]
            }),
        ))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

fn build_app_with_state(state: AppState) -> axum::Router {
    veld::app::build_app_with_state(state)
}
