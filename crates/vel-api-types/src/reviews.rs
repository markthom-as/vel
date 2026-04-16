use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReviewSnapshotData {
    #[serde(default)]
    pub open_action_count: u32,
    #[serde(default)]
    pub triage_count: u32,
    #[serde(default)]
    pub projects_needing_review: u32,
    #[serde(default)]
    pub pending_execution_reviews: u32,
}

impl From<vel_core::ReviewSnapshot> for ReviewSnapshotData {
    fn from(value: vel_core::ReviewSnapshot) -> Self {
        Self {
            open_action_count: value.open_action_count,
            triage_count: value.triage_count,
            projects_needing_review: value.projects_needing_review,
            pending_execution_reviews: value.pending_execution_reviews,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_snapshot_default_serializes_named_counts() {
        let value = serde_json::to_value(ReviewSnapshotData::default())
            .expect("review snapshot should serialize");

        assert_eq!(value["open_action_count"], 0);
        assert_eq!(value["triage_count"], 0);
        assert_eq!(value["projects_needing_review"], 0);
        assert_eq!(value["pending_execution_reviews"], 0);
    }
}
