use serde::{Deserialize, Serialize};

use crate::{ActionContract, ConfirmationMode, Grant, GrantEnvelope};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillInvocationMode {
    Mediated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillInvocation {
    pub workflow_id: String,
    pub module_id: String,
    pub skill_id: String,
    pub action_name: String,
    pub target_object_refs: Vec<String>,
    pub dry_run: bool,
    pub input_json: serde_json::Value,
    pub mode: SkillInvocationMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillInvocationOutcome {
    pub invocation: SkillInvocation,
    pub grant_envelope: GrantEnvelope,
    pub effective_grant: Grant,
    pub action_contract: ActionContract,
    pub confirmation: ConfirmationMode,
    pub audit_required: bool,
    pub mediated: bool,
    pub audit_record_ref: Option<String>,
    pub run_record_ref: Option<String>,
}

impl SkillInvocation {
    pub fn validate(&self) -> Result<(), String> {
        if self.workflow_id.trim().is_empty() {
            return Err("skill invocation missing workflow_id".to_string());
        }
        if self.module_id.trim().is_empty() {
            return Err("skill invocation missing module_id".to_string());
        }
        if self.skill_id.trim().is_empty() {
            return Err("skill invocation missing skill_id".to_string());
        }
        if self.action_name.trim().is_empty() {
            return Err("skill invocation missing action_name".to_string());
        }
        if self.action_name.starts_with("tool.") {
            return Err("skill invocation remains mediated and cannot call a raw tool".to_string());
        }
        if !matches!(self.mode, SkillInvocationMode::Mediated) {
            return Err("skill invocation must use mediated mode".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{SkillInvocation, SkillInvocationMode};
    use serde_json::json;

    #[test]
    fn skill_invocation_rejects_raw_tool_bypass() {
        let invocation = SkillInvocation {
            workflow_id: "workflow_01brief".to_string(),
            module_id: "module.core.orientation".to_string(),
            skill_id: "skill.core.daily-brief".to_string(),
            action_name: "tool.object.get".to_string(),
            target_object_refs: vec!["task_01".to_string()],
            dry_run: false,
            input_json: json!({}),
            mode: SkillInvocationMode::Mediated,
        };

        assert!(invocation.validate().unwrap_err().contains("raw tool"));
    }
}
