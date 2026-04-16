use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{AssistantContextData, ThreadContinuationData, UnixSeconds};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    #[serde(default)]
    pub call_mode_active: bool,
    pub created_at: UnixSeconds,
    pub updated_at: UnixSeconds,
    #[serde(default)]
    pub message_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<UnixSeconds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<ConversationContinuationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContinuationData {
    pub thread_id: String,
    pub thread_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    pub continuation: ThreadContinuationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationCreateRequest {
    pub title: Option<String>,
    #[serde(default = "default_conversation_kind")]
    pub kind: String,
}

fn default_conversation_kind() -> String {
    "general".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationUpdateRequest {
    pub title: Option<String>,
    pub pinned: Option<bool>,
    pub archived: Option<bool>,
    pub call_mode_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content: JsonValue,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub created_at: UnixSeconds,
    pub updated_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageResponse {
    pub user_message: MessageData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message: Option<MessageData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_error: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub assistant_error_retryable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assistant_context: Option<AssistantContextData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateRequest {
    pub role: String,
    pub kind: String,
    pub content: JsonValue,
}
