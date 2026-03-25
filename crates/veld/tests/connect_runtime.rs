use std::{fs, path::PathBuf, time::Duration};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::{
    ApiResponse, ConnectInstanceData, ConnectRunEventData, ConnectStdinWriteAckData,
};
use vel_config::AppConfig;
use vel_core::{ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRootRef, ProjectStatus};
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
async fn connect_runtime_accepts_stdin_and_persists_output_events() {
    let state = test_state().await;
    let app = build_app_with_state(state);
    let workspace = unique_dir("stdin_workspace");

    let launch_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "local_command",
                "actor_id": "codex_stdio",
                "display_name": "Codex Stdio",
                "command": ["/bin/sh", "-lc", "cat"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "capability_allowlist": [],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_payload: ApiResponse<ConnectInstanceData> = decode_json(launch_response).await;
    let run_id = launch_payload.data.expect("launch data").id;

    let stdin_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/stdin"),
            json!({ "input": "hello from stdin\n" }),
        ))
        .await
        .expect("stdin response");
    assert_eq!(stdin_response.status(), StatusCode::OK);
    let stdin_payload: ApiResponse<ConnectStdinWriteAckData> = decode_json(stdin_response).await;
    assert!(stdin_payload.data.expect("stdin ack").accepted_bytes > 0);

    let mut events = Vec::new();
    let mut saw_stdout_echo = false;
    for _ in 0..20 {
        let events_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/v1/connect/instances/{run_id}/events?limit=50"))
                    .body(Body::empty())
                    .expect("events request"),
            )
            .await
            .expect("events response");
        assert_eq!(events_response.status(), StatusCode::OK);
        let events_payload: ApiResponse<Vec<ConnectRunEventData>> =
            decode_json(events_response).await;
        events = events_payload.data.expect("events payload");
        saw_stdout_echo = events
            .iter()
            .any(|event| event.stream == "stdout" && event.chunk.contains("hello from stdin"));
        if saw_stdout_echo {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    assert!(
        events.iter().any(|event| event.stream == "stdin"),
        "stdin event should be persisted"
    );
    assert!(saw_stdout_echo, "stdout should include cat echo");

    let terminate_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/terminate"),
            json!({ "reason": "test_done" }),
        ))
        .await
        .expect("terminate response");
    assert_eq!(terminate_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn connect_runtime_stream_endpoint_replays_and_streams_events() {
    let state = test_state().await;
    let app = build_app_with_state(state);
    let workspace = unique_dir("stream_workspace");

    let launch_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "local_command",
                "actor_id": "codex_stream",
                "display_name": "Codex Stream",
                "command": ["/bin/sh", "-lc", "cat"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "capability_allowlist": [],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_payload: ApiResponse<ConnectInstanceData> = decode_json(launch_response).await;
    let run_id = launch_payload.data.expect("launch data").id;

    let stdin_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/stdin"),
            json!({ "input": "stream replay line\n" }),
        ))
        .await
        .expect("stdin response");
    assert_eq!(stdin_response.status(), StatusCode::OK);
    let stdin_payload: ApiResponse<ConnectStdinWriteAckData> = decode_json(stdin_response).await;
    let stdin_event_id = stdin_payload.data.expect("stdin ack").event_id;

    let replay_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/v1/connect/instances/{run_id}/events/stream?max_events=1&poll_ms=50"
                ))
                .body(Body::empty())
                .expect("replay stream request"),
        )
        .await
        .expect("replay stream response");
    assert_eq!(replay_response.status(), StatusCode::OK);
    let replay_body = tokio::time::timeout(
        Duration::from_secs(2),
        axum::body::to_bytes(replay_response.into_body(), usize::MAX),
    )
    .await
    .expect("replay stream timeout")
    .expect("replay body");
    let replay_events = parse_connect_sse_events(&replay_body);
    assert_eq!(replay_events.len(), 1);
    assert_eq!(replay_events[0].stream, "stdin");

    let stream_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/v1/connect/instances/{run_id}/events/stream?after_id={stdin_event_id}&max_events=1&poll_ms=50"
                ))
                .body(Body::empty())
                .expect("live stream request"),
        )
        .await
        .expect("live stream response");
    assert_eq!(stream_response.status(), StatusCode::OK);

    let stdin_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/stdin"),
            json!({ "input": "stream live line\n" }),
        ))
        .await
        .expect("stdin response");
    assert_eq!(stdin_response.status(), StatusCode::OK);

    let live_body = tokio::time::timeout(
        Duration::from_secs(2),
        axum::body::to_bytes(stream_response.into_body(), usize::MAX),
    )
    .await
    .expect("live stream timeout")
    .expect("live body");
    let live_events = parse_connect_sse_events(&live_body);
    assert_eq!(live_events.len(), 1);
    assert!(live_events[0].id > stdin_event_id);

    let terminate_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/terminate"),
            json!({ "reason": "test_done" }),
        ))
        .await
        .expect("terminate response");
    assert_eq!(terminate_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn connect_runtime_attach_returns_latest_cursor_and_stream_path() {
    let state = test_state().await;
    let app = build_app_with_state(state);
    let workspace = unique_dir("attach_workspace");

    let launch_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/connect/instances",
            json!({
                "runtime_kind": "local_command",
                "actor_id": "codex_attach",
                "display_name": "Codex Attach",
                "command": ["/bin/sh", "-lc", "cat"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "capability_allowlist": [],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_payload: ApiResponse<ConnectInstanceData> = decode_json(launch_response).await;
    let run_id = launch_payload.data.expect("launch data").id;

    let stdin_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/stdin"),
            json!({ "input": "attach cursor line\n" }),
        ))
        .await
        .expect("stdin response");
    assert_eq!(stdin_response.status(), StatusCode::OK);
    let stdin_payload: ApiResponse<ConnectStdinWriteAckData> = decode_json(stdin_response).await;
    let stdin_event_id = stdin_payload.data.expect("stdin ack").event_id;

    let attach_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/connect/instances/{run_id}/attach"))
                .body(Body::empty())
                .expect("attach request"),
        )
        .await
        .expect("attach response");
    assert_eq!(attach_response.status(), StatusCode::OK);
    let attach_payload: ApiResponse<vel_api_types::ConnectAttachData> =
        decode_json(attach_response).await;
    let attach_data = attach_payload.data.expect("attach data");
    assert_eq!(attach_data.instance.id, run_id);
    assert!(attach_data.latest_event_id.unwrap_or(0) >= stdin_event_id);
    assert!(attach_data.stream_path.contains("/events/stream"));
    assert!(attach_data.stream_path.contains("after_id="));

    let terminate_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/terminate"),
            json!({ "reason": "test_done" }),
        ))
        .await
        .expect("terminate response");
    assert_eq!(terminate_response.status(), StatusCode::OK);
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

