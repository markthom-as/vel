use uuid::Uuid;
use vel_storage::MessageInsert;

use crate::{
    errors::AppError,
    services::chat::{
        assistant::generate_assistant_reply,
        events::{broadcast_chat_ws_event, emit_chat_event, WS_EVENT_MESSAGES_NEW},
        interventions::{create_intervention_for_message_if_needed, InterventionMessageInput},
        mapping::{message_record_to_data, MessageServiceData},
    },
    state::AppState,
};

#[derive(Debug, Clone)]
pub(crate) struct ChatMessageCreateInput {
    pub role: String,
    pub kind: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ChatMessage {
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
pub(crate) struct ChatMessageCreateResult {
    pub user_message: ChatMessage,
    pub assistant_message: Option<ChatMessage>,
    pub assistant_error: Option<String>,
}

impl From<MessageServiceData> for ChatMessage {
    fn from(value: MessageServiceData) -> Self {
        Self {
            id: value.id,
            conversation_id: value.conversation_id,
            role: value.role,
            kind: value.kind,
            content: value.content,
            status: value.status,
            importance: value.importance,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

fn chat_model_not_configured_error() -> String {
    "Chat model not configured. Set VEL_LLM_MODEL and run llama-server, or see configs/models/README.md.".to_string()
}

pub(crate) async fn create_user_message(
    state: &AppState,
    conversation_id: &str,
    payload: &ChatMessageCreateInput,
) -> Result<ChatMessage, AppError> {
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
    let created_message = ChatMessage::from(message_record_to_data(msg)?);
    let _ = create_intervention_for_message_if_needed(
        state,
        &InterventionMessageInput {
            id: created_message.id.clone(),
            conversation_id: created_message.conversation_id.clone(),
            role: created_message.role.clone(),
            kind: created_message.kind.clone(),
            content: created_message.content.clone(),
        },
    )
    .await?;

    let ws_payload =
        serde_json::to_value(&created_message).unwrap_or_else(|_| serde_json::json!({ "id": id }));
    broadcast_chat_ws_event(state, WS_EVENT_MESSAGES_NEW, ws_payload);

    Ok(created_message)
}

pub(crate) async fn create_message_response(
    state: &AppState,
    conversation_id: &str,
    payload: &ChatMessageCreateInput,
) -> Result<ChatMessageCreateResult, AppError> {
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
                broadcast_chat_ws_event(state, WS_EVENT_MESSAGES_NEW, ws_payload);
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

    Ok(ChatMessageCreateResult {
        user_message,
        assistant_message,
        assistant_error,
    })
}
