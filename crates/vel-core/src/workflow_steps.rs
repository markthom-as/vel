use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStepKind {
    Action,
    Skill,
    Approval,
    Sync,
    Condition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionStep {
    pub step_id: String,
    pub action_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillStep {
    pub step_id: String,
    pub skill_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalStep {
    pub step_id: String,
    pub approval_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncStep {
    pub step_id: String,
    pub sync_target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionStep {
    pub step_id: String,
    pub condition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkflowStep {
    Action(ActionStep),
    Skill(SkillStep),
    Approval(ApprovalStep),
    Sync(SyncStep),
    Condition(ConditionStep),
}

impl WorkflowStep {
    pub fn kind(&self) -> WorkflowStepKind {
        match self {
            Self::Action(_) => WorkflowStepKind::Action,
            Self::Skill(_) => WorkflowStepKind::Skill,
            Self::Approval(_) => WorkflowStepKind::Approval,
            Self::Sync(_) => WorkflowStepKind::Sync,
            Self::Condition(_) => WorkflowStepKind::Condition,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Action(step) => {
                validate_step_id(&step.step_id)?;
                if step.action_name.trim().is_empty() {
                    return Err("action step missing action_name".to_string());
                }
            }
            Self::Skill(step) => {
                validate_step_id(&step.step_id)?;
                if step.skill_id.trim().is_empty() {
                    return Err("skill step missing skill_id".to_string());
                }
            }
            Self::Approval(step) => {
                validate_step_id(&step.step_id)?;
                if step.approval_key.trim().is_empty() {
                    return Err("approval step missing approval_key".to_string());
                }
            }
            Self::Sync(step) => {
                validate_step_id(&step.step_id)?;
                if step.sync_target.trim().is_empty() {
                    return Err("sync step missing sync_target".to_string());
                }
            }
            Self::Condition(step) => {
                validate_step_id(&step.step_id)?;
                if step.condition.trim().is_empty() {
                    return Err("condition step missing condition".to_string());
                }
            }
        }

        Ok(())
    }
}

fn validate_step_id(step_id: &str) -> Result<(), String> {
    if step_id.trim().is_empty() {
        return Err("workflow step missing step_id".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ActionStep, ApprovalStep, ConditionStep, SkillStep, SyncStep, WorkflowStep, WorkflowStepKind};

    #[test]
    fn workflow_step_taxonomy_stays_minimal_and_validated() {
        let steps = vec![
            WorkflowStep::Action(ActionStep {
                step_id: "step_action".to_string(),
                action_name: "object.get".to_string(),
            }),
            WorkflowStep::Skill(SkillStep {
                step_id: "step_skill".to_string(),
                skill_id: "skill.core.daily-brief".to_string(),
            }),
            WorkflowStep::Approval(ApprovalStep {
                step_id: "step_approval".to_string(),
                approval_key: "operator".to_string(),
            }),
            WorkflowStep::Sync(SyncStep {
                step_id: "step_sync".to_string(),
                sync_target: "integration.todoist".to_string(),
            }),
            WorkflowStep::Condition(ConditionStep {
                step_id: "step_condition".to_string(),
                condition: "task.status == ready".to_string(),
            }),
        ];

        assert_eq!(steps[0].kind(), WorkflowStepKind::Action);
        assert_eq!(steps[1].kind(), WorkflowStepKind::Skill);
        assert_eq!(steps[2].kind(), WorkflowStepKind::Approval);
        assert_eq!(steps[3].kind(), WorkflowStepKind::Sync);
        assert_eq!(steps[4].kind(), WorkflowStepKind::Condition);
        assert!(steps.iter().all(|step| step.validate().is_ok()));
    }

    #[test]
    fn workflow_step_validation_rejects_malformed_condition() {
        let step = WorkflowStep::Condition(ConditionStep {
            step_id: "step_condition".to_string(),
            condition: String::new(),
        });

        assert!(step.validate().unwrap_err().contains("condition"));
    }
}
