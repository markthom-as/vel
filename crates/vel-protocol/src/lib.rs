use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CURRENT_PROTOCOL_VERSION: &str = "vel_protocol/v1";

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolManifestReference {
    pub manifest_id: String,
    pub runtime_kind: String,
    pub working_directory: String,
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

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ProtocolValidationError {
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(String),
    #[error("handshake payload version does not match envelope version")]
    HandshakeVersionMismatch,
    #[error("protocol field must not be empty: {0}")]
    EmptyField(&'static str),
    #[error("sent_at must be RFC3339: {0}")]
    InvalidSentAt(String),
}

impl ProtocolEnvelope {
    pub fn validate(&self) -> Result<(), ProtocolValidationError> {
        if self.protocol_version != CURRENT_PROTOCOL_VERSION {
            return Err(ProtocolValidationError::UnsupportedVersion(
                self.protocol_version.clone(),
            ));
        }
        validate_non_empty("message_id", &self.message_id)?;
        validate_non_empty("sender.node_id", &self.sender.node_id)?;
        validate_non_empty("sender.actor_id", &self.sender.actor_id)?;
        validate_non_empty("sender.actor_kind", &self.sender.actor_kind)?;
        validate_non_empty("trace.run_id", &self.trace.run_id)?;
        validate_non_empty("trace.trace_id", &self.trace.trace_id)?;
        time::OffsetDateTime::parse(
            &self.sent_at,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|error| ProtocolValidationError::InvalidSentAt(error.to_string()))?;

        if let ProtocolPayload::Handshake {
            protocol_version, ..
        } = &self.payload
        {
            if protocol_version != &self.protocol_version {
                return Err(ProtocolValidationError::HandshakeVersionMismatch);
            }
        }

        Ok(())
    }
}

fn validate_non_empty(field: &'static str, value: &str) -> Result<(), ProtocolValidationError> {
    if value.trim().is_empty() {
        return Err(ProtocolValidationError::EmptyField(field));
    }
    Ok(())
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

    fn render_template_placeholders(raw: &str) -> String {
        raw.replace("{{message_id}}", "msg_template_01")
            .replace("{{sent_at}}", "2026-03-18T21:10:00Z")
            .replace("{{node_id}}", "node_template")
            .replace("{{actor_id}}", "sdk_template")
            .replace("{{run_id}}", "run_template")
            .replace("{{trace_id}}", "trace_template")
            .replace("{{parent_run_id}}", "run_parent_template")
            .replace("{{lease_id}}", "lease_template")
    }

    #[test]
    fn example_parses_and_validates() {
        let raw = repo_file("config/examples/swarm-protocol-envelope.example.json");
        let envelope: ProtocolEnvelope =
            serde_json::from_str(&raw).expect("protocol envelope should parse");
        envelope.validate().expect("example should validate");
    }

    #[test]
    fn template_renders_and_validates() {
        let raw = repo_file("config/templates/swarm-protocol-envelope.template.json");
        let rendered = render_template_placeholders(&raw);
        let envelope: ProtocolEnvelope =
            serde_json::from_str(&rendered).expect("rendered template should parse");
        envelope
            .validate()
            .expect("rendered template should validate");
    }

    #[test]
    fn handshake_version_mismatch_is_rejected() {
        let envelope = ProtocolEnvelope {
            protocol_version: CURRENT_PROTOCOL_VERSION.to_string(),
            message_id: "msg_bad".to_string(),
            sent_at: "2026-03-18T21:00:00Z".to_string(),
            sender: ProtocolSender {
                node_id: "node_local".to_string(),
                actor_id: "sdk_worker".to_string(),
                actor_kind: "external_limb".to_string(),
            },
            trace: ProtocolTraceContext {
                run_id: "run_bad".to_string(),
                trace_id: "trace_bad".to_string(),
                parent_run_id: None,
            },
            payload: ProtocolPayload::Handshake {
                protocol_version: "vel_protocol/v2".to_string(),
                sdk_name: "vel-agent-sdk-rust".to_string(),
                sdk_version: "0.1.0".to_string(),
                requested_capabilities: Vec::new(),
            },
        };

        let error = envelope.validate().expect_err("mismatch should fail");
        assert_eq!(error, ProtocolValidationError::HandshakeVersionMismatch);
    }

    #[test]
    fn manifest_reference_round_trips() {
        let reference = ProtocolManifestReference {
            manifest_id: "manifest_local_cli".to_string(),
            runtime_kind: "local_cli".to_string(),
            working_directory: "/home/jove/code/vel".to_string(),
        };

        let value = serde_json::to_value(&reference).expect("manifest ref should serialize");
        assert_eq!(value["manifest_id"], "manifest_local_cli");
        let decoded: ProtocolManifestReference =
            serde_json::from_value(value).expect("manifest ref should deserialize");
        assert_eq!(decoded.runtime_kind, "local_cli");
    }
}
