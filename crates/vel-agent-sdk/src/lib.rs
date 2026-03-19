use serde::{Deserialize, Serialize};
use vel_protocol::{
    CapabilityRequest, ProtocolEnvelope, ProtocolManifestReference, ProtocolPayload,
    ProtocolSender, ProtocolTraceContext, CURRENT_PROTOCOL_VERSION,
};

#[derive(Debug, Clone)]
pub struct AgentSdkClient {
    sender: ProtocolSender,
    sdk_name: String,
    sdk_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSdkCapabilityGrant {
    pub scope: String,
    #[serde(default)]
    pub resource: Option<String>,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSdkConnectLaunchRequest {
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
    pub capability_allowlist: Vec<AgentSdkCapabilityGrant>,
    #[serde(default)]
    pub lease_seconds: Option<i64>,
}

impl AgentSdkClient {
    pub fn new(
        node_id: impl Into<String>,
        actor_id: impl Into<String>,
        actor_kind: impl Into<String>,
        sdk_name: impl Into<String>,
        sdk_version: impl Into<String>,
    ) -> Self {
        Self {
            sender: ProtocolSender {
                node_id: node_id.into(),
                actor_id: actor_id.into(),
                actor_kind: actor_kind.into(),
            },
            sdk_name: sdk_name.into(),
            sdk_version: sdk_version.into(),
        }
    }

    pub fn handshake(
        &self,
        message_id: impl Into<String>,
        sent_at: impl Into<String>,
        trace: ProtocolTraceContext,
        requested_capabilities: Vec<CapabilityRequest>,
    ) -> ProtocolEnvelope {
        self.envelope(
            message_id,
            sent_at,
            trace,
            ProtocolPayload::Handshake {
                protocol_version: CURRENT_PROTOCOL_VERSION.to_string(),
                sdk_name: self.sdk_name.clone(),
                sdk_version: self.sdk_version.clone(),
                requested_capabilities,
            },
        )
    }

    pub fn heartbeat(
        &self,
        message_id: impl Into<String>,
        sent_at: impl Into<String>,
        trace: ProtocolTraceContext,
        lease_id: impl Into<String>,
        status: impl Into<String>,
    ) -> ProtocolEnvelope {
        self.envelope(
            message_id,
            sent_at,
            trace,
            ProtocolPayload::Heartbeat {
                lease_id: lease_id.into(),
                status: status.into(),
            },
        )
    }

    pub fn capability_request(
        &self,
        message_id: impl Into<String>,
        sent_at: impl Into<String>,
        trace: ProtocolTraceContext,
        requests: Vec<CapabilityRequest>,
    ) -> ProtocolEnvelope {
        self.envelope(
            message_id,
            sent_at,
            trace,
            ProtocolPayload::CapabilityRequest { requests },
        )
    }

    pub fn action_batch_submit(
        &self,
        message_id: impl Into<String>,
        sent_at: impl Into<String>,
        trace: ProtocolTraceContext,
        batch_id: impl Into<String>,
        actions: Vec<serde_json::Value>,
    ) -> ProtocolEnvelope {
        self.envelope(
            message_id,
            sent_at,
            trace,
            ProtocolPayload::ActionBatchSubmit {
                batch_id: batch_id.into(),
                actions,
            },
        )
    }

    pub fn manifest_reference(
        &self,
        manifest_id: impl Into<String>,
        runtime_kind: impl Into<String>,
        working_directory: impl Into<String>,
    ) -> ProtocolManifestReference {
        ProtocolManifestReference {
            manifest_id: manifest_id.into(),
            runtime_kind: runtime_kind.into(),
            working_directory: working_directory.into(),
        }
    }

    pub fn connect_launch_request(
        &self,
        runtime_kind: impl Into<String>,
        display_name: Option<String>,
        command: Vec<String>,
        working_dir: Option<String>,
        writable_roots: Vec<String>,
        capability_allowlist: Vec<AgentSdkCapabilityGrant>,
        lease_seconds: Option<i64>,
    ) -> AgentSdkConnectLaunchRequest {
        AgentSdkConnectLaunchRequest {
            runtime_kind: runtime_kind.into(),
            actor_id: self.sender.actor_id.clone(),
            display_name,
            command,
            working_dir,
            writable_roots,
            capability_allowlist,
            lease_seconds,
        }
    }

    fn envelope(
        &self,
        message_id: impl Into<String>,
        sent_at: impl Into<String>,
        trace: ProtocolTraceContext,
        payload: ProtocolPayload,
    ) -> ProtocolEnvelope {
        ProtocolEnvelope {
            protocol_version: CURRENT_PROTOCOL_VERSION.to_string(),
            message_id: message_id.into(),
            sent_at: sent_at.into(),
            sender: self.sender.clone(),
            trace,
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trace() -> ProtocolTraceContext {
        ProtocolTraceContext {
            run_id: "run_sdk_01".to_string(),
            trace_id: "trace_sdk_01".to_string(),
            parent_run_id: None,
        }
    }

    #[test]
    fn handshake_uses_current_protocol_version() {
        let sdk = AgentSdkClient::new(
            "node_local",
            "sdk_worker",
            "external_limb",
            "vel-agent-sdk-rust",
            "0.1.0",
        );
        let envelope = sdk.handshake(
            "msg_1",
            "2026-03-18T21:20:00Z",
            trace(),
            vec![CapabilityRequest {
                name: "context.read".to_string(),
                scope: "today_brief".to_string(),
                reason: "Need current context.".to_string(),
            }],
        );

        assert_eq!(envelope.protocol_version, CURRENT_PROTOCOL_VERSION);
        match envelope.payload {
            ProtocolPayload::Handshake {
                protocol_version, ..
            } => assert_eq!(protocol_version, CURRENT_PROTOCOL_VERSION),
            other => panic!("expected handshake payload, got {other:?}"),
        }
    }

    #[test]
    fn action_batch_submit_wraps_actions_in_protocol_envelope() {
        let sdk = AgentSdkClient::new(
            "node_local",
            "sdk_worker",
            "external_limb",
            "vel-agent-sdk-rust",
            "0.1.0",
        );
        let envelope = sdk.action_batch_submit(
            "msg_2",
            "2026-03-18T21:21:00Z",
            trace(),
            "batch_1",
            vec![serde_json::json!({"kind": "read_context", "query": "current_context"})],
        );

        match envelope.payload {
            ProtocolPayload::ActionBatchSubmit { batch_id, actions } => {
                assert_eq!(batch_id, "batch_1");
                assert_eq!(actions.len(), 1);
            }
            other => panic!("expected action batch payload, got {other:?}"),
        }
    }

    #[test]
    fn connect_launch_request_matches_live_transport_shape() {
        let sdk = AgentSdkClient::new(
            "node_local",
            "sdk_worker",
            "external_limb",
            "vel-agent-sdk-rust",
            "0.1.0",
        );
        let launch = sdk.connect_launch_request(
            "wasm_guest",
            Some("SDK Guest".to_string()),
            vec!["/tmp/vel/guest.json".to_string()],
            Some("/tmp/vel".to_string()),
            vec!["/tmp/vel".to_string()],
            vec![AgentSdkCapabilityGrant {
                scope: "read:context".to_string(),
                resource: None,
                action: "read".to_string(),
            }],
            Some(300),
        );

        let value = serde_json::to_value(&launch).expect("launch should serialize");
        assert_eq!(value["runtime_kind"], "wasm_guest");
        assert_eq!(value["actor_id"], "sdk_worker");
        assert_eq!(value["working_dir"], "/tmp/vel");
        assert_eq!(value["writable_roots"][0], "/tmp/vel");
        assert_eq!(value["capability_allowlist"][0]["scope"], "read:context");
    }

    #[test]
    fn manifest_reference_uses_protocol_contract() {
        let sdk = AgentSdkClient::new(
            "node_local",
            "sdk_worker",
            "external_limb",
            "vel-agent-sdk-rust",
            "0.1.0",
        );
        let manifest = sdk.manifest_reference("local_coder", "local_cli", "/tmp/vel");

        assert_eq!(manifest.manifest_id, "local_coder");
        assert_eq!(manifest.runtime_kind, "local_cli");
        assert_eq!(manifest.working_directory, "/tmp/vel");
    }
}
