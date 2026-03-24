use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMode {
    Auto,
    Ask,
    AskIfDestructive,
    AskIfCrossSource,
    AskIfExternalWrite,
    Deny,
}

pub fn confirmation_mode_vocabulary() -> [&'static str; 6] {
    [
        "auto",
        "ask",
        "ask_if_destructive",
        "ask_if_cross_source",
        "ask_if_external_write",
        "deny",
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyLayerKind {
    Workspace,
    Module,
    IntegrationAccount,
    Object,
    Action,
    Execution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLayerDecision {
    pub layer: PolicyLayerKind,
    pub read_only: bool,
    pub confirmation: ConfirmationMode,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEvaluationInput {
    pub action_name: String,
    pub allows_external_write: bool,
    pub is_destructive: bool,
    pub is_cross_source: bool,
    pub workspace: PolicyLayerDecision,
    pub module: PolicyLayerDecision,
    pub integration_account: PolicyLayerDecision,
    pub object: PolicyLayerDecision,
    pub action: PolicyLayerDecision,
    pub execution: PolicyLayerDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecisionKind {
    Allowed,
    ConfirmationRequired,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub kind: PolicyDecisionKind,
    pub confirmation: ConfirmationMode,
    pub read_only: bool,
    pub reasons: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::confirmation_mode_vocabulary;

    #[test]
    fn confirmation_vocabulary_is_stable() {
        assert_eq!(
            confirmation_mode_vocabulary(),
            [
                "auto",
                "ask",
                "ask_if_destructive",
                "ask_if_cross_source",
                "ask_if_external_write",
                "deny",
            ]
        );
    }
}
