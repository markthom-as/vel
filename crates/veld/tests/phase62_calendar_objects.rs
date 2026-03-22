use serde_json::json;
use time::OffsetDateTime;
use vel_core::{
    belongs_to_calendar, Calendar, CalendarEnvelope, CalendarId, CalendarVisibility,
    CanonicalObjectEnvelope, DurableStatus, Event, EventEnvelope, EventId, EventLocation,
    EventMoment, EventMomentKind, EventTransparency, ObjectClass, ObjectProvenance,
};

fn calendar() -> Calendar {
    Calendar {
        id: CalendarId::new(),
        display_name: "Personal".to_string(),
        timezone: "America/Denver".to_string(),
        visibility: CalendarVisibility::Private,
        is_default: true,
        description: Some("Primary calendar".to_string()),
        color: Some("#2274a5".to_string()),
    }
}

fn event() -> Event {
    Event {
        id: EventId::new(),
        title: "Design review".to_string(),
        description: Some("Native canonical event".to_string()),
        start: EventMoment {
            kind: EventMomentKind::ZonedDateTime,
            value: "2026-03-22T09:00:00-06:00".to_string(),
            timezone: Some("America/Denver".to_string()),
        },
        end: EventMoment {
            kind: EventMomentKind::ZonedDateTime,
            value: "2026-03-22T10:00:00-06:00".to_string(),
            timezone: Some("America/Denver".to_string()),
        },
        transparency: EventTransparency::Opaque,
        location: Some(EventLocation {
            label: "Home Office".to_string(),
            address: Some("Denver".to_string()),
            notes: Some("Desk".to_string()),
            uri: None,
        }),
    }
}

#[test]
fn calendars_and_events_are_first_class_content_objects() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let calendar = calendar();
    let event = event();

    calendar.validate().unwrap();
    event.validate().unwrap();

    let calendar_envelope: CalendarEnvelope = CanonicalObjectEnvelope {
        id: calendar.id.clone(),
        object_type: "calendar".to_string(),
        object_class: ObjectClass::Content,
        schema_version: "0.5".to_string(),
        created_at: now,
        updated_at: now,
        status: DurableStatus::Active,
        provenance: ObjectProvenance {
            origin: Some("user".to_string()),
            source_refs: vec![],
            basis: Some("exact".to_string()),
        },
        facets: json!({"display_name": calendar.display_name, "timezone": calendar.timezone}),
        deleted_at: None,
        archived_at: None,
        source_summary: None,
    };

    let event_envelope: EventEnvelope = CanonicalObjectEnvelope {
        id: event.id.clone(),
        object_type: "event".to_string(),
        object_class: ObjectClass::Content,
        schema_version: "0.5".to_string(),
        created_at: now,
        updated_at: now,
        status: DurableStatus::Active,
        provenance: ObjectProvenance {
            origin: Some("imported".to_string()),
            source_refs: vec![],
            basis: Some("mixed".to_string()),
        },
        facets: json!({"title": event.title, "transparency": "opaque"}),
        deleted_at: None,
        archived_at: None,
        source_summary: None,
    };

    assert_eq!(calendar_envelope.object_class, ObjectClass::Content);
    assert_eq!(event_envelope.object_class, ObjectClass::Content);
    assert!(calendar_envelope.id.to_string().starts_with("calendar_"));
    assert!(event_envelope.id.to_string().starts_with("event_"));
}

#[test]
fn events_relate_canonically_to_calendars_and_keep_location_as_payload() {
    let calendar = calendar();
    let event = event();

    let relation = belongs_to_calendar(&event.id, &calendar.id);

    assert_eq!(relation.from.object_type, "event");
    assert_eq!(relation.to.object_type, "calendar");
    assert_eq!(event.location.as_ref().unwrap().label, "Home Office");
    assert!(event.location.as_ref().unwrap().uri.is_none());
}
