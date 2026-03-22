use chrono::{TimeZone, Utc};
use serde_json::Value;
use time::OffsetDateTime;
use vel_adapters_google_calendar::{
    GoogleAttendeePayload, GoogleAvailabilityBridgeInput, GoogleCalendarMappingPayload,
    GoogleEventLocationPayload, GoogleEventMappingPayload, GoogleEventMomentPayload,
    GoogleRecurrencePayload, apply_google_upstream_delete, bridge_google_availability_input,
    google_availability_projection_envelope, map_google_calendar, map_google_event,
    map_google_recurrence, restore_google_from_tombstone,
};
use vel_core::{
    AllDayHandlingRule, AvailabilityPolicyConfig, AvailabilityResult, CalendarId,
    DeclinedResponsePolicy, EventId, EventMomentKind, ExceptionStatus, ParticipationResponseStatus,
};
use vel_storage::{CanonicalObjectRecord, SyncLinkRecord};
use veld::services::{
    availability_projection::{AvailabilityEventInput, AvailabilityProjectionService},
    calendar_explain::CalendarExplainService,
    recurrence_materialization::RecurrenceMaterializationService,
};

fn calendar_payload() -> GoogleCalendarMappingPayload {
    GoogleCalendarMappingPayload {
        remote_id: "primary".to_string(),
        summary: "Primary".to_string(),
        timezone: "UTC".to_string(),
        description: None,
        color: None,
        access_role: "owner".to_string(),
        is_primary: true,
    }
}

