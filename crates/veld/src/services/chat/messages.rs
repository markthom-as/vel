use uuid::Uuid;
use vel_core::{
    ActionPermissionMode, ActionState, AssistantActionProposal, AssistantProposalState,
    DailyLoopSurface, PlanningProfileEditProposal, PlanningProfileSurface,
};
use vel_storage::MessageInsert;

use crate::{
    errors::AppError,
    services::chat::{
        assistant::generate_assistant_reply,
        conversations::{create_conversation, ConversationCreateInput},
        events::{broadcast_chat_ws_event, emit_chat_event, WS_EVENT_MESSAGES_NEW},
        interventions::{create_intervention_for_message_if_needed, InterventionMessageInput},
        mapping::{message_record_to_data, MessageServiceData},
    },
    services::execution_routing::HandoffReviewState,
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
    pub assistant_error_retryable: bool,
    pub assistant_context: Option<vel_api_types::AssistantContextData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssistantEntryRouteTarget {
    Inbox,
    Threads,
    Inline,
}

#[derive(Debug, Clone)]
pub(crate) struct AssistantEntryCreateInput {
    pub text: String,
    pub conversation_id: Option<String>,
    pub intent: Option<vel_api_types::NowDockedInputIntentData>,
    pub attachments: Vec<vel_api_types::AssistantEntryAttachmentData>,
    pub voice: Option<VoiceEntryProvenance>,
}

#[derive(Debug, Clone)]
pub(crate) struct AssistantEntryCreateResult {
    pub route_target: AssistantEntryRouteTarget,
    pub entry_intent: Option<String>,
    pub continuation_category: Option<String>,
    pub follow_up: Option<vel_api_types::AssistantEntryFollowUpData>,
    pub user_message: ChatMessage,
    pub assistant_message: Option<ChatMessage>,
    pub assistant_error: Option<String>,
    pub assistant_error_retryable: bool,
    pub assistant_context: Option<vel_api_types::AssistantContextData>,
    pub conversation: Option<crate::services::chat::mapping::ConversationServiceData>,
    pub proposal: Option<AssistantActionProposal>,
    pub planning_profile_proposal: Option<PlanningProfileEditProposal>,
    pub daily_loop_session: Option<vel_core::DailyLoopSession>,
    pub end_of_day: Option<crate::services::context_generation::EndOfDayContextData>,
}

#[derive(Debug, Clone)]
struct AssistantProposalStageResult {
    proposal: AssistantActionProposal,
    follow_through: serde_json::Value,
}

#[derive(Debug, Clone)]
pub(crate) struct VoiceEntryProvenance {
    pub surface: Option<String>,
    pub source_device: Option<String>,
    pub locale: Option<String>,
    pub transcript_origin: Option<String>,
    pub recorded_at: Option<time::OffsetDateTime>,
    pub offline_captured_at: Option<time::OffsetDateTime>,
    pub queued_at: Option<time::OffsetDateTime>,
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

fn assistant_error_retryable(message: &str) -> bool {
    let lowered = message.trim().to_ascii_lowercase();
    lowered.starts_with("provider error: transport:")
        || lowered.starts_with("provider error: protocol:")
        || lowered.starts_with("provider error: backend:")
        || lowered.starts_with("provider error: capability unsupported:")
        || lowered.starts_with("no provider registered for profile")
}

fn looks_like_thread_continuity_request(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.contains('?') {
        return true;
    }
    let lowered = trimmed.to_ascii_lowercase();
    const THREAD_PREFIXES: &[&str] = &[
        "what ",
        "what's ",
        "whats ",
        "why ",
        "how ",
        "when ",
        "where ",
        "who ",
        "help ",
        "show ",
        "find ",
        "search ",
        "summarize ",
        "explain ",
        "tell me ",
        "can you ",
        "could you ",
        "should i ",
        "do i ",
    ];
    THREAD_PREFIXES
        .iter()
        .any(|prefix| lowered.starts_with(prefix))
        || lowered.split_whitespace().count() >= 12
}

fn assistant_entry_route_target(
    text: &str,
    conversation_id: Option<&str>,
) -> AssistantEntryRouteTarget {
    if crate::services::daily_loop::assistant_requested_phase(text).is_some() {
        return AssistantEntryRouteTarget::Inline;
    }
    if crate::services::context_runs::assistant_requested_end_of_day(text) {
        return AssistantEntryRouteTarget::Inline;
    }
    if conversation_id.is_some() {
        return AssistantEntryRouteTarget::Threads;
    }
    if looks_like_thread_continuity_request(text) {
        AssistantEntryRouteTarget::Threads
    } else {
        AssistantEntryRouteTarget::Inbox
    }
}

fn assistant_entry_intent_with_override(
    text: &str,
    conversation_id: Option<&str>,
    override_intent: Option<&vel_api_types::NowDockedInputIntentData>,
) -> &'static str {
    if let Some(intent) = override_intent {
        return match intent {
            vel_api_types::NowDockedInputIntentData::Task => "task",
            vel_api_types::NowDockedInputIntentData::Url => "url",
            vel_api_types::NowDockedInputIntentData::Question => "question",
            vel_api_types::NowDockedInputIntentData::Note => "note",
            vel_api_types::NowDockedInputIntentData::Command => "command",
            vel_api_types::NowDockedInputIntentData::Continuation => "continuation",
            vel_api_types::NowDockedInputIntentData::Reflection => "reflection",
            vel_api_types::NowDockedInputIntentData::Scheduling => "scheduling",
        };
    }
    let lowered = text.trim().to_ascii_lowercase();
    if conversation_id.is_some() {
        return "continuation";
    }
    if looks_like_explicit_command(text) {
        return "command";
    }
    if contains_url_or_path_token(text) {
        return "url";
    }
    if lowered.contains('?') || lowered.starts_with("who ") || lowered.starts_with("what ") {
        return "question";
    }
    if lowered.starts_with("note ") || lowered.starts_with("remember ") {
        return "note";
    }
    if lowered.starts_with("reflect ") || lowered.starts_with("journal ") {
        return "reflection";
    }
    if lowered.contains("schedule") || lowered.contains("calendar") || lowered.contains("reflow") {
        return "scheduling";
    }
    if looks_like_mutation_request(text) {
        return "command";
    }
    "task"
}

fn looks_like_explicit_command(text: &str) -> bool {
    vel_command_lang::shell::explicit_command_name(text).is_some()
}

fn contains_url_or_path_token(text: &str) -> bool {
    text.split_whitespace()
        .map(trim_intent_token)
        .filter(|token| !token.is_empty())
        .any(|token| {
            token.starts_with("http://")
                || token.starts_with("https://")
                || token.starts_with("www.")
                || token.starts_with("file://")
                || looks_like_local_path_token(token)
        })
}

fn trim_intent_token(token: &str) -> &str {
    token.trim_matches(|c: char| {
        matches!(
            c,
            '(' | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '<'
                | '>'
                | '"'
                | '\''
                | '`'
                | ','
                | '.'
                | ';'
                | ':'
                | '!'
                | '?'
        )
    })
}

fn looks_like_local_path_token(token: &str) -> bool {
    if token.is_empty() || token.contains("://") {
        return false;
    }
    if token.starts_with('/')
        || token.starts_with("./")
        || token.starts_with("../")
        || token.starts_with("~/")
    {
        return true;
    }
    token.contains('/') && token.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn assistant_entry_conversation_title(text: &str) -> String {
    let trimmed = text.trim();
    let mut title = trimmed
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ");
    if title.is_empty() {
        title = "Assistant entry".to_string();
    }
    if trimmed.split_whitespace().count() > 8 {
        title.push('…');
    }
    title
}

fn looks_like_action_proposal_request(text: &str) -> bool {
    let lowered = text.trim().to_ascii_lowercase();
    [
        "what should i",
        "what do i",
        "what should we",
        "what should i focus on",
        "what should i do next",
        "what next",
        "focus on",
        "what needs review",
        "what needs attention",
        "what should i handle",
        "what should i review",
        "can you ",
        "please ",
        "send ",
        "reply ",
        "update ",
        "create ",
        "mark ",
        "complete ",
        "snooze ",
        "dismiss ",
        "apply ",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

fn looks_like_mutation_request(text: &str) -> bool {
    let lowered = text.trim().to_ascii_lowercase();
    [
        "send ",
        "reply ",
        "update ",
        "create ",
        "mark ",
        "complete ",
        "snooze ",
        "dismiss ",
        "apply ",
        "write ",
        "change ",
        "edit ",
        "reschedule ",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

fn looks_like_repo_write_request(text: &str) -> bool {
    let lowered = text.trim().to_ascii_lowercase();
    [
        "repo ",
        "code ",
        "file ",
        "commit ",
        "patch ",
        "branch ",
        "pull request",
        "fix this",
        "edit the code",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

fn proposal_from_action_item(item: vel_core::ActionItem) -> AssistantActionProposal {
    AssistantActionProposal {
        action_item_id: item.id,
        state: AssistantProposalState::Staged,
        kind: item.kind,
        permission_mode: item.permission_mode,
        scope_affinity: item.scope_affinity,
        title: item.title,
        summary: item.summary,
        project_id: item.project_id,
        project_label: item.project_label,
        project_family: item.project_family,
        thread_route: item.thread_route,
    }
}

fn proposal_with_gate(
    mut proposal: AssistantActionProposal,
    permission_mode: ActionPermissionMode,
    guidance: Option<String>,
) -> AssistantActionProposal {
    proposal.permission_mode = permission_mode;
    if let Some(guidance) = guidance {
        proposal.summary = format!("{} Gate: {}", proposal.summary, guidance);
    }
    proposal
}

fn proposal_with_state(
    mut proposal: AssistantActionProposal,
    state: AssistantProposalState,
) -> AssistantActionProposal {
    proposal.state = state;
    proposal
}

fn assistant_confirmation_follow_through(proposal: &AssistantActionProposal) -> serde_json::Value {
    serde_json::json!({
        "kind": "action_confirmation",
        "action_item_id": proposal.action_item_id,
        "permission_mode": proposal.permission_mode.to_string(),
        "scope_affinity": proposal.scope_affinity.to_string(),
    })
}

fn execution_handoff_follow_through(handoff_id: &str) -> serde_json::Value {
    serde_json::json!({
        "kind": "execution_handoff_review",
        "handoff_id": handoff_id,
        "review_state": "pending_review",
        "launch_preview_path": format!("/v1/execution/handoffs/{handoff_id}/launch-preview"),
        "approve_path": format!("/v1/execution/handoffs/{handoff_id}/approve"),
        "reject_path": format!("/v1/execution/handoffs/{handoff_id}/reject"),
    })
}

fn execution_handoff_ready_follow_through(handoff_id: &str) -> serde_json::Value {
    serde_json::json!({
        "kind": "execution_handoff_ready",
        "handoff_id": handoff_id,
        "review_state": "approved",
        "launch_preview_path": format!("/v1/execution/handoffs/{handoff_id}/launch-preview"),
    })
}

fn writeback_ready_follow_through(proposal: &AssistantActionProposal) -> serde_json::Value {
    serde_json::json!({
        "kind": "writeback_ready",
        "permission_mode": proposal.permission_mode.to_string(),
        "summary": proposal.summary,
    })
}

fn blocked_follow_through(proposal: &AssistantActionProposal) -> serde_json::Value {
    serde_json::json!({
        "kind": "gated",
        "permission_mode": proposal.permission_mode.to_string(),
        "summary": proposal.summary,
    })
}

pub(crate) fn assistant_proposal_thread_id(message_id: &str) -> String {
    format!("thr_assistant_proposal_{message_id}")
}

pub(crate) fn planning_profile_proposal_thread_id(message_id: &str) -> String {
    format!("thr_planning_profile_edit_{message_id}")
}

fn initial_reversal_metadata() -> serde_json::Value {
    serde_json::json!({
        "supported": false,
        "note": "Reversal is only available after a proposal is explicitly applied through an existing operator lane.",
    })
}

async fn load_conversation_with_continuation(
    state: &AppState,
    conversation_id: &str,
) -> Result<Option<crate::services::chat::mapping::ConversationServiceData>, AppError> {
    let Some(record) = state.storage.get_conversation(conversation_id).await? else {
        return Ok(None);
    };
    let mut conversation = crate::services::chat::mapping::conversation_record_to_data(record);
    conversation.continuation =
        crate::services::chat::thread_continuation::conversation_continuation_data(
            &state.storage,
            &conversation.id,
        )
        .await?;
    Ok(Some(conversation))
}

async fn ensure_assistant_proposal_thread(
    state: &AppState,
    user_message: &ChatMessage,
    proposal: &AssistantActionProposal,
    follow_through: &serde_json::Value,
) -> Result<vel_core::ActionThreadRoute, AppError> {
    let thread_id = assistant_proposal_thread_id(&user_message.id);
    if state.storage.get_thread_by_id(&thread_id).await?.is_none() {
        let metadata = serde_json::json!({
            "source": "assistant_proposal",
            "source_message_id": user_message.id,
            "conversation_id": user_message.conversation_id,
            "action_item_id": proposal.action_item_id,
            "proposal_state": proposal.state.to_string(),
            "proposal_kind": proposal.kind.to_string(),
            "permission_mode": proposal.permission_mode.to_string(),
            "scope_affinity": proposal.scope_affinity.to_string(),
            "title": proposal.title,
            "summary": proposal.summary,
            "project_id": proposal.project_id,
            "follow_through": follow_through,
            "lineage": {
                "source_message_id": user_message.id,
                "conversation_id": user_message.conversation_id,
                "action_item_id": proposal.action_item_id,
                "project_id": proposal.project_id,
                "initial_follow_through": follow_through,
                "input_mode": user_message
                    .content
                    .get("input_mode")
                    .and_then(serde_json::Value::as_str),
            },
            "reversal": initial_reversal_metadata(),
            "upstream_thread_route": proposal
                .thread_route
                .as_ref()
                .and_then(|route| serde_json::to_value(route).ok()),
        })
        .to_string();
        state
            .storage
            .insert_thread(
                &thread_id,
                "assistant_proposal",
                &proposal.title,
                "open",
                &metadata,
            )
            .await?;
        if let Some(project_id) = proposal.project_id.as_ref() {
            let _ = state
                .storage
                .insert_thread_link(&thread_id, "project", project_id.as_ref(), "about")
                .await?;
        }
        let _ = state
            .storage
            .insert_thread_link(
                &thread_id,
                "conversation",
                &user_message.conversation_id,
                "continues",
            )
            .await?;
        if let Some(handoff_id) = follow_through
            .get("handoff_id")
            .and_then(serde_json::Value::as_str)
        {
            let _ = state
                .storage
                .insert_thread_link(&thread_id, "execution_handoff", handoff_id, "approves")
                .await?;
        }
        if let Some(upstream_thread_id) = proposal
            .thread_route
            .as_ref()
            .and_then(|route| route.thread_id.as_deref())
        {
            let _ = state
                .storage
                .insert_thread_link(&thread_id, "thread", upstream_thread_id, "continues")
                .await?;
        }
    }

    Ok(vel_core::ActionThreadRoute {
        target: vel_core::ActionThreadRouteTarget::ExistingThread,
        label: format!("Continue in Threads: {}", proposal.title),
        thread_id: Some(thread_id),
        thread_type: Some("assistant_proposal".to_string()),
        project_id: proposal.project_id.clone(),
    })
}

pub(crate) async fn attach_planning_profile_proposal_thread(
    state: &AppState,
    user_message: &ChatMessage,
    proposal: &mut PlanningProfileEditProposal,
) -> Result<(), AppError> {
    let thread_id = planning_profile_proposal_thread_id(&user_message.id);
    if state.storage.get_thread_by_id(&thread_id).await?.is_none() {
        let metadata = serde_json::json!({
            "source": "planning_profile_proposal",
            "source_message_id": user_message.id,
            "conversation_id": user_message.conversation_id,
            "proposal_state": proposal.state.to_string(),
            "summary": proposal.summary,
            "requires_confirmation": proposal.requires_confirmation,
            "continuity": serde_json::to_value(proposal.continuity).unwrap_or_else(|_| serde_json::json!("thread")),
            "outcome_summary": proposal.outcome_summary,
            "mutation": serde_json::to_value(&proposal.mutation).unwrap_or_else(|_| serde_json::json!({})),
            "lineage": {
                "source_message_id": user_message.id,
                "conversation_id": user_message.conversation_id,
                "source_surface": serde_json::to_value(proposal.source_surface).unwrap_or_else(|_| serde_json::json!("assistant")),
                "input_mode": user_message
                    .content
                    .get("input_mode")
                    .and_then(serde_json::Value::as_str),
            },
        })
        .to_string();
        state
            .storage
            .insert_thread(
                &thread_id,
                "planning_profile_edit",
                &proposal.summary,
                "open",
                &metadata,
            )
            .await?;
        let _ = state
            .storage
            .insert_thread_link(
                &thread_id,
                "conversation",
                &user_message.conversation_id,
                "continues",
            )
            .await?;
    }

    proposal.thread_id = Some(thread_id);
    proposal.thread_type = Some("planning_profile_edit".to_string());
    Ok(())
}

fn proposal_priority(kind: vel_core::ActionKind) -> i32 {
    match kind {
        vel_core::ActionKind::Intervention => 0,
        vel_core::ActionKind::NextStep => 1,
        vel_core::ActionKind::Review => 2,
        vel_core::ActionKind::CheckIn => 3,
        vel_core::ActionKind::Conflict => 4,
        vel_core::ActionKind::Recovery => 5,
        vel_core::ActionKind::Blocked => 6,
        vel_core::ActionKind::Linking => 7,
        vel_core::ActionKind::Freshness => 8,
    }
}

async fn staged_action_proposal_for_entry(
    state: &AppState,
    text: &str,
    route_target: AssistantEntryRouteTarget,
) -> Result<Option<AssistantProposalStageResult>, AppError> {
    if route_target != AssistantEntryRouteTarget::Threads
        || !looks_like_action_proposal_request(text)
    {
        return Ok(None);
    }

    let snapshot =
        crate::services::operator_queue::build_action_items(&state.storage, &state.config).await?;
    let proposal = snapshot
        .action_items
        .into_iter()
        .filter(|item| item.state == ActionState::Active)
        .min_by(|left, right| {
            proposal_priority(left.kind)
                .cmp(&proposal_priority(right.kind))
                .then_with(|| right.rank.cmp(&left.rank))
        })
        .map(proposal_from_action_item);

    let Some(proposal) = proposal else {
        return Ok(None);
    };

    if looks_like_repo_write_request(text) {
        let pending_handoffs = crate::services::execution_routing::list_execution_handoffs(
            state,
            None,
            Some(HandoffReviewState::PendingReview),
        )
        .await?;
        let approved_handoffs = crate::services::execution_routing::list_execution_handoffs(
            state,
            None,
            Some(HandoffReviewState::Approved),
        )
        .await?;

        if let Some(handoff) = approved_handoffs
            .iter()
            .find(|handoff| !handoff.routing.write_scopes.is_empty())
        {
            let proposal = proposal_with_state(proposal, AssistantProposalState::Approved);
            return Ok(Some(AssistantProposalStageResult {
                follow_through: execution_handoff_ready_follow_through(&handoff.id),
                proposal,
            }));
        }
        if let Some(handoff) = pending_handoffs
            .iter()
            .find(|handoff| !handoff.routing.write_scopes.is_empty())
        {
            let proposal = proposal_with_gate(
                proposal,
                ActionPermissionMode::Unavailable,
                Some(
                    "A repo-local write grant exists but still needs operator review.".to_string(),
                ),
            );
            return Ok(Some(AssistantProposalStageResult {
                follow_through: execution_handoff_follow_through(&handoff.id),
                proposal,
            }));
        }
        let proposal = proposal_with_gate(
            proposal,
            ActionPermissionMode::Unavailable,
            Some(
                "No approved repo-local handoff currently grants write scope for mutation work."
                    .to_string(),
            ),
        );
        return Ok(Some(AssistantProposalStageResult {
            follow_through: blocked_follow_through(&proposal),
            proposal,
        }));
    }

    if looks_like_mutation_request(text)
        && !crate::services::operator_settings::runtime_writeback_enabled(
            &state.storage,
            &state.config,
        )
        .await?
    {
        let proposal = proposal_with_gate(
            proposal,
            ActionPermissionMode::Blocked,
            Some(crate::services::writeback::SAFE_MODE_MESSAGE.to_string()),
        );
        return Ok(Some(AssistantProposalStageResult {
            follow_through: blocked_follow_through(&proposal),
            proposal,
        }));
    }

    if looks_like_mutation_request(text) {
        let proposal = proposal_with_state(proposal, AssistantProposalState::Approved);
        return Ok(Some(AssistantProposalStageResult {
            follow_through: writeback_ready_follow_through(&proposal),
            proposal,
        }));
    }

    Ok(Some(AssistantProposalStageResult {
        follow_through: assistant_confirmation_follow_through(&proposal),
        proposal,
    }))
}

fn proposal_intervention_content(
    text: &str,
    route_target: AssistantEntryRouteTarget,
    voice: Option<&VoiceEntryProvenance>,
    proposal: &AssistantActionProposal,
) -> serde_json::Value {
    let mut content = assistant_entry_content(text, route_target, None, &[], voice);
    if let Some(object) = content.as_object_mut() {
        object.insert(
            "title".to_string(),
            serde_json::Value::String(proposal.title.clone()),
        );
        object.insert(
            "summary".to_string(),
            serde_json::Value::String(proposal.summary.clone()),
        );
        object.insert(
            "reason".to_string(),
            serde_json::Value::String(proposal.summary.clone()),
        );
        object.insert(
            "proposal".to_string(),
            serde_json::to_value(proposal).unwrap_or_else(|_| serde_json::json!({})),
        );
    }
    content
}

fn planning_profile_surface_for_entry(
    voice: Option<&VoiceEntryProvenance>,
) -> PlanningProfileSurface {
    if voice.is_some() {
        PlanningProfileSurface::Voice
    } else {
        PlanningProfileSurface::Assistant
    }
}

pub(crate) fn voice_entry_provenance_json(provenance: &VoiceEntryProvenance) -> serde_json::Value {
    serde_json::json!({
        "surface": provenance.surface,
        "source_device": provenance.source_device,
        "locale": provenance.locale,
        "transcript_origin": provenance.transcript_origin,
        "recorded_at": provenance.recorded_at.map(|value| value.unix_timestamp()),
        "offline_captured_at": provenance
            .offline_captured_at
            .map(|value| value.unix_timestamp()),
        "queued_at": provenance.queued_at.map(|value| value.unix_timestamp()),
    })
}

fn assistant_entry_content(
    text: &str,
    route_target: AssistantEntryRouteTarget,
    intent: Option<&vel_api_types::NowDockedInputIntentData>,
    attachments: &[vel_api_types::AssistantEntryAttachmentData],
    voice: Option<&VoiceEntryProvenance>,
) -> serde_json::Value {
    let mut content = serde_json::json!({
        "text": text,
        "entry_route": match route_target {
            AssistantEntryRouteTarget::Inbox => "inbox",
            AssistantEntryRouteTarget::Threads => "threads",
            AssistantEntryRouteTarget::Inline => "inline",
        },
        "input_mode": if voice.is_some() { "voice" } else { "text" },
    });

    if let Some(intent) = intent {
        if let Some(object) = content.as_object_mut() {
            object.insert(
                "entry_intent".to_string(),
                serde_json::to_value(intent).unwrap_or_else(|_| serde_json::json!("task")),
            );
        }
    }

    if !attachments.is_empty() {
        if let Some(object) = content.as_object_mut() {
            object.insert(
                "attachments".to_string(),
                serde_json::to_value(attachments).unwrap_or_else(|_| serde_json::json!([])),
            );
        }
    }

    if let Some(voice) = voice {
        if let Some(object) = content.as_object_mut() {
            object.insert(
                "voice_provenance".to_string(),
                voice_entry_provenance_json(voice),
            );
        }
    }

    content
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

    let (assistant_message, assistant_error, assistant_error_retryable) =
        if let (Some(router), Some(profile_id)) =
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
                    (Some(assistant_message), None, false)
                }
                Ok(None) => (None, None, false),
                Err(error) => {
                    tracing::error!(error = %error, "assistant reply failed");
                    (
                        None,
                        Some(error.to_string()),
                        assistant_error_retryable(&error.to_string()),
                    )
                }
            }
        } else {
            (None, Some(chat_model_not_configured_error()), false)
        };

    let assistant_context = if assistant_message.is_some() {
        Some(
            crate::services::chat::tools::build_assistant_context(
                state,
                &extract_text_query(&payload.content),
                5,
            )
            .await?,
        )
    } else {
        None
    };

    Ok(ChatMessageCreateResult {
        user_message,
        assistant_message,
        assistant_error,
        assistant_error_retryable,
        assistant_context,
    })
}

async fn create_assistant_message(
    state: &AppState,
    conversation_id: &str,
    text: &str,
) -> Result<ChatMessage, AppError> {
    let id = format!("msg_{}", Uuid::new_v4().simple());
    let content_json = serde_json::to_string(&serde_json::json!({ "text": text }))
        .map_err(|e| AppError::bad_request(e.to_string()))?;
    state
        .storage
        .create_message(MessageInsert {
            id: id.clone(),
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
        &id,
        serde_json::json!({
            "id": id,
            "conversation_id": conversation_id,
            "kind": "text",
        }),
    )
    .await;
    let msg = state
        .storage
        .get_message(&id)
        .await?
        .ok_or_else(|| AppError::internal("assistant message not found after create"))?;
    let created_message = ChatMessage::from(message_record_to_data(msg)?);
    let ws_payload =
        serde_json::to_value(&created_message).unwrap_or_else(|_| serde_json::json!({ "id": id }));
    broadcast_chat_ws_event(state, WS_EVENT_MESSAGES_NEW, ws_payload);
    Ok(created_message)
}

pub(crate) async fn create_assistant_entry_response(
    state: &AppState,
    payload: &AssistantEntryCreateInput,
) -> Result<AssistantEntryCreateResult, AppError> {
    let text = payload.text.trim();
    if text.is_empty() {
        return Err(AppError::bad_request("text must not be empty"));
    }

    let planning_profile_proposal_candidate =
        crate::services::planning_profile::staged_edit_proposal_from_text(
            text,
            planning_profile_surface_for_entry(payload.voice.as_ref()),
        );
    let entry_intent = assistant_entry_intent_with_override(
        text,
        payload.conversation_id.as_deref(),
        payload.intent.as_ref(),
    )
    .to_string();
    let route_target = if planning_profile_proposal_candidate.is_some() {
        AssistantEntryRouteTarget::Threads
    } else {
        assistant_entry_route_target(text, payload.conversation_id.as_deref())
    };
    let conversation_id = if let Some(conversation_id) = payload
        .conversation_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let _ = state
            .storage
            .get_conversation(conversation_id)
            .await?
            .ok_or_else(|| AppError::not_found("conversation not found"))?;
        conversation_id.to_string()
    } else {
        let conversation = create_conversation(
            state,
            ConversationCreateInput {
                title: Some(assistant_entry_conversation_title(text)),
                kind: "general".to_string(),
            },
        )
        .await?;
        if route_target == AssistantEntryRouteTarget::Inbox {
            state
                .storage
                .archive_conversation(conversation.id.as_ref(), true)
                .await?;
        }
        conversation.id.as_ref().to_string()
    };

    let user_message = create_user_message(
        state,
        &conversation_id,
        &ChatMessageCreateInput {
            role: "user".to_string(),
            kind: "text".to_string(),
            content: assistant_entry_content(
                text,
                route_target,
                payload.intent.as_ref(),
                &payload.attachments,
                payload.voice.as_ref(),
            ),
        },
    )
    .await?;

    if let Some(session) = crate::services::daily_loop::start_or_resume_assistant_session(
        &state.storage,
        &state.config,
        text,
        DailyLoopSurface::Web,
    )
    .await?
    {
        let assistant_message = Some(
            create_assistant_message(
                state,
                &conversation_id,
                &crate::services::daily_loop::assistant_entry_summary(&session),
            )
            .await?,
        );
        let conversation = load_conversation_with_continuation(state, &conversation_id).await?;

        return Ok(AssistantEntryCreateResult {
            route_target: AssistantEntryRouteTarget::Inline,
            entry_intent: Some(entry_intent.clone()),
            continuation_category: Some("needs_input".to_string()),
            follow_up: None,
            user_message,
            assistant_message,
            assistant_error: None,
            assistant_error_retryable: false,
            assistant_context: None,
            conversation,
            proposal: None,
            planning_profile_proposal: None,
            daily_loop_session: Some(session),
            end_of_day: None,
        });
    }

    if crate::services::context_runs::assistant_requested_end_of_day(text) {
        let output = crate::services::context_runs::generate_end_of_day(state).await?;
        let assistant_message = Some(
            create_assistant_message(
                state,
                &conversation_id,
                &crate::services::context_runs::assistant_end_of_day_summary(&output.data),
            )
            .await?,
        );
        let conversation = load_conversation_with_continuation(state, &conversation_id).await?;

        return Ok(AssistantEntryCreateResult {
            route_target: AssistantEntryRouteTarget::Inline,
            entry_intent: Some(entry_intent.clone()),
            continuation_category: Some("follow_up".to_string()),
            follow_up: None,
            user_message,
            assistant_message,
            assistant_error: None,
            assistant_error_retryable: false,
            assistant_context: None,
            conversation,
            proposal: None,
            planning_profile_proposal: None,
            daily_loop_session: None,
            end_of_day: Some(output.data),
        });
    }

    if let Some(mut planning_profile_proposal) = planning_profile_proposal_candidate {
        attach_planning_profile_proposal_thread(
            state,
            &user_message,
            &mut planning_profile_proposal,
        )
        .await?;
        emit_chat_event(
            state,
            "planning_profile.proposal.staged",
            "message",
            &user_message.id,
            serde_json::json!({
                "message_id": user_message.id,
                "conversation_id": conversation_id,
                "proposal": planning_profile_proposal,
            }),
        )
        .await;
        let conversation = load_conversation_with_continuation(state, &conversation_id).await?;

        return Ok(AssistantEntryCreateResult {
            route_target: AssistantEntryRouteTarget::Threads,
            entry_intent: Some(entry_intent.clone()),
            continuation_category: Some("review_apply".to_string()),
            follow_up: None,
            user_message,
            assistant_message: None,
            assistant_error: None,
            assistant_error_retryable: false,
            assistant_context: None,
            conversation,
            proposal: None,
            planning_profile_proposal: Some(planning_profile_proposal),
            daily_loop_session: None,
            end_of_day: None,
        });
    }

    let mut proposal = None;
    let mut follow_up = None;
    if let Some(staged) = staged_action_proposal_for_entry(state, text, route_target).await? {
        let mut staged_proposal = staged.proposal;
        staged_proposal.thread_route = Some(
            ensure_assistant_proposal_thread(
                state,
                &user_message,
                &staged_proposal,
                &staged.follow_through,
            )
            .await?,
        );
        emit_chat_event(
            state,
            "assistant.proposal.staged",
            "message",
            &user_message.id,
            serde_json::json!({
                "message_id": user_message.id,
                "conversation_id": conversation_id,
                "proposal": staged_proposal,
            }),
        )
        .await;
        let intervention = create_intervention_for_message_if_needed(
            state,
            &InterventionMessageInput {
                id: user_message.id.clone(),
                conversation_id: user_message.conversation_id.clone(),
                role: user_message.role.clone(),
                kind: user_message.kind.clone(),
                content: proposal_intervention_content(
                    text,
                    route_target,
                    payload.voice.as_ref(),
                    &staged_proposal,
                ),
            },
        )
        .await?;
        proposal = Some(staged_proposal);
        follow_up = intervention.map(|item| vel_api_types::AssistantEntryFollowUpData {
            intervention_id: item.id,
            message_id: item.message_id,
            conversation_id: user_message.conversation_id.clone(),
            kind: item.kind,
            state: item.state,
            surfaced_at: item.surfaced_at,
            snoozed_until: item.snoozed_until,
            confidence: item.confidence,
        });
    }

    let (assistant_message, assistant_error, assistant_error_retryable) =
        if route_target == AssistantEntryRouteTarget::Threads {
            if let (Some(router), Some(profile_id)) =
                (state.llm_router.as_ref(), state.chat_profile_id.as_ref())
            {
                match generate_assistant_reply(
                    state,
                    &conversation_id,
                    profile_id,
                    state.chat_fallback_profile_id.as_deref(),
                    router,
                )
                .await
                {
                    Ok(Some(assistant_message)) => (Some(assistant_message), None, false),
                    Ok(None) => (None, None, false),
                    Err(error) => (
                        None,
                        Some(error.to_string()),
                        assistant_error_retryable(&error.to_string()),
                    ),
                }
            } else {
                (None, Some(chat_model_not_configured_error()), false)
            }
        } else {
            (None, None, false)
        };

    let assistant_context = if assistant_message.is_some() {
        Some(crate::services::chat::tools::build_assistant_context(state, text, 5).await?)
    } else {
        None
    };

    let conversation = load_conversation_with_continuation(state, &conversation_id).await?;

    Ok(AssistantEntryCreateResult {
        route_target,
        entry_intent: Some(entry_intent),
        continuation_category: if proposal.is_some() {
            Some("review_apply".to_string())
        } else {
            match route_target {
                AssistantEntryRouteTarget::Inline => None,
                AssistantEntryRouteTarget::Inbox | AssistantEntryRouteTarget::Threads => {
                    Some("follow_up".to_string())
                }
            }
        },
        user_message,
        assistant_message,
        assistant_error,
        assistant_error_retryable,
        assistant_context,
        conversation,
        proposal,
        follow_up,
        planning_profile_proposal: None,
        daily_loop_session: None,
        end_of_day: None,
    })
}

fn extract_text_query(content: &serde_json::Value) -> String {
    content
        .get("text")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{assistant_entry_intent_with_override, assistant_error_retryable};

    #[test]
    fn assistant_error_retryable_flags_retryable_provider_failures() {
        assert!(assistant_error_retryable(
            "provider error: transport: connection reset"
        ));
        assert!(assistant_error_retryable(
            "provider error: backend: upstream unavailable"
        ));
        assert!(assistant_error_retryable(
            "no provider registered for profile 'primary'"
        ));
    }

    #[test]
    fn assistant_error_retryable_rejects_configuration_and_auth_failures() {
        assert!(!assistant_error_retryable(
            "Chat model not configured. Set VEL_LLM_MODEL and run llama-server."
        ));
        assert!(!assistant_error_retryable(
            "provider error: auth: invalid token"
        ));
        assert!(!assistant_error_retryable(
            "provider error: rate limit: maxed"
        ));
    }

    #[test]
    fn assistant_entry_intent_detects_urls_and_paths() {
        assert_eq!(
            assistant_entry_intent_with_override("https://example.com/spec", None, None),
            "url"
        );
        assert_eq!(
            assistant_entry_intent_with_override("/home/jove/code/vel/README.md", None, None),
            "url"
        );
        assert_eq!(
            assistant_entry_intent_with_override("clients/web/src/App.tsx", None, None),
            "url"
        );
    }

    #[test]
    fn assistant_entry_intent_detects_slash_commands_without_confusing_paths() {
        assert_eq!(
            assistant_entry_intent_with_override("/morning", None, None),
            "command"
        );
        assert_eq!(
            assistant_entry_intent_with_override("vel morning", None, None),
            "command"
        );
        assert_eq!(
            assistant_entry_intent_with_override("/run status run_123 blocked", None, None),
            "command"
        );
        assert_eq!(
            assistant_entry_intent_with_override("/home/jove/code/vel/README.md", None, None),
            "url"
        );
    }
}
