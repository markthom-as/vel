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
    MessageData, ProvenanceData,
};

use crate::services::chat::{
    conversations::{
        create_conversation as create_conversation_data,
        update_conversation as update_conversation_data,
    },
    interventions::{dismiss_intervention, resolve_intervention, snooze_intervention},
    mapping::{conversation_record_to_data, message_record_to_data},
    messages::create_message_response,
    reads::{
        build_message_provenance_data, list_conversation_intervention_items, list_inbox_items,
        list_message_intervention_items,
    },
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
    let conv = create_conversation_data(&state, payload).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(conv, request_id)))
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
    let conv = update_conversation_data(&state, id.trim(), payload).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(conv, request_id)))
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
    let response = create_message_response(&state, conversation_id.trim(), &payload).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(response, request_id)))
}

// --- Inbox handler ---

pub async fn get_inbox(
    State(state): State<AppState>,
    Query(q): Query<InboxQuery>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let data = list_inbox_items(&state, limit).await?;
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
    let data = list_conversation_intervention_items(&state, conversation_id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn list_message_interventions(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let data = list_message_intervention_items(&state, message_id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_message_provenance(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<ProvenanceData>>, AppError> {
    let data = build_message_provenance_data(&state, message_id.trim()).await?;
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
