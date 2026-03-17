use uuid::Uuid;
use vel_api_types::{CreateMessageResponse, MessageCreateRequest, MessageData, WsEventType};
use vel_storage::MessageInsert;

use crate::{
    broadcast::WsEnvelope,
    errors::AppError,
    services::chat::{
        assistant::generate_assistant_reply, events::emit_chat_event,
        interventions::create_intervention_for_message_if_needed, mapping::message_record_to_data,
    },
    state::AppState,
};

fn chat_model_not_configured_error() -> String {
    "Chat model not configured. Set VEL_LLM_MODEL and run llama-server, or see configs/models/README.md.".to_string()
}

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

pub(crate) async fn create_message_response(
    state: &AppState,
    conversation_id: &str,
    payload: &MessageCreateRequest,
) -> Result<CreateMessageResponse, AppError> {
    let conversation_id = conversation_id.trim();
    let user_message = create_user_message(state, conversation_id, payload).await?;

    let (assistant_message, assistant_error) = if let (Some(router), Some(profile_id)) =
        (state.llm_router.as_ref(), state.chat_profile_id.as_ref())
    {
        tracing::info!(conversation_id = %conversation_id, "calling LLM for assistant reply");
        match generate_assistant_reply(
            state,
            conversation_id,
            profile_id,
            state.chat_fallback_profile_id.as_deref(),
            router,
        )
        .await
        {
            Ok(Some(assistant_message)) => {
                let ws_payload = serde_json::to_value(&assistant_message).unwrap_or_default();
                let _ = state
                    .broadcast_tx
                    .send(WsEnvelope::new(WsEventType::MessagesNew, ws_payload));
                (Some(assistant_message), None)
            }
            Ok(None) => (None, None),
            Err(error) => {
                tracing::error!(error = %error, "assistant reply failed");
                (None, Some(error.to_string()))
            }
        }
    } else {
        (None, Some(chat_model_not_configured_error()))
    };

    Ok(CreateMessageResponse {
        user_message,
        assistant_message,
        assistant_error,
    })
}
