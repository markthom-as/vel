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
    ActionEvidenceRefData, ApiResponse, AssistantActionProposalData, AssistantEntryRequest,
    AssistantEntryResponse, AssistantEntryRouteTargetData, ConversationContinuationData,
    ConversationCreateRequest, ConversationData, ConversationUpdateRequest, CreateMessageResponse,
    InboxItemData, InterventionActionData, MessageCreateRequest, MessageData,
    PlanningProfileEditProposalData, ProvenanceData, ProvenanceEvent,
};

use crate::services::chat::{
    conversations::{
        create_conversation as create_conversation_data,
        update_conversation as update_conversation_data, ConversationCreateInput,
        ConversationUpdateInput,
    },
    interventions::acknowledge_intervention,
    interventions::InterventionAction,
    interventions::{
        dismiss_intervention, reactivate_intervention, resolve_intervention, snooze_intervention,
    },
    mapping::{conversation_record_to_data, message_record_to_data},
    messages::{
        create_assistant_entry_response, create_message_response, AssistantEntryCreateInput,
        AssistantEntryCreateResult, AssistantEntryRouteTarget, ChatMessage, ChatMessageCreateInput,
        ChatMessageCreateResult, VoiceEntryProvenance,
    },
    reads::{
        build_message_provenance_data, list_conversation_intervention_items,
        list_inbox_archived_items, list_inbox_items, list_message_intervention_items, MessageProvenance,
        ProvenanceMessageEvent,
    },
    settings::settings_payload,
};
use crate::{errors::AppError, state::AppState};

fn map_conversation_data(
    data: crate::services::chat::mapping::ConversationServiceData,
) -> ConversationData {
    ConversationData {
        id: data.id,
        title: data.title,
        kind: data.kind,
        pinned: data.pinned,
        archived: data.archived,
        created_at: data.created_at,
        updated_at: data.updated_at,
        continuation: data.continuation,
    }
}

async fn enrich_conversation_continuation(
    state: &AppState,
    data: &mut crate::services::chat::mapping::ConversationServiceData,
) -> Result<(), AppError> {
    data.continuation = crate::services::chat::thread_continuation::conversation_continuation_data(
        &state.storage,
        &data.id,
    )
    .await?;
    Ok(())
}

fn conversation_data_with_continuation(
    data: crate::services::chat::mapping::ConversationServiceData,
    continuation: Option<ConversationContinuationData>,
) -> ConversationData {
    ConversationData {
        continuation,
        ..map_conversation_data(data)
    }
}

fn map_message_data(data: crate::services::chat::mapping::MessageServiceData) -> MessageData {
    MessageData {
        id: data.id,
        conversation_id: data.conversation_id,
        role: data.role,
        kind: data.kind,
        content: data.content,
        status: data.status,
        importance: data.importance,
        created_at: data.created_at,
        updated_at: data.updated_at,
    }
}

fn map_chat_message_data(data: ChatMessage) -> MessageData {
    MessageData {
        id: data.id,
        conversation_id: data.conversation_id,
        role: data.role,
        kind: data.kind,
        content: data.content,
        status: data.status,
        importance: data.importance,
        created_at: data.created_at,
        updated_at: data.updated_at,
    }
}

fn map_inbox_item_data(data: crate::services::chat::reads::InboxItem) -> InboxItemData {
    InboxItemData {
        id: data.id,
        message_id: data.message_id,
        kind: data.kind,
        state: data.state,
        surfaced_at: data.surfaced_at,
        snoozed_until: data.snoozed_until,
        confidence: data.confidence,
        conversation_id: data.conversation_id,
        title: data.title,
        summary: data.summary,
        project_id: data.project_id,
        project_label: data.project_label,
        available_actions: data.available_actions,
        evidence: data
            .evidence
            .into_iter()
            .map(ActionEvidenceRefData::from)
            .collect(),
    }
}

fn map_provenance_event(event: ProvenanceMessageEvent) -> ProvenanceEvent {
    ProvenanceEvent {
        id: event.id,
        event_name: event.event_name,
        created_at: event.created_at,
        payload: event.payload,
    }
}

fn map_provenance_data(data: MessageProvenance) -> ProvenanceData {
    ProvenanceData {
        message_id: data.message_id,
        events: data.events.into_iter().map(map_provenance_event).collect(),
        signals: data.signals,
        policy_decisions: data.policy_decisions,
        linked_objects: data.linked_objects,
    }
}

fn map_intervention_action(action: InterventionAction) -> InterventionActionData {
    InterventionActionData {
        id: action.id,
        state: action.state,
    }
}

fn map_create_message_response(data: ChatMessageCreateResult) -> CreateMessageResponse {
    CreateMessageResponse {
        user_message: map_chat_message_data(data.user_message),
        assistant_message: data.assistant_message.map(map_chat_message_data),
        assistant_error: data.assistant_error,
        assistant_context: data.assistant_context,
    }
}

