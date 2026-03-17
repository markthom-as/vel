//! Chat API: conversations, messages, inbox, interventions.
//! Tickets 014–017. Routes under /api/.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ConversationCreateRequest, ConversationData, ConversationUpdateRequest,
    CreateMessageResponse, InboxItemData, InterventionActionData, MessageCreateRequest,
    MessageData, ProvenanceData, ProvenanceEvent, WsEventType,
};
use vel_storage::ConversationInsert;

use crate::broadcast::WsEnvelope;
use crate::services::chat::{
    assistant::generate_assistant_reply,
    events::emit_chat_event,
    interventions::{dismiss_intervention, resolve_intervention, snooze_intervention},
    mapping::{
        conversation_record_to_data, intervention_record_to_inbox_item, message_record_to_data,
    },
    messages::create_user_message,
    provenance::{build_linked_objects, build_policy_decisions, build_provenance_signals},
    settings::settings_payload,
};
use crate::{errors::AppError, state::AppState};

// --- Conversation handlers ---

pub async fn list_conversations(
    State(state): State<AppState>,
    Query(q): Query<ListConversationsQuery>,
) -> Result<Json<ApiResponse<Vec<ConversationData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let list = state.storage.list_conversations(q.archived, limit).await?;
    let data: Vec<ConversationData> = list.into_iter().map(conversation_record_to_data).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct ListConversationsQuery {
    pub archived: Option<bool>,
    pub limit: Option<u32>,
}

pub async fn create_conversation(
    State(state): State<AppState>,
    Json(payload): Json<ConversationCreateRequest>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
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
        &state,
        "conversation.created",
        "conversation",
        &id,
        serde_json::json!({ "id": id, "kind": kind }),
    )
    .await;
    let conv = state
        .storage
        .get_conversation(&id)
        .await?
        .ok_or_else(|| AppError::internal("conversation not found after create"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        conversation_record_to_data(conv),
        request_id,
    )))
}

pub async fn get_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
    let conv = state
        .storage
        .get_conversation(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        conversation_record_to_data(conv),
        request_id,
    )))
}

pub async fn update_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ConversationUpdateRequest>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
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
        &state,
        "conversation.updated",
        "conversation",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let conv = state
        .storage
        .get_conversation(id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        conversation_record_to_data(conv),
        request_id,
    )))
}

// --- Message handlers ---

