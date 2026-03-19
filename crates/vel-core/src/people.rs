use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::IntegrationSourceRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PersonId(pub(crate) String);

impl PersonId {
    pub fn new() -> Self {
        Self(format!("per_{}", Uuid::new_v4().simple()))
    }
}

impl Default for PersonId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PersonId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for PersonId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for PersonId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonAlias {
    pub platform: String,
    pub handle: String,
    pub display: String,
    pub source_ref: Option<IntegrationSourceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonLinkRef {
    pub kind: String,
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonRecord {
    pub id: PersonId,
    pub display_name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub relationship_context: Option<String>,
    pub birthday: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_contacted_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub aliases: Vec<PersonAlias>,
    #[serde(default)]
    pub links: Vec<PersonLinkRef>,
}

#[cfg(test)]
mod tests {
    use super::PersonRecord;

    #[test]
    fn person_record_example_parses() {
        let record: PersonRecord = serde_json::from_str(include_str!(
            "../../../config/examples/person-record.example.json"
        ))
        .expect("person record example should parse");

        assert_eq!(record.display_name, "Annie Case");
        assert_eq!(record.aliases.len(), 2);
        assert_eq!(record.links.len(), 2);
    }
}
