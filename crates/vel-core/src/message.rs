//! Message domain model for chat. See docs/tickets/vel-agent-ticket-pack/005-implement-message-domain-model.md

use serde::{Deserialize, Serialize};
use crate::types::{ConversationId, MessageId};

/// Discriminated message body for structured cards and text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MessageBody {
    Text(TextMessage),
    ReminderCard(ReminderCard),
    RiskCard(RiskCard),
    SuggestionCard(SuggestionCard),
    SummaryCard(SummaryCard),
    SystemNotice(SystemNotice),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextMessage {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReminderCard {
    pub title: String,
    pub due_time: Option<i64>,
    pub reason: Option<String>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskCard {
    pub commitment_title: String,
    pub risk_level: String,
    pub top_drivers: Vec<String>,
    pub proposed_next_step: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestionCard {
    pub suggestion_text: String,
    pub linked_goal: Option<String>,
    pub expected_benefit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SummaryCard {
    pub title: String,
    pub timeframe: Option<String>,
    pub top_items: Vec<String>,
    pub recommended_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemNotice {
    pub text: String,
}

/// Role of the message sender.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

/// Importance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageImportance {
    Low,
    Normal,
    High,
    Urgent,
}

/// Message status for interventions and lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    Active,
    Resolved,
    Dismissed,
    Snoozed,
}

/// Provenance reference for explainability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceRef {
    pub id: String,
    #[serde(rename = "type")]
    pub ref_type: String,
    pub label: String,
    pub detail: Option<String>,
}

/// Inline action on a message/card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageAction {
    pub id: String,
    #[serde(rename = "type")]
    pub action_type: String,
    pub label: String,
    pub payload: Option<serde_json::Value>,
}

/// Full message with shared fields and body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub thread_id: ConversationId,
    pub role: MessageRole,
    pub body: MessageBody,
    pub importance: Option<MessageImportance>,
    pub status: Option<MessageStatus>,
    pub provenance: Option<Vec<ProvenanceRef>>,
    pub actions: Option<Vec<MessageAction>>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}
