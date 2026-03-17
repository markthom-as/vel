use serde_json::Value;
use vel_storage::EventLogInsert;

use crate::state::AppState;

pub(crate) async fn emit_chat_event(
    state: &AppState,
    event_name: &str,
    aggregate_type: &str,
    aggregate_id: &str,
    payload: Value,
) {
    let _ = state
        .storage
        .append_event(EventLogInsert {
            id: None,
            event_name: event_name.to_string(),
            aggregate_type: Some(aggregate_type.to_string()),
            aggregate_id: Some(aggregate_id.to_string()),
            payload_json: payload.to_string(),
        })
        .await;
}
