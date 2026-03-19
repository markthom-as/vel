use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use vel_api_types::{
    ApiResponse, DailyLoopPhaseData, DailyLoopSessionData, DailyLoopSessionStateData,
    DailyLoopStartMetadataData, DailyLoopStartRequestData, DailyLoopStartSourceData,
    DailyLoopSurfaceData, DailyLoopTurnActionData, DailyLoopTurnRequestData,
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
    assert!(state.snapshot.contains("Today task"));
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
