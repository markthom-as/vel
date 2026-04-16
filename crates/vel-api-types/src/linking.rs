use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStatusData {
    Pending,
    Linked,
    Revoked,
    Expired,
}

impl From<vel_core::LinkStatus> for LinkStatusData {
    fn from(value: vel_core::LinkStatus) -> Self {
        match value {
            vel_core::LinkStatus::Pending => Self::Pending,
            vel_core::LinkStatus::Linked => Self::Linked,
            vel_core::LinkStatus::Revoked => Self::Revoked,
            vel_core::LinkStatus::Expired => Self::Expired,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LinkScopeData {
    #[serde(default)]
    pub read_context: bool,
    #[serde(default)]
    pub write_safe_actions: bool,
    #[serde(default)]
    pub execute_repo_tasks: bool,
}

impl From<vel_core::LinkScope> for LinkScopeData {
    fn from(value: vel_core::LinkScope) -> Self {
        Self {
            read_context: value.read_context,
            write_safe_actions: value.write_safe_actions,
            execute_repo_tasks: value.execute_repo_tasks,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTargetSuggestionData {
    pub label: String,
    pub base_url: String,
    pub transport_hint: String,
    pub recommended: bool,
    pub redeem_command_hint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedNodeEndpointKindData {
    Sync,
    Tailscale,
    Lan,
    Localhost,
    Public,
    Relay,
    Introducer,
}

impl From<vel_core::TrustedNodeEndpointKind> for TrustedNodeEndpointKindData {
    fn from(value: vel_core::TrustedNodeEndpointKind) -> Self {
        match value {
            vel_core::TrustedNodeEndpointKind::Sync => Self::Sync,
            vel_core::TrustedNodeEndpointKind::Tailscale => Self::Tailscale,
            vel_core::TrustedNodeEndpointKind::Lan => Self::Lan,
            vel_core::TrustedNodeEndpointKind::Localhost => Self::Localhost,
            vel_core::TrustedNodeEndpointKind::Public => Self::Public,
            vel_core::TrustedNodeEndpointKind::Relay => Self::Relay,
            vel_core::TrustedNodeEndpointKind::Introducer => Self::Introducer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedNodeEndpointData {
    pub kind: TrustedNodeEndpointKindData,
    pub base_url: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_seen_at: Option<OffsetDateTime>,
    pub advertised: bool,
}

impl From<vel_core::TrustedNodeEndpointRecord> for TrustedNodeEndpointData {
    fn from(value: vel_core::TrustedNodeEndpointRecord) -> Self {
        Self {
            kind: value.kind.into(),
            base_url: value.base_url,
            last_seen_at: value.last_seen_at,
            advertised: value.advertised,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedNodeReachabilityData {
    Unknown,
    Reachable,
    Unreachable,
    Stale,
}

impl From<vel_core::TrustedNodeReachability> for TrustedNodeReachabilityData {
    fn from(value: vel_core::TrustedNodeReachability) -> Self {
        match value {
            vel_core::TrustedNodeReachability::Unknown => Self::Unknown,
            vel_core::TrustedNodeReachability::Reachable => Self::Reachable,
            vel_core::TrustedNodeReachability::Unreachable => Self::Unreachable,
            vel_core::TrustedNodeReachability::Stale => Self::Stale,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustBootstrapArtifactData {
    pub artifact_id: String,
    pub trusted_node_id: String,
    pub trusted_node_display_name: String,
    pub scopes: LinkScopeData,
    #[serde(default)]
    pub preferred_transport_hint: Option<String>,
    #[serde(default)]
    pub endpoints: Vec<TrustedNodeEndpointData>,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
}

impl From<vel_core::TrustBootstrapArtifactRecord> for TrustBootstrapArtifactData {
    fn from(value: vel_core::TrustBootstrapArtifactRecord) -> Self {
        Self {
            artifact_id: value.artifact_id,
            trusted_node_id: value.trusted_node_id,
            trusted_node_display_name: value.trusted_node_display_name,
            scopes: value.scopes.into(),
            preferred_transport_hint: value.preferred_transport_hint,
            endpoints: value.endpoints.into_iter().map(Into::into).collect(),
            issued_at: value.issued_at,
            expires_at: value.expires_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingTokenData {
    pub token_id: String,
    pub token_code: String,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub issued_by_node_id: String,
    pub scopes: LinkScopeData,
    #[serde(default)]
    pub suggested_targets: Vec<LinkTargetSuggestionData>,
    #[serde(default)]
    pub bootstrap_artifact: Option<TrustBootstrapArtifactData>,
}

impl From<vel_core::PairingTokenRecord> for PairingTokenData {
    fn from(value: vel_core::PairingTokenRecord) -> Self {
        Self {
            token_id: value.token_id,
            token_code: value.token_code,
            issued_at: value.issued_at,
            expires_at: value.expires_at,
            issued_by_node_id: value.issued_by_node_id,
            scopes: value.scopes.into(),
            suggested_targets: Vec::new(),
            bootstrap_artifact: None,
        }
    }
}
