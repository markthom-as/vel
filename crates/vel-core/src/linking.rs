use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LinkScope {
    #[serde(default)]
    pub read_context: bool,
    #[serde(default)]
    pub write_safe_actions: bool,
    #[serde(default)]
    pub execute_repo_tasks: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStatus {
    Pending,
    Linked,
    Revoked,
    Expired,
}

impl Display for LinkStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Pending => "pending",
            Self::Linked => "linked",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for LinkStatus {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Self::Pending),
            "linked" => Ok(Self::Linked),
            "revoked" => Ok(Self::Revoked),
            "expired" => Ok(Self::Expired),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown link status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairingTokenRecord {
    pub token_id: String,
    pub token_code: String,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub issued_by_node_id: String,
    pub scopes: LinkScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedNodeEndpointKind {
    Sync,
    Tailscale,
    Lan,
    Localhost,
    Public,
    Relay,
    Introducer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedNodeEndpointRecord {
    pub kind: TrustedNodeEndpointKind,
    pub base_url: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_seen_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub advertised: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustedNodeReachability {
    Unknown,
    Reachable,
    Unreachable,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustBootstrapArtifactRecord {
    pub artifact_id: String,
    pub trusted_node_id: String,
    pub trusted_node_display_name: String,
    pub scopes: LinkScope,
    #[serde(default)]
    pub preferred_transport_hint: Option<String>,
    pub endpoints: Vec<TrustedNodeEndpointRecord>,
    #[serde(with = "time::serde::rfc3339")]
    pub issued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedNodeRecord {
    pub node_id: String,
    pub node_display_name: String,
    pub status: LinkStatus,
    pub scopes: LinkScope,
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
    pub endpoint_inventory: Vec<TrustedNodeEndpointRecord>,
    #[serde(default = "default_trusted_node_reachability")]
    pub reachability: TrustedNodeReachability,
}

fn default_trusted_node_reachability() -> TrustedNodeReachability {
    TrustedNodeReachability::Unknown
}

pub fn trusted_node_endpoint_inventory_from_urls(
    sync_base_url: Option<&str>,
    tailscale_base_url: Option<&str>,
    lan_base_url: Option<&str>,
    localhost_base_url: Option<&str>,
    public_base_url: Option<&str>,
) -> Vec<TrustedNodeEndpointRecord> {
    let mut endpoints = Vec::new();
    let mut seen = HashSet::new();

    fn push_endpoint(
        endpoints: &mut Vec<TrustedNodeEndpointRecord>,
        seen: &mut HashSet<String>,
        kind: TrustedNodeEndpointKind,
        base_url: Option<&str>,
    ) {
        let Some(base_url) = base_url.map(str::trim).filter(|value| !value.is_empty()) else {
            return;
        };
        if !seen.insert(base_url.to_string()) {
            return;
        }
        endpoints.push(TrustedNodeEndpointRecord {
            kind,
            base_url: base_url.to_string(),
            last_seen_at: None,
            advertised: true,
        });
    }

    push_endpoint(
        &mut endpoints,
        &mut seen,
        TrustedNodeEndpointKind::Sync,
        sync_base_url,
    );
    push_endpoint(
        &mut endpoints,
        &mut seen,
        TrustedNodeEndpointKind::Tailscale,
        tailscale_base_url,
    );
    push_endpoint(
        &mut endpoints,
        &mut seen,
        TrustedNodeEndpointKind::Lan,
        lan_base_url,
    );
    push_endpoint(
        &mut endpoints,
        &mut seen,
        TrustedNodeEndpointKind::Localhost,
        localhost_base_url,
    );
    push_endpoint(
        &mut endpoints,
        &mut seen,
        TrustedNodeEndpointKind::Public,
        public_base_url,
    );

    endpoints
}

fn transport_hint_for_endpoint_kind(kind: TrustedNodeEndpointKind) -> &'static str {
    match kind {
        TrustedNodeEndpointKind::Sync => "configured",
        TrustedNodeEndpointKind::Tailscale => "tailscale",
        TrustedNodeEndpointKind::Lan => "lan",
        TrustedNodeEndpointKind::Localhost => "localhost",
        TrustedNodeEndpointKind::Public => "public",
        TrustedNodeEndpointKind::Relay => "relay",
        TrustedNodeEndpointKind::Introducer => "introducer",
    }
}

impl TrustBootstrapArtifactRecord {
    pub fn from_advertised_routes(
        artifact_id: String,
        trusted_node_id: String,
        trusted_node_display_name: String,
        scopes: LinkScope,
        preferred_transport_hint: Option<String>,
        issued_at: OffsetDateTime,
        expires_at: Option<OffsetDateTime>,
        sync_base_url: Option<&str>,
        tailscale_base_url: Option<&str>,
        lan_base_url: Option<&str>,
        localhost_base_url: Option<&str>,
        public_base_url: Option<&str>,
    ) -> Self {
        Self {
            artifact_id,
            trusted_node_id,
            trusted_node_display_name,
            scopes,
            preferred_transport_hint,
            endpoints: trusted_node_endpoint_inventory_from_urls(
                sync_base_url,
                tailscale_base_url,
                lan_base_url,
                localhost_base_url,
                public_base_url,
            ),
            issued_at,
            expires_at,
        }
    }

    pub fn to_linked_node_record(&self, linked_at: OffsetDateTime) -> LinkedNodeRecord {
        let endpoint_url = |kind: TrustedNodeEndpointKind| {
            self.endpoints
                .iter()
                .find(|endpoint| endpoint.kind == kind)
                .map(|endpoint| endpoint.base_url.clone())
        };

        LinkedNodeRecord {
            node_id: self.trusted_node_id.clone(),
            node_display_name: self.trusted_node_display_name.clone(),
            status: LinkStatus::Linked,
            scopes: self.scopes,
            linked_at,
            last_seen_at: Some(linked_at),
            transport_hint: self.preferred_transport_hint.clone().or_else(|| {
                self.endpoints
                    .first()
                    .map(|endpoint| transport_hint_for_endpoint_kind(endpoint.kind).to_string())
            }),
            sync_base_url: endpoint_url(TrustedNodeEndpointKind::Sync),
            tailscale_base_url: endpoint_url(TrustedNodeEndpointKind::Tailscale),
            lan_base_url: endpoint_url(TrustedNodeEndpointKind::Lan),
            localhost_base_url: endpoint_url(TrustedNodeEndpointKind::Localhost),
            public_base_url: endpoint_url(TrustedNodeEndpointKind::Public),
            endpoint_inventory: self.endpoints.clone(),
            reachability: TrustedNodeReachability::Unknown,
        }
        .with_endpoint_defaults()
    }
}

impl LinkedNodeRecord {
    pub fn with_endpoint_defaults(mut self) -> Self {
        if self.endpoint_inventory.is_empty() {
            self.endpoint_inventory = trusted_node_endpoint_inventory_from_urls(
                self.sync_base_url.as_deref(),
                self.tailscale_base_url.as_deref(),
                self.lan_base_url.as_deref(),
                self.localhost_base_url.as_deref(),
                self.public_base_url.as_deref(),
            );
        }
        self
    }

    pub fn merge_trust_state_from(mut self, existing: &LinkedNodeRecord) -> Self {
        self.transport_hint = self
            .transport_hint
            .or_else(|| existing.transport_hint.clone());
        self.sync_base_url = self
            .sync_base_url
            .or_else(|| existing.sync_base_url.clone());
        self.tailscale_base_url = self
            .tailscale_base_url
            .or_else(|| existing.tailscale_base_url.clone());
        self.lan_base_url = self.lan_base_url.or_else(|| existing.lan_base_url.clone());
        self.localhost_base_url = self
            .localhost_base_url
            .or_else(|| existing.localhost_base_url.clone());
        self.public_base_url = self
            .public_base_url
            .or_else(|| existing.public_base_url.clone());
        if self.endpoint_inventory.is_empty() {
            self.endpoint_inventory = existing.endpoint_inventory.clone();
        }
        if self.reachability == TrustedNodeReachability::Unknown {
            self.reachability = existing.reachability;
        }
        self.with_endpoint_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        trusted_node_endpoint_inventory_from_urls, LinkScope, LinkStatus, LinkedNodeRecord,
        TrustBootstrapArtifactRecord, TrustedNodeEndpointKind, TrustedNodeEndpointRecord,
        TrustedNodeReachability,
    };

    #[test]
    fn linked_node_record_serializes_scopes_and_status() {
        let record = LinkedNodeRecord {
            node_id: "node_alpha".to_string(),
            node_display_name: "Alpha".to_string(),
            status: LinkStatus::Linked,
            scopes: LinkScope {
                read_context: true,
                write_safe_actions: true,
                execute_repo_tasks: false,
            },
            linked_at: time::macros::datetime!(2026-03-18 10:00:00 UTC),
            last_seen_at: None,
            transport_hint: Some("tailscale".to_string()),
            sync_base_url: Some("http://alpha.tailnet.ts.net:4130".to_string()),
            tailscale_base_url: Some("http://alpha.tailnet.ts.net:4130".to_string()),
            lan_base_url: Some("http://192.168.1.10:4130".to_string()),
            localhost_base_url: Some("http://127.0.0.1:4130".to_string()),
            public_base_url: None,
            endpoint_inventory: vec![
                TrustedNodeEndpointRecord {
                    kind: TrustedNodeEndpointKind::Tailscale,
                    base_url: "http://alpha.tailnet.ts.net:4130".to_string(),
                    last_seen_at: None,
                    advertised: true,
                },
                TrustedNodeEndpointRecord {
                    kind: TrustedNodeEndpointKind::Lan,
                    base_url: "http://192.168.1.10:4130".to_string(),
                    last_seen_at: None,
                    advertised: true,
                },
            ],
            reachability: TrustedNodeReachability::Reachable,
        };

        let value = serde_json::to_value(record).expect("linked node should serialize");
        assert_eq!(value["status"], "linked");
        assert_eq!(value["scopes"]["read_context"], true);
        assert_eq!(value["transport_hint"], "tailscale");
        assert_eq!(value["sync_base_url"], "http://alpha.tailnet.ts.net:4130");
        assert_eq!(value["reachability"], "reachable");
        assert_eq!(value["endpoint_inventory"][0]["kind"], "tailscale");
    }

    #[test]
    fn trust_bootstrap_artifact_serializes_endpoint_inventory() {
        let artifact = TrustBootstrapArtifactRecord {
            artifact_id: "artifact_123".to_string(),
            trusted_node_id: "node_alpha".to_string(),
            trusted_node_display_name: "Alpha".to_string(),
            scopes: LinkScope {
                read_context: true,
                write_safe_actions: true,
                execute_repo_tasks: false,
            },
            preferred_transport_hint: Some("tailscale".to_string()),
            endpoints: vec![TrustedNodeEndpointRecord {
                kind: TrustedNodeEndpointKind::Tailscale,
                base_url: "http://alpha.tailnet.ts.net:4130".to_string(),
                last_seen_at: Some(time::macros::datetime!(2026-03-26 06:00:00 UTC)),
                advertised: true,
            }],
            issued_at: time::macros::datetime!(2026-03-26 05:00:00 UTC),
            expires_at: Some(time::macros::datetime!(2026-03-26 05:15:00 UTC)),
        };

        let value = serde_json::to_value(artifact).expect("artifact should serialize");
        assert_eq!(value["trusted_node_id"], "node_alpha");
        assert_eq!(value["endpoints"][0]["kind"], "tailscale");
        assert_eq!(value["endpoints"][0]["advertised"], true);
        assert_eq!(value["preferred_transport_hint"], "tailscale");
    }

    #[test]
    fn endpoint_inventory_builder_dedupes_and_orders_routes() {
        let endpoints = trusted_node_endpoint_inventory_from_urls(
            Some("http://alpha:4130"),
            Some("http://alpha:4130"),
            Some("http://192.168.1.10:4130"),
            Some("http://127.0.0.1:4130"),
            None,
        );

        assert_eq!(endpoints.len(), 3);
        assert_eq!(endpoints[0].kind, TrustedNodeEndpointKind::Sync);
        assert_eq!(endpoints[1].kind, TrustedNodeEndpointKind::Lan);
        assert_eq!(endpoints[2].kind, TrustedNodeEndpointKind::Localhost);
    }

    #[test]
    fn trust_bootstrap_artifact_builds_linked_node_record() {
        let artifact = TrustBootstrapArtifactRecord::from_advertised_routes(
            "artifact_123".to_string(),
            "node_alpha".to_string(),
            "Alpha".to_string(),
            LinkScope {
                read_context: true,
                write_safe_actions: false,
                execute_repo_tasks: false,
            },
            Some("tailscale".to_string()),
            time::macros::datetime!(2026-03-26 05:00:00 UTC),
            Some(time::macros::datetime!(2026-03-26 05:15:00 UTC)),
            Some("http://alpha.tailnet.ts.net:4130"),
            Some("http://alpha.tailnet.ts.net:4130"),
            Some("http://192.168.1.10:4130"),
            None,
            None,
        );

        let linked =
            artifact.to_linked_node_record(time::macros::datetime!(2026-03-26 05:01:00 UTC));
        assert_eq!(linked.node_id, "node_alpha");
        assert_eq!(linked.transport_hint.as_deref(), Some("tailscale"));
        assert_eq!(
            linked.sync_base_url.as_deref(),
            Some("http://alpha.tailnet.ts.net:4130")
        );
        assert_eq!(
            linked.lan_base_url.as_deref(),
            Some("http://192.168.1.10:4130")
        );
        assert_eq!(linked.endpoint_inventory.len(), 2);
    }
}
