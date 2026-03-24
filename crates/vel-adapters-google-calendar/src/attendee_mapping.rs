use sha2::{Digest, Sha256};
use vel_core::{
    ParticipantConfidence, ParticipantRef, ParticipantStub, Participation,
    ParticipationResponseStatus, PersonId,
};

use crate::google_ids::GOOGLE_CALENDAR_PROVIDER;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleAttendeePayload {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub response_status: String,
    pub is_organizer: bool,
    pub is_self: bool,
    pub is_optional: bool,
    pub is_resource: bool,
    pub resource_name: Option<String>,
}

pub fn map_google_attendee(attendee: &GoogleAttendeePayload) -> Participation {
    Participation {
        participant_ref: participant_ref(attendee),
        response_status: response_status(&attendee.response_status),
        is_organizer: attendee.is_organizer,
        is_self: attendee.is_self,
        is_optional: attendee.is_optional,
        is_resource: attendee.is_resource,
        source_provenance: Some(GOOGLE_CALENDAR_PROVIDER.to_string()),
    }
}

fn participant_ref(attendee: &GoogleAttendeePayload) -> ParticipantRef {
    if let Some(email) = normalized_email(attendee.email.as_deref()) {
        ParticipantRef::Person(person_id_for_email(email))
    } else {
        ParticipantRef::Stub(ParticipantStub {
            stable_key: stub_key(attendee),
            display_name: attendee
                .display_name
                .clone()
                .or_else(|| attendee.resource_name.clone()),
            email: attendee.email.clone(),
            confidence: ParticipantConfidence::Low,
            auto_promotable: false,
            source_provider: Some(GOOGLE_CALENDAR_PROVIDER.to_string()),
        })
    }
}

fn normalized_email(email: Option<&str>) -> Option<&str> {
    let email = email?.trim();
    (!email.is_empty()).then_some(email)
}

fn person_id_for_email(email: &str) -> PersonId {
    let mut hasher = Sha256::new();
    hasher.update(email.trim().to_lowercase().as_bytes());
    let digest = hasher.finalize();
    PersonId::from(format!("per_{}", hex::encode(&digest[..12])))
}

fn stub_key(attendee: &GoogleAttendeePayload) -> String {
    let basis = attendee
        .email
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("email:{}", value.trim().to_lowercase()))
        .or_else(|| {
            attendee
                .resource_name
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .map(|value| format!("resource:{}", value.trim()))
        })
        .or_else(|| {
            attendee
                .display_name
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .map(|value| format!("display:{}", value.trim()))
        })
        .unwrap_or_else(|| "unknown".to_string());

    format!("{GOOGLE_CALENDAR_PROVIDER}:attendee:{basis}")
}

fn response_status(value: &str) -> ParticipationResponseStatus {
    match value {
        "accepted" => ParticipationResponseStatus::Accepted,
        "tentative" => ParticipationResponseStatus::Tentative,
        "declined" => ParticipationResponseStatus::Declined,
        _ => ParticipationResponseStatus::NeedsAction,
    }
}

#[cfg(test)]
mod tests {
    use super::{map_google_attendee, GoogleAttendeePayload};
    use vel_core::{ParticipantRef, ParticipationResponseStatus};

    #[test]
    fn attendee_mapping_uses_person_when_email_is_stable_and_stub_when_it_is_not() {
        let resolved = map_google_attendee(&GoogleAttendeePayload {
            display_name: Some("Jove".to_string()),
            email: Some("jove@example.com".to_string()),
            response_status: "accepted".to_string(),
            is_organizer: true,
            is_self: false,
            is_optional: false,
            is_resource: false,
            resource_name: None,
        });
        let stub = map_google_attendee(&GoogleAttendeePayload {
            display_name: Some("Room A".to_string()),
            email: None,
            response_status: "needsAction".to_string(),
            is_organizer: false,
            is_self: false,
            is_optional: true,
            is_resource: true,
            resource_name: Some("Room A".to_string()),
        });

        assert!(matches!(
            resolved.participant_ref,
            ParticipantRef::Person(_)
        ));
        assert!(matches!(stub.participant_ref, ParticipantRef::Stub(_)));
        assert_eq!(
            resolved.response_status,
            ParticipationResponseStatus::Accepted
        );
        assert_eq!(
            stub.response_status,
            ParticipationResponseStatus::NeedsAction
        );
    }
}
