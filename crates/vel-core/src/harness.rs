use crate::{ConfirmationMode, PolicyDecisionKind};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityTargetKind {
    ToolInvocation,
    Mutation,
    ReadOnlyExecution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityResolutionRequest {
    pub capability: String,
    pub target_kind: CapabilityTargetKind,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityDecisionReasonCode {
    AllowedReadOnly,
    AllowedDryRunPreview,
    ConfirmationRequired,
    DeniedUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityResolutionDecision {
    pub decision: PolicyDecisionKind,
    pub confirmation: ConfirmationMode,
    pub reason_code: CapabilityDecisionReasonCode,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolInvocationRequest {
    pub tool_name: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub metadata_json: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "outcome")]
pub enum ToolInvocationOutcome {
    Success {
        #[serde(default)]
        stdout: String,
        #[serde(default)]
        stderr: String,
        exit_code: i32,
    },
    Timeout {
        timeout_ms: u64,
    },
    Refusal {
        reason: String,
    },
    Failure {
        error: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationProposal {
    pub mutation_kind: String,
    pub idempotency_key: String,
    pub target_ref: String,
    #[serde(default)]
    pub write_scope: Vec<String>,
    #[serde(default)]
    pub metadata_json: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationCommitRequest {
    pub proposal: MutationProposal,
    #[serde(default)]
    pub approved: bool,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationCommitStatus {
    SkippedDryRun,
    WaitingForApproval,
    RejectedOutOfScope,
    Applied,
    AlreadyApplied,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutationCommitResult {
    pub status: MutationCommitStatus,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynthesisRequest {
    pub intent: String,
    #[serde(default)]
    pub context_json: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynthesisResponse {
    #[serde(default)]
    pub plan_steps: Vec<String>,
    #[serde(default)]
    pub rationale: String,
    #[serde(default)]
    pub cautions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynthesisFailureKind {
    ProviderUnavailable,
    InvalidResponse,
    Timeout,
    Unauthorized,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynthesisFailure {
    pub kind: SynthesisFailureKind,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::{CapabilityDecisionReasonCode, ToolInvocationOutcome};

    #[test]
    fn reason_code_serialization_is_stable() {
        let value = serde_json::to_string(&CapabilityDecisionReasonCode::AllowedDryRunPreview)
            .expect("serialize reason code");
        assert_eq!(value, "\"allowed_dry_run_preview\"");
    }

    #[test]
    fn tool_outcome_serialization_is_structured() {
        let value = serde_json::to_value(ToolInvocationOutcome::Timeout { timeout_ms: 250 })
            .expect("serialize tool outcome");
        assert_eq!(value["outcome"], "timeout");
        assert_eq!(value["timeout_ms"], 250);
    }
}
