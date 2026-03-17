use uuid::Uuid;
use vel_api_types::{ConversationCreateRequest, ConversationData, ConversationUpdateRequest};
use vel_storage::ConversationInsert;

use crate::{
    errors::AppError,
    services::chat::{events::emit_chat_event, mapping::conversation_record_to_data},
    state::AppState,
};

pub(crate) async fn create_conversation(
    state: &AppState,
    payload: ConversationCreateRequest,
) -> Result<ConversationData, AppError> {
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
    Ok(conversation_record_to_data(conversation))
}

pub(crate) async fn update_conversation(
    state: &AppState,
    id: &str,
    payload: ConversationUpdateRequest,
) -> Result<ConversationData, AppError> {
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
    Ok(conversation_record_to_data(conversation))
}
