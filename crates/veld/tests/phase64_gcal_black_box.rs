use chrono::TimeZone;
use serde_json::json;
use sqlx::SqlitePool;
use time::{macros::datetime, OffsetDateTime};
use vel_adapters_google_calendar::{
    apply_google_upstream_delete, bridge_google_availability_input,
    google_availability_projection_envelope, google_calendar_module_manifest, import_google_window,
    link_google_calendar_account, map_google_calendar, map_google_event, map_google_recurrence,
    GoogleAvailabilityBridgeInput, GoogleCalendarAccountLinkRequest, GoogleCalendarCheckpointState,
    GoogleCalendarMappingPayload, GoogleEventLocationPayload, GoogleEventMappingPayload,
    GoogleEventMomentPayload, GoogleEventPayload, GoogleImportWindow, GoogleRecurrencePayload,
};
use vel_core::{
    AllDayHandlingRule, AvailabilityPolicyConfig, AvailabilityResult, CalendarId,
    DeclinedResponsePolicy, EventId, EventMomentKind, ParticipationResponseStatus,
};
use vel_storage::{
    get_canonical_object, list_sync_links_for_object, migrate_storage, query_canonical_objects,
    CanonicalObjectQuery,
};
use veld::services::{
    availability_projection::{AvailabilityEventInput, AvailabilityProjectionService},
    calendar_explain::CalendarExplainService,
    gcal_write_bridge::{bridge_google_calendar_write, GoogleCalendarWriteBridgeRequest},
    recurrence_materialization::RecurrenceMaterializationService,
};

fn to_projection_input(input: GoogleAvailabilityBridgeInput) -> AvailabilityEventInput {
    AvailabilityEventInput {
        calendar: input.calendar,
        event: input.event,
        source_account_id: input.source_account_id,
        response_status: input.response_status,
        cancelled: input.cancelled,
    }
}

