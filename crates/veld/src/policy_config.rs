//! Policy configuration loaded from config/policies.yaml at startup.
//! See docs/specs/vel-agent-next-steps-policy-config-commute.md

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PolicyConfig {
    #[serde(default)]
    pub loops: LoopPolicies,
    #[serde(default)]
    pub suggestions: SuggestionPolicies,
    pub policies: PoliciesMap,
}

/// Policy map: fields are populated from YAML and read via accessors (commute_leave_time in use; others for future use).
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PoliciesMap {
    pub meds_not_logged: Option<PolicyMedsNotLogged>,
    pub meeting_prep_window: Option<PolicyMeetingPrepWindow>,
    pub commute_leave_time: Option<PolicyCommuteLeaveTime>,
    pub morning_drift: Option<PolicyMorningDrift>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PolicyMedsNotLogged {
    pub enabled: bool,
    pub gentle_after_minutes: u32,
    pub warning_after_minutes: u32,
    pub danger_after_minutes: u32,
    pub default_snooze_minutes: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PolicyMeetingPrepWindow {
    pub enabled: bool,
    pub default_prep_minutes: u32,
    pub gentle_before_minutes: i32,
    pub warning_before_minutes: i32,
    pub danger_before_minutes: i32,
    pub default_snooze_minutes: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PolicyCommuteLeaveTime {
    pub enabled: bool,
    pub require_travel_minutes: bool,
    pub gentle_before_minutes: u32,
    pub warning_before_minutes: u32,
    pub danger_before_minutes: u32,
    pub default_snooze_minutes: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PolicyMorningDrift {
    pub enabled: bool,
    pub gentle_after_minutes: u32,
    pub warning_after_minutes: u32,
    pub danger_after_minutes: u32,
    pub default_snooze_minutes: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoopPolicies {
    pub queue_work_scheduler: Option<LoopPolicy>,
    pub evaluate_current_state: Option<LoopPolicy>,
    pub sync_calendar: Option<LoopPolicy>,
    pub sync_todoist: Option<LoopPolicy>,
    pub sync_activity: Option<LoopPolicy>,
    pub sync_health: Option<LoopPolicy>,
    pub sync_git: Option<LoopPolicy>,
    pub sync_messaging: Option<LoopPolicy>,
    pub sync_notes: Option<LoopPolicy>,
    pub sync_transcripts: Option<LoopPolicy>,
    pub weekly_synthesis: Option<LoopPolicy>,
    pub stale_nudge_reconciliation: Option<LoopPolicy>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoopPolicy {
    pub enabled: bool,
    pub interval_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionPolicies {
    pub enabled: bool,
    pub window_days: i64,
    pub suppression_days: i64,
    pub max_new_per_evaluate: usize,
    pub commute: ThresholdSuggestionPolicy,
    pub prep: ThresholdSuggestionPolicy,
    pub response_debt: ResponseDebtSuggestionPolicy,
    pub morning_drift: SimpleThresholdSuggestionPolicy,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThresholdSuggestionPolicy {
    pub threshold: usize,
    pub increment_minutes: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseDebtSuggestionPolicy {
    pub threshold: usize,
    pub followup_block_minutes: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SimpleThresholdSuggestionPolicy {
    pub threshold: usize,
}

impl Default for PoliciesMap {
    fn default() -> Self {
        Self {
            meds_not_logged: Some(PolicyMedsNotLogged::default()),
            meeting_prep_window: Some(PolicyMeetingPrepWindow::default()),
            commute_leave_time: Some(PolicyCommuteLeaveTime::default()),
            morning_drift: Some(PolicyMorningDrift::default()),
        }
    }
}

impl Default for PolicyMedsNotLogged {
    fn default() -> Self {
        Self {
            enabled: true,
            gentle_after_minutes: 10,
            warning_after_minutes: 30,
            danger_after_minutes: 60,
            default_snooze_minutes: 10,
        }
    }
}

impl Default for PolicyMeetingPrepWindow {
    fn default() -> Self {
        Self {
            enabled: true,
            default_prep_minutes: 30,
            gentle_before_minutes: 15,
            warning_before_minutes: 0,
            danger_before_minutes: -10,
            default_snooze_minutes: 10,
        }
    }
}

impl Default for PolicyCommuteLeaveTime {
    fn default() -> Self {
        Self {
            enabled: true,
            require_travel_minutes: true,
            gentle_before_minutes: 20,
            warning_before_minutes: 5,
            danger_before_minutes: 0,
            default_snooze_minutes: 5,
        }
    }
}

impl Default for PolicyMorningDrift {
    fn default() -> Self {
        Self {
            enabled: true,
            gentle_after_minutes: 20,
            warning_after_minutes: 40,
            danger_after_minutes: 60,
            default_snooze_minutes: 10,
        }
    }
}

impl Default for LoopPolicies {
    fn default() -> Self {
        Self {
            queue_work_scheduler: Some(LoopPolicy {
                enabled: true,
                interval_seconds: 30,
            }),
            evaluate_current_state: Some(LoopPolicy::default()),
            sync_calendar: Some(LoopPolicy {
                enabled: true,
                interval_seconds: 900,
            }),
            sync_todoist: Some(LoopPolicy {
                enabled: true,
                interval_seconds: 600,
            }),
            sync_activity: Some(LoopPolicy {
                enabled: false,
                interval_seconds: 300,
            }),
            sync_health: Some(LoopPolicy {
                enabled: false,
                interval_seconds: 900,
            }),
            sync_git: Some(LoopPolicy {
                enabled: false,
                interval_seconds: 600,
            }),
            sync_messaging: Some(LoopPolicy {
                enabled: true,
                interval_seconds: 300,
            }),
            sync_notes: Some(LoopPolicy {
                enabled: false,
                interval_seconds: 900,
            }),
            sync_transcripts: Some(LoopPolicy {
                enabled: false,
                interval_seconds: 900,
            }),
            weekly_synthesis: Some(LoopPolicy {
                enabled: true,
                interval_seconds: 86_400,
            }),
            stale_nudge_reconciliation: Some(LoopPolicy {
                enabled: true,
                interval_seconds: 1_800,
            }),
        }
    }
}

impl Default for LoopPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 300,
        }
    }
}

impl Default for SuggestionPolicies {
    fn default() -> Self {
        Self {
            enabled: true,
            window_days: 7,
            suppression_days: 14,
            max_new_per_evaluate: 3,
            commute: ThresholdSuggestionPolicy {
                threshold: 2,
                increment_minutes: 10,
            },
            prep: ThresholdSuggestionPolicy {
                threshold: 2,
                increment_minutes: 15,
            },
            response_debt: ResponseDebtSuggestionPolicy {
                threshold: 3,
                followup_block_minutes: 20,
            },
            morning_drift: SimpleThresholdSuggestionPolicy { threshold: 3 },
        }
    }
}

impl PolicyConfig {
    /// Load policy config from path. Fails clearly if file is missing or malformed.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, PolicyConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| PolicyConfigError::Read(path.display().to_string(), e))?;
        serde_yaml::from_str(&content).map_err(PolicyConfigError::Parse)
    }

    #[allow(dead_code)]
    pub fn meds_not_logged(&self) -> Option<&PolicyMedsNotLogged> {
        self.policies.meds_not_logged.as_ref()
    }
    #[allow(dead_code)]
    pub fn meeting_prep_window(&self) -> Option<&PolicyMeetingPrepWindow> {
        self.policies.meeting_prep_window.as_ref()
    }
    pub fn commute_leave_time(&self) -> Option<&PolicyCommuteLeaveTime> {
        self.policies.commute_leave_time.as_ref()
    }
    #[allow(dead_code)]
    pub fn morning_drift(&self) -> Option<&PolicyMorningDrift> {
        self.policies.morning_drift.as_ref()
    }
    pub fn queue_work_scheduler_loop(&self) -> Option<&LoopPolicy> {
        self.loops.queue_work_scheduler.as_ref()
    }
    pub fn evaluate_current_state_loop(&self) -> Option<&LoopPolicy> {
        self.loops.evaluate_current_state.as_ref()
    }
    pub fn sync_calendar_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_calendar.as_ref()
    }
    pub fn sync_todoist_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_todoist.as_ref()
    }
    pub fn sync_activity_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_activity.as_ref()
    }
    pub fn sync_health_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_health.as_ref()
    }
    pub fn sync_git_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_git.as_ref()
    }
    pub fn sync_messaging_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_messaging.as_ref()
    }
    pub fn sync_notes_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_notes.as_ref()
    }
    pub fn sync_transcripts_loop(&self) -> Option<&LoopPolicy> {
        self.loops.sync_transcripts.as_ref()
    }
    pub fn weekly_synthesis_loop(&self) -> Option<&LoopPolicy> {
        self.loops.weekly_synthesis.as_ref()
    }
    pub fn stale_nudge_reconciliation_loop(&self) -> Option<&LoopPolicy> {
        self.loops.stale_nudge_reconciliation.as_ref()
    }
    pub fn suggestions(&self) -> &SuggestionPolicies {
        &self.suggestions
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyConfigError {
    #[error("failed to read policy config from {0}: {1}")]
    Read(String, std::io::Error),
    #[error("failed to parse policy config: {0}")]
    Parse(#[from] serde_yaml::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_all_policies() {
        let config = PolicyConfig::default();
        assert!(config.meds_not_logged().is_some());
        assert!(config.meeting_prep_window().is_some());
        assert!(config.commute_leave_time().is_some());
        assert!(config.morning_drift().is_some());
        assert!(config.evaluate_current_state_loop().is_some());
        assert!(config.sync_calendar_loop().is_some());
        assert!(config.sync_todoist_loop().is_some());
        assert!(config.sync_activity_loop().is_some());
        assert!(config.sync_health_loop().is_some());
        assert!(config.sync_messaging_loop().is_some());
        assert!(config.weekly_synthesis_loop().is_some());
        assert!(config.stale_nudge_reconciliation_loop().is_some());
        assert!(config.suggestions().enabled);
        assert_eq!(config.suggestions().window_days, 7);
        assert_eq!(config.suggestions().response_debt.threshold, 3);
        assert!(config.meeting_prep_window().unwrap().default_prep_minutes == 30);
        assert!(config.commute_leave_time().unwrap().require_travel_minutes);
        assert_eq!(
            config
                .evaluate_current_state_loop()
                .unwrap()
                .interval_seconds,
            300
        );
        assert_eq!(config.sync_calendar_loop().unwrap().interval_seconds, 900);
        assert_eq!(config.sync_todoist_loop().unwrap().interval_seconds, 600);
        assert!(!config.sync_activity_loop().unwrap().enabled);
        assert!(!config.sync_health_loop().unwrap().enabled);
        assert_eq!(config.sync_messaging_loop().unwrap().interval_seconds, 300);
        assert_eq!(
            config.weekly_synthesis_loop().unwrap().interval_seconds,
            86_400
        );
        assert_eq!(
            config
                .stale_nudge_reconciliation_loop()
                .unwrap()
                .interval_seconds,
            1_800
        );
    }

    #[test]
    fn load_from_missing_path_fails() {
        let err = PolicyConfig::load("/nonexistent/policies.yaml").unwrap_err();
        assert!(matches!(err, PolicyConfigError::Read(_, _)));
    }
}
