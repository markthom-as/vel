use uuid::Uuid;
use vel_storage::{ConversationInsert, ConversationRecord};

use crate::{
    errors::AppError,
    services::chat::events::emit_chat_event,
    state::AppState,
};

#[derive(Debug, Clone)]
pub(crate) struct ConversationCreateInput {
    pub title: Option<String>,
    pub kind: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ConversationUpdateInput {
    pub title: Option<String>,
    pub pinned: Option<bool>,
    pub archived: Option<bool>,
}

pub(crate) async fn create_conversation(
    state: &AppState,
    payload: ConversationCreateInput,
) -> Result<ConversationRecord, AppError> {
    let id = format!("conv_{}", Uuid::new_v4().simple());
    let kind = payload.kind.clone();
    state
        .storage
        .create_conversation(ConversationInsert {
            id: id.clone(),
            title: payload.title,
            kind: kind.clone(),
            pinned: false,
            archived: false,
        })
        .await?;
    emit_chat_event(
        state,
        "conversation.created",
        "conversation",
        &id,
        serde_json::json!({ "id": id, "kind": kind }),
    )
    .await;
    let conversation = state
        .storage
        .get_conversation(&id)
        .await?
        .ok_or_else(|| AppError::internal("conversation not found after create"))?;
    Ok(conversation)
}

pub(crate) async fn update_conversation(
    state: &AppState,
    id: &str,
    payload: ConversationUpdateInput,
) -> Result<ConversationRecord, AppError> {
    let id = id.trim();
    if let Some(title) = payload.title {
        state.storage.rename_conversation(id, &title).await?;
    }
    if let Some(pinned) = payload.pinned {
        state.storage.pin_conversation(id, pinned).await?;
    }
    if let Some(archived) = payload.archived {
        state.storage.archive_conversation(id, archived).await?;
    }
    emit_chat_event(
        state,
        "conversation.updated",
        "conversation",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let conversation = state
        .storage
        .get_conversation(id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    Ok(conversation)
}
