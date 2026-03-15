//! Policy configuration loaded from config/policies.yaml at startup.
//! See docs/specs/vel-agent-next-steps-policy-config-commute.md

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyConfig {
    pub policies: PoliciesMap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PoliciesMap {
    pub meds_not_logged: Option<PolicyMedsNotLogged>,
    pub meeting_prep_window: Option<PolicyMeetingPrepWindow>,
    pub commute_leave_time: Option<PolicyCommuteLeaveTime>,
    pub morning_drift: Option<PolicyMorningDrift>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyMedsNotLogged {
    pub enabled: bool,
    pub gentle_after_minutes: u32,
    pub warning_after_minutes: u32,
    pub danger_after_minutes: u32,
    pub default_snooze_minutes: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyMeetingPrepWindow {
    pub enabled: bool,
    pub default_prep_minutes: u32,
    pub gentle_before_minutes: i32,
    pub warning_before_minutes: i32,
    pub danger_before_minutes: i32,
    pub default_snooze_minutes: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyCommuteLeaveTime {
    pub enabled: bool,
    pub require_travel_minutes: bool,
    pub gentle_before_minutes: u32,
    pub warning_before_minutes: u32,
    pub danger_before_minutes: u32,
    pub default_snooze_minutes: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyMorningDrift {
    pub enabled: bool,
    pub gentle_after_minutes: u32,
    pub warning_after_minutes: u32,
    pub danger_after_minutes: u32,
    pub default_snooze_minutes: u32,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            policies: PoliciesMap::default(),
        }
    }
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

impl PolicyConfig {
    /// Load policy config from path. Fails clearly if file is missing or malformed.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, PolicyConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| PolicyConfigError::Read(path.display().to_string(), e))?;
        serde_yaml::from_str(&content).map_err(|e| PolicyConfigError::Parse(e))
    }

    pub fn meds_not_logged(&self) -> Option<&PolicyMedsNotLogged> {
        self.policies.meds_not_logged.as_ref()
    }
    pub fn meeting_prep_window(&self) -> Option<&PolicyMeetingPrepWindow> {
        self.policies.meeting_prep_window.as_ref()
    }
    pub fn commute_leave_time(&self) -> Option<&PolicyCommuteLeaveTime> {
        self.policies.commute_leave_time.as_ref()
    }
    pub fn morning_drift(&self) -> Option<&PolicyMorningDrift> {
        self.policies.morning_drift.as_ref()
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
        assert!(config.meeting_prep_window().unwrap().default_prep_minutes == 30);
        assert!(config.commute_leave_time().unwrap().require_travel_minutes);
    }

    #[test]
    fn load_from_missing_path_fails() {
        let err = PolicyConfig::load("/nonexistent/policies.yaml").unwrap_err();
        assert!(matches!(err, PolicyConfigError::Read(_, _)));
    }
}
