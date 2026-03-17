use uuid::Uuid;
use vel_llm::{LlmError, LlmRequest, Message as LlmMessage, ProviderError, Router};
use vel_storage::MessageInsert;

use crate::{
    errors::AppError,
    services::chat::{
        events::emit_chat_event,
        mapping::{message_record_to_data, message_record_to_llm_content},
        messages::ChatMessage,
    },
    state::AppState,
};

pub(crate) fn should_fallback_for_assistant_error(error: &LlmError) -> bool {
    match error {
        LlmError::NoProviderForProfile(_) => true,
        LlmError::Config(_) => false,
        LlmError::Provider(provider_error) => matches!(
            provider_error,
            ProviderError::Transport(_) | ProviderError::Protocol(_) | ProviderError::Backend(_)
        ),
    }
}

pub(crate) async fn generate_assistant_reply(
    state: &AppState,
    conversation_id: &str,
    profile_id: &str,
    fallback_profile_id: Option<&str>,
    router: &Router,
) -> Result<Option<ChatMessage>, AppError> {
    let list = state
        .storage
        .list_messages_by_conversation(conversation_id, 50)
        .await?;
    let messages: Vec<LlmMessage> = list
        .iter()
        .map(|r| LlmMessage {
            role: r.role.clone(),
            content: message_record_to_llm_content(r),
        })
        .filter(|m| !m.content.is_empty() || m.role == "assistant")
        .collect();
    if messages.is_empty() {
        return Ok(None);
    }

    let system = "You are Vel, a helpful assistant for capture, recall, and daily orientation. Be concise and direct.".to_string();
    let profile_ids = {
        let mut ids = vec![profile_id];
        if let Some(fallback_profile_id) = fallback_profile_id {
            if fallback_profile_id != profile_id {
                ids.push(fallback_profile_id);
            }
        }
        ids
    };

    for (idx, attempt_profile_id) in profile_ids.iter().enumerate() {
        let req = LlmRequest {
            system: system.clone(),
            messages: messages.clone(),
            tools: vec![],
            response_format: vel_llm::ResponseFormat::Text,
            temperature: 0.2,
            max_output_tokens: 2048,
            model_profile: attempt_profile_id.to_string(),
            metadata: serde_json::json!({}),
        };
        let res = router.generate(&req).await;
        match res {
            Ok(res) => {
                let text = res.text.unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    return Ok(None);
                }
                let assistant_id = format!("msg_{}", Uuid::new_v4().simple());
                let content_json =
                    serde_json::to_string(&serde_json::json!({ "text": text })).unwrap_or_default();
                state
                    .storage
                    .create_message(MessageInsert {
                        id: assistant_id.clone(),
                        conversation_id: conversation_id.to_string(),
                        role: "assistant".to_string(),
                        kind: "text".to_string(),
                        content_json,
                        status: None,
                        importance: None,
                    })
                    .await?;
                emit_chat_event(
                    state,
                    "message.created",
                    "message",
                    &assistant_id,
                    serde_json::json!({
                        "id": assistant_id,
                        "conversation_id": conversation_id,
                        "kind": "text"
                    }),
                )
                .await;
                let assistant_msg =
                    state
                        .storage
                        .get_message(&assistant_id)
                        .await?
                        .ok_or_else(|| {
                            AppError::internal("assistant message not found after create")
                        })?;
                return message_record_to_data(assistant_msg)
                    .map(ChatMessage::from)
                    .map(Some);
            }
            Err(err) => {
                let should_try_fallback =
                    idx + 1 < profile_ids.len() && should_fallback_for_assistant_error(&err);
                if should_try_fallback {
                    tracing::warn!(
                        from_profile = attempt_profile_id,
                        to_profile = fallback_profile_id,
                        error = %err,
                        "primary chat profile failed, falling back"
                    );
                    continue;
                }
                return Err(AppError::internal(err.to_string()));
            }
        }
    }
    Ok(None)
}
