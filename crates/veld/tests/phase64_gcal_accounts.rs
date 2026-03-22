use serde_json::json;
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};
use vel_adapters_google_calendar::{
    DEFAULT_FUTURE_DAYS, DEFAULT_PAST_DAYS, GoogleCalendarAccountLinkRequest,
    GoogleCalendarCheckpointState, GoogleCalendarPayload, GoogleEventPayload, GoogleImportWindow,
    GoogleWindowedImportRequest, google_calendar_module_manifest, import_google_window,
    link_google_calendar_account,
};
use vel_storage::{
    CanonicalObjectQuery, get_canonical_object, get_integration_account,
    list_sync_links_for_object, migrate_storage, query_canonical_objects,
};

#[tokio::test]
async fn google_calendar_multi_account_bounded_import_uses_canonical_account_and_synclink_substrate()
 {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let module = google_calendar_module_manifest();
    assert_eq!(
        module.registry_id.as_string(),
        "module.integration.google-calendar"
    );

    let primary = link_google_calendar_account(
        &pool,
        &GoogleCalendarAccountLinkRequest {
            external_account_ref: "google_primary".to_string(),
            display_name: "Google Primary".to_string(),
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
    let secondary = link_google_calendar_account(
        &pool,
        &GoogleCalendarAccountLinkRequest {
            external_account_ref: "google_secondary".to_string(),
            display_name: "Google Secondary".to_string(),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "bounded_window".to_string(),
            metadata_json: json!({"workspace":"sidecar"}),
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_secondary".to_string()),
            },
        },
    )
    .await
    .unwrap();

    assert_ne!(primary.id, secondary.id);

    let imported_at = OffsetDateTime::now_utc();
    let bounded_window = GoogleImportWindow::bounded_default(imported_at);
    let expanded_window = GoogleImportWindow {
        start: imported_at - Duration::days(DEFAULT_PAST_DAYS + 30),
        end: imported_at + Duration::days(DEFAULT_FUTURE_DAYS + 30),
    };

    let primary_import = import_google_window(
        &pool,
        &GoogleWindowedImportRequest {
            integration_account: primary.clone(),
            calendars: vec![
                GoogleCalendarPayload {
                    remote_id: "primary".to_string(),
                    summary: "Personal".to_string(),
                    timezone: "America/Denver".to_string(),
                    color: Some("#123456".to_string()),
                    description: None,
                    is_primary: true,
                },
                GoogleCalendarPayload {
                    remote_id: "team".to_string(),
                    summary: "Team".to_string(),
                    timezone: "America/Denver".to_string(),
                    color: Some("#654321".to_string()),
                    description: Some("Shared calendar".to_string()),
                    is_primary: false,
                },
            ],
            events: vec![
                GoogleEventPayload {
                    remote_id: "evt_inside".to_string(),
                    calendar_remote_id: "primary".to_string(),
                    summary: "Planning".to_string(),
                    description: Some("Windowed import".to_string()),
                    start: imported_at + Duration::days(10),
                    end: imported_at + Duration::days(10) + Duration::hours(1),
                    transparency: "opaque".to_string(),
                    remote_version: Some("v1".to_string()),
                },
                GoogleEventPayload {
                    remote_id: "evt_outside".to_string(),
                    calendar_remote_id: "team".to_string(),
                    summary: "Ancient".to_string(),
                    description: None,
                    start: imported_at - Duration::days(DEFAULT_PAST_DAYS + 15),
                    end: imported_at - Duration::days(DEFAULT_PAST_DAYS + 15) + Duration::hours(1),
                    transparency: "transparent".to_string(),
                    remote_version: Some("v1".to_string()),
                },
            ],
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_primary_v1".to_string()),
            },
            window: bounded_window.clone(),
            imported_at,
        },
    )
    .await
    .unwrap();

    let secondary_import = import_google_window(
        &pool,
        &GoogleWindowedImportRequest {
            integration_account: secondary.clone(),
            calendars: vec![GoogleCalendarPayload {
                remote_id: "primary".to_string(),
                summary: "Secondary Personal".to_string(),
                timezone: "UTC".to_string(),
                color: None,
                description: None,
                is_primary: true,
            }],
            events: vec![GoogleEventPayload {
                remote_id: "evt_inside".to_string(),
                calendar_remote_id: "primary".to_string(),
                summary: "Secondary Planning".to_string(),
                description: None,
                start: imported_at + Duration::days(5),
                end: imported_at + Duration::days(5) + Duration::hours(1),
                transparency: "opaque".to_string(),
                remote_version: Some("v7".to_string()),
            }],
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_secondary_v1".to_string()),
            },
            window: expanded_window.clone(),
            imported_at,
        },
    )
    .await
    .unwrap();

    let repeat_primary = import_google_window(
        &pool,
        &GoogleWindowedImportRequest {
            integration_account: primary.clone(),
            calendars: vec![GoogleCalendarPayload {
                remote_id: "primary".to_string(),
                summary: "Personal".to_string(),
                timezone: "America/Denver".to_string(),
                color: Some("#123456".to_string()),
                description: None,
                is_primary: true,
            }],
            events: vec![GoogleEventPayload {
                remote_id: "evt_inside".to_string(),
                calendar_remote_id: "primary".to_string(),
                summary: "Planning".to_string(),
                description: Some("Windowed import".to_string()),
                start: imported_at + Duration::days(10),
                end: imported_at + Duration::days(10) + Duration::hours(1),
                transparency: "opaque".to_string(),
                remote_version: Some("v2".to_string()),
            }],
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_primary_v2".to_string()),
            },
            window: bounded_window,
            imported_at,
        },
    )
    .await
    .unwrap();

    assert_eq!(primary_import.imported_calendars.len(), 2);
    assert_eq!(primary_import.imported_events.len(), 1);
    assert_eq!(primary_import.skipped_outside_window, 1);
    assert_eq!(secondary_import.imported_calendars.len(), 1);
    assert_eq!(secondary_import.imported_events.len(), 1);
    assert!(!repeat_primary.imported_calendars[0].created);
    assert!(!repeat_primary.imported_events[0].created);

    let calendars = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            object_type: Some("calendar".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    let events = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            object_type: Some("event".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(calendars.len(), 3);
    assert_eq!(events.len(), 2);

    let primary_event = get_canonical_object(&pool, &primary_import.imported_events[0].event_id)
        .await
        .unwrap()
        .expect("primary event should exist");
    let secondary_event =
        get_canonical_object(&pool, &secondary_import.imported_events[0].event_id)
            .await
            .unwrap()
            .expect("secondary event should exist");
    assert_ne!(primary_event.id, secondary_event.id);
    assert_eq!(
        primary_event.provenance_json["source_refs"][0],
        "module.integration.google-calendar"
    );

    let primary_links = list_sync_links_for_object(&pool, &primary_event.id)
        .await
        .unwrap();
    let secondary_links = list_sync_links_for_object(&pool, &secondary_event.id)
        .await
        .unwrap();
    assert_eq!(primary_links.len(), 1);
    assert_eq!(secondary_links.len(), 1);
    assert_eq!(primary_links[0].integration_account_id, primary.id);
    assert_eq!(secondary_links[0].integration_account_id, secondary.id);

    let stored_primary = get_integration_account(&pool, &primary.id)
        .await
        .unwrap()
        .expect("primary account should persist");
    assert_eq!(
        stored_primary.metadata_json["checkpoints"]["sync_cursor"],
        "sync_primary_v2"
    );
    assert_eq!(stored_primary.metadata_json["window"]["bounded"], true);
    assert_eq!(
        stored_primary.metadata_json["window"]["past_days"],
        DEFAULT_PAST_DAYS
    );
    assert_eq!(
        stored_primary.metadata_json["window"]["future_days"],
        DEFAULT_FUTURE_DAYS
    );
}