pub async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Query(q): Query<ListMessagesQuery>,
) -> Result<Json<ApiResponse<Vec<MessageData>>>, AppError> {
    let limit = q.limit.unwrap_or(200).min(2000);
    let list = state
        .storage
        .list_messages_by_conversation(conversation_id.trim(), limit)
        .await?;
    let mut data = Vec::with_capacity(list.len());
    for r in list {
        data.push(message_record_to_data(r)?);
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct ListMessagesQuery {
    pub limit: Option<u32>,
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<MessageCreateRequest>,
) -> Result<Json<ApiResponse<CreateMessageResponse>>, AppError> {
    let conversation_id = conversation_id.trim();
    let created_message = create_user_message(&state, conversation_id, &payload).await?;

    let (assistant_message, assistant_error) = if let (Some(router), Some(profile_id)) =
        (state.llm_router.as_ref(), state.chat_profile_id.as_ref())
    {
        tracing::info!(conversation_id = %conversation_id, "calling LLM for assistant reply");
        match generate_assistant_reply(
            &state,
            conversation_id,
            profile_id,
            state.chat_fallback_profile_id.as_deref(),
            router,
        )
        .await
        {
            Ok(Some(assistant_data)) => {
                let payload = serde_json::to_value(&assistant_data).unwrap_or_default();
                let _ = state
                    .broadcast_tx
                    .send(WsEnvelope::new(WsEventType::MessagesNew, payload));
                (Some(assistant_data), None)
            }
            Ok(None) => (None, None),
            Err(e) => {
                let msg = e.to_string();
                tracing::error!(error = %e, "assistant reply failed");
                (None, Some(msg))
            }
        }
    } else {
        (
            None,
            Some("Chat model not configured. Set VEL_LLM_MODEL and run llama-server, or see configs/models/README.md.".to_string()),
        )
    };

    let response = CreateMessageResponse {
        user_message: created_message,
        assistant_message,
        assistant_error,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(response, request_id)))
}

// --- Inbox handler ---

pub async fn get_inbox(
    State(state): State<AppState>,
    Query(q): Query<InboxQuery>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let list = state.storage.list_interventions_active(limit).await?;
    let data: Vec<InboxItemData> = list
        .into_iter()
        .map(intervention_record_to_inbox_item)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    pub limit: Option<u32>,
}

// --- Message interventions (for inline actions) ---

pub async fn list_conversation_interventions(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let conversation_id = conversation_id.trim();
    let _ = state
        .storage
        .get_conversation(conversation_id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let list = state
        .storage
        .get_interventions_by_conversation(conversation_id)
        .await?;
    let data: Vec<InboxItemData> = list
        .into_iter()
        .map(intervention_record_to_inbox_item)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn list_message_interventions(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let list = state
        .storage
        .get_interventions_by_message(message_id.trim())
        .await?;
    let data: Vec<InboxItemData> = list
        .into_iter()
        .map(intervention_record_to_inbox_item)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_message_provenance(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<ProvenanceData>>, AppError> {
    let message_id = message_id.trim();
    let message = state
        .storage
        .get_message(message_id)
        .await?
        .ok_or_else(|| AppError::not_found("message not found"))?;
    let message = message_record_to_data(message)?;
    let interventions = state
        .storage
        .get_interventions_by_message(message_id)
        .await?;
    let events = state
        .storage
        .list_events_by_aggregate("message", message_id, 50)
        .await?;
    let events_data: Vec<ProvenanceEvent> = events
        .into_iter()
        .map(|r| {
            let payload: serde_json::Value =
                serde_json::from_str(&r.payload_json).unwrap_or_else(|_| serde_json::json!({}));
            ProvenanceEvent {
                id: r.id.as_ref().to_string(),
                event_name: r.event_name,
                created_at: r.created_at,
                payload,
            }
        })
        .collect();
    let linked_objects = build_linked_objects(&message, &interventions);
    let signals = build_provenance_signals(&message, &interventions);
    let policy_decisions = build_policy_decisions(&message, &interventions);
    let data = ProvenanceData {
        message_id: message_id.to_string(),
        events: events_data,
        signals,
        policy_decisions,
        linked_objects,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

// --- Settings (031) ---

pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let data = settings_payload(&state.storage).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct SettingsUpdateRequest {
    pub quiet_hours: Option<serde_json::Value>,
    pub disable_proactive: Option<bool>,
    pub toggle_risks: Option<bool>,
    pub toggle_reminders: Option<bool>,
    pub timezone: Option<String>,
}

pub async fn patch_settings(
    State(state): State<AppState>,
    Json(payload): Json<SettingsUpdateRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    if let Some(v) = payload.quiet_hours {
        state.storage.set_setting("quiet_hours", &v).await?;
    }
    if let Some(v) = payload.disable_proactive {
        state
            .storage
            .set_setting("disable_proactive", &serde_json::json!(v))
            .await?;
    }
    if let Some(v) = payload.toggle_risks {
        state
            .storage
            .set_setting("toggle_risks", &serde_json::json!(v))
            .await?;
    }
    if let Some(v) = payload.toggle_reminders {
        state
            .storage
            .set_setting("toggle_reminders", &serde_json::json!(v))
            .await?;
    }
    if let Some(value) = payload.timezone {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            state
                .storage
                .set_setting("timezone", &serde_json::Value::Null)
                .await?;
        } else {
            let timezone = crate::services::timezone::ResolvedTimeZone::parse(trimmed)?;
            state
                .storage
                .set_setting("timezone", &serde_json::json!(timezone.name))
                .await?;
        }
    }
    let data = settings_payload(&state.storage).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

// --- Intervention action handlers ---

pub async fn intervention_snooze(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<SnoozeRequest>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let until_ts = payload
        .snoozed_until_ts
        .or(payload
            .minutes
            .map(|m| time::OffsetDateTime::now_utc().unix_timestamp() + (m as i64) * 60))
        .ok_or_else(|| AppError::bad_request("snoozed_until_ts or minutes required"))?;
    let payload = snooze_intervention(&state, id.trim(), until_ts).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(payload, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct SnoozeRequest {
    pub snoozed_until_ts: Option<i64>,
    pub minutes: Option<u32>,
}

pub async fn intervention_resolve(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let payload = resolve_intervention(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(payload, request_id)))
}

pub async fn intervention_dismiss(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let payload = dismiss_intervention(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(payload, request_id)))
}

pub fn chat_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/conversations",
            get(list_conversations).post(create_conversation),
        )
        .route(
            "/api/conversations/:id",
            get(get_conversation).patch(update_conversation),
        )
        .route(
            "/api/conversations/:id/messages",
            get(list_messages).post(create_message),
        )
        .route(
            "/api/conversations/:id/interventions",
            get(list_conversation_interventions),
        )
        .route("/api/inbox", get(get_inbox))
        .route(
            "/api/messages/:id/interventions",
            get(list_message_interventions),
        )
        .route("/api/messages/:id/provenance", get(get_message_provenance))
        .route("/api/settings", get(get_settings).patch(patch_settings))
        .route("/api/interventions/:id/snooze", post(intervention_snooze))
        .route("/api/interventions/:id/resolve", post(intervention_resolve))
        .route("/api/interventions/:id/dismiss", post(intervention_dismiss))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use vel_api_types::MessageData;
    use vel_llm::{LlmError, ProviderError};

    #[test]
    fn should_fallback_for_assistant_error_true_when_retryable() {
        assert!(
            crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::NoProviderForProfile("primary".to_string(),)
            )
        );
        assert!(
            crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Provider(ProviderError::Transport("conn reset".to_string()),)
            )
        );
        assert!(
            crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Provider(ProviderError::Protocol("json decode".to_string()),)
            )
        );
        assert!(
            crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Provider(ProviderError::Backend("rate limit".to_string()),)
            )
        );
    }

    #[test]
    fn should_fallback_for_assistant_error_false_when_not_retryable() {
        assert!(
            !crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Config("invalid routing".to_string(),)
            )
        );
        assert!(
            !crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Provider(ProviderError::Capability("unsupported tools".to_string(),),)
            )
        );
        assert!(
            !crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Provider(ProviderError::Auth("invalid token".to_string()),)
            )
        );
        assert!(
            !crate::services::chat::assistant::should_fallback_for_assistant_error(
                &LlmError::Provider(ProviderError::RateLimit("maxed".to_string()),)
            )
        );
    }

    #[test]
    fn risk_card_summaries_normalize_risk_level() {
        let message = MessageData {
            id: "msg_1".to_string(),
            conversation_id: "con_1".to_string(),
            role: "assistant".to_string(),
            kind: "risk_card".to_string(),
            content: json!({
                "commitment_title": "Ship report",
                "risk_level": "danger",
                "top_drivers": ["due soon"],
                "proposed_next_step": "Start now"
            }),
            status: None,
            importance: None,
            created_at: 1,
            updated_at: None,
        };

        let signal_summary = crate::services::chat::provenance::message_signal_summary(&message)
            .expect("signal summary");
        let policy_summary = crate::services::chat::provenance::message_policy_summary(&message)
            .expect("policy summary");

        assert_eq!(signal_summary["risk_level"], "unknown");
        assert_eq!(policy_summary["risk_level"], "unknown");
    }
}
