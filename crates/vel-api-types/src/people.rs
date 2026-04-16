use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_core::PersonId;

use crate::IntegrationSourceRefData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAliasData {
    pub platform: String,
    pub handle: String,
    pub display: String,
    pub source_ref: Option<IntegrationSourceRefData>,
}

impl From<vel_core::PersonAlias> for PersonAliasData {
    fn from(value: vel_core::PersonAlias) -> Self {
        Self {
            platform: value.platform,
            handle: value.handle,
            display: value.display,
            source_ref: value.source_ref.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonLinkRefData {
    pub kind: String,
    pub id: String,
    pub label: String,
}

impl From<vel_core::PersonLinkRef> for PersonLinkRefData {
    fn from(value: vel_core::PersonLinkRef) -> Self {
        Self {
            kind: value.kind,
            id: value.id,
            label: value.label,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRecordData {
    pub id: PersonId,
    pub display_name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub relationship_context: Option<String>,
    pub birthday: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_contacted_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub aliases: Vec<PersonAliasData>,
    #[serde(default)]
    pub links: Vec<PersonLinkRefData>,
}

impl From<vel_core::PersonRecord> for PersonRecordData {
    fn from(value: vel_core::PersonRecord) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            given_name: value.given_name,
            family_name: value.family_name,
            relationship_context: value.relationship_context,
            birthday: value.birthday,
            last_contacted_at: value.last_contacted_at,
            aliases: value.aliases.into_iter().map(Into::into).collect(),
            links: value.links.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAliasUpsertRequestData {
    pub platform: String,
    pub handle: String,
    pub display: Option<String>,
    pub source_ref: Option<IntegrationSourceRefData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn person_record_last_contacted_at_serializes_as_rfc3339_string() {
        let person = PersonRecordData {
            id: PersonId::from("per_1".to_string()),
            display_name: "Annie".to_string(),
            given_name: Some("Annie".to_string()),
            family_name: None,
            relationship_context: Some("collaborator".to_string()),
            birthday: None,
            last_contacted_at: Some(datetime!(2026-03-19 02:10:00 UTC)),
            aliases: vec![PersonAliasData {
                platform: "email".to_string(),
                handle: "annie@example.com".to_string(),
                display: "Annie".to_string(),
                source_ref: None,
            }],
            links: vec![PersonLinkRefData {
                kind: "project".to_string(),
                id: "proj_1".to_string(),
                label: "Vel".to_string(),
            }],
        };

        let value = serde_json::to_value(person).expect("person should serialize");
        assert_eq!(value["last_contacted_at"], "2026-03-19T02:10:00Z");
        assert_eq!(value["aliases"][0]["platform"], "email");
        assert_eq!(value["links"][0]["kind"], "project");
    }
}
