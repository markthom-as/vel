use tokio::sync::broadcast;
use vel_agent_sdk::AgentSdkClient;
use vel_config::AppConfig;
use vel_core::{
    CapabilityDescriptor, FilesystemAccessPolicy, NetworkAccessPolicy, SandboxCapabilityPolicy,
    SandboxPolicyMode, SandboxResourceLimits,
};
use vel_protocol::{CapabilityRequest, ProtocolPayload, ProtocolTraceContext};
use vel_storage::Storage;
use veld::{policy_config::PolicyConfig, services::agent_protocol, state::AppState};

async fn test_state() -> AppState {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let (broadcast_tx, _) = broadcast::channel(16);
    let config = AppConfig {
        artifact_root: std::env::temp_dir()
            .join(format!("vel_agent_sdk_{}", uuid::Uuid::new_v4().simple()))
            .to_string_lossy()
            .to_string(),
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

fn trace() -> ProtocolTraceContext {
    ProtocolTraceContext {
        run_id: "run_sdk_flow".to_string(),
        trace_id: "trace_sdk_flow".to_string(),
        parent_run_id: None,
    }
}

#[tokio::test]
async fn sdk_flow_handles_handshake_heartbeat_and_scoped_action_batch() {
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
