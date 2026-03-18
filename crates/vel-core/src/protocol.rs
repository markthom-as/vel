use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolTraceContext {
    pub run_id: String,
    pub trace_id: String,
    #[serde(default)]
    pub parent_run_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolSender {
    pub node_id: String,
    pub actor_id: String,
    pub actor_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub name: String,
    pub scope: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProtocolPayload {
    Handshake {
        protocol_version: String,
        sdk_name: String,
        sdk_version: String,
        requested_capabilities: Vec<CapabilityRequest>,
    },
    Heartbeat {
        lease_id: String,
        status: String,
    },
    CapabilityRequest {
        requests: Vec<CapabilityRequest>,
    },
    ActionBatchSubmit {
        batch_id: String,
        actions: Vec<Value>,
    },
    ActionResult {
        batch_id: String,
        outcome: String,
        #[serde(default)]
        details: Value,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolEnvelope {
    pub protocol_version: String,
    pub message_id: String,
    pub sent_at: String,
    pub sender: ProtocolSender,
    pub trace: ProtocolTraceContext,
    pub payload: ProtocolPayload,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    fn repo_file(relative: &str) -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join(relative),
        )
        .expect("repo file should be readable")
    }

    #[test]
    fn protocol_envelope_example_parses() {
        let raw = repo_file("config/examples/swarm-protocol-envelope.example.json");
        let envelope: ProtocolEnvelope =
            serde_json::from_str(&raw).expect("protocol envelope should parse");
        assert_eq!(envelope.protocol_version, "vel_protocol/v1");
    }
}
