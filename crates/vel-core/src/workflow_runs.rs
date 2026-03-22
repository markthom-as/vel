use serde::{Deserialize, Serialize};

use crate::RunId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunStatus {
    Created,
    Ready,
    Running,
    AwaitingApproval,
    DryRunComplete,
    Completed,
    Failed,
    Refused,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: RunId,
    pub workflow_id: String,
    pub status: WorkflowRunStatus,
    pub dry_run: bool,
    pub current_step_id: Option<String>,
    pub reason: Option<String>,
}

impl RunRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.workflow_id.trim().is_empty() {
            return Err("run record missing workflow_id".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{RunRecord, WorkflowRunStatus};
    use crate::RunId;

    #[test]
    fn run_record_tracks_manual_workflow_lifecycle_states() {
        let statuses = [
            WorkflowRunStatus::Created,
            WorkflowRunStatus::Ready,
            WorkflowRunStatus::Running,
            WorkflowRunStatus::AwaitingApproval,
            WorkflowRunStatus::DryRunComplete,
            WorkflowRunStatus::Completed,
            WorkflowRunStatus::Failed,
            WorkflowRunStatus::Refused,
            WorkflowRunStatus::Cancelled,
        ];

        let run = RunRecord {
            run_id: RunId::new(),
            workflow_id: "workflow_01brief".to_string(),
            status: WorkflowRunStatus::Created,
            dry_run: false,
            current_step_id: None,
            reason: None,
        };

        assert_eq!(statuses.len(), 9);
        assert!(run.validate().is_ok());
    }
}
