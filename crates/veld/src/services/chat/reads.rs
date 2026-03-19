use crate::{
    errors::AppError,
    services::chat::{
        mapping::{
            intervention_record_to_inbox_item, message_project_details, message_record_to_data,
            message_title_summary,
        },
        messages::ChatMessage,
        provenance::{build_linked_objects, build_policy_decisions, build_provenance_signals},
    },
    state::AppState,
};
use vel_core::{ActionEvidenceRef, ActionItem, ProjectId};

#[derive(Debug, Clone)]
pub(crate) struct InboxItem {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
    pub rank: i64,
    pub conversation_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub project_label: Option<String>,
    pub available_actions: Vec<vel_api_types::AvailableActionData>,
    pub evidence: Vec<ActionEvidenceRef>,
}

#[derive(Debug, Clone)]
pub(crate) struct MessageProvenance {
    pub message_id: String,
    pub events: Vec<ProvenanceMessageEvent>,
    pub signals: Vec<serde_json::Value>,
    pub policy_decisions: Vec<serde_json::Value>,
    pub linked_objects: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub(crate) struct ProvenanceMessageEvent {
    pub id: String,
    pub event_name: String,
    pub created_at: i64,
    pub payload: serde_json::Value,
}

async fn intervention_record_to_inbox_output(
    state: &AppState,
    record: vel_storage::InterventionRecord,
    action_item: Option<&ActionItem>,
) -> Result<InboxItem, AppError> {
    let message = state
        .storage
        .get_message(record.message_id.as_ref())
        .await?;
    let message = message.map(message_record_to_data).transpose()?;
    let conversation_id = message
        .as_ref()
        .map(|message| message.conversation_id.clone());

    let (fallback_title, fallback_summary) = message
        .as_ref()
        .map(message_title_summary)
        .unwrap_or_else(|| {
            (
                "Inbox intervention".to_string(),
                "Needs operator review from the current inbox/intervention queue.".to_string(),
            )
        });
    let (message_project_id, message_project_label) = message
        .as_ref()
        .map(message_project_details)
        .unwrap_or((None, None));

    let title = action_item
        .map(|item| item.title.clone())
        .unwrap_or(fallback_title);
    let summary = action_item
        .map(|item| item.summary.clone())
        .unwrap_or(fallback_summary);
    let project_id = action_item
        .and_then(|item| item.project_id.clone())
        .or(message_project_id);
    let project_label = message_project_label;
    let evidence = action_item
        .map(|item| item.evidence.clone())
        .unwrap_or_else(|| {
            vec![ActionEvidenceRef {
                source_kind: "intervention".to_string(),
                source_id: record.id.as_ref().to_string(),
                label: record.kind.clone(),
                detail: Some(format!("message_id={}", record.message_id)),
            }]
        });
    let item = intervention_record_to_inbox_item(
        record,
        action_item.map(|item| item.rank).unwrap_or_default(),
        conversation_id,
        title,
        summary,
        project_id,
        project_label,
        evidence,
    );

    Ok(InboxItem {
        id: item.id,
        message_id: item.message_id,
        kind: item.kind,
        state: item.state,
        surfaced_at: item.surfaced_at,
        snoozed_until: item.snoozed_until,
        confidence: item.confidence,
        rank: item.rank,
        conversation_id: item.conversation_id,
        title: item.title,
        summary: item.summary,
        project_id: item.project_id,
        project_label: item.project_label,
        available_actions: item.available_actions,
        evidence: item.evidence,
    })
}

async fn intervention_action_index(
    state: &AppState,
) -> Result<std::collections::HashMap<String, ActionItem>, AppError> {
    let action_items =
        crate::services::operator_queue::build_action_items(&state.storage, &state.config)
            .await?
            .action_items;

    Ok(action_items
        .into_iter()
        .filter_map(|item| {
            let intervention_id = item
                .evidence
                .iter()
                .find(|evidence| evidence.source_kind == "intervention")
                .map(|evidence| evidence.source_id.clone())?;
            Some((intervention_id, item))
        })
        .collect())
}

pub(crate) async fn list_inbox_items(
    state: &AppState,
    limit: u32,
) -> Result<Vec<InboxItem>, AppError> {
    let action_index = intervention_action_index(state).await?;
    let list = state.storage.list_interventions_active(limit).await?;
    let mut items = Vec::with_capacity(list.len());
    for record in list {
        items.push(
            intervention_record_to_inbox_output(
                state,
                record.clone(),
                action_index.get(record.id.as_ref()),
            )
            .await?,
        );
    }
    items.sort_by(|left, right| {
        right
            .rank
            .cmp(&left.rank)
            .then_with(|| right.surfaced_at.cmp(&left.surfaced_at))
    });
    items.truncate(limit as usize);
    Ok(items)
}

pub(crate) async fn list_conversation_intervention_items(
    state: &AppState,
    conversation_id: &str,
) -> Result<Vec<InboxItem>, AppError> {
    let action_index = intervention_action_index(state).await?;
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
    let mut items = Vec::with_capacity(list.len());
    for record in list {
        items.push(
            intervention_record_to_inbox_output(
                state,
                record.clone(),
                action_index.get(record.id.as_ref()),
            )
            .await?,
        );
    }
    items.sort_by(|left, right| {
        right
            .rank
            .cmp(&left.rank)
            .then_with(|| right.surfaced_at.cmp(&left.surfaced_at))
    });
    Ok(items)
}

pub(crate) async fn list_message_intervention_items(
    state: &AppState,
    message_id: &str,
) -> Result<Vec<InboxItem>, AppError> {
    let action_index = intervention_action_index(state).await?;
    let list = state
        .storage
        .get_interventions_by_message(message_id.trim())
        .await?;
    let mut items = Vec::with_capacity(list.len());
    for record in list {
        items.push(
            intervention_record_to_inbox_output(
                state,
                record.clone(),
                action_index.get(record.id.as_ref()),
            )
            .await?,
        );
    }
    items.sort_by(|left, right| {
        right
            .rank
            .cmp(&left.rank)
            .then_with(|| right.surfaced_at.cmp(&left.surfaced_at))
    });
    Ok(items)
}

pub(crate) async fn build_message_provenance_data(
    state: &AppState,
    message_id: &str,
) -> Result<MessageProvenance, AppError> {
    let message_id = message_id.trim();
    let message = state
        .storage
        .get_message(message_id)
        .await?
        .ok_or_else(|| AppError::not_found("message not found"))?;
    let message = ChatMessage::from(message_record_to_data(message)?);
    let interventions = state
        .storage
        .get_interventions_by_message(message_id)
        .await?;
    let events = state
        .storage
        .list_events_by_aggregate("message", message_id, 50)
        .await?;
    let events_data: Vec<ProvenanceMessageEvent> = events
        .into_iter()
        .map(|record| {
            let payload: serde_json::Value = serde_json::from_str(&record.payload_json)
                .unwrap_or_else(|_| serde_json::json!({}));
            ProvenanceMessageEvent {
                id: record.id.as_ref().to_string(),
                event_name: record.event_name,
                created_at: record.created_at,
                payload,
            }
        })
        .collect();
    let linked_objects = build_linked_objects(&message, &interventions);
    let signals = build_provenance_signals(&message, &interventions);
    let policy_decisions = build_policy_decisions(&message, &interventions);

    Ok(MessageProvenance {
        message_id: message_id.to_string(),
        events: events_data,
        signals,
        policy_decisions,
        linked_objects,
    })
}
