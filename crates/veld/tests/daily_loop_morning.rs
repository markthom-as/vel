use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use vel_api_types::{
    ApiResponse, DailyLoopCheckInEventData, DailyLoopCheckInSkipRequestData,
    DailyLoopCheckInSkipResponseData, DailyLoopCheckInSubmitRequestData,
    DailyLoopCheckInSubmitResponseData, DailyLoopPhaseData, DailyLoopSessionData,
    DailyLoopSessionStateData, DailyLoopStartMetadataData, DailyLoopStartRequestData,
    DailyLoopStartSourceData, DailyLoopSurfaceData, DailyLoopTurnActionData,
    DailyLoopTurnRequestData,
};
use vel_config::AppConfig;
use vel_core::CommitmentStatus;
use vel_storage::{CommitmentInsert, SignalInsert, Storage};
use veld::{app::build_app, policy_config::PolicyConfig};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn test_policy_config() -> PolicyConfig {
    PolicyConfig::default()
}

async fn test_storage() -> Storage {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
}

fn authed_json_request<T: serde::Serialize>(method: &str, uri: &str, payload: &T) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .body(Body::from(serde_json::to_vec(payload).unwrap()))
        .unwrap()
}

fn authed_get(uri: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .body(Body::empty())
        .unwrap()
}

async fn decode_session(response: axum::response::Response) -> DailyLoopSessionData {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<DailyLoopSessionData> = serde_json::from_slice(&body).unwrap();
    payload.data.expect("daily loop session payload")
}

async fn decode_optional_session(
    response: axum::response::Response,
) -> Option<DailyLoopSessionData> {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<Option<DailyLoopSessionData>> = serde_json::from_slice(&body).unwrap();
    payload.data.flatten()
}

async fn decode_check_in_events(
    response: axum::response::Response,
) -> Vec<DailyLoopCheckInEventData> {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<Vec<DailyLoopCheckInEventData>> =
        serde_json::from_slice(&body).unwrap();
    payload.data.unwrap_or_default()
}

async fn decode_check_in_submit(
    response: axum::response::Response,
) -> DailyLoopCheckInSubmitResponseData {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<DailyLoopCheckInSubmitResponseData> =
        serde_json::from_slice(&body).unwrap();
    payload.data.expect("check-in submit payload")
}

async fn decode_check_in_skip(
    response: axum::response::Response,
) -> DailyLoopCheckInSkipResponseData {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<DailyLoopCheckInSkipResponseData> =
        serde_json::from_slice(&body).unwrap();
    payload.data.expect("check-in skip payload")
}

fn morning_start_request() -> DailyLoopStartRequestData {
    DailyLoopStartRequestData {
        phase: DailyLoopPhaseData::MorningOverview,
        session_date: "2026-03-19".to_string(),
        start: DailyLoopStartMetadataData {
            source: DailyLoopStartSourceData::Manual,
            surface: DailyLoopSurfaceData::Cli,
        },
    }
}