fn map_assistant_entry_response(data: AssistantEntryCreateResult) -> AssistantEntryResponse {
    AssistantEntryResponse {
        route_target: match data.route_target {
            AssistantEntryRouteTarget::Inbox => AssistantEntryRouteTargetData::Inbox,
            AssistantEntryRouteTarget::Threads => AssistantEntryRouteTargetData::Threads,
            AssistantEntryRouteTarget::Inline => AssistantEntryRouteTargetData::Inline,
        },
        entry_intent: data.entry_intent.as_deref().map(map_entry_intent),
        continuation_category: data
            .continuation_category
            .as_deref()
            .map(map_continuation_category),
        user_message: map_chat_message_data(data.user_message),
        assistant_message: data.assistant_message.map(map_chat_message_data),
        assistant_error: data.assistant_error,
        assistant_context: data.assistant_context,
        conversation: data.conversation.map(map_conversation_data),
        proposal: data.proposal.map(AssistantActionProposalData::from),
        planning_profile_proposal: data
            .planning_profile_proposal
            .map(PlanningProfileEditProposalData::from),
        daily_loop_session: data.daily_loop_session.map(Into::into),
        end_of_day: data
            .end_of_day
            .map(crate::routes::context::map_end_of_day_data),
    }
}

fn map_entry_intent(value: &str) -> vel_api_types::NowDockedInputIntentData {
    match value {
        "question" => vel_api_types::NowDockedInputIntentData::Question,
        "note" => vel_api_types::NowDockedInputIntentData::Note,
        "command" => vel_api_types::NowDockedInputIntentData::Command,
        "continuation" => vel_api_types::NowDockedInputIntentData::Continuation,
        "reflection" => vel_api_types::NowDockedInputIntentData::Reflection,
        "scheduling" => vel_api_types::NowDockedInputIntentData::Scheduling,
        _ => vel_api_types::NowDockedInputIntentData::Task,
    }
}

fn map_continuation_category(value: &str) -> vel_api_types::NowHeaderBucketKindData {
    match value {
        "needs_input" => vel_api_types::NowHeaderBucketKindData::NeedsInput,
        "new_nudges" => vel_api_types::NowHeaderBucketKindData::NewNudges,
        "search_filter" => vel_api_types::NowHeaderBucketKindData::SearchFilter,
        "snoozed" => vel_api_types::NowHeaderBucketKindData::Snoozed,
        "review_apply" => vel_api_types::NowHeaderBucketKindData::ReviewApply,
        "reflow" => vel_api_types::NowHeaderBucketKindData::Reflow,
        "follow_up" => vel_api_types::NowHeaderBucketKindData::FollowUp,
        _ => vel_api_types::NowHeaderBucketKindData::ThreadsByType,
    }
}

// --- Conversation handlers ---

pub async fn list_conversations(
    State(state): State<AppState>,
    Query(q): Query<ListConversationsQuery>,
) -> Result<Json<ApiResponse<Vec<ConversationData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let list = state.storage.list_conversations(q.archived, limit).await?;
    let mut data = Vec::with_capacity(list.len());
    for conversation in list
        .into_iter()
        .map(conversation_record_to_data)
        .collect::<Vec<_>>()
    {
        let continuation =
            crate::services::chat::thread_continuation::conversation_continuation_data(
                &state.storage,
                &conversation.id,
            )
            .await?;
        data.push(conversation_data_with_continuation(
            conversation,
            continuation,
        ));
    }
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
    let conv = create_conversation_data(
        &state,
        ConversationCreateInput {
            title: payload.title,
            kind: payload.kind,
        },
    )
    .await?;
    let mut conv = conversation_record_to_data(conv);
    enrich_conversation_continuation(&state, &mut conv).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_conversation_data(conv),
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
    let mut conv = conversation_record_to_data(conv);
    enrich_conversation_continuation(&state, &mut conv).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_conversation_data(conv),
        request_id,
    )))
}

pub async fn update_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ConversationUpdateRequest>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
    let conv = update_conversation_data(
        &state,
        id.trim(),
        ConversationUpdateInput {
            title: payload.title,
            pinned: payload.pinned,
            archived: payload.archived,
        },
    )
    .await?;
    let mut conv = conversation_record_to_data(conv);
    enrich_conversation_continuation(&state, &mut conv).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_conversation_data(conv),
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
        data.push(map_message_data(message_record_to_data(r)?));
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
    let response = create_message_response(
        &state,
        conversation_id.trim(),
        &ChatMessageCreateInput {
            role: payload.role,
            kind: payload.kind,
            content: payload.content,
        },
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_create_message_response(response),
        request_id,
    )))
}

