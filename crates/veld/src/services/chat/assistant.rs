use uuid::Uuid;
use vel_llm::{LlmError, LlmRequest, Message as LlmMessage, ProviderError, Router};
use vel_storage::MessageInsert;

use crate::{
    errors::AppError,
    services::chat::{
        events::emit_chat_event,
        mapping::{message_record_to_data, message_record_to_llm_content},
        messages::ChatMessage,
        tools::{build_chat_grounding_prompt, chat_tool_specs, execute_chat_tool},
    },
    state::AppState,
};

const MAX_CHAT_TOOL_ROUNDS: usize = 3;

pub(crate) fn should_fallback_for_assistant_error(error: &LlmError) -> bool {
    match error {
        LlmError::NoProviderForProfile(_) => true,
        LlmError::Config(_) => false,
        LlmError::Provider(provider_error) => matches!(
            provider_error,
            ProviderError::Transport(_)
                | ProviderError::Protocol(_)
                | ProviderError::Backend(_)
                | ProviderError::Capability(_)
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

    let system = build_chat_grounding_prompt(state).await?;
    let tools = chat_tool_specs();
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
        match generate_with_profile(
            state,
            conversation_id,
            attempt_profile_id,
            router,
            &system,
            &messages,
            &tools,
        )
        .await
        {
            Ok(result) => return Ok(result),
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

async fn generate_with_profile(
    state: &AppState,
    conversation_id: &str,
    profile_id: &str,
    router: &Router,
    system: &str,
    seed_messages: &[LlmMessage],
    tools: &[vel_llm::ToolSpec],
) -> Result<Option<ChatMessage>, LlmError> {
    let mut messages = seed_messages.to_vec();
    let mut tools_enabled = true;

    for round in 0..=MAX_CHAT_TOOL_ROUNDS {
        let req = LlmRequest {
            system: system.to_string(),
            messages: messages.clone(),
            tools: if tools_enabled {
                tools.to_vec()
            } else {
                Vec::new()
            },
            response_format: vel_llm::ResponseFormat::Text,
            temperature: 0.2,
            max_output_tokens: 2048,
            model_profile: profile_id.to_string(),
            metadata: serde_json::json!({
                "conversation_id": conversation_id,
                "assistant_surface": "chat",
                "tools_enabled": tools_enabled,
                "tool_round": round,
            }),
        };
        let res = match router.generate(&req).await {
            Ok(response) => response,
            Err(LlmError::Provider(ProviderError::Capability(_))) if tools_enabled => {
                tools_enabled = false;
                continue;
            }
            Err(error) => return Err(error),
        };

        if tools_enabled && !res.tool_calls.is_empty() {
            if round >= MAX_CHAT_TOOL_ROUNDS {
                return Err(LlmError::Provider(ProviderError::Backend(
                    "assistant exceeded max tool rounds".to_string(),
                )));
            }

            let mut results = Vec::with_capacity(res.tool_calls.len());
            for call in res.tool_calls {
                let output = match execute_chat_tool(state, &call.name, &call.arguments).await {
                    Ok(value) => value,
                    Err(error) => serde_json::json!({
                        "error": error.to_string(),
                    }),
                };
                results.push(serde_json::json!({
                    "id": call.id,
                    "name": call.name,
                    "arguments": call.arguments,
                    "output": output,
                }));
            }

            emit_chat_event(
                state,
                "assistant.tools_used",
                "conversation",
                conversation_id,
                serde_json::json!({
                    "conversation_id": conversation_id,
                    "profile_id": profile_id,
                    "tool_round": round,
                    "tool_results": results,
                }),
            )
            .await;

            messages.push(LlmMessage {
                role: "user".to_string(),
                content: format!(
                    "Vel tool results for the current request:\n{}\n\nUse these results to answer the user directly. If another lookup is required, call another Vel tool.",
                    serde_json::to_string_pretty(&results).unwrap_or_else(|_| "[]".to_string())
                ),
            });
            continue;
        }

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
            .await
            .map_err(|error| LlmError::Provider(ProviderError::Backend(error.to_string())))?;
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
        let assistant_msg = state
            .storage
            .get_message(&assistant_id)
            .await
            .map_err(|error| LlmError::Provider(ProviderError::Backend(error.to_string())))?
            .ok_or_else(|| {
                LlmError::Provider(ProviderError::Backend(
                    "assistant message not found after create".to_string(),
                ))
            })?;
        let data = message_record_to_data(assistant_msg)
            .map_err(|error| LlmError::Provider(ProviderError::Backend(error.to_string())))?;
        return Ok(Some(ChatMessage::from(data)));
    }

    Ok(None)
}
