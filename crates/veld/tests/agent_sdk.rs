use std::{fs, path::PathBuf};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_agent_sdk::{AgentSdkCapabilityGrant, AgentSdkClient};
use vel_api_types::{ApiResponse, ConnectInstanceData};
use vel_config::AppConfig;
use vel_core::{
    CapabilityDescriptor, FilesystemAccessPolicy, NetworkAccessPolicy, SandboxCapabilityPolicy,
    SandboxPolicyMode, SandboxResourceLimits,
};
use vel_protocol::{CapabilityRequest, ProtocolPayload, ProtocolTraceContext};
use vel_storage::Storage;
use veld::{
    app::build_app_with_state, policy_config::PolicyConfig, services::agent_protocol,
    state::AppState,
};

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_agent_sdk_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

async fn test_state() -> AppState {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let (broadcast_tx, _) = broadcast::channel(16);
    let config = AppConfig {
        artifact_root: unique_dir("artifacts").to_string_lossy().to_string(),
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

fn sandbox_policy() -> SandboxCapabilityPolicy {
    SandboxCapabilityPolicy {
        default_mode: SandboxPolicyMode::Deny,
        allowed_calls: vec!["read_context".to_string()],
        filesystem: FilesystemAccessPolicy {
            read_roots: Vec::new(),
            write_roots: Vec::new(),
        },
        network: NetworkAccessPolicy {
            allowed_hosts: Vec::new(),
        },
        resources: SandboxResourceLimits {
            max_fuel: 10_000,
            max_memory_bytes: 4 * 1024 * 1024,
            wall_timeout_ms: 5_000,
        },
        review_gate: "operator".to_string(),
    }
}

fn runtime_allowlist() -> Vec<CapabilityDescriptor> {
    vec![
        CapabilityDescriptor {
            scope: "read:context".to_string(),
            resource: None,
            action: "read".to_string(),
        },
        CapabilityDescriptor {
            scope: "execute:action_batch".to_string(),
            resource: None,
            action: "execute".to_string(),
        },
    ]
}

fn sdk_capability_allowlist() -> Vec<AgentSdkCapabilityGrant> {
    runtime_allowlist()
        .into_iter()
        .map(|capability| AgentSdkCapabilityGrant {
            scope: capability.scope,
            resource: capability.resource,
            action: capability.action,
        })
        .collect()
}

fn trace() -> ProtocolTraceContext {
    ProtocolTraceContext {
        run_id: "run_sdk_flow".to_string(),
        trace_id: "trace_sdk_flow".to_string(),
        parent_run_id: None,
    }
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
async fn agent_sdk_flow_handles_handshake_heartbeat_and_scoped_action_batch() {
    let state = test_state().await;
    let sdk = AgentSdkClient::new(
        "node_sdk",
        "sdk_reference",
        "external_limb",
        "vel-agent-sdk-rust",
        "0.1.0",
    );

    let handshake = sdk.handshake(
        "msg_handshake",
        "2026-03-18T21:30:00Z",
        trace(),
        vec![
            CapabilityRequest {
                name: "context.read".to_string(),
                scope: "today_brief".to_string(),
                reason: "Need orientation context.".to_string(),
            },
            CapabilityRequest {
                name: "action.execute".to_string(),
                scope: "sandbox_batch".to_string(),
                reason: "Need to submit a scoped action batch.".to_string(),
            },
        ],
    );

    let handshake_response = agent_protocol::handle_envelope(
        &state,
        &handshake,
        &sandbox_policy(),
        &runtime_allowlist(),
    )
    .await
    .unwrap();
    match &handshake_response.payload {
        ProtocolPayload::ActionResult {
            outcome, details, ..
        } => {
            assert_eq!(outcome, "handshake");
            assert_eq!(details["lease_id"], "run_sdk_flow");
        }
        other => panic!("expected action result handshake response, got {other:?}"),
    }

    let heartbeat = sdk.heartbeat(
        "msg_heartbeat",
        "2026-03-18T21:31:00Z",
        trace(),
        "run_sdk_flow",
        "healthy",
    );
    let heartbeat_response = agent_protocol::handle_envelope(
        &state,
        &heartbeat,
        &sandbox_policy(),
        &runtime_allowlist(),
    )
    .await
    .unwrap();
    match &heartbeat_response.payload {
        ProtocolPayload::ActionResult {
            outcome, details, ..
        } => {
            assert_eq!(outcome, "heartbeat_ack");
            assert_eq!(details["lease_id"], "run_sdk_flow");
        }
        other => panic!("expected action result heartbeat response, got {other:?}"),
    }

    let batch = sdk.action_batch_submit(
        "msg_batch",
        "2026-03-18T21:32:00Z",
        trace(),
        "batch_01",
        vec![serde_json::json!({
            "kind": "read_context",
            "query": "current_context"
        })],
    );
    let batch_response =
        agent_protocol::handle_envelope(&state, &batch, &sandbox_policy(), &runtime_allowlist())
            .await
            .unwrap();

    match &batch_response.payload {
        ProtocolPayload::ActionResult {
            outcome, details, ..
        } => {
            assert_eq!(outcome, "action_batch_processed");
            assert_eq!(details["batch_id"], "batch_01");
            assert_eq!(details["terminal_status"], "approved");
        }
        other => panic!("expected action batch response, got {other:?}"),
    }

    let connect_run = state
        .storage
        .get_connect_run("run_sdk_flow")
        .await
        .unwrap()
        .expect("connect run should be created");
    assert_eq!(connect_run.status, "running");
    assert!(connect_run.capabilities_json.contains("read:context"));
    assert!(connect_run
        .capabilities_json
        .contains("execute:action_batch"));
}

#[tokio::test]
async fn agent_sdk_flow_launches_guest_runtime_through_live_connect_transport() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let sdk = AgentSdkClient::new(
        "node_sdk",
        "sdk_reference",
        "external_limb",
        "vel-agent-sdk-rust",
        "0.1.0",
    );
    let workspace = unique_dir("connect_transport");
    let guest_spec = workspace.join("guest.json");
    fs::write(
        &guest_spec,
        serde_json::json!({
            "module_id": "sdk_reference_guest",
            "requested_writable_roots": [workspace.to_string_lossy()],
            "requested_hosts": [],
            "host_calls": [
                { "kind": "read_context", "query": "current_context" }
            ]
        })
        .to_string(),
    )
    .expect("guest spec should be written");

    let manifest = sdk.manifest_reference(
        "sdk_reference_guest",
        "wasm_guest",
        workspace.to_string_lossy().to_string(),
    );
    let launch = sdk.connect_launch_request(
        manifest.runtime_kind.clone(),
        Some("SDK Guest".to_string()),
        vec![guest_spec.to_string_lossy().to_string()],
        Some(manifest.working_directory.clone()),
        vec![workspace.to_string_lossy().to_string()],
        sdk_capability_allowlist(),
        Some(300),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/connect/instances")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&launch).expect("launch payload should serialize"),
                ))
                .expect("request"),
        )
        .await
        .expect("launch response");
    assert_eq!(response.status(), StatusCode::OK);
    let payload: ApiResponse<ConnectInstanceData> = decode_json(response).await;
    let instance = payload.data.expect("instance data");
    assert_eq!(instance.status, "offline");
    assert_eq!(
        instance
            .metadata
            .get("terminal_reason")
            .and_then(serde_json::Value::as_str),
        Some("guest_completed")
    );

    let run = state
        .storage
        .get_run_by_id(&instance.id)
        .await
        .expect("run lookup")
        .expect("run should exist");
    assert_eq!(run.status.to_string(), "succeeded");
}
