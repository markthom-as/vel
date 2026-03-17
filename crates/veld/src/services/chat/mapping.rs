use crate::errors::AppError;

#[derive(Debug, Clone)]
pub(crate) struct ConversationServiceData {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: i64,
    pub updated_at: i64,
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

pub(crate) fn intervention_record_to_inbox_item(
    r: vel_storage::InterventionRecord,
) -> InboxItemServiceData {
    InboxItemServiceData {
        id: r.id.as_ref().to_string(),
        message_id: r.message_id.as_ref().to_string(),
        kind: r.kind,
        state: r.state,
        surfaced_at: r.surfaced_at,
        snoozed_until: r.snoozed_until,
        confidence: r.confidence,
    }
}
