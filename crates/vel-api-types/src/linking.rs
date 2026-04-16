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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkingPromptData {
    pub target_node_id: String,
    pub target_node_display_name: Option<String>,
    pub issued_by_node_id: String,
    pub issued_by_node_display_name: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub scopes: LinkScopeData,
    #[serde(default)]
    pub issuer_sync_base_url: String,
    #[serde(default)]
    pub issuer_sync_transport: String,
    #[serde(default)]
    pub issuer_tailscale_base_url: Option<String>,
    #[serde(default)]
    pub issuer_lan_base_url: Option<String>,
    #[serde(default)]
    pub issuer_localhost_base_url: Option<String>,
    #[serde(default)]
    pub issuer_public_base_url: Option<String>,
    #[serde(default)]
    pub bootstrap_artifact: Option<TrustBootstrapArtifactData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedNodeData {
    pub node_id: String,
    pub node_display_name: String,
    pub status: LinkStatusData,
    pub scopes: LinkScopeData,
    #[serde(with = "time::serde::rfc3339")]
    pub linked_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_seen_at: Option<OffsetDateTime>,
    pub transport_hint: Option<String>,
    pub sync_base_url: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub public_base_url: Option<String>,
    #[serde(default)]
    pub endpoint_inventory: Vec<TrustedNodeEndpointData>,
    pub reachability: TrustedNodeReachabilityData,
}

impl From<vel_core::LinkedNodeRecord> for LinkedNodeData {
    fn from(value: vel_core::LinkedNodeRecord) -> Self {
        Self {
            node_id: value.node_id,
            node_display_name: value.node_display_name,
            status: value.status.into(),
            scopes: value.scopes.into(),
            linked_at: value.linked_at,
            last_seen_at: value.last_seen_at,
            transport_hint: value.transport_hint,
            sync_base_url: value.sync_base_url,
            tailscale_base_url: value.tailscale_base_url,
            lan_base_url: value.lan_base_url,
            localhost_base_url: value.localhost_base_url,
            public_base_url: value.public_base_url,
            endpoint_inventory: value
                .endpoint_inventory
                .into_iter()
                .map(Into::into)
                .collect(),
            reachability: value.reachability.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairing_and_linking_datetimes_serialize_as_rfc3339_strings() {
        let issued_at = OffsetDateTime::from_unix_timestamp(1_710_590_400).unwrap();
        let expires_at = OffsetDateTime::from_unix_timestamp(1_710_590_700).unwrap();

        let token = serde_json::to_value(PairingTokenData {
            token_id: "ptok_1".to_string(),
            token_code: "ABC123".to_string(),
            issued_at,
            expires_at,
            issued_by_node_id: "vel-node".to_string(),
            scopes: LinkScopeData {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            suggested_targets: Vec::new(),
            bootstrap_artifact: Some(TrustBootstrapArtifactData {
                artifact_id: "artifact_123".to_string(),
                trusted_node_id: "vel-node".to_string(),
                trusted_node_display_name: "Vel Node".to_string(),
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                preferred_transport_hint: Some("tailscale".to_string()),
                endpoints: vec![TrustedNodeEndpointData {
                    kind: TrustedNodeEndpointKindData::Tailscale,
                    base_url: "http://vel-node.tailnet.ts.net:4130".to_string(),
                    last_seen_at: Some(expires_at),
                    advertised: true,
                }],
                issued_at,
                expires_at: Some(expires_at),
            }),
        })
        .unwrap();
        assert!(token["issued_at"].is_string());
        assert!(token["expires_at"].is_string());
        assert_eq!(
            token["bootstrap_artifact"]["preferred_transport_hint"],
            "tailscale"
        );

        let prompt = serde_json::to_value(LinkingPromptData {
            target_node_id: "node_remote".to_string(),
            target_node_display_name: Some("Remote".to_string()),
            issued_by_node_id: "vel-node".to_string(),
            issued_by_node_display_name: Some("Local".to_string()),
            issued_at,
            expires_at,
            scopes: LinkScopeData {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            issuer_sync_base_url: "http://vel-node.tailnet.ts.net:4130".to_string(),
            issuer_sync_transport: "tailscale".to_string(),
            issuer_tailscale_base_url: Some("http://vel-node.tailnet.ts.net:4130".to_string()),
            issuer_lan_base_url: Some("http://192.168.1.10:4130".to_string()),
            issuer_localhost_base_url: Some("http://127.0.0.1:4130".to_string()),
            issuer_public_base_url: None,
            bootstrap_artifact: Some(TrustBootstrapArtifactData {
                artifact_id: "artifact_123".to_string(),
                trusted_node_id: "vel-node".to_string(),
                trusted_node_display_name: "Local".to_string(),
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                preferred_transport_hint: Some("tailscale".to_string()),
                endpoints: vec![TrustedNodeEndpointData {
                    kind: TrustedNodeEndpointKindData::Tailscale,
                    base_url: "http://vel-node.tailnet.ts.net:4130".to_string(),
                    last_seen_at: Some(expires_at),
                    advertised: true,
                }],
                issued_at,
                expires_at: Some(expires_at),
            }),
        })
        .unwrap();
        assert!(prompt["issued_at"].is_string());
        assert!(prompt["expires_at"].is_string());
        assert_eq!(prompt["bootstrap_artifact"]["trusted_node_id"], "vel-node");

        let linked = serde_json::to_value(LinkedNodeData {
            node_id: "node_remote".to_string(),
            node_display_name: "Remote".to_string(),
            status: LinkStatusData::Linked,
            scopes: LinkScopeData {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            linked_at: issued_at,
            last_seen_at: Some(expires_at),
            transport_hint: Some("tailscale".to_string()),
            sync_base_url: Some("http://node-remote.tailnet.ts.net:4130".to_string()),
            tailscale_base_url: Some("http://node-remote.tailnet.ts.net:4130".to_string()),
            lan_base_url: Some("http://192.168.1.20:4130".to_string()),
            localhost_base_url: None,
            public_base_url: None,
            endpoint_inventory: vec![TrustedNodeEndpointData {
                kind: TrustedNodeEndpointKindData::Tailscale,
                base_url: "http://node-remote.tailnet.ts.net:4130".to_string(),
                last_seen_at: Some(expires_at),
                advertised: true,
            }],
            reachability: TrustedNodeReachabilityData::Reachable,
        })
        .unwrap();
        assert!(linked["linked_at"].is_string());
        assert!(linked["last_seen_at"].is_string());
        assert_eq!(linked["endpoint_inventory"][0]["kind"], "tailscale");
        assert_eq!(linked["reachability"], "reachable");
    }
}
