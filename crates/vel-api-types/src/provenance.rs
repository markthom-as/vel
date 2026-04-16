use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::UnixSeconds;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceData {
    pub message_id: String,
    pub events: Vec<ProvenanceEvent>,
    pub signals: Vec<JsonValue>,
    pub policy_decisions: Vec<JsonValue>,
    pub linked_objects: Vec<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEvent {
    pub id: String,
    pub event_name: String,
    pub created_at: UnixSeconds,
    pub payload: JsonValue,
}
