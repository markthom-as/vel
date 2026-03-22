use serde::{Deserialize, Serialize};

use crate::RunId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub approval_id: String,
    pub run_id: RunId,
    pub workflow_id: String,
    pub step_id: String,
    pub approval_key: String,
    pub status: ApprovalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRequired {
    pub approval_id: String,
    pub step_id: String,
    pub approval_key: String,
}

#[cfg(test)]
mod tests {
    use super::{ApprovalRecord, ApprovalRequired, ApprovalStatus};
    use crate::RunId;

    #[test]
    fn approval_records_keep_pending_and_terminal_statuses_explicit() {
        let record = ApprovalRecord {
            approval_id: "approval_01".to_string(),
            run_id: RunId::new(),
            workflow_id: "workflow_01brief".to_string(),
            step_id: "step_approval".to_string(),
            approval_key: "operator".to_string(),
            status: ApprovalStatus::Pending,
        };
        let required = ApprovalRequired {
            approval_id: "approval_01".to_string(),
            step_id: "step_approval".to_string(),
            approval_key: "operator".to_string(),
        };

        assert_eq!(record.status, ApprovalStatus::Pending);
        assert_eq!(required.approval_key, "operator");
    }
}
