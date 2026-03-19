use crate::{CapabilityDescriptor, ProjectId, ProjectRootRef, VelCoreError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionTaskKind {
    Planning,
    Implementation,
    Debugging,
    Review,
    Research,
    Documentation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentProfile {
    Budget,
    Balanced,
    Quality,
    Inherit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenBudgetClass {
    Small,
    Medium,
    Large,
    Xlarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionReviewGate {
    None,
    OperatorApproval,
    OperatorPreview,
    PostRunReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalRuntimeKind {
    LocalCli,
    WasmGuest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoWorktreeRef {
    pub path: String,
    pub label: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub head_rev: Option<String>,
}

impl RepoWorktreeRef {
    pub fn validate(&self) -> Result<(), VelCoreError> {
        validate_non_empty("repo worktree path", &self.path)?;
        validate_non_empty("repo worktree label", &self.label)?;
        if matches!(self.branch.as_deref(), Some(value) if value.trim().is_empty()) {
            return Err(VelCoreError::Validation(
                "repo worktree branch must not be empty when present".to_string(),
            ));
        }
        if matches!(self.head_rev.as_deref(), Some(value) if value.trim().is_empty()) {
            return Err(VelCoreError::Validation(
                "repo worktree head_rev must not be empty when present".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalAgentManifest {
    pub manifest_id: String,
    pub runtime_kind: LocalRuntimeKind,
    pub entrypoint: String,
    pub working_directory: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env_keys: Vec<String>,
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<CapabilityDescriptor>,
    pub review_gate: ExecutionReviewGate,
}

impl LocalAgentManifest {
    pub fn validate(&self) -> Result<(), VelCoreError> {
        validate_non_empty("local agent manifest_id", &self.manifest_id)?;
        validate_non_empty("local agent entrypoint", &self.entrypoint)?;
        validate_non_empty("local agent working_directory", &self.working_directory)?;
        validate_non_empty_list("local agent read_roots", &self.read_roots)?;
        validate_non_empty_list("local agent write_roots", &self.write_roots)?;
        validate_non_empty_list("local agent allowed_tools", &self.allowed_tools)?;
        validate_non_empty_list("local agent env_keys", &self.env_keys)?;
        validate_non_empty_list("local agent args", &self.args)?;
        for capability in &self.capabilities {
            validate_non_empty("capability scope", &capability.scope)?;
            validate_non_empty("capability action", &capability.action)?;
            if matches!(capability.resource.as_deref(), Some(resource) if resource.trim().is_empty()) {
                return Err(VelCoreError::Validation(
                    "capability resource must not be empty when present".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPolicyInput {
    pub task_kind: ExecutionTaskKind,
    pub agent_profile: AgentProfile,
    pub token_budget: TokenBudgetClass,
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
    pub review_gate: ExecutionReviewGate,
    #[serde(default)]
    pub requires_network: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectExecutionContext {
    pub project_id: ProjectId,
    pub repo: RepoWorktreeRef,
    pub notes_root: ProjectRootRef,
    pub gsd_artifact_dir: String,
    pub default_task_kind: ExecutionTaskKind,
    pub default_agent_profile: AgentProfile,
    pub default_token_budget: TokenBudgetClass,
    pub review_gate: ExecutionReviewGate,
    #[serde(default)]
    pub read_roots: Vec<String>,
    #[serde(default)]
    pub write_roots: Vec<String>,
    #[serde(default)]
    pub local_manifests: Vec<LocalAgentManifest>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl ProjectExecutionContext {
    pub fn validate(&self) -> Result<(), VelCoreError> {
        self.repo.validate()?;
        validate_non_empty("notes root path", &self.notes_root.path)?;
        validate_non_empty("notes root label", &self.notes_root.label)?;
        validate_non_empty("notes root kind", &self.notes_root.kind)?;
        validate_non_empty("gsd_artifact_dir", &self.gsd_artifact_dir)?;
        validate_non_empty_list("execution context read_roots", &self.read_roots)?;
        validate_non_empty_list("execution context write_roots", &self.write_roots)?;
        for manifest in &self.local_manifests {
            manifest.validate()?;
        }
        for (key, value) in &self.metadata {
            validate_non_empty("execution context metadata key", key)?;
            validate_non_empty("execution context metadata value", value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionHandoff {
    pub handoff: crate::HandoffEnvelope,
    pub project_id: ProjectId,
    pub task_kind: ExecutionTaskKind,
    pub agent_profile: AgentProfile,
    pub token_budget: TokenBudgetClass,
    pub review_gate: ExecutionReviewGate,
    pub repo: RepoWorktreeRef,
    pub notes_root: ProjectRootRef,
    #[serde(default)]
    pub manifest_id: Option<String>,
}

impl ExecutionHandoff {
    pub fn validate(&self) -> Result<(), VelCoreError> {
        self.repo.validate()?;
        validate_non_empty("notes root path", &self.notes_root.path)?;
        validate_non_empty("notes root label", &self.notes_root.label)?;
        validate_non_empty("notes root kind", &self.notes_root.kind)?;
        if matches!(self.manifest_id.as_deref(), Some(value) if value.trim().is_empty()) {
            return Err(VelCoreError::Validation(
                "execution handoff manifest_id must not be empty when present".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_non_empty(field: &'static str, value: &str) -> Result<(), VelCoreError> {
    if value.trim().is_empty() {
        return Err(VelCoreError::Validation(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn validate_non_empty_list(field: &'static str, values: &[String]) -> Result<(), VelCoreError> {
    if values.iter().any(|value| value.trim().is_empty()) {
        return Err(VelCoreError::Validation(format!(
            "{field} entries must not be empty"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ExecutionHandoff, LocalAgentManifest, ProjectExecutionContext};
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
    fn project_execution_context_example_parses_and_validates() {
        let raw = repo_file("config/examples/project-execution-context.example.json");
        let context: ProjectExecutionContext =
            serde_json::from_str(&raw).expect("project execution context should parse");
        context.validate().expect("project execution context should validate");
    }

    #[test]
    fn local_agent_manifest_example_parses_and_validates() {
        let raw = repo_file("config/examples/local-agent-manifest.example.json");
        let manifest: LocalAgentManifest =
            serde_json::from_str(&raw).expect("local agent manifest should parse");
        manifest.validate().expect("local agent manifest should validate");
    }

    #[test]
    fn execution_handoff_example_parses_and_validates() {
        let raw = repo_file("config/examples/execution-handoff.example.json");
        let handoff: ExecutionHandoff =
            serde_json::from_str(&raw).expect("execution handoff should parse");
        handoff.validate().expect("execution handoff should validate");
    }
}