#[tokio::test]
async fn morning_start_uses_bounded_inputs_and_creates_resumable_session() {
    let storage = test_storage().await;
    let now = time::OffsetDateTime::now_utc();
    storage
        .insert_signal(SignalInsert {
            signal_type: "calendar_event".to_string(),
            source: "google_calendar".to_string(),
            source_ref: Some("evt_soon".to_string()),
            timestamp: (now + time::Duration::hours(2)).unix_timestamp(),
            payload_json: Some(serde_json::json!({
                "event_id": "evt_soon",
                "title": "Design Review",
                "start": (now + time::Duration::hours(2)).unix_timestamp(),
                "end": (now + time::Duration::hours(3)).unix_timestamp()
            })),
        })
        .await
        .unwrap();
    storage
        .insert_signal(SignalInsert {
            signal_type: "calendar_event".to_string(),
            source: "google_calendar".to_string(),
            source_ref: Some("evt_late".to_string()),
            timestamp: (now + time::Duration::hours(13)).unix_timestamp(),
            payload_json: Some(serde_json::json!({
                "event_id": "evt_late",
                "title": "Too Late Event",
                "start": (now + time::Duration::hours(13)).unix_timestamp(),
                "end": (now + time::Duration::hours(14)).unix_timestamp()
            })),
        })
        .await
        .unwrap();
    storage
        .insert_commitment(CommitmentInsert {
            text: "Today task".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todo_today".to_string(),
            status: CommitmentStatus::Open,
            due_at: Some(now + time::Duration::hours(1)),
            project: None,
            commitment_kind: Some("todo".to_string()),
            metadata_json: None,
        })
        .await
        .unwrap();
    storage
        .insert_commitment(CommitmentInsert {
            text: "Overdue task".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todo_overdue".to_string(),
            status: CommitmentStatus::Open,
            due_at: Some(now - time::Duration::days(1)),
            project: None,
            commitment_kind: Some("todo".to_string()),
            metadata_json: None,
        })
        .await
        .unwrap();
    storage
        .insert_commitment(CommitmentInsert {
            text: "Tomorrow task".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todo_tomorrow".to_string(),
            status: CommitmentStatus::Open,
            due_at: Some(now + time::Duration::days(1)),
            project: None,
            commitment_kind: Some("todo".to_string()),
            metadata_json: None,
        })
        .await
        .unwrap();

    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );
    let response = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let session = decode_session(response).await;
    let DailyLoopSessionStateData::MorningOverview(state) = session.state.clone() else {
        panic!("expected morning state");
    };
    assert!(state.snapshot.contains("Design Review"));
    assert!(state.snapshot.contains("Overdue task"));
    assert!(!state.snapshot.contains("Too Late Event"));
    assert!(!state.snapshot.contains("Tomorrow task"));
    assert!(state.friction_callouts.len() <= 2);

    let active = app
        .oneshot(authed_get(
            "/v1/daily-loop/sessions/active?session_date=2026-03-19&phase=morning_overview",
        ))
        .await
        .unwrap();
    let active_session = decode_optional_session(active)
        .await
        .expect("active session");
    assert_eq!(active_session.id, session.id);
    assert_eq!(
        active_session
            .current_prompt
            .expect("current prompt")
            .ordinal,
        1
    );
}

#[tokio::test]
async fn morning_turns_stay_bounded_and_resume_without_creating_commitments() {
    let storage = test_storage().await;
    let initial_commitments = storage
        .list_commitments(None, None, None, 64)
        .await
        .unwrap()
        .len();
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let skip_one = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Skip,
                response_text: Some("Need to confirm the noon dependency first".to_string()),
            },
        ))
        .await
        .unwrap();
    let session = decode_session(skip_one).await;
    assert_eq!(
        session.current_prompt.as_ref().expect("prompt 2").ordinal,
        2
    );
    let session_json = serde_json::to_value(&session).unwrap();
    assert_eq!(
        session_json["state"]["check_in_history"][0]["kind"],
        "bypassed"
    );
    assert_eq!(
        session_json["state"]["check_in_history"][0]["note_text"],
        "Need to confirm the noon dependency first"
    );

    let resume = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Resume,
                response_text: None,
            },
        ))
        .await
        .unwrap();
    let session = decode_session(resume).await;
    assert_eq!(session.current_prompt.expect("prompt 2 resume").ordinal, 2);

    let submit = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Protect focus for implementation".to_string()),
            },
        ))
        .await
        .unwrap();
    let session = decode_session(submit).await;
    assert_eq!(session.current_prompt.expect("prompt 3").ordinal, 3);

    let complete = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Meeting prep might slip".to_string()),
            },
        ))
        .await
        .unwrap();
    let session = decode_session(complete).await;
    assert!(session.current_prompt.is_none());
    assert_eq!(format!("{:?}", session.status), "Completed");
    let outcome_json = serde_json::to_value(session.outcome.expect("morning outcome")).unwrap();
    assert_eq!(outcome_json["phase"], "morning_overview");
    assert!(outcome_json.get("commitments").is_none());
    assert_eq!(outcome_json["check_in_history"][0]["kind"], "bypassed");
    assert_eq!(outcome_json["check_in_history"][1]["kind"], "submitted");

    let final_commitments = storage
        .list_commitments(None, None, None, 64)
        .await
        .unwrap()
        .len();
    assert_eq!(final_commitments, initial_commitments);

    let active = app
        .oneshot(authed_get(
            "/v1/daily-loop/sessions/active?session_date=2026-03-19&phase=morning_overview",
        ))
        .await
        .unwrap();
    assert!(decode_optional_session(active).await.is_none());
}

