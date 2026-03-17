use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Named background loops owned by the runtime worker.
///
/// These are distinct from run kinds: loop kinds describe recurring runtime
/// responsibilities, while run kinds describe persisted execution records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoopKind {
    CaptureIngest,
    RetryDueRuns,
    EvaluateCurrentState,
    SyncCalendar,
    SyncTodoist,
    SyncActivity,
    SyncMessaging,
    WeeklySynthesis,
    StaleNudgeReconciliation,
}

impl Display for LoopKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::CaptureIngest => "capture_ingest",
            Self::RetryDueRuns => "retry_due_runs",
            Self::EvaluateCurrentState => "evaluate_current_state",
            Self::SyncCalendar => "sync_calendar",
            Self::SyncTodoist => "sync_todoist",
            Self::SyncActivity => "sync_activity",
            Self::SyncMessaging => "sync_messaging",
            Self::WeeklySynthesis => "weekly_synthesis",
            Self::StaleNudgeReconciliation => "stale_nudge_reconciliation",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for LoopKind {
    type Err = crate::VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "capture_ingest" => Ok(Self::CaptureIngest),
            "retry_due_runs" => Ok(Self::RetryDueRuns),
            "evaluate_current_state" => Ok(Self::EvaluateCurrentState),
            "sync_calendar" => Ok(Self::SyncCalendar),
            "sync_todoist" => Ok(Self::SyncTodoist),
            "sync_activity" => Ok(Self::SyncActivity),
            "sync_messaging" => Ok(Self::SyncMessaging),
            "weekly_synthesis" => Ok(Self::WeeklySynthesis),
            "stale_nudge_reconciliation" => Ok(Self::StaleNudgeReconciliation),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown loop kind: {value}"
            ))),
        }
    }
}
