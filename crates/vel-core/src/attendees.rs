use serde::{Deserialize, Serialize};

use crate::PersonId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParticipantStub {
    pub stable_key: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub confidence: ParticipantConfidence,
    pub auto_promotable: bool,
    pub source_provider: Option<String>,
}

impl ParticipantStub {
    pub fn validate(&self) -> Result<(), String> {
        if self.stable_key.trim().is_empty() {
            return Err("participant stub stable_key must not be empty".to_string());
        }
        if self.email.as_deref().unwrap_or_default().trim().is_empty()
            && self
                .display_name
                .as_deref()
                .unwrap_or_default()
                .trim()
                .is_empty()
        {
            return Err("participant stub requires email or display_name".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
pub enum ParticipantRef {
    Person(PersonId),
    Stub(ParticipantStub),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipationResponseStatus {
    NeedsAction,
    Accepted,
    Tentative,
    Declined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Participation {
    pub participant_ref: ParticipantRef,
    pub response_status: ParticipationResponseStatus,
    pub is_organizer: bool,
    pub is_self: bool,
    pub is_optional: bool,
    pub is_resource: bool,
    pub source_provenance: Option<String>,
}

impl Participation {
    pub fn validate(&self) -> Result<(), String> {
        if self.is_self && self.is_resource {
            return Err("participation cannot be both self and resource".to_string());
        }

        if let ParticipantRef::Stub(stub) = &self.participant_ref {
            stub.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ParticipantConfidence, ParticipantRef, ParticipantStub, Participation,
        ParticipationResponseStatus,
    };
    use crate::PersonId;

    #[test]
    fn participation_supports_person_links_and_provider_stubs() {
        Participation {
            participant_ref: ParticipantRef::Person(PersonId::new()),
            response_status: ParticipationResponseStatus::Accepted,
            is_organizer: true,
            is_self: false,
            is_optional: false,
            is_resource: false,
            source_provenance: Some("google-calendar".to_string()),
        }
        .validate()
        .unwrap();

        let invalid = Participation {
            participant_ref: ParticipantRef::Stub(ParticipantStub {
                stable_key: String::new(),
                display_name: None,
                email: None,
                confidence: ParticipantConfidence::Low,
                auto_promotable: false,
                source_provider: Some("google-calendar".to_string()),
            }),
            response_status: ParticipationResponseStatus::NeedsAction,
            is_organizer: false,
            is_self: true,
            is_optional: false,
            is_resource: true,
            source_provenance: None,
        };
        assert!(invalid.validate().is_err());
    }
}
