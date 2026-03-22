use chrono::{TimeZone, Utc};
use serde_json::Value;
use sqlx::SqlitePool;
use vel_core::{
    AllDayHandlingRule, AvailabilityPolicyConfig, AvailabilityResult, Calendar, CalendarId,
    CalendarVisibility, DeclinedResponsePolicy, Event, EventId, EventLocation, EventMoment,
    EventMomentKind, EventTransparency, ParticipationResponseStatus,
};
use vel_storage::{get_projection, migrate_storage, query_canonical_objects, CanonicalObjectQuery};
use veld::services::{
    availability_projection::{AvailabilityEventInput, AvailabilityProjectionService},
    calendar_explain::CalendarExplainService,
};

fn calendar(id: &str) -> Calendar {
    Calendar {
        id: CalendarId::from(id.to_string()),
        display_name: "Personal".to_string(),
        timezone: "UTC".to_string(),
        visibility: CalendarVisibility::Private,
        is_default: true,
        description: None,
        color: None,
    }
}

fn event(
    id: &str,
    start: &str,
    end: &str,
    transparency: EventTransparency,
    all_day: bool,
) -> Event {
    let kind = if all_day {
        EventMomentKind::AllDay
    } else {
        EventMomentKind::ZonedDateTime
    };

    Event {
        id: EventId::from(id.to_string()),
        title: "Focus block".to_string(),
        description: None,
        start: EventMoment {
            kind: kind.clone(),
            value: start.to_string(),
            timezone: if all_day { None } else { Some("UTC".to_string()) },
        },
        end: EventMoment {
            kind,
            value: end.to_string(),
            timezone: if all_day { None } else { Some("UTC".to_string()) },
        },
        transparency,
        location: Some(EventLocation {
            label: "Desk".to_string(),
            address: None,
            notes: None,
            uri: None,
        }),
    }
}

fn config() -> AvailabilityPolicyConfig {
    AvailabilityPolicyConfig {
        included_calendar_ids: vec![CalendarId::from("calendar_personal".to_string())],
        source_account_ids: vec!["integration_account_google".to_string()],
        declined_response_policy: DeclinedResponsePolicy::IgnoreDeclined,
        exclude_cancelled_events: true,
        all_day_handling_rule: AllDayHandlingRule::RespectTransparency,
    }
}

#[tokio::test]
async fn availability_derives_from_canonical_state_and_stays_projection_only() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();

    let availability = AvailabilityProjectionService::project(
        Utc.with_ymd_and_hms(2026, 3, 23, 8, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 3, 23, 9, 0, 0).unwrap(),
        &config(),
        &[
            AvailabilityEventInput {
                calendar: calendar("calendar_personal"),
                event: event(
                    "event_busy",
                    "2026-03-23T08:15:00Z",
                    "2026-03-23T08:45:00Z",
                    EventTransparency::Opaque,
                    false,
                ),
                source_account_id: Some("integration_account_google".to_string()),
                response_status: Some(ParticipationResponseStatus::Accepted),
                cancelled: false,
            },
            AvailabilityEventInput {
                calendar: calendar("calendar_personal"),
                event: event(
                    "event_declined",
                    "2026-03-23T08:10:00Z",
                    "2026-03-23T08:20:00Z",
                    EventTransparency::Opaque,
                    false,
                ),
                source_account_id: Some("integration_account_google".to_string()),
                response_status: Some(ParticipationResponseStatus::Declined),
                cancelled: false,
            },
            AvailabilityEventInput {
                calendar: calendar("calendar_personal"),
                event: event(
                    "event_transparent_all_day",
                    "2026-03-23",
                    "2026-03-23",
                    EventTransparency::Transparent,
                    true,
                ),
                source_account_id: Some("integration_account_google".to_string()),
                response_status: None,
                cancelled: false,
            },
        ],
    )
    .unwrap();

    assert_eq!(availability.result, AvailabilityResult::Busy);
    assert_eq!(availability.blocking_intervals.len(), 1);
    assert_eq!(availability.blocking_intervals[0].event_id, "event_busy");

    AvailabilityProjectionService::materialize(
        &pool,
        "projection.availability.2026-03-23T08:00:00Z",
        &availability,
    )
    .await
    .unwrap();

    let stored = get_projection(&pool, "projection.availability.2026-03-23T08:00:00Z")
        .await
        .unwrap()
        .expect("availability projection should exist");
    let canonical = query_canonical_objects(
        &pool,
        &CanonicalObjectQuery {
            object_type: Some("availability".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(stored.projection_type, "availability");
    assert!(canonical.is_empty(), "availability must not become canonical content");
}

#[test]
fn availability_explain_reports_basis_sources_filters_and_decision() {
    let availability = AvailabilityProjectionService::project(
        Utc.with_ymd_and_hms(2026, 3, 23, 8, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 3, 23, 9, 0, 0).unwrap(),
        &config(),
        &[AvailabilityEventInput {
            calendar: calendar("calendar_personal"),
            event: event(
                "event_busy",
                "2026-03-23T08:15:00Z",
                "2026-03-23T08:45:00Z",
                EventTransparency::Opaque,
                false,
            ),
            source_account_id: Some("integration_account_google".to_string()),
            response_status: Some(ParticipationResponseStatus::Accepted),
            cancelled: false,
        }],
    )
    .unwrap();

    let explain = CalendarExplainService::explain_availability(
        &availability,
        "2026-03-23T08:00:00Z",
        "2026-03-23T09:00:00Z",
    );

    assert_eq!(explain["accepted"], Value::Bool(false));
    assert_eq!(explain["basis"], "exact");
    assert!(explain["sources_consulted"].as_array().unwrap().len() == 1);
    assert!(!explain["filters_applied"].as_array().unwrap().is_empty());
    assert!(explain["decision_reason"].as_str().unwrap().contains("rejected"));
}