pub async fn create_assistant_entry(
    State(state): State<AppState>,
    Json(payload): Json<AssistantEntryRequest>,
) -> Result<Json<ApiResponse<AssistantEntryResponse>>, AppError> {
    let response = create_assistant_entry_response(
        &state,
        &AssistantEntryCreateInput {
            text: payload.text,
            conversation_id: payload.conversation_id,
            voice: payload.voice.map(|voice| VoiceEntryProvenance {
                surface: voice.surface,
                source_device: voice.source_device,
                locale: voice.locale,
                transcript_origin: voice.transcript_origin,
                recorded_at: voice.recorded_at,
                offline_captured_at: voice.offline_captured_at,
                queued_at: voice.queued_at,
            }),
        },
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_assistant_entry_response(response),
        request_id,
    )))
}

// --- Inbox handler ---

pub async fn get_inbox(
    State(state): State<AppState>,
    Query(q): Query<InboxQuery>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let items = match q.scope.as_deref() {
        Some("archive") => list_inbox_archived_items(&state, limit).await?,
        _ => list_inbox_items(&state, limit).await?,
    };
    let data = items.into_iter().map(map_inbox_item_data).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    pub limit: Option<u32>,
    /// `archive` returns resolved/dismissed interventions; default is the operator queue.
    pub scope: Option<String>,
}

// --- Message interventions (for inline actions) ---

pub async fn list_conversation_interventions(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let data = list_conversation_intervention_items(&state, conversation_id.trim())
        .await?
        .into_iter()
        .map(map_inbox_item_data)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn list_message_interventions(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let data = list_message_intervention_items(&state, message_id.trim())
        .await?
        .into_iter()
        .map(map_inbox_item_data)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_message_provenance(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<ProvenanceData>>, AppError> {
    let data = build_message_provenance_data(&state, message_id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_provenance_data(data),
        request_id,
    )))
}

// --- Settings (031) ---

pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let data = settings_payload(&state).await?;
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
    pub node_display_name: Option<String>,
    pub writeback_enabled: Option<bool>,
    pub tailscale_preferred: Option<bool>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub llm: Option<crate::services::llm_settings::LlmSettingsUpdateRequest>,
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
    if let Some(value) = payload.node_display_name {
        write_optional_string_setting(&state, "node_display_name", &value).await?;
    }
    if let Some(value) = payload.writeback_enabled {
        state
            .storage
            .set_setting("writeback_enabled", &serde_json::json!(value))
            .await?;
    }
    if let Some(value) = payload.tailscale_preferred {
        state
            .storage
            .set_setting("tailscale_preferred", &serde_json::json!(value))
            .await?;
    }
    if let Some(value) = payload.tailscale_base_url {
        write_optional_url_setting(&state, "tailscale_base_url", &value).await?;
    }
    if let Some(value) = payload.lan_base_url {
        write_optional_url_setting(&state, "lan_base_url", &value).await?;
    }
    if let Some(request) = payload.llm.as_ref() {
        crate::services::llm_settings::apply_llm_settings_update(request)?;
    }
    let data = settings_payload(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

async fn write_optional_string_setting(
    state: &AppState,
    key: &str,
    value: &str,
) -> Result<(), AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        state
            .storage
            .set_setting(key, &serde_json::Value::Null)
            .await?;
    } else {
        state
            .storage
            .set_setting(key, &serde_json::json!(trimmed))
            .await?;
    }
    Ok(())
}

async fn write_optional_url_setting(
    state: &AppState,
    key: &str,
    value: &str,
) -> Result<(), AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        state
            .storage
            .set_setting(key, &serde_json::Value::Null)
            .await?;
        return Ok(());
    }

    reqwest::Url::parse(trimmed)
        .map_err(|error| AppError::bad_request(format!("invalid {}: {}", key, error)))?;
    state
        .storage
        .set_setting(key, &serde_json::json!(trimmed))
        .await?;
    Ok(())
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
    let res = snooze_intervention(&state, id.trim(), until_ts).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_intervention_action(res),
        request_id,
    )))
}

#[derive(Debug, Deserialize)]
pub struct SnoozeRequest {
    pub snoozed_until_ts: Option<i64>,
    pub minutes: Option<u32>,
}

pub async fn intervention_acknowledge(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let res = acknowledge_intervention(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_intervention_action(res),
        request_id,
    )))
}

pub async fn intervention_resolve(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let res = resolve_intervention(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_intervention_action(res),
        request_id,
    )))
}

pub async fn intervention_reactivate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let res = reactivate_intervention(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_intervention_action(res),
        request_id,
    )))
}

pub async fn intervention_dismiss(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let res = dismiss_intervention(&state, id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_intervention_action(res),
        request_id,
    )))
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
        .route("/api/assistant/entry", post(create_assistant_entry))
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
        .route(
            "/api/interventions/:id/acknowledge",
            post(intervention_acknowledge),
        )
        .route("/api/interventions/:id/snooze", post(intervention_snooze))
        .route("/api/interventions/:id/resolve", post(intervention_resolve))
        .route("/api/interventions/:id/reactivate", post(intervention_reactivate))
        .route("/api/interventions/:id/dismiss", post(intervention_dismiss))
}

#[cfg(test)]
mod tests {
    use crate::services::chat::messages::ChatMessage;
    use serde_json::json;
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
        let message = ChatMessage {
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
