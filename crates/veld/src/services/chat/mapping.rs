use vel_api_types::{ConversationData, InboxItemData, MessageData};

use crate::errors::AppError;

pub(crate) fn conversation_record_to_data(r: vel_storage::ConversationRecord) -> ConversationData {
    ConversationData {
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
) -> Result<MessageData, AppError> {
    let content: serde_json::Value = serde_json::from_str(&r.content_json)
        .unwrap_or_else(|_| serde_json::json!({ "raw": r.content_json }));
    Ok(MessageData {
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
) -> InboxItemData {
    InboxItemData {
        id: r.id.as_ref().to_string(),
        message_id: r.message_id.as_ref().to_string(),
        kind: r.kind,
        state: r.state,
        surfaced_at: r.surfaced_at,
        snoozed_until: r.snoozed_until,
        confidence: r.confidence,
    }
}
