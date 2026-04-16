use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use vel_core::{IntegrationConnectionId, IntegrationFamily};

use crate::UnixSeconds;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationFamilyData {
    Calendar,
    Tasks,
    Activity,
    Git,
    Messaging,
    Notes,
    Transcripts,
    Documents,
    Health,
    Gaming,
}

impl From<IntegrationFamily> for IntegrationFamilyData {
    fn from(value: IntegrationFamily) -> Self {
        match value {
            IntegrationFamily::Calendar => Self::Calendar,
            IntegrationFamily::Tasks => Self::Tasks,
            IntegrationFamily::Activity => Self::Activity,
            IntegrationFamily::Git => Self::Git,
            IntegrationFamily::Messaging => Self::Messaging,
            IntegrationFamily::Notes => Self::Notes,
            IntegrationFamily::Transcripts => Self::Transcripts,
            IntegrationFamily::Documents => Self::Documents,
            IntegrationFamily::Health => Self::Health,
            IntegrationFamily::Gaming => Self::Gaming,
        }
    }
}

impl From<IntegrationFamilyData> for IntegrationFamily {
    fn from(value: IntegrationFamilyData) -> Self {
        match value {
            IntegrationFamilyData::Calendar => Self::Calendar,
            IntegrationFamilyData::Tasks => Self::Tasks,
            IntegrationFamilyData::Activity => Self::Activity,
            IntegrationFamilyData::Git => Self::Git,
            IntegrationFamilyData::Messaging => Self::Messaging,
            IntegrationFamilyData::Notes => Self::Notes,
            IntegrationFamilyData::Transcripts => Self::Transcripts,
            IntegrationFamilyData::Documents => Self::Documents,
            IntegrationFamilyData::Health => Self::Health,
            IntegrationFamilyData::Gaming => Self::Gaming,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSourceRefData {
    pub family: IntegrationFamilyData,
    pub provider_key: String,
    pub connection_id: IntegrationConnectionId,
    pub external_id: String,
}

impl From<vel_core::IntegrationSourceRef> for IntegrationSourceRefData {
    fn from(value: vel_core::IntegrationSourceRef) -> Self {
        Self {
            family: value.family.into(),
            provider_key: value.provider_key,
            connection_id: value.connection_id,
            external_id: value.external_id,
        }
    }
}

impl From<IntegrationSourceRefData> for vel_core::IntegrationSourceRef {
    fn from(value: IntegrationSourceRefData) -> Self {
        Self {
            family: value.family.into(),
            provider_key: value.provider_key,
            connection_id: value.connection_id,
            external_id: value.external_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnectionSettingRefData {
    pub setting_key: String,
    pub setting_value: String,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnectionData {
    pub id: String,
    pub family: String,
    pub provider_key: String,
    pub status: String,
    pub display_name: String,
    pub account_ref: Option<String>,
    pub metadata: JsonValue,
    pub created_at: UnixSeconds,
    pub updated_at: UnixSeconds,
    #[serde(default)]
    pub setting_refs: Vec<IntegrationConnectionSettingRefData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConnectionEventData {
    pub id: String,
    pub connection_id: String,
    pub event_type: String,
    pub payload: JsonValue,
    pub timestamp: UnixSeconds,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCalendarData {
    pub id: String,
    pub summary: String,
    pub primary: bool,
    pub sync_enabled: bool,
    pub display_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationGuidanceData {
    pub title: String,
    pub detail: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarIntegrationData {
    pub configured: bool,
    pub connected: bool,
    pub has_client_id: bool,
    pub has_client_secret: bool,
    pub calendars: Vec<IntegrationCalendarData>,
    pub all_calendars_selected: bool,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoistWriteCapabilitiesData {
    pub completion_status: bool,
    pub due_date: bool,
    pub tags: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoistIntegrationData {
    pub configured: bool,
    pub connected: bool,
    pub has_api_token: bool,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
    pub write_capabilities: TodoistWriteCapabilitiesData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalIntegrationData {
    pub configured: bool,
    pub source_path: Option<String>,
    #[serde(default)]
    pub selected_paths: Vec<String>,
    #[serde(default)]
    pub available_paths: Vec<String>,
    #[serde(default)]
    pub internal_paths: Vec<String>,
    #[serde(default)]
    pub suggested_paths: Vec<String>,
    pub source_kind: String,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalIntegrationPathSelectionData {
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsData {
    pub google_calendar: GoogleCalendarIntegrationData,
    pub todoist: TodoistIntegrationData,
    pub activity: LocalIntegrationData,
    pub health: LocalIntegrationData,
    pub git: LocalIntegrationData,
    pub messaging: LocalIntegrationData,
    pub reminders: LocalIntegrationData,
    pub notes: LocalIntegrationData,
    pub transcripts: LocalIntegrationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarAuthStartData {
    pub auth_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalTodoistWriteIntentRequestData {
    pub object_id: String,
    pub revision: i64,
    pub object_status: String,
    pub integration_account_id: String,
    pub requested_change: JsonValue,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub write_enabled: bool,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub approved: bool,
    #[serde(default)]
    pub pending_reconciliation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalGoogleCalendarWriteIntentRequestData {
    pub object_id: String,
    pub expected_revision: i64,
    pub actual_revision: i64,
    pub object_status: String,
    pub integration_account_id: String,
    pub requested_change: JsonValue,
    pub recurrence_scope: Option<String>,
    #[serde(default)]
    pub source_owned_fields: Vec<String>,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub write_enabled: bool,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub approved: bool,
    #[serde(default)]
    pub pending_reconciliation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyExplainData {
    pub action_name: String,
    pub decision: vel_core::PolicyDecisionKind,
    pub confirmation: vel_core::ConfirmationMode,
    pub read_only: bool,
    #[serde(default)]
    pub reasons: Vec<String>,
}

impl From<vel_core::PolicyExplain> for PolicyExplainData {
    fn from(value: vel_core::PolicyExplain) -> Self {
        Self {
            action_name: value.action_name,
            decision: value.decision,
            confirmation: value.confirmation,
            read_only: value.read_only,
            reasons: value.reasons,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectExplainData {
    pub object_ref: String,
    pub status: String,
    pub revision: i64,
    pub source_summary: Option<JsonValue>,
    pub linked_provider_count: usize,
    pub basis: vel_core::ExplainBasis,
}

impl From<vel_core::ObjectExplain> for ObjectExplainData {
    fn from(value: vel_core::ObjectExplain) -> Self {
        Self {
            object_ref: value.object_ref,
            status: value.status,
            revision: value.revision,
            source_summary: value.source_summary,
            linked_provider_count: value.linked_provider_count,
            basis: value.basis,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipExplainData {
    pub field: String,
    pub owner: vel_core::OwnershipClass,
    pub overlay_applied: bool,
    pub source_favored: bool,
    pub pending_write_intent: bool,
    pub confirmation_required: bool,
    pub reason: String,
}

impl From<vel_core::OwnershipExplain> for OwnershipExplainData {
    fn from(value: vel_core::OwnershipExplain) -> Self {
        Self {
            field: value.field,
            owner: value.owner,
            overlay_applied: value.overlay_applied,
            source_favored: value.source_favored,
            pending_write_intent: value.pending_write_intent,
            confirmation_required: value.confirmation_required,
            reason: value.reason,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionExplainData {
    pub action_name: String,
    pub capability: String,
    pub allows_external_write: bool,
    pub dry_run: bool,
    pub policy_explain: PolicyExplainData,
    pub object_explain: Option<ObjectExplainData>,
    #[serde(default)]
    pub ownership_explain: Vec<OwnershipExplainData>,
}

impl From<vel_core::ActionExplain> for ActionExplainData {
    fn from(value: vel_core::ActionExplain) -> Self {
        Self {
            action_name: value.action_name,
            capability: value.capability,
            allows_external_write: value.allows_external_write,
            dry_run: value.dry_run,
            policy_explain: value.policy_explain.into(),
            object_explain: value.object_explain.map(Into::into),
            ownership_explain: value
                .ownership_explain
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalExecutionDispatchData {
    pub write_intent_id: String,
    pub approved_record_id: String,
    pub executing_record_id: String,
    pub terminal_record_id: String,
    pub downstream_operation_ref: String,
    pub downstream_status: String,
    pub downstream_result: Option<JsonValue>,
    pub downstream_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFieldChangeData {
    pub field_name: String,
    pub old_value: Option<JsonValue>,
    pub new_value: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEventData {
    pub id: String,
    pub task_ref: String,
    pub event_type: String,
    pub provenance: String,
    #[serde(default)]
    pub field_changes: Vec<TaskFieldChangeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalWriteIntentResponseData {
    pub write_intent_id: String,
    pub explain: ActionExplainData,
    pub dispatch: Option<CanonicalExecutionDispatchData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalTodoistWriteIntentResponseData {
    pub write_intent_id: String,
    pub explain: ActionExplainData,
    pub dispatch: Option<CanonicalExecutionDispatchData>,
    #[serde(default)]
    pub task_events: Vec<TaskEventData>,
}