#[tokio::test]
async fn morning_bypass_requires_operator_note() {
    let storage = test_storage().await;
    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let response = app
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Skip,
                response_text: None,
            },
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn morning_check_in_events_filter_defaults_and_skipped_inclusion() {
    let storage = test_storage().await;
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let _ = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Skip,
                response_text: Some("Need prep first".to_string()),
            },
        ))
        .await
        .unwrap();

    let _ = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Focus on release notes".to_string()),
            },
        ))
        .await
        .unwrap();

    let _ = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Blocker triage done".to_string()),
            },
        ))
        .await
        .unwrap();

    let all_events = app
        .clone()
        .oneshot(authed_get(&format!(
            "/v1/daily-loop/sessions/{}/check-ins",
            session.id
        )))
        .await
        .unwrap();
    let all_events = decode_check_in_events(all_events).await;
    assert_eq!(all_events.len(), 2);
    assert!(all_events.iter().all(|event| !event.skipped));

    let include_skipped = app
        .clone()
        .oneshot(authed_get(&format!(
            "/v1/daily-loop/sessions/{}/check-ins?include_skipped=true",
            session.id
        )))
        .await
        .unwrap();
    let include_skipped = decode_check_in_events(include_skipped).await;
    assert_eq!(include_skipped.len(), 3);
    assert!(include_skipped.iter().any(|event| event.skipped));

    let morning_only = app
        .clone()
        .oneshot(authed_get(&format!(
            "/v1/daily-loop/sessions/{}/check-ins?session_phase=morning",
            session.id
        )))
        .await
        .unwrap();
    assert_eq!(decode_check_in_events(morning_only).await.len(), 2);

    let standup_only = app
        .clone()
        .oneshot(authed_get(&format!(
            "/v1/daily-loop/sessions/{}/check-ins?session_phase=standup",
            session.id
        )))
        .await
        .unwrap();
    assert!(decode_check_in_events(standup_only).await.is_empty());

    let check_in_count = storage
        .list_daily_check_in_events_for_session(&session.id, None, None, true, 50)
        .await
        .unwrap()
        .len();
    assert_eq!(check_in_count, 3);
}

#[tokio::test]
async fn list_session_check_in_events_respects_check_in_type_filter() {
    let storage = test_storage().await;
    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let _ = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Mood: energized".to_string()),
            },
        ))
        .await
        .unwrap();

    let _ = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Plan review notes".to_string()),
            },
        ))
        .await
        .unwrap();

    let _ = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", session.id),
            &DailyLoopTurnRequestData {
                session_id: session.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Meeting prep complete".to_string()),
            },
        ))
        .await
        .unwrap();

    let mood = app
        .clone()
        .oneshot(authed_get(&format!(
            "/v1/daily-loop/sessions/{}/check-ins?check_in_type=mood&include_skipped=true",
            session.id
        )))
        .await
        .unwrap();
    assert_eq!(decode_check_in_events(mood).await.len(), 1);

    let other = app
        .clone()
        .oneshot(authed_get(&format!(
            "/v1/daily-loop/sessions/{}/check-ins?check_in_type=other&include_skipped=true",
            session.id
        )))
        .await
        .unwrap();
    assert_eq!(decode_check_in_events(other).await.len(), 2);
}

