use chrono::{TimeZone, Utc};
use vel_core::{
    Event, EventId, EventLocation, EventMoment, EventMomentKind, EventTransparency, Exception,
    ExceptionStatus, ParticipantConfidence, ParticipantRef, ParticipantStub, Participation,
    ParticipationResponseStatus, PersonId, RecurrenceFrequency, RecurrenceWeekday, Series,
    SeriesRule,
};
use veld::services::recurrence_materialization::RecurrenceMaterializationService;

fn recurring_event() -> Event {
    Event {
        id: EventId::new(),
        title: "Standup".to_string(),
        description: None,
        start: EventMoment {
            kind: EventMomentKind::ZonedDateTime,
            value: "2026-03-23T08:00:00Z".to_string(),
            timezone: Some("UTC".to_string()),
        },
        end: EventMoment {
            kind: EventMomentKind::ZonedDateTime,
            value: "2026-03-23T08:30:00Z".to_string(),
            timezone: Some("UTC".to_string()),
        },
        transparency: EventTransparency::Opaque,
        location: Some(EventLocation {
            label: "Zoom".to_string(),
            address: None,
            notes: None,
            uri: Some("https://example.com/standup".to_string()),
        }),
    }
}

#[test]
fn recurrence_materialization_represents_series_occurrences_and_exceptions() {
    let event = recurring_event();
    let modified_key = format!("{}:2026-03-24T08:00:00+00:00", event.id);
    let cancelled_key = format!("{}:2026-03-25T08:00:00+00:00", event.id);
    let series = Series {
        series_id: "series_daily_standup".to_string(),
        anchor_event_id: event.id.clone(),
        timezone: Some("UTC".to_string()),
        rule: SeriesRule {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            by_weekdays: vec![],
            count: Some(4),
            until: None,
            raw_rrule: Some("FREQ=DAILY;COUNT=4".to_string()),
        },
        exceptions: vec![
            Exception {
                occurrence_key: modified_key.clone(),
                status: ExceptionStatus::Modified,
                replacement_start: Some(EventMoment {
                    kind: EventMomentKind::ZonedDateTime,
                    value: "2026-03-24T09:00:00Z".to_string(),
                    timezone: Some("UTC".to_string()),
                }),
                replacement_end: Some(EventMoment {
                    kind: EventMomentKind::ZonedDateTime,
                    value: "2026-03-24T09:30:00Z".to_string(),
                    timezone: Some("UTC".to_string()),
                }),
            },
            Exception {
                occurrence_key: cancelled_key.clone(),
                status: ExceptionStatus::Cancelled,
                replacement_start: None,
                replacement_end: None,
            },
        ],
    };

    let occurrences = RecurrenceMaterializationService::materialize(
        &event,
        &series,
        Utc.with_ymd_and_hms(2026, 3, 23, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 3, 27, 0, 0, 0).unwrap(),
    )
    .unwrap();

    assert_eq!(occurrences.len(), 4);
    assert!(occurrences.iter().all(|value| value.materialized));
    assert!(occurrences
        .iter()
        .any(|value| value.occurrence_key == modified_key));
    assert!(occurrences.iter().any(|value| {
        value.occurrence_key == cancelled_key
            && value.exception_status == Some(ExceptionStatus::Cancelled)
    }));
}

#[test]
fn attendee_participation_is_person_linked_or_stub_based_not_opaque_blob() {
    let resolved = Participation {
        participant_ref: ParticipantRef::Person(PersonId::new()),
        response_status: ParticipationResponseStatus::Accepted,
        is_organizer: true,
        is_self: false,
        is_optional: false,
        is_resource: false,
        source_provenance: Some("google-calendar".to_string()),
    };

    let stub = Participation {
        participant_ref: ParticipantRef::Stub(ParticipantStub {
            stable_key: "google-calendar:attendee:room-a".to_string(),
            display_name: Some("Room A".to_string()),
            email: None,
            confidence: ParticipantConfidence::Low,
            auto_promotable: false,
            source_provider: Some("google-calendar".to_string()),
        }),
        response_status: ParticipationResponseStatus::NeedsAction,
        is_organizer: false,
        is_self: false,
        is_optional: true,
        is_resource: true,
        source_provenance: Some("provider_stub".to_string()),
    };

    resolved.validate().unwrap();
    stub.validate().unwrap();

    assert!(matches!(
        resolved.participant_ref,
        ParticipantRef::Person(_)
    ));
    assert!(matches!(stub.participant_ref, ParticipantRef::Stub(_)));
}

#[test]
fn weekly_series_requires_explicit_weekday_selection() {
    let event = recurring_event();
    let series = Series {
        series_id: "series_weekly_standup".to_string(),
        anchor_event_id: event.id,
        timezone: Some("UTC".to_string()),
        rule: SeriesRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            by_weekdays: vec![RecurrenceWeekday::Monday, RecurrenceWeekday::Wednesday],
            count: Some(2),
            until: None,
            raw_rrule: Some("FREQ=WEEKLY;BYDAY=MO,WE;COUNT=2".to_string()),
        },
        exceptions: vec![],
    };

    assert!(series.validate().is_ok());
}