#[tokio::test]
async fn execution_handoff_launches_connect_runtime_and_marks_handoff_launched() {
    let state = test_state().await;
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let workspace = unique_dir("handoff_launch_workspace");
    let notes = workspace.join("notes");
    fs::create_dir_all(&notes).expect("notes dir should be created");

    let now = time::OffsetDateTime::now_utc();
    storage
        .create_project(vel_core::ProjectRecord {
            id: ProjectId::from("proj_connect_launch".to_string()),
            slug: "connect-launch".to_string(),
            name: "Connect Launch".to_string(),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: workspace.to_string_lossy().into_owned(),
                label: "workspace".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: notes.to_string_lossy().into_owned(),
                label: "notes".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: Vec::new(),
            secondary_notes_roots: Vec::new(),
            upstream_ids: std::collections::BTreeMap::new(),
            pending_provision: ProjectProvisionRequest::default(),
            created_at: now,
            updated_at: now,
            archived_at: None,
        })
        .await
        .expect("project should be created");

    let create_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/execution/handoffs",
            json!({
                "project_id": "proj_connect_launch",
                "from_agent": "operator",
                "to_agent": "codex_worker",
                "origin_kind": "human_to_agent",
                "objective": "Apply bounded repo edits",
                "read_scopes": [workspace],
                "write_scopes": [workspace],
                "allowed_tools": ["rg"],
                "inputs": { "ticket": "launch-connect" },
                "expected_output_schema": { "type": "object" },
                "manifest_id": "manifest_local_cli",
                "requested_by": "integration_test"
            }),
        ))
        .await
        .expect("create handoff response");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_payload: ApiResponse<serde_json::Value> = decode_json(create_response).await;
    let handoff_id = create_payload
        .data
        .and_then(|value| {
            value
                .get("id")
                .and_then(serde_json::Value::as_str)
                .map(ToString::to_string)
        })
        .expect("handoff id");

    let approve_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/execution/handoffs/{handoff_id}/approve"),
            json!({
                "reviewed_by": "integration_test",
                "decision_reason": "approved for bounded launch"
            }),
        ))
        .await
        .expect("approve handoff response");
    assert_eq!(approve_response.status(), StatusCode::OK);

    let launch_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/execution/handoffs/{handoff_id}/launch"),
            json!({
                "runtime_kind": "local_command",
                "display_name": "Codex Worker",
                "command": ["/bin/sh", "-lc", "sleep 30"],
                "working_dir": workspace,
                "writable_roots": [workspace],
                "lease_seconds": 30
            }),
        ))
        .await
        .expect("launch handoff response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_payload: ApiResponse<ConnectInstanceData> = decode_json(launch_response).await;
    let launched = launch_payload.data.expect("launch payload data");
    assert_eq!(launched.status, "ready");

    let row = storage
        .get_execution_handoff(&handoff_id)
        .await
        .expect("handoff lookup should succeed")
        .expect("handoff row should exist");
    assert!(row.11.is_some(), "launched_at should be set after launch");

    let second_launch = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/execution/handoffs/{handoff_id}/launch"),
            json!({
                "runtime_kind": "local_command",
                "command": ["/bin/sh", "-lc", "sleep 30"],
                "working_dir": workspace,
                "writable_roots": [workspace]
            }),
        ))
        .await
        .expect("second launch response");
    assert_eq!(second_launch.status(), StatusCode::BAD_REQUEST);
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

fn parse_connect_sse_events(body: &[u8]) -> Vec<ConnectRunEventData> {
    String::from_utf8_lossy(body)
        .split("\n\n")
        .filter_map(|block| {
            let data_line = block.lines().find(|line| line.starts_with("data:"))?;
            let payload = data_line.trim_start_matches("data:").trim();
            serde_json::from_str::<ConnectRunEventData>(payload).ok()
        })
        .collect()
}
