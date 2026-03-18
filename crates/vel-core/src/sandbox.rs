use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxPolicyMode {
    Deny,
    Brokered,
    Allow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemAccessPolicy {
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAccessPolicy {
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxResourceLimits {
    pub max_fuel: u64,
    pub max_memory_bytes: u64,
    pub wall_timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxCapabilityPolicy {
    pub default_mode: SandboxPolicyMode,
    #[serde(default)]
    pub allowed_calls: Vec<String>,
    pub filesystem: FilesystemAccessPolicy,
    pub network: NetworkAccessPolicy,
    pub resources: SandboxResourceLimits,
    pub review_gate: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SandboxHostCall {
    ReadContext { query: String },
    RequestCapability { capability: String, reason: String },
    SubmitActionBatch { actions: Vec<Value> },
    ReadArtifact { artifact_id: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxHostCallEnvelope {
    pub abi_version: String,
    pub module_id: String,
    pub run_id: String,
    pub trace_id: String,
    pub call: SandboxHostCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxDecisionStatus {
    Approved,
    Denied,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxDecisionRecord {
    pub abi_version: String,
    pub run_id: String,
    pub trace_id: String,
    pub call_kind: String,
    pub status: SandboxDecisionStatus,
    pub reason: String,
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
    fn sandbox_policy_template_parses() {
        let raw = repo_file("config/templates/sandbox-policy.template.json");
        let policy: SandboxCapabilityPolicy =
            serde_json::from_str(&raw).expect("sandbox policy template should parse");
        assert_eq!(policy.default_mode, SandboxPolicyMode::Deny);
    }

    #[test]
    fn sandbox_host_call_example_parses() {
        let raw = repo_file("config/examples/sandbox-host-call.example.json");
        let envelope: SandboxHostCallEnvelope =
            serde_json::from_str(&raw).expect("sandbox host call should parse");
        assert_eq!(envelope.abi_version, "sandbox_abi/v1");
    }
}