#[tokio::test]
async fn list_session_check_in_events_returns_not_found_for_missing_session() {
    let app = build_app(
        test_storage().await,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let response = app
        .oneshot(authed_get(
            "/v1/daily-loop/sessions/missing_session/check-ins",
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn submit_check_in_creates_event() {
    let storage = test_storage().await;
    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let response = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/check-ins", session.id),
            &DailyLoopCheckInSubmitRequestData {
                check_in_type: "mood".to_string(),
                session_phase: "morning".to_string(),
                source: "user".to_string(),
                prompt_id: "manual_mood_01".to_string(),
                answered_at: None,
                text: Some("Sleep was rough but productive".to_string()),
                scale: Some(-2),
                keywords: vec!["sleep".to_string(), "focus".to_string()],
                confidence: Some(0.84),
                skipped: false,
                skip_reason_code: None,
                skip_reason_text: None,
                replace_if_conflict: false,
            },
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let result = decode_check_in_submit(response).await;
    assert_eq!(result.status, "recorded");
    assert!(result.supersedes_event_id.is_none());

    let events = decode_check_in_events(
        app.clone()
            .oneshot(authed_get(&format!(
                "/v1/daily-loop/sessions/{}/check-ins",
                session.id
            )))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].check_in_type, "mood");
    assert_eq!(
        events[0].text.as_deref(),
        Some("Sleep was rough but productive")
    );
}

#[tokio::test]
async fn submit_check_in_can_replace_latest_matching_event() {
    let storage = test_storage().await;
    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let first = decode_check_in_submit(
        app.clone()
            .oneshot(authed_json_request(
                "POST",
                &format!("/v1/daily-loop/sessions/{}/check-ins", session.id),
                &DailyLoopCheckInSubmitRequestData {
                    check_in_type: "pain".to_string(),
                    session_phase: "morning".to_string(),
                    source: "user".to_string(),
                    prompt_id: "morning_pain_01".to_string(),
                    answered_at: None,
                    text: Some("Upper back tension".to_string()),
                    scale: Some(-6),
                    keywords: vec!["neck".to_string()],
                    confidence: Some(0.78),
                    skipped: false,
                    skip_reason_code: None,
                    skip_reason_text: None,
                    replace_if_conflict: false,
                },
            ))
            .await
            .unwrap(),
    )
    .await;

    let second = decode_check_in_submit(
        app.clone()
            .oneshot(authed_json_request(
                "POST",
                &format!("/v1/daily-loop/sessions/{}/check-ins", session.id),
                &DailyLoopCheckInSubmitRequestData {
                    check_in_type: "pain".to_string(),
                    session_phase: "morning".to_string(),
                    source: "user".to_string(),
                    prompt_id: "morning_pain_01".to_string(),
                    answered_at: None,
                    text: Some("Midday pain spike".to_string()),
                    scale: Some(-4),
                    keywords: vec!["midday".to_string()],
                    confidence: Some(0.82),
                    skipped: false,
                    skip_reason_code: None,
                    skip_reason_text: None,
                    replace_if_conflict: true,
                },
            ))
            .await
            .unwrap(),
    )
    .await;

    assert_eq!(
        second.supersedes_event_id,
        Some(first.check_in_event_id.clone())
    );

    let events = decode_check_in_events(
        app.clone()
            .oneshot(authed_get(&format!(
                "/v1/daily-loop/sessions/{}/check-ins",
                session.id
            )))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(events.len(), 2);
    assert_eq!(
        events[0].replaced_by_event_id,
        Some(first.check_in_event_id)
    );
}

#[tokio::test]
async fn submit_check_in_rejects_missing_session_or_phase_mismatch() {
    let app = build_app(
        test_storage().await,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let mismatch = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/check-ins", session.id),
            &DailyLoopCheckInSubmitRequestData {
                check_in_type: "mood".to_string(),
                session_phase: "standup".to_string(),
                source: "user".to_string(),
                prompt_id: "manual_01".to_string(),
                answered_at: None,
                text: Some("No check-in".to_string()),
                scale: None,
                keywords: vec![],
                confidence: None,
                skipped: false,
                skip_reason_code: None,
                skip_reason_text: None,
                replace_if_conflict: false,
            },
        ))
        .await
        .unwrap();
    assert_eq!(mismatch.status(), StatusCode::BAD_REQUEST);

    let missing_session = app
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions/missing_session/check-ins",
            &DailyLoopCheckInSubmitRequestData {
                check_in_type: "mood".to_string(),
                session_phase: "morning".to_string(),
                source: "user".to_string(),
                prompt_id: "manual_01".to_string(),
                answered_at: None,
                text: Some("No check-in".to_string()),
                scale: None,
                keywords: vec![],
                confidence: None,
                skipped: false,
                skip_reason_code: None,
                skip_reason_text: None,
                replace_if_conflict: false,
            },
        ))
        .await
        .unwrap();
    assert_eq!(missing_session.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn skip_check_in_creates_reasoned_skip_event() {
    let storage = test_storage().await;
    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let original = decode_check_in_submit(
        app.clone()
            .oneshot(authed_json_request(
                "POST",
                &format!("/v1/daily-loop/sessions/{}/check-ins", session.id),
                &DailyLoopCheckInSubmitRequestData {
                    check_in_type: "mood".to_string(),
                    session_phase: "morning".to_string(),
                    source: "user".to_string(),
                    prompt_id: "manual_mood_01".to_string(),
                    answered_at: None,
                    text: Some("Steady but tired".to_string()),
                    scale: Some(-1),
                    keywords: vec!["tired".to_string()],
                    confidence: Some(0.8),
                    skipped: false,
                    skip_reason_code: None,
                    skip_reason_text: None,
                    replace_if_conflict: false,
                },
            ))
            .await
            .unwrap(),
    )
    .await;

    let skip_result = decode_check_in_skip(
        app.clone()
            .oneshot(authed_json_request(
                "POST",
                &format!(
                    "/v1/daily-loop/check-ins/{}/skip",
                    original.check_in_event_id
                ),
                &DailyLoopCheckInSkipRequestData {
                    source: Some("user".to_string()),
                    answered_at: None,
                    reason_code: Some("not_now".to_string()),
                    reason_text: Some("Need to postpone".to_string()),
                },
            ))
            .await
            .unwrap(),
    )
    .await;

    assert_eq!(skip_result.status, "skipped");
    assert_eq!(
        skip_result.supersedes_event_id,
        Some(original.check_in_event_id.clone())
    );
    assert_eq!(skip_result.session_id, session.id);

    let events = decode_check_in_events(
        app.clone()
            .oneshot(authed_get(&format!(
                "/v1/daily-loop/sessions/{}/check-ins?include_skipped=true",
                session.id
            )))
            .await
            .unwrap(),
    )
    .await;

    assert_eq!(events.len(), 2);
    assert!(events[0].skipped);
    assert_eq!(
        events[0].replaced_by_event_id,
        Some(original.check_in_event_id)
    );
    assert_eq!(events[0].skip_reason_code.as_deref(), Some("not_now"));
    assert_eq!(
        events[0].skip_reason_text.as_deref(),
        Some("Need to postpone")
    );
}

#[tokio::test]
async fn skip_check_in_requires_reason() {
    let storage = test_storage().await;
    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let started = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let session = decode_session(started).await;

    let original = decode_check_in_submit(
        app.clone()
            .oneshot(authed_json_request(
                "POST",
                &format!("/v1/daily-loop/sessions/{}/check-ins", session.id),
                &DailyLoopCheckInSubmitRequestData {
                    check_in_type: "body".to_string(),
                    session_phase: "morning".to_string(),
                    source: "user".to_string(),
                    prompt_id: "manual_body_01".to_string(),
                    answered_at: None,
                    text: Some("Tightness in shoulders".to_string()),
                    scale: Some(-3),
                    keywords: vec!["shoulder".to_string()],
                    confidence: Some(0.7),
                    skipped: false,
                    skip_reason_code: None,
                    skip_reason_text: None,
                    replace_if_conflict: false,
                },
            ))
            .await
            .unwrap(),
    )
    .await;

    let response = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!(
                "/v1/daily-loop/check-ins/{}/skip",
                original.check_in_event_id
            ),
            &DailyLoopCheckInSkipRequestData {
                source: Some("user".to_string()),
                answered_at: None,
                reason_code: None,
                reason_text: None,
            },
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn skip_check_in_missing_event_returns_not_found() {
    let app = build_app(
        test_storage().await,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let response = app
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/check-ins/dci_missing/skip",
            &DailyLoopCheckInSkipRequestData {
                source: Some("user".to_string()),
                answered_at: None,
                reason_code: Some("not_now".to_string()),
                reason_text: Some("Busy".to_string()),
            },
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
