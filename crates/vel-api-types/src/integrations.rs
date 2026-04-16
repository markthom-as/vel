use serde::{Deserialize, Serialize};
use vel_core::{IntegrationConnectionId, IntegrationFamily};

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