#[tokio::test]
async fn google_calendar_black_box_proves_account_window_calendar_event_participation_availability_occurrence_tombstone_and_write_flow(
) {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let module = google_calendar_module_manifest();
    assert_eq!(
        module.registry_id.as_string(),
        "module.integration.google-calendar"
    );

    let account = link_google_calendar_account(
        &pool,
        &GoogleCalendarAccountLinkRequest {
            external_account_ref: "gcal_primary".to_string(),
            display_name: "Primary Google".to_string(),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "bounded_window".to_string(),
            metadata_json: json!({"workspace":"main"}),
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_primary".to_string()),
            },
        },
    )
    .await
    .unwrap();

    let imported_at = OffsetDateTime::now_utc();
    let window = GoogleImportWindow {
        start: imported_at - time::Duration::days(1),
        end: imported_at + time::Duration::days(30),
    };
    let report = import_google_window(
        &pool,
        &vel_adapters_google_calendar::GoogleWindowedImportRequest {
            integration_account: account.clone(),
            calendars: vec![GoogleEventlessCalendar::primary().into_payload()],
            events: vec![GoogleEventPayload {
                remote_id: "evt_black_box".to_string(),
                calendar_remote_id: "primary".to_string(),
                summary: "Focus block".to_string(),
                description: None,
                start: imported_at + time::Duration::days(1),
                end: imported_at + time::Duration::days(1) + time::Duration::minutes(30),
                transparency: "opaque".to_string(),
                remote_version: Some("etag-1".to_string()),
            }],
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_primary_v1".to_string()),
            },
            window: window.clone(),
            imported_at,
        },
    )
    .await
    .unwrap();

    let event_id = &report.imported_events[0].event_id;
    let imported_event = get_canonical_object(&pool, event_id)
        .await
        .unwrap()
        .expect("imported event should exist");
    let imported_events = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            object_type: Some("event".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    let links = list_sync_links_for_object(&pool, event_id).await.unwrap();

    assert_eq!(report.skipped_outside_window, 0);
    assert_eq!(imported_events.len(), 1);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].provider, "google-calendar");

    let mapped_calendar = map_google_calendar(
        CalendarId::from("calendar_primary".to_string()),
        &GoogleCalendarMappingPayload {
            remote_id: "primary".to_string(),
            summary: "Primary".to_string(),
            timezone: "UTC".to_string(),
            description: None,
            color: None,
            access_role: "owner".to_string(),
            is_primary: true,
        },
    );
    let mapped_event = map_google_event(
        EventId::from(event_id.clone()),
        mapped_calendar.calendar.id.clone(),
        &GoogleEventMappingPayload {
            remote_id: "evt_black_box".to_string(),
            calendar_remote_id: "primary".to_string(),
            summary: "Focus block".to_string(),
            description: None,
            start: GoogleEventMomentPayload {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-24T09:00:00Z".to_string(),
                timezone: Some("UTC".to_string()),
            },
            end: GoogleEventMomentPayload {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-24T09:30:00Z".to_string(),
                timezone: Some("UTC".to_string()),
            },
            transparency: "opaque".to_string(),
            location: Some(GoogleEventLocationPayload {
                label: "Desk".to_string(),
                address: None,
                notes: None,
                uri: None,
            }),
            attendees: vec![],
        },
    );
    assert_eq!(
        mapped_event.provider_facets["google_calendar"]["module_id"],
        "module.integration.google-calendar"
    );

    let recurrence = map_google_recurrence(
        &mapped_event.event,
        &GoogleRecurrencePayload {
            remote_id: "evt_black_box_override".to_string(),
            recurring_event_remote_id: Some("evt_black_box".to_string()),
            original_start: Some(GoogleEventMomentPayload {
                kind: EventMomentKind::ZonedDateTime,
                value: "2026-03-25T09:00:00Z".to_string(),
                timezone: Some("UTC".to_string()),
            }),
            rrule: Some("FREQ=DAILY;COUNT=2".to_string()),
            status: Some("cancelled".to_string()),
        },
    )
    .unwrap();
    let occurrences = RecurrenceMaterializationService::materialize(
        &mapped_event.event,
        &recurrence.series.clone().unwrap(),
        chrono::Utc.with_ymd_and_hms(2026, 3, 24, 0, 0, 0).unwrap(),
        chrono::Utc.with_ymd_and_hms(2026, 3, 27, 0, 0, 0).unwrap(),
    )
    .unwrap();
    assert_eq!(occurrences.len(), 2);

    let availability = AvailabilityProjectionService::project(
        datetime!(2026-03-24 9:00:00 UTC),
        datetime!(2026-03-24 10:00:00 UTC),
        &AvailabilityPolicyConfig {
            included_calendar_ids: vec![mapped_calendar.calendar.id.clone()],
            source_account_ids: vec![account.id.clone()],
            declined_response_policy: DeclinedResponsePolicy::IgnoreDeclined,
            exclude_cancelled_events: true,
            all_day_handling_rule: AllDayHandlingRule::RespectTransparency,
        },
        &[to_projection_input(bridge_google_availability_input(
            mapped_calendar.calendar.clone(),
            mapped_event.event.clone(),
            Some(account.id.clone()),
            Some(ParticipationResponseStatus::Accepted),
            false,
        ))],
    )
    .unwrap();
    let explain = CalendarExplainService::explain_availability(
        &availability,
        "2026-03-24T09:00:00Z",
        "2026-03-24T10:00:00Z",
    );
    let projection = google_availability_projection_envelope(
        &account.id,
        &AvailabilityPolicyConfig {
            included_calendar_ids: vec![mapped_calendar.calendar.id.clone()],
            source_account_ids: vec![account.id.clone()],
            declined_response_policy: DeclinedResponsePolicy::IgnoreDeclined,
            exclude_cancelled_events: true,
            all_day_handling_rule: AllDayHandlingRule::RespectTransparency,
        },
        "2026-03-24T09:00:00Z",
        "2026-03-24T10:00:00Z",
    );
    assert_eq!(availability.result, AvailabilityResult::Busy);
    assert_eq!(projection.explain_metadata["basis"], "native_projection");
    assert_eq!(explain["basis"], "exact");

    let deleted = apply_google_upstream_delete(&imported_event, &links[0], imported_at);
    assert_eq!(deleted.sync_link_state, "deleted_upstream");
    assert_eq!(
        deleted.object.source_summary_json.as_ref().unwrap()["hidden_from_default_queries"],
        json!(true)
    );

    let dry_run = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: event_id.clone(),
            expected_revision: imported_event.revision,
            actual_revision: imported_event.revision,
            object_status: imported_event.status.clone(),
            integration_account_id: account.id.clone(),
            requested_change: json!({"title":"Moved focus block"}),
            recurrence_scope: Some("single_occurrence".to_string()),
            source_owned_fields: vec![],
            read_only: false,
            write_enabled: true,
            dry_run: true,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .unwrap();
    assert!(dry_run.dispatch.is_none());

    let executed = bridge_google_calendar_write(
        &pool,
        &GoogleCalendarWriteBridgeRequest {
            object_id: event_id.clone(),
            expected_revision: imported_event.revision,
            actual_revision: imported_event.revision,
            object_status: imported_event.status.clone(),
            integration_account_id: account.id.clone(),
            requested_change: json!({"title":"Moved focus block","scope":"single_occurrence"}),
            recurrence_scope: Some("single_occurrence".to_string()),
            source_owned_fields: vec![],
            read_only: false,
            write_enabled: true,
            dry_run: false,
            approved: true,
            pending_reconciliation: false,
        },
    )
    .await
    .unwrap();
    assert!(executed.dispatch.is_some());
}

struct GoogleEventlessCalendar;

impl GoogleEventlessCalendar {
    fn primary() -> Self {
        Self
    }

    fn into_payload(self) -> vel_adapters_google_calendar::GoogleCalendarPayload {
        vel_adapters_google_calendar::GoogleCalendarPayload {
            remote_id: "primary".to_string(),
            summary: "Primary".to_string(),
            timezone: "UTC".to_string(),
            color: None,
            description: None,
            is_primary: true,
        }
    }
}
