use serde_json::{Value as JsonValue, json};
use vel_core::{
    CalendarId, Event, EventId, EventLocation, EventMoment, EventMomentKind, EventTransparency,
    Participation,
};

use crate::{
    attendee_mapping::{GoogleAttendeePayload, map_google_attendee},
    google_ids::{GOOGLE_CALENDAR_MODULE_ID, google_provider_object_ref},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleEventMomentPayload {
    pub kind: EventMomentKind,
    pub value: String,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleEventLocationPayload {
    pub label: String,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleEventMappingPayload {
    pub remote_id: String,
    pub calendar_remote_id: String,
    pub summary: String,
    pub description: Option<String>,
    pub start: GoogleEventMomentPayload,
    pub end: GoogleEventMomentPayload,
    pub transparency: String,
    pub location: Option<GoogleEventLocationPayload>,
    pub attendees: Vec<GoogleAttendeePayload>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoogleMappedEvent {
    pub event: Event,
    pub calendar_ref: CalendarId,
    pub participations: Vec<Participation>,
    pub provider_facets: JsonValue,
}

pub fn map_google_event(
    event_id: EventId,
    calendar_ref: CalendarId,
    payload: &GoogleEventMappingPayload,
) -> GoogleMappedEvent {
    GoogleMappedEvent {
        event: Event {
            id: event_id,
            title: payload.summary.clone(),
            description: payload.description.clone(),
            start: to_moment(&payload.start),
            end: to_moment(&payload.end),
            transparency: transparency(&payload.transparency),
            location: payload.location.as_ref().map(|location| EventLocation {
                label: location.label.clone(),
                address: location.address.clone(),
                notes: location.notes.clone(),
                uri: location.uri.clone(),
            }),
        },
        calendar_ref,
        participations: payload.attendees.iter().map(map_google_attendee).collect(),
        provider_facets: json!({
            "google_calendar": {
                "event_id": payload.remote_id,
                "calendar_id": payload.calendar_remote_id,
                "source_ref": google_provider_object_ref("event", &payload.remote_id),
                "module_id": GOOGLE_CALENDAR_MODULE_ID,
            }
        }),
    }
}

fn to_moment(payload: &GoogleEventMomentPayload) -> EventMoment {
    EventMoment {
        kind: payload.kind.clone(),
        value: payload.value.clone(),
        timezone: payload.timezone.clone(),
    }
}

fn transparency(value: &str) -> EventTransparency {
    match value {
        "transparent" => EventTransparency::Transparent,
        _ => EventTransparency::Opaque,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GoogleEventLocationPayload, GoogleEventMappingPayload, GoogleEventMomentPayload,
        map_google_event,
    };
    use crate::attendee_mapping::GoogleAttendeePayload;
    use vel_core::{CalendarId, EventId, EventMomentKind, EventTransparency, ParticipantRef};

    #[test]
    fn event_mapping_keeps_native_event_shape_location_and_participation() {
        let mapped = map_google_event(
            EventId::from("event_design_review".to_string()),
            CalendarId::from("calendar_primary".to_string()),
            &GoogleEventMappingPayload {
                remote_id: "evt_123".to_string(),
                calendar_remote_id: "primary".to_string(),
                summary: "Design review".to_string(),
                description: Some("Native canonical event".to_string()),
                start: GoogleEventMomentPayload {
                    kind: EventMomentKind::ZonedDateTime,
                    value: "2026-03-22T09:00:00-06:00".to_string(),
                    timezone: Some("America/Denver".to_string()),
                },
                end: GoogleEventMomentPayload {
                    kind: EventMomentKind::ZonedDateTime,
                    value: "2026-03-22T10:00:00-06:00".to_string(),
                    timezone: Some("America/Denver".to_string()),
                },
                transparency: "opaque".to_string(),
                location: Some(GoogleEventLocationPayload {
                    label: "Home Office".to_string(),
                    address: Some("Denver".to_string()),
                    notes: Some("Desk".to_string()),
                    uri: None,
                }),
                attendees: vec![GoogleAttendeePayload {
                    display_name: Some("Jove".to_string()),
                    email: Some("jove@example.com".to_string()),
                    response_status: "accepted".to_string(),
                    is_organizer: true,
                    is_self: false,
                    is_optional: false,
                    is_resource: false,
                    resource_name: None,
                }],
            },
        );

        assert_eq!(mapped.event.title, "Design review");
        assert_eq!(mapped.event.transparency, EventTransparency::Opaque);
        assert_eq!(mapped.event.location.as_ref().unwrap().label, "Home Office");
        assert!(matches!(
            mapped.participations[0].participant_ref,
            ParticipantRef::Person(_)
        ));
    }
}
