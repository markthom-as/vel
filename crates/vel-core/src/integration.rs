use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;

use crate::{types::IntegrationConnectionId, VelCoreError};

/// Canonical integration family. This is the stable product-facing category, not a provider name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationFamily {
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

impl Display for IntegrationFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Calendar => "calendar",
            Self::Tasks => "tasks",
            Self::Activity => "activity",
            Self::Git => "git",
            Self::Messaging => "messaging",
            Self::Notes => "notes",
            Self::Transcripts => "transcripts",
            Self::Documents => "documents",
            Self::Health => "health",
            Self::Gaming => "gaming",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for IntegrationFamily {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "calendar" => Ok(Self::Calendar),
            "tasks" => Ok(Self::Tasks),
            "activity" => Ok(Self::Activity),
            "git" => Ok(Self::Git),
            "messaging" => Ok(Self::Messaging),
            "notes" => Ok(Self::Notes),
            "transcripts" => Ok(Self::Transcripts),
            "documents" => Ok(Self::Documents),
            "health" => Ok(Self::Health),
            "gaming" => Ok(Self::Gaming),
            _ => Err(VelCoreError::Validation(format!(
                "unknown integration family: {}",
                value
            ))),
        }
    }
}

/// Provider identity within a family. Kept provider-agnostic and string-keyed so new vendors do not require core schema edits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntegrationProvider {
    pub family: IntegrationFamily,
    pub key: String,
}

impl IntegrationProvider {
    pub fn new(family: IntegrationFamily, key: impl Into<String>) -> Result<Self, VelCoreError> {
        let key = normalize_provider_key(key.into())?;
        Ok(Self { family, key })
    }
}

impl Display for IntegrationProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.family, self.key)
    }
}

/// Lifecycle status of a provider connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationConnectionStatus {
    Pending,
    Connected,
    Degraded,
    Disabled,
    Error,
}

impl Display for IntegrationConnectionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Pending => "pending",
            Self::Connected => "connected",
            Self::Degraded => "degraded",
            Self::Disabled => "disabled",
            Self::Error => "error",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for IntegrationConnectionStatus {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pending" => Ok(Self::Pending),
            "connected" => Ok(Self::Connected),
            "degraded" => Ok(Self::Degraded),
            "disabled" => Ok(Self::Disabled),
            "error" => Ok(Self::Error),
            _ => Err(VelCoreError::Validation(format!(
                "unknown integration connection status: {}",
                value
            ))),
        }
    }
}

/// Minimal canonical connection record. Existing family-oriented settings can map into this without changing current routes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationConnection {
    pub id: IntegrationConnectionId,
    pub provider: IntegrationProvider,
    pub status: IntegrationConnectionStatus,
    pub display_name: String,
    pub account_ref: Option<String>,
    pub metadata_json: JsonValue,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Mapping from a canonical connection to existing family/provider-specific config keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrationConnectionSettingRef {
    pub connection_id: IntegrationConnectionId,
    pub setting_key: String,
    pub setting_value: String,
    pub created_at: OffsetDateTime,
}

/// Append-only connection lifecycle/sync event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationConnectionEventType {
    Created,
    Updated,
    Enabled,
    Disabled,
    SyncRequested,
    SyncStarted,
    SyncSucceeded,
    SyncFailed,
    AuthStarted,
    AuthSucceeded,
    AuthFailed,
}

impl Display for IntegrationConnectionEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::SyncRequested => "sync_requested",
            Self::SyncStarted => "sync_started",
            Self::SyncSucceeded => "sync_succeeded",
            Self::SyncFailed => "sync_failed",
            Self::AuthStarted => "auth_started",
            Self::AuthSucceeded => "auth_succeeded",
            Self::AuthFailed => "auth_failed",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for IntegrationConnectionEventType {
    type Err = VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "created" => Ok(Self::Created),
            "updated" => Ok(Self::Updated),
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            "sync_requested" => Ok(Self::SyncRequested),
            "sync_started" => Ok(Self::SyncStarted),
            "sync_succeeded" => Ok(Self::SyncSucceeded),
            "sync_failed" => Ok(Self::SyncFailed),
            "auth_started" => Ok(Self::AuthStarted),
            "auth_succeeded" => Ok(Self::AuthSucceeded),
            "auth_failed" => Ok(Self::AuthFailed),
            _ => Err(VelCoreError::Validation(format!(
                "unknown integration connection event type: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationConnectionEvent {
    pub id: String,
    pub connection_id: IntegrationConnectionId,
    pub event_type: IntegrationConnectionEventType,
    pub payload_json: JsonValue,
    pub timestamp: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

/// Provider-scoped provenance reference every adapter can attach to emitted domain objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrationSourceRef {
    pub family: IntegrationFamily,
    pub provider_key: String,
    pub connection_id: IntegrationConnectionId,
    pub external_id: String,
}

fn normalize_provider_key(value: String) -> Result<String, VelCoreError> {
    let trimmed = value.trim().to_ascii_lowercase();
    if trimmed.is_empty() {
        return Err(VelCoreError::Validation(
            "integration provider key cannot be empty".to_string(),
        ));
    }
    if trimmed
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
    {
        Ok(trimmed)
    } else {
        Err(VelCoreError::Validation(format!(
            "invalid integration provider key: {}",
            value
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_provider_normalizes_key() {
        let provider = IntegrationProvider::new(IntegrationFamily::Messaging, " Signal ").unwrap();
        assert_eq!(provider.family, IntegrationFamily::Messaging);
        assert_eq!(provider.key, "signal");
    }

    #[test]
    fn integration_provider_rejects_invalid_key() {
        let error =
            IntegrationProvider::new(IntegrationFamily::Calendar, "google workspace").unwrap_err();
        assert!(error
            .to_string()
            .contains("invalid integration provider key"));
    }
}
