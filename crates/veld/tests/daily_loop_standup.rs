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
use vel_storage::{CommitmentInsert, Storage};
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

async fn decode_session(response: axum::response::Response) -> DailyLoopSessionData {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<DailyLoopSessionData> = serde_json::from_slice(&body).unwrap();
    payload.data.expect("daily loop session payload")
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

fn standup_start_request() -> DailyLoopStartRequestData {
    DailyLoopStartRequestData {
        phase: DailyLoopPhaseData::Standup,
        session_date: "2026-03-19".to_string(),
        start: DailyLoopStartMetadataData {
            source: DailyLoopStartSourceData::Manual,
            surface: DailyLoopSurfaceData::Cli,
        },
    }
}

#[tokio::test]
async fn standup_can_resume_from_morning_and_persists_three_commitments_max() {
    let storage = test_storage().await;
    for idx in 0..5 {
        storage
            .insert_commitment(CommitmentInsert {
                text: format!("Backlog item {}", idx + 1),
                source_type: "todoist".to_string(),
                source_id: format!("todo_{}", idx + 1),
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
    }

    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let morning = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &morning_start_request(),
        ))
        .await
        .unwrap();
    let morning = decode_session(morning).await;
    let morning = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", morning.id),
            &DailyLoopTurnRequestData {
                session_id: morning.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Ship Phase 10, review standup calendar".to_string()),
            },
        ))
        .await
        .unwrap();
    let morning = decode_session(morning).await;
    let morning = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", morning.id),
            &DailyLoopTurnRequestData {
                session_id: morning.id.clone(),
                action: DailyLoopTurnActionData::Skip,
                response_text: Some("Need one more pass over the morning constraints".to_string()),
            },
        ))
        .await
        .unwrap();
    let morning = decode_session(morning).await;
    let _morning = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", morning.id),
            &DailyLoopTurnRequestData {
                session_id: morning.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some("Protect focus block".to_string()),
            },
        ))
        .await
        .unwrap();

    let standup = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &standup_start_request(),
        ))
        .await
        .unwrap();
    assert_eq!(standup.status(), StatusCode::OK);
    let standup = decode_session(standup).await;
    let DailyLoopSessionStateData::Standup(state) = standup.state.clone() else {
        panic!("expected standup state");
    };
    assert!(state.commitments.len() <= 3);
    assert!(
        state
            .commitments
            .iter()
            .any(|item| item.title.contains("Ship Phase 10")),
        "morning signal should carry into standup"
    );

    let completed = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", standup.id),
            &DailyLoopTurnRequestData {
                session_id: standup.id.clone(),
                action: DailyLoopTurnActionData::Submit,
                response_text: Some(
                    "Ship Phase 10, Review PRs, Clean inbox, Update docs".to_string(),
                ),
            },
        ))
        .await
        .unwrap();
    let completed = decode_session(completed).await;
    let outcome = serde_json::to_value(completed.outcome.expect("standup outcome")).unwrap();
    assert_eq!(outcome["phase"], "standup");
    assert!(outcome["commitments"].as_array().unwrap().len() <= 3);

    let daily_loop_commitments = storage
        .list_commitments(
            Some(vel_core::CommitmentStatus::Open),
            None,
            Some("daily_loop"),
            16,
        )
        .await
        .unwrap();
    assert!(daily_loop_commitments.len() <= 3);
    assert!(!daily_loop_commitments.is_empty());
}

#[tokio::test]
async fn standup_repompts_once_then_exits_with_typed_outcome_when_no_commitments_are_defined() {
    let storage = test_storage().await;
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let standup = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            "/v1/daily-loop/sessions",
            &standup_start_request(),
        ))
        .await
        .unwrap();
    let standup = decode_session(standup).await;
    assert_eq!(standup.current_prompt.as_ref().unwrap().ordinal, 1);

    let reprompt = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", standup.id),
            &DailyLoopTurnRequestData {
                session_id: standup.id.clone(),
                action: DailyLoopTurnActionData::Skip,
                response_text: Some("Still too vague to commit".to_string()),
            },
        ))
        .await
        .unwrap();
    let reprompt = decode_session(reprompt).await;
    assert_eq!(reprompt.current_prompt.as_ref().unwrap().ordinal, 2);
    let reprompt_json = serde_json::to_value(&reprompt).unwrap();
    assert_eq!(
        reprompt_json["state"]["check_in_history"][0]["kind"],
        "bypassed"
    );

    let complete = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/turn", reprompt.id),
            &DailyLoopTurnRequestData {
                session_id: reprompt.id.clone(),
                action: DailyLoopTurnActionData::Skip,
                response_text: Some("Still blocked on upstream input".to_string()),
            },
        ))
        .await
        .unwrap();
    let complete = decode_session(complete).await;
    assert!(complete.current_prompt.is_none());
    let outcome = serde_json::to_value(complete.outcome.expect("typed standup outcome")).unwrap();
    assert_eq!(outcome["phase"], "standup");
    assert!(outcome["commitments"].as_array().unwrap().is_empty());
    assert!(outcome["deferred_tasks"].is_array());
    assert!(outcome["confirmed_calendar"].is_array());
    assert!(outcome["focus_blocks"].is_array());
    assert_eq!(outcome["check_in_history"][0]["kind"], "bypassed");
    assert_eq!(outcome["check_in_history"][1]["kind"], "bypassed");
}
