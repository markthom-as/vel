use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStatusData {
    Pending,
    Linked,
    Revoked,
    Expired,
}

impl From<vel_core::LinkStatus> for LinkStatusData {
    fn from(value: vel_core::LinkStatus) -> Self {
        match value {
            vel_core::LinkStatus::Pending => Self::Pending,
            vel_core::LinkStatus::Linked => Self::Linked,
            vel_core::LinkStatus::Revoked => Self::Revoked,
            vel_core::LinkStatus::Expired => Self::Expired,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LinkScopeData {
    #[serde(default)]
    pub read_context: bool,
    #[serde(default)]
    pub write_safe_actions: bool,
    #[serde(default)]
    pub execute_repo_tasks: bool,
}

impl From<vel_core::LinkScope> for LinkScopeData {
    fn from(value: vel_core::LinkScope) -> Self {
        Self {
            read_context: value.read_context,
            write_safe_actions: value.write_safe_actions,
            execute_repo_tasks: value.execute_repo_tasks,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTargetSuggestionData {
    pub label: String,
    pub base_url: String,
    pub transport_hint: String,
    pub recommended: bool,
    pub redeem_command_hint: String,
}
