use crate::errors::AppError;
use vel_api_types::{AvailableActionData, ConversationContinuationData};
use vel_core::{ActionEvidenceRef, ProjectId};

#[derive(Debug, Clone)]
pub(crate) struct ConversationServiceData {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub message_count: i64,
    pub last_message_at: Option<i64>,
    pub project_label: Option<String>,
    pub continuation: Option<ConversationContinuationData>,
}

#[derive(Debug, Clone)]
pub(crate) struct MessageServiceData {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content: serde_json::Value,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub(crate) struct InboxItemServiceData {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
    pub rank: i64,
    pub conversation_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub project_label: Option<String>,
    pub available_actions: Vec<AvailableActionData>,
    pub evidence: Vec<ActionEvidenceRef>,
}

pub(crate) fn conversation_record_to_data(
    r: vel_storage::ConversationRecord,
) -> ConversationServiceData {
    ConversationServiceData {
        id: r.id.as_ref().to_string(),
        title: r.title,
        kind: r.kind,
        pinned: r.pinned,
        archived: r.archived,
        created_at: r.created_at,
        updated_at: r.updated_at,
        message_count: r.message_count,
        last_message_at: r.last_message_at,
        project_label: r.project_label,
        continuation: None,
    }
}

pub(crate) fn message_record_to_data(
    r: vel_storage::MessageRecord,
) -> Result<MessageServiceData, AppError> {
    let content: serde_json::Value = serde_json::from_str(&r.content_json)
        .unwrap_or_else(|_| serde_json::json!({ "raw": r.content_json }));
    Ok(MessageServiceData {
        id: r.id.as_ref().to_string(),
        conversation_id: r.conversation_id.as_ref().to_string(),
        role: r.role,
        kind: r.kind,
        content,
        status: r.status,
        importance: r.importance,
        created_at: r.created_at,
        updated_at: r.updated_at,
    })
}

/// Extract plain text from a message record for LLM context (kind "text" -> content.text, else raw).
pub(crate) fn message_record_to_llm_content(r: &vel_storage::MessageRecord) -> String {
    let content: serde_json::Value = serde_json::from_str(&r.content_json)
        .unwrap_or_else(|_| serde_json::json!({ "raw": r.content_json }));
    if r.kind == "text" {
        content
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        content.to_string()
    }
}

pub(crate) fn message_title_summary(message: &MessageServiceData) -> (String, String) {
    let title = message
        .content
        .get("title")
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            message
                .content
                .get("text")
                .and_then(serde_json::Value::as_str)
        })
        .unwrap_or("Inbox intervention")
        .to_string();
    let summary = message
        .content
        .get("reason")
        .or_else(|| message.content.get("summary"))
        .or_else(|| message.content.get("text"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("Needs operator review from the current inbox/intervention queue.")
        .to_string();
    (title, summary)
}

pub(crate) fn message_project_details(
    message: &MessageServiceData,
) -> (Option<ProjectId>, Option<String>) {
    let project_id = message
        .content
        .get("project_id")
        .and_then(serde_json::Value::as_str)
        .map(|value| ProjectId::from(value.to_string()));
    let project_label = message
        .content
        .get("project_label")
        .or_else(|| message.content.get("project"))
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);
    (project_id, project_label)
}

pub(crate) fn inbox_available_actions(has_conversation: bool) -> Vec<AvailableActionData> {
    // available_actions: acknowledge, snooze, dismiss, open_thread
    let mut actions = vec![
        AvailableActionData::Acknowledge,
        AvailableActionData::Snooze,
        AvailableActionData::Dismiss,
    ];
    if has_conversation {
        actions.push(AvailableActionData::OpenThread);
    }
    actions
}

pub(crate) fn intervention_record_to_inbox_item(
    r: vel_storage::InterventionRecord,
    rank: i64,
    conversation_id: Option<String>,
    title: String,
    summary: String,
    project_id: Option<ProjectId>,
    project_label: Option<String>,
    evidence: Vec<ActionEvidenceRef>,
) -> InboxItemServiceData {
    InboxItemServiceData {
        id: r.id.as_ref().to_string(),
        message_id: r.message_id.as_ref().to_string(),
        kind: r.kind,
        state: r.state,
        surfaced_at: r.surfaced_at,
        snoozed_until: r.snoozed_until,
        confidence: r.confidence,
        rank,
        conversation_id: conversation_id.clone(),
        title,
        summary,
        project_id,
        project_label,
        available_actions: inbox_available_actions(conversation_id.is_some()),
        evidence,
    }
}
