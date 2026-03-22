use serde::{Deserialize, Serialize};

use crate::CalendarId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeclinedResponsePolicy {
    IgnoreDeclined,
    DeclinedBlocks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllDayHandlingRule {
    RespectTransparency,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailabilityPolicyConfig {
    #[serde(default)]
    pub included_calendar_ids: Vec<CalendarId>,
    #[serde(default)]
    pub source_account_ids: Vec<String>,
    pub declined_response_policy: DeclinedResponsePolicy,
    pub exclude_cancelled_events: bool,
    pub all_day_handling_rule: AllDayHandlingRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityResult {
    Free,
    Busy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityBasis {
    Exact,
    Inferred,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockingInterval {
    pub event_id: String,
    pub calendar_id: String,
    pub start: String,
    pub end: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailabilityWindow {
    pub window_start: String,
    pub window_end: String,
    pub result: AvailabilityResult,
    #[serde(default)]
    pub blocking_intervals: Vec<BlockingInterval>,
    #[serde(default)]
    pub sources_consulted: Vec<String>,
    #[serde(default)]
    pub filters_applied: Vec<String>,
    pub basis: AvailabilityBasis,
    pub confidence: Option<AvailabilityConfidence>,
}

#[cfg(test)]
mod tests {
    use super::{
        AllDayHandlingRule, AvailabilityBasis, AvailabilityConfidence, AvailabilityPolicyConfig,
        AvailabilityResult, AvailabilityWindow, DeclinedResponsePolicy,
    };
    use crate::CalendarId;

    #[test]
    fn availability_policy_and_window_capture_governed_read_model_shape() {
        let config = AvailabilityPolicyConfig {
            included_calendar_ids: vec![CalendarId::new()],
            source_account_ids: vec!["integration_account_01".to_string()],
            declined_response_policy: DeclinedResponsePolicy::IgnoreDeclined,
            exclude_cancelled_events: true,
            all_day_handling_rule: AllDayHandlingRule::RespectTransparency,
        };

        let window = AvailabilityWindow {
            window_start: "2026-03-23T08:00:00Z".to_string(),
            window_end: "2026-03-23T09:00:00Z".to_string(),
            result: AvailabilityResult::Free,
            blocking_intervals: vec![],
            sources_consulted: vec!["calendar.personal".to_string()],
            filters_applied: vec!["declined_ignored".to_string()],
            basis: AvailabilityBasis::Exact,
            confidence: Some(AvailabilityConfidence::High),
        };

        assert!(config.exclude_cancelled_events);
        assert_eq!(window.result, AvailabilityResult::Free);
        assert_eq!(window.basis, AvailabilityBasis::Exact);
    }
}
