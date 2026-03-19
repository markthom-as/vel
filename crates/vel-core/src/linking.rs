use serde::{Deserialize, Serialize};
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
}

#[cfg(test)]
mod tests {
    use super::{LinkScope, LinkStatus, LinkedNodeRecord};

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
        };

        let value = serde_json::to_value(record).expect("linked node should serialize");
        assert_eq!(value["status"], "linked");
        assert_eq!(value["scopes"]["read_context"], true);
        assert_eq!(value["transport_hint"], "tailscale");
        assert_eq!(value["sync_base_url"], "http://alpha.tailnet.ts.net:4130");
    }
}
