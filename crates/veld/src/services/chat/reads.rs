use crate::{
    errors::AppError,
    services::chat::{
        mapping::{intervention_record_to_inbox_item, message_record_to_data},
        provenance::{build_linked_objects, build_policy_decisions, build_provenance_signals},
    },
    state::AppState,
};

#[derive(Debug, Clone)]
pub(crate) struct InboxItem {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
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

fn intervention_record_to_inbox_output(record: vel_storage::InterventionRecord) -> InboxItem {
    let item = intervention_record_to_inbox_item(record);
    InboxItem {
        id: item.id,
        message_id: item.message_id,
        kind: item.kind,
        state: item.state,
        surfaced_at: item.surfaced_at,
        snoozed_until: item.snoozed_until,
        confidence: item.confidence,
    }
}

pub(crate) async fn list_inbox_items(
    state: &AppState,
    limit: u32,
) -> Result<Vec<InboxItem>, AppError> {
    let list = state.storage.list_interventions_active(limit).await?;
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_output)
        .collect())
}

pub(crate) async fn list_conversation_intervention_items(
    state: &AppState,
    conversation_id: &str,
) -> Result<Vec<InboxItem>, AppError> {
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
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_output)
        .collect())
}

pub(crate) async fn list_message_intervention_items(
    state: &AppState,
    message_id: &str,
) -> Result<Vec<InboxItem>, AppError> {
    let list = state
        .storage
        .get_interventions_by_message(message_id.trim())
        .await?;
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_output)
        .collect())
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
    let message = message_record_to_data(message)?;
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
