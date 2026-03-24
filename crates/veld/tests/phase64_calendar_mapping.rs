use vel_adapters_google_calendar::{
    map_google_attendee, map_google_calendar, map_google_event, GoogleAttendeePayload,
    GoogleCalendarMappingPayload, GoogleEventLocationPayload, GoogleEventMappingPayload,
    GoogleEventMomentPayload,
};
use vel_core::{
    CalendarId, CalendarVisibility, EventId, EventMomentKind, EventTransparency, ParticipantRef,
    ParticipationResponseStatus,
};

#[test]
fn google_calendar_event_and_attendee_mapping_stays_native_canonical_first() {
    let calendar = map_google_calendar(
        CalendarId::from("calendar_primary".to_string()),
        &GoogleCalendarMappingPayload {
            remote_id: "primary".to_string(),
            summary: "Personal".to_string(),
            timezone: "America/Denver".to_string(),
            description: Some("Primary calendar".to_string()),
            color: Some("#2274a5".to_string()),
            access_role: "owner".to_string(),
            is_primary: true,
        },
    );
    let resolved = GoogleAttendeePayload {
        display_name: Some("Jove".to_string()),
        email: Some("jove@example.com".to_string()),
        response_status: "accepted".to_string(),
        is_organizer: true,
        is_self: false,
        is_optional: false,
        is_resource: false,
        resource_name: None,
    };
    let stub = GoogleAttendeePayload {
        display_name: Some("Room A".to_string()),
        email: None,
        response_status: "needsAction".to_string(),
        is_organizer: false,
        is_self: false,
        is_optional: true,
        is_resource: true,
        resource_name: Some("Room A".to_string()),
    };
    let mapped_event = map_google_event(
        EventId::from("event_design_review".to_string()),
        calendar.calendar.id.clone(),
        &GoogleEventMappingPayload {
            remote_id: "evt_design".to_string(),
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
            transparency: "transparent".to_string(),
            location: Some(GoogleEventLocationPayload {
                label: "Home Office".to_string(),
                address: Some("Denver".to_string()),
                notes: Some("Desk".to_string()),
                uri: None,
            }),
            attendees: vec![resolved.clone(), stub.clone()],
        },
    );
    let stub_participation = map_google_attendee(&stub);

    calendar.calendar.validate().unwrap();
    mapped_event.event.validate().unwrap();
    mapped_event
        .participations
        .iter()
        .for_each(|value| value.validate().unwrap());

    assert_eq!(calendar.calendar.visibility, CalendarVisibility::Private);
    assert!(calendar.calendar.is_default);
    assert_eq!(
        mapped_event.event.transparency,
        EventTransparency::Transparent
    );
    assert_eq!(
        mapped_event.event.location.as_ref().unwrap().label,
        "Home Office"
    );
    assert_eq!(mapped_event.calendar_ref.as_ref(), "calendar_primary");
    assert!(matches!(
        mapped_event.participations[0].participant_ref,
        ParticipantRef::Person(_)
    ));
    assert!(matches!(
        mapped_event.participations[1].participant_ref,
        ParticipantRef::Stub(_)
    ));
    assert_eq!(
        mapped_event.participations[0].response_status,
        ParticipationResponseStatus::Accepted
    );
    assert_eq!(
        stub_participation.response_status,
        ParticipationResponseStatus::NeedsAction
    );
    assert_eq!(
        calendar.provider_facets["google_calendar"]["module_id"],
        "module.integration.google-calendar"
    );
    assert_eq!(
        mapped_event.provider_facets["google_calendar"]["calendar_id"],
        "primary"
    );
}
