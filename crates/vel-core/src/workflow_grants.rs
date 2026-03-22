use serde::{Deserialize, Serialize};

use crate::Grant;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantEnvelope {
    pub workflow_id: String,
    pub module_id: String,
    pub skill_id: String,
    pub caller_grant: Grant,
    pub workflow_capabilities: Vec<String>,
    pub module_capabilities: Vec<String>,
    pub read_only: bool,
}

impl GrantEnvelope {
    pub fn validate(&self) -> Result<(), String> {
        if self.workflow_id.trim().is_empty() {
            return Err("grant envelope missing workflow_id".to_string());
        }
        if self.module_id.trim().is_empty() {
            return Err("grant envelope missing module_id".to_string());
        }
        if self.skill_id.trim().is_empty() {
            return Err("grant envelope missing skill_id".to_string());
        }
        if self.workflow_capabilities.is_empty() {
            return Err("grant envelope requires workflow capabilities".to_string());
        }
        if self.module_capabilities.is_empty() {
            return Err("grant envelope requires module capabilities".to_string());
        }

        Ok(())
    }

    pub fn effective_capabilities(&self) -> Vec<String> {
        self.caller_grant
            .capabilities
            .iter()
            .filter(|capability| {
                self.workflow_capabilities
                    .iter()
                    .any(|allowed| allowed == *capability)
                    && self
                        .module_capabilities
                        .iter()
                        .any(|allowed| allowed == *capability)
            })
            .cloned()
            .collect()
    }

    pub fn effective_grant(&self) -> Grant {
        let mut effective = self.caller_grant.clone();
        effective.id = format!("{}_workflow", effective.id);
        effective.capabilities = self.effective_capabilities();
        effective.read_only = effective.read_only || self.read_only;
        effective
    }
}

#[cfg(test)]
mod tests {
    use super::GrantEnvelope;
    use crate::{Grant, GrantScope};

    #[test]
    fn grant_envelope_narrows_caller_workflow_and_module_authority() {
        let envelope = GrantEnvelope {
            workflow_id: "workflow_01brief".to_string(),
            module_id: "module.core.orientation".to_string(),
            skill_id: "skill.core.daily-brief".to_string(),
            caller_grant: Grant {
                id: "grant_01".to_string(),
                scope: vec![
                    GrantScope::Workspace,
                    GrantScope::Module("module.core.orientation".to_string()),
                    GrantScope::Action("object.get".to_string()),
                ],
                capabilities: vec!["object.read".to_string(), "object.write".to_string()],
                durable: false,
                run_scoped: true,
                read_only: false,
            },
            workflow_capabilities: vec!["object.read".to_string()],
            module_capabilities: vec!["object.read".to_string(), "object.write".to_string()],
            read_only: false,
        };

        let effective = envelope.effective_grant();
        assert_eq!(effective.capabilities, vec!["object.read".to_string()]);
        assert!(effective.run_scoped);
    }

    #[test]
    fn grant_envelope_carries_read_only_narrowing_into_effective_grant() {
        let envelope = GrantEnvelope {
            workflow_id: "workflow_01brief".to_string(),
            module_id: "module.core.orientation".to_string(),
            skill_id: "skill.core.daily-brief".to_string(),
            caller_grant: Grant {
                id: "grant_02".to_string(),
                scope: vec![GrantScope::Workspace],
                capabilities: vec!["object.read".to_string()],
                durable: false,
                run_scoped: true,
                read_only: false,
            },
            workflow_capabilities: vec!["object.read".to_string()],
            module_capabilities: vec!["object.read".to_string()],
            read_only: true,
        };

        assert!(envelope.effective_grant().read_only);
    }
}