fn event_payload(
    remote_id: &str,
    start: &str,
    end: &str,
    transparency: &str,
) -> GoogleEventMappingPayload {
    GoogleEventMappingPayload {
        remote_id: remote_id.to_string(),
        calendar_remote_id: "primary".to_string(),
        summary: "Focus block".to_string(),
        description: None,
        start: GoogleEventMomentPayload {
            kind: EventMomentKind::ZonedDateTime,
            value: start.to_string(),
            timezone: Some("UTC".to_string()),
        },
        end: GoogleEventMomentPayload {
            kind: EventMomentKind::ZonedDateTime,
            value: end.to_string(),
            timezone: Some("UTC".to_string()),
        },
        transparency: transparency.to_string(),
        location: Some(GoogleEventLocationPayload {
            label: "Desk".to_string(),
            address: None,
            notes: None,
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
    }
}

fn availability_config() -> AvailabilityPolicyConfig {
    AvailabilityPolicyConfig {
        included_calendar_ids: vec![CalendarId::from("calendar_primary".to_string())],
        source_account_ids: vec!["integration_account_google".to_string()],
        declined_response_policy: DeclinedResponsePolicy::IgnoreDeclined,
        exclude_cancelled_events: true,
        all_day_handling_rule: AllDayHandlingRule::RespectTransparency,
    }
}

fn to_projection_input(input: GoogleAvailabilityBridgeInput) -> AvailabilityEventInput {
    AvailabilityEventInput {
        calendar: input.calendar,
        event: input.event,
        source_account_id: input.source_account_id,
        response_status: input.response_status,
        cancelled: input.cancelled,
    }
}

#[test]
fn google_recurrence_and_availability_stay_native_core_aligned() {
    let mapped_calendar = map_google_calendar(
        CalendarId::from("calendar_primary".to_string()),
        &calendar_payload(),
    );
    let mapped_event = map_google_event(
        EventId::from("event_series".to_string()),
        mapped_calendar.calendar.id.clone(),
        &event_payload(
            "evt_series",
            "2026-03-23T08:00:00Z",
            "2026-03-23T08:30:00Z",
            "opaque",
        ),
    );

    let recurrence = map_google_recurrence(
        &mapped_event.event,
        &GoogleRecurrencePayload {
            remote_id: "evt_override".to_string(),
            recurring_event_remote_id: Some("evt_series".to_string()),
            original_start: Some(GoogleEventMomentPayload {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-24T08:00:00Z".to_string(),
                timezone: Some("UTC".to_string()),
            }),
            rrule: Some("FREQ=DAILY;COUNT=3".to_string()),
            status: Some("cancelled".to_string()),
        },
    )
    .unwrap();

    let series = recurrence.series.expect("series should exist");
    let occurrences = RecurrenceMaterializationService::materialize(
        &mapped_event.event,
        &series,
        Utc.with_ymd_and_hms(2026, 3, 23, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 3, 26, 0, 0, 0).unwrap(),
    )
    .unwrap();

    assert_eq!(series.rule.raw_rrule.as_deref(), Some("FREQ=DAILY;COUNT=3"));
    assert_eq!(series.exceptions[0].status, ExceptionStatus::Cancelled);
    assert_eq!(
        recurrence.occurrence.as_ref().unwrap().exception_status,
        Some(ExceptionStatus::Cancelled)
    );
    assert_eq!(occurrences.len(), 3);

    let busy = bridge_google_availability_input(
        mapped_calendar.calendar.clone(),
        mapped_event.event.clone(),
        Some("integration_account_google".to_string()),
        Some(ParticipationResponseStatus::Accepted),
        false,
    );
    let declined = bridge_google_availability_input(
        mapped_calendar.calendar.clone(),
        map_google_event(
            EventId::from("event_declined".to_string()),
            mapped_calendar.calendar.id.clone(),
            &event_payload(
                "evt_declined",
                "2026-03-23T08:05:00Z",
                "2026-03-23T08:10:00Z",
                "opaque",
            ),
        )
        .event,
        Some("integration_account_google".to_string()),
        Some(ParticipationResponseStatus::Declined),
        false,
    );
    let cancelled = bridge_google_availability_input(
        mapped_calendar.calendar.clone(),
        map_google_event(
            EventId::from("event_cancelled".to_string()),
            mapped_calendar.calendar.id.clone(),
            &event_payload(
                "evt_cancelled",
                "2026-03-23T08:15:00Z",
                "2026-03-23T08:20:00Z",
                "opaque",
            ),
        )
        .event,
        Some("integration_account_google".to_string()),
        Some(ParticipationResponseStatus::Accepted),
        true,
    );

    let availability = AvailabilityProjectionService::project(
        Utc.with_ymd_and_hms(2026, 3, 23, 8, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 3, 23, 9, 0, 0).unwrap(),
        &availability_config(),
        &[
            to_projection_input(busy),
            to_projection_input(declined),
            to_projection_input(cancelled),
        ],
    )
    .unwrap();
    let explain = CalendarExplainService::explain_availability(
        &availability,
        "2026-03-23T08:00:00Z",
        "2026-03-23T09:00:00Z",
    );
    let projection = google_availability_projection_envelope(
        "integration_account_google",
        &availability_config(),
        "2026-03-23T08:00:00Z",
        "2026-03-23T09:00:00Z",
    );

    assert_eq!(availability.result, AvailabilityResult::Busy);
    assert_eq!(availability.blocking_intervals.len(), 1);
    assert_eq!(availability.blocking_intervals[0].event_id, "event_series");
    assert_eq!(projection.explain_metadata["basis"], "native_projection");
    assert_eq!(explain["basis"], "exact");
    assert_eq!(explain["accepted"], Value::Bool(false));
}

#[test]
fn google_tombstones_hide_deleted_events_but_preserve_restore_lineage() {
    let object = CanonicalObjectRecord {
        id: "event_01gcal".to_string(),
        object_type: "event".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: serde_json::json!({"origin":"imported"}),
        facets_json: serde_json::json!({"provider_facets":{"google_calendar":{"deleted_upstream":false}}}),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    };
    let sync_link = SyncLinkRecord {
        id: "sync_link_01gcal".to_string(),
        provider: "google-calendar".to_string(),
        integration_account_id: "integration_account_google".to_string(),
        object_id: "event_01gcal".to_string(),
        remote_id: "evt_123".to_string(),
        remote_type: "event".to_string(),
        state: "reconciled".to_string(),
        authority_mode: "shared".to_string(),
        remote_version: Some("etag-1".to_string()),
        metadata_json: serde_json::json!({}),
        linked_at: OffsetDateTime::UNIX_EPOCH,
        last_seen_at: OffsetDateTime::UNIX_EPOCH,
    };

    let deleted = apply_google_upstream_delete(&object, &sync_link, OffsetDateTime::UNIX_EPOCH);
    assert_eq!(deleted.sync_link_state, "deleted_upstream");
    assert_eq!(
        deleted.object.facets_json["provider_facets"]["google_calendar"]["tombstone_state"],
        "pending_reconcile"
    );
    assert_eq!(
        deleted.object.source_summary_json.as_ref().unwrap()["hidden_from_default_queries"],
        Value::Bool(true)
    );

    let restored =
        restore_google_from_tombstone(&deleted.object, &sync_link, OffsetDateTime::UNIX_EPOCH);
    assert_eq!(restored.sync_link_state, "restored");
    assert_eq!(
        restored.object.source_summary_json.as_ref().unwrap()["audit_lineage_preserved"],
        Value::Bool(true)
    );
}
