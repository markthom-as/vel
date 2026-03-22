use serde_json::{Value as JsonValue, json};
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};
use vel_storage::{
    CanonicalObjectRecord, IntegrationAccountRecord, StorageError, SyncLinkRecord, get_sync_link,
    insert_canonical_object, upsert_integration_account, upsert_sync_link,
};

use crate::{
    account_linking::GoogleCalendarCheckpointState,
    google_ids::{
        GOOGLE_CALENDAR_MODULE_ID, GOOGLE_CALENDAR_PROVIDER, GOOGLE_CALENDAR_REMOTE_TYPE,
        GOOGLE_EVENT_REMOTE_TYPE, google_calendar_id, google_event_id, google_provider_object_ref,
        google_sync_link_id,
    },
};

pub const DEFAULT_PAST_DAYS: i64 = 90;
pub const DEFAULT_FUTURE_DAYS: i64 = 365;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleImportWindow {
    pub start: OffsetDateTime,
    pub end: OffsetDateTime,
}

impl GoogleImportWindow {
    pub fn bounded_default(now: OffsetDateTime) -> Self {
        Self {
            start: now - Duration::days(DEFAULT_PAST_DAYS),
            end: now + Duration::days(DEFAULT_FUTURE_DAYS),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GoogleCalendarPayload {
    pub remote_id: String,
    pub summary: String,
    pub timezone: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct GoogleEventPayload {
    pub remote_id: String,
    pub calendar_remote_id: String,
    pub summary: String,
    pub description: Option<String>,
    pub start: OffsetDateTime,
    pub end: OffsetDateTime,
    pub transparency: String,
    pub remote_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GoogleWindowedImportRequest {
    pub integration_account: IntegrationAccountRecord,
    pub calendars: Vec<GoogleCalendarPayload>,
    pub events: Vec<GoogleEventPayload>,
    pub checkpoints: GoogleCalendarCheckpointState,
    pub window: GoogleImportWindow,
    pub imported_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedGoogleCalendar {
    pub calendar_id: String,
    pub sync_link_id: String,
    pub created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedGoogleEvent {
    pub event_id: String,
    pub sync_link_id: String,
    pub created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleWindowedImportReport {
    pub imported_calendars: Vec<ImportedGoogleCalendar>,
    pub imported_events: Vec<ImportedGoogleEvent>,
    pub skipped_outside_window: usize,
}

pub async fn import_google_window(
    pool: &SqlitePool,
    request: &GoogleWindowedImportRequest,
) -> Result<GoogleWindowedImportReport, StorageError> {
    let mut imported_calendars = Vec::with_capacity(request.calendars.len());
    let mut imported_events = Vec::new();
    let mut skipped_outside_window = 0usize;

    for calendar in &request.calendars {
        let calendar_id =
            google_calendar_id(&request.integration_account.id, &calendar.remote_id).to_string();
        let sync_link_id = google_sync_link_id(
            &request.integration_account.id,
            GOOGLE_CALENDAR_REMOTE_TYPE,
            &calendar.remote_id,
        )
        .to_string();
        let existing = get_sync_link(pool, &sync_link_id).await?;
        let created = existing.is_none();

        if created {
            insert_canonical_object(
                pool,
                &map_google_calendar(
                    &calendar_id,
                    &request.integration_account.id,
                    calendar,
                    request.imported_at,
                ),
            )
            .await?;
        }

        upsert_sync_link(
            pool,
            &SyncLinkRecord {
                id: sync_link_id.clone(),
                provider: GOOGLE_CALENDAR_PROVIDER.to_string(),
                integration_account_id: request.integration_account.id.clone(),
                object_id: calendar_id.clone(),
                remote_id: calendar.remote_id.clone(),
                remote_type: GOOGLE_CALENDAR_REMOTE_TYPE.to_string(),
                state: "reconciled".to_string(),
                authority_mode: "shared".to_string(),
                remote_version: None,
                metadata_json: json!({
                    "module_id": GOOGLE_CALENDAR_MODULE_ID,
                    "import_mode": "bounded_window",
                    "window": {
                        "start": request.window.start,
                        "end": request.window.end,
                    },
                }),
                linked_at: request.imported_at,
                last_seen_at: request.imported_at,
            },
        )
        .await?;

        imported_calendars.push(ImportedGoogleCalendar {
            calendar_id,
            sync_link_id,
            created,
        });
    }

    for event in &request.events {
        if event.end < request.window.start || event.start > request.window.end {
            skipped_outside_window += 1;
            continue;
        }

        let event_id =
            google_event_id(&request.integration_account.id, &event.remote_id).to_string();
        let sync_link_id = google_sync_link_id(
            &request.integration_account.id,
            GOOGLE_EVENT_REMOTE_TYPE,
            &event.remote_id,
        )
        .to_string();
        let existing = get_sync_link(pool, &sync_link_id).await?;
        let created = existing.is_none();

        if created {
            insert_canonical_object(
                pool,
                &map_google_event(
                    &event_id,
                    &request.integration_account.id,
                    event,
                    request.imported_at,
                ),
            )
            .await?;
        }

        upsert_sync_link(
            pool,
            &SyncLinkRecord {
                id: sync_link_id.clone(),
                provider: GOOGLE_CALENDAR_PROVIDER.to_string(),
                integration_account_id: request.integration_account.id.clone(),
                object_id: event_id.clone(),
                remote_id: event.remote_id.clone(),
                remote_type: GOOGLE_EVENT_REMOTE_TYPE.to_string(),
                state: "reconciled".to_string(),
                authority_mode: "shared".to_string(),
                remote_version: event.remote_version.clone(),
                metadata_json: json!({
                    "module_id": GOOGLE_CALENDAR_MODULE_ID,
                    "calendar_remote_id": event.calendar_remote_id,
                    "import_mode": "bounded_window",
                    "window": {
                        "start": request.window.start,
                        "end": request.window.end,
                    },
                }),
                linked_at: request.imported_at,
                last_seen_at: request.imported_at,
            },
        )
        .await?;

        imported_events.push(ImportedGoogleEvent {
            event_id,
            sync_link_id,
            created,
        });
    }

    let updated_account = stamp_account_window(
        request.integration_account.clone(),
        &request.checkpoints,
        &request.window,
        request.imported_at,
    );
    upsert_integration_account(pool, &updated_account).await?;

    Ok(GoogleWindowedImportReport {
        imported_calendars,
        imported_events,
        skipped_outside_window,
    })
}

fn stamp_account_window(
    mut account: IntegrationAccountRecord,
    checkpoints: &GoogleCalendarCheckpointState,
    window: &GoogleImportWindow,
    imported_at: OffsetDateTime,
) -> IntegrationAccountRecord {
    let mut metadata = account.metadata_json;
    if !metadata.is_object() {
        metadata = json!({});
    }

    let JsonValue::Object(ref mut map) = metadata else {
        account.metadata_json = metadata;
        account.updated_at = imported_at;
        return account;
    };

    map.insert(
        "checkpoints".to_string(),
        json!({
            "sync_cursor": checkpoints.sync_cursor,
        }),
    );
    map.insert(
        "window".to_string(),
        json!({
            "bounded": true,
            "start": window.start,
            "end": window.end,
            "past_days": DEFAULT_PAST_DAYS,
            "future_days": DEFAULT_FUTURE_DAYS,
        }),
    );

    account.metadata_json = metadata;
    account.updated_at = imported_at;
    account
}

fn map_google_calendar(
    calendar_id: &str,
    integration_account_id: &str,
    payload: &GoogleCalendarPayload,
    imported_at: OffsetDateTime,
) -> CanonicalObjectRecord {
    CanonicalObjectRecord {
        id: calendar_id.to_string(),
        object_type: "calendar".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({
            "origin": "imported",
            "basis": "google_calendar_windowed_import",
            "source_refs": [
                GOOGLE_CALENDAR_MODULE_ID,
                google_provider_object_ref("calendar", &payload.remote_id),
            ],
        }),
        facets_json: json!({
            "display_name": payload.summary,
            "timezone": payload.timezone,
            "visibility": "private",
            "is_default": payload.is_primary,
            "description": payload.description,
            "color": payload.color,
            "provider_facets": {
                "google_calendar": {
                    "calendar_id": payload.remote_id,
                    "integration_account_id": integration_account_id,
                }
            }
        }),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: imported_at,
        updated_at: imported_at,
    }
}

fn map_google_event(
    event_id: &str,
    integration_account_id: &str,
    payload: &GoogleEventPayload,
    imported_at: OffsetDateTime,
) -> CanonicalObjectRecord {
    CanonicalObjectRecord {
        id: event_id.to_string(),
        object_type: "event".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({
            "origin": "imported",
            "basis": "google_calendar_windowed_import",
            "source_refs": [
                GOOGLE_CALENDAR_MODULE_ID,
                google_provider_object_ref("event", &payload.remote_id),
            ],
        }),
        facets_json: json!({
            "title": payload.summary,
            "description": payload.description,
            "calendar_ref": google_calendar_id(integration_account_id, &payload.calendar_remote_id).to_string(),
            "start": {
                "kind": "zoned_datetime",
                "value": payload.start,
                "timezone": "UTC",
            },
            "end": {
                "kind": "zoned_datetime",
                "value": payload.end,
                "timezone": "UTC",
            },
            "transparency": payload.transparency,
            "provider_facets": {
                "google_calendar": {
                    "event_id": payload.remote_id,
                    "calendar_id": payload.calendar_remote_id,
                    "integration_account_id": integration_account_id,
                }
            }
        }),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: imported_at,
        updated_at: imported_at,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_FUTURE_DAYS, DEFAULT_PAST_DAYS, GoogleCalendarPayload, GoogleEventPayload,
        GoogleImportWindow, GoogleWindowedImportRequest, import_google_window,
    };
    use crate::account_linking::GoogleCalendarCheckpointState;
    use serde_json::json;
    use sqlx::{SqlitePool, migrate::Migrator};
    use time::{Duration, OffsetDateTime};
    use vel_storage::{
        IntegrationAccountRecord, get_canonical_object, get_integration_account,
        list_sync_links_for_object, upsert_integration_account,
    };

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn integration_account(id: &str, external_account_ref: &str) -> IntegrationAccountRecord {
        let now = OffsetDateTime::now_utc();
        IntegrationAccountRecord {
            id: id.to_string(),
            provider: "google-calendar".to_string(),
            display_name: "Google Calendar".to_string(),
            external_account_ref: Some(external_account_ref.to_string()),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "bounded_window".to_string(),
            metadata_json: json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn windowed_import_creates_canonical_calendar_and_event_once_with_bounded_window() {
        let pool = test_pool().await;
        let account = integration_account("integration_account_test", "google_primary");
        upsert_integration_account(&pool, &account).await.unwrap();
        let imported_at = OffsetDateTime::now_utc();
        let window = GoogleImportWindow::bounded_default(imported_at);

        let first = import_google_window(
            &pool,
            &GoogleWindowedImportRequest {
                integration_account: account.clone(),
                calendars: vec![GoogleCalendarPayload {
                    remote_id: "primary".to_string(),
                    summary: "Personal".to_string(),
                    timezone: "America/Denver".to_string(),
                    color: Some("#123456".to_string()),
                    description: None,
                    is_primary: true,
                }],
                events: vec![
                    GoogleEventPayload {
                        remote_id: "evt_inside".to_string(),
                        calendar_remote_id: "primary".to_string(),
                        summary: "Planning".to_string(),
                        description: Some("Inside window".to_string()),
                        start: imported_at + Duration::days(10),
                        end: imported_at + Duration::days(10) + Duration::hours(1),
                        transparency: "opaque".to_string(),
                        remote_version: Some("v1".to_string()),
                    },
                    GoogleEventPayload {
                        remote_id: "evt_outside".to_string(),
                        calendar_remote_id: "primary".to_string(),
                        summary: "Ancient".to_string(),
                        description: None,
                        start: imported_at - Duration::days(DEFAULT_PAST_DAYS + 10),
                        end: imported_at - Duration::days(DEFAULT_PAST_DAYS + 10)
                            + Duration::hours(1),
                        transparency: "transparent".to_string(),
                        remote_version: Some("v1".to_string()),
                    },
                ],
                checkpoints: GoogleCalendarCheckpointState {
                    sync_cursor: Some("sync_v1".to_string()),
                },
                window: window.clone(),
                imported_at,
            },
        )
        .await
        .unwrap();

        let second = import_google_window(
            &pool,
            &GoogleWindowedImportRequest {
                integration_account: account.clone(),
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
                    description: Some("Inside window".to_string()),
                    start: imported_at + Duration::days(10),
                    end: imported_at + Duration::days(10) + Duration::hours(1),
                    transparency: "opaque".to_string(),
                    remote_version: Some("v2".to_string()),
                }],
                checkpoints: GoogleCalendarCheckpointState {
                    sync_cursor: Some("sync_v2".to_string()),
                },
                window,
                imported_at,
            },
        )
        .await
        .unwrap();

        assert!(first.imported_calendars[0].created);
        assert!(first.imported_events[0].created);
        assert_eq!(first.skipped_outside_window, 1);
        assert!(!second.imported_calendars[0].created);
        assert!(!second.imported_events[0].created);

        let stored_calendar = get_canonical_object(&pool, &first.imported_calendars[0].calendar_id)
            .await
            .unwrap()
            .expect("calendar should exist");
        let stored_event = get_canonical_object(&pool, &first.imported_events[0].event_id)
            .await
            .unwrap()
            .expect("event should exist");
        assert_eq!(stored_calendar.object_type, "calendar");
        assert_eq!(stored_event.object_type, "event");
        assert_eq!(
            list_sync_links_for_object(&pool, &stored_event.id)
                .await
                .unwrap()
                .len(),
            1
        );

        let account = get_integration_account(&pool, "integration_account_test")
            .await
            .unwrap()
            .expect("account should exist");
        assert_eq!(account.metadata_json["window"]["bounded"], true);
        assert_eq!(
            account.metadata_json["window"]["past_days"],
            DEFAULT_PAST_DAYS
        );
        assert_eq!(
            account.metadata_json["window"]["future_days"],
            DEFAULT_FUTURE_DAYS
        );
    }
}
