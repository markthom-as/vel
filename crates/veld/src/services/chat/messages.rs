use uuid::Uuid;
use vel_api_types::{MessageCreateRequest, MessageData, WsEventType};
use vel_storage::MessageInsert;

use crate::{
    broadcast::WsEnvelope,
    errors::AppError,
    services::chat::{
        events::emit_chat_event, interventions::create_intervention_for_message_if_needed,
        mapping::message_record_to_data,
    },
    state::AppState,
};

pub(crate) async fn create_user_message(
    state: &AppState,
    conversation_id: &str,
    payload: &MessageCreateRequest,
) -> Result<MessageData, AppError> {
    let conversation_id = conversation_id.trim();
    let _ = state
        .storage
        .get_conversation(conversation_id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;

    let id = format!("msg_{}", Uuid::new_v4().simple());
    let content_json = serde_json::to_string(&payload.content)
        .map_err(|e| AppError::bad_request(e.to_string()))?;
    let kind = payload.kind.clone();
    state
        .storage
        .create_message(MessageInsert {
            id: id.clone(),
            conversation_id: conversation_id.to_string(),
            role: payload.role.clone(),
            kind: kind.clone(),
            content_json,
            status: None,
            importance: None,
        })
        .await?;
    emit_chat_event(
        state,
        "message.created",
        "message",
        &id,
        serde_json::json!({ "id": id, "conversation_id": conversation_id, "kind": kind }),
    )
    .await;

    let msg = state
        .storage
        .get_message(&id)
        .await?
        .ok_or_else(|| AppError::internal("message not found after create"))?;
    let created_message = message_record_to_data(msg)?;
    let _ = create_intervention_for_message_if_needed(state, &created_message).await?;

    let ws_payload =
        serde_json::to_value(&created_message).unwrap_or_else(|_| serde_json::json!({ "id": id }));
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(WsEventType::MessagesNew, ws_payload));

    Ok(created_message)
}
