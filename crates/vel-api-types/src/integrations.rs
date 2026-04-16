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
