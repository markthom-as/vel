use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientActionKind {
    NudgeDone,
    NudgeSnooze,
    CommitmentDone,
    CommitmentCreate,
    CaptureCreate,
    BranchSyncRequest,
    ValidationRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionData {
    pub action_id: Option<String>,
    pub action_type: ClientActionKind,
    pub target_id: Option<String>,
    pub text: Option<String>,
    pub minutes: Option<u32>,
    #[serde(default)]
    pub payload: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionBatchRequest {
    pub actions: Vec<ClientActionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionResultData {
    pub action_id: Option<String>,
    pub action_type: ClientActionKind,
    pub target_id: Option<String>,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientActionBatchResultData {
    pub applied: u32,
    pub results: Vec<ClientActionResultData>,
}
