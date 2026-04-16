use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tower::util::ServiceExt;
use vel_api_types::{
    ApiResponse, DailyLoopOverdueActionData, DailyLoopOverdueApplyRequestData,
    DailyLoopOverdueApplyResponseData, DailyLoopOverdueConfirmRequestData,
    DailyLoopOverdueConfirmResponseData, DailyLoopOverdueGuessConfidenceData,
    DailyLoopOverdueMenuRequestData, DailyLoopOverdueMenuResponseData,
    DailyLoopOverdueReschedulePayloadData, DailyLoopOverdueUndoRequestData,
    DailyLoopOverdueUndoResponseData, DailyLoopPhaseData, DailyLoopSessionData,
    DailyLoopSessionStateData, DailyLoopStartMetadataData, DailyLoopStartRequestData,
    DailyLoopStartSourceData, DailyLoopSurfaceData, DailyLoopTurnActionData,
    DailyLoopTurnRequestData,
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
    decode_api_response(response).await
}

async fn decode_api_response<T: DeserializeOwned>(response: axum::response::Response) -> T {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<T> = serde_json::from_slice(&body).unwrap();
    payload.data.expect("api response payload")
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

#[tokio::test]
async fn overdue_workflow_rejects_non_standup_sessions_before_action_state() {
    let storage = test_storage().await;
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

    let menu = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/menu", morning.id),
            &DailyLoopOverdueMenuRequestData {
                today: "2026-03-19".to_string(),
                include_vel_guess: true,
                limit: 10,
            },
        ))
        .await
        .unwrap();
    assert_eq!(menu.status(), StatusCode::BAD_REQUEST);

    let confirm = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/confirm", morning.id),
            &DailyLoopOverdueConfirmRequestData {
                commitment_id: "com_missing".to_string(),
                action: DailyLoopOverdueActionData::Close,
                payload: None,
                operator_reason: None,
            },
        ))
        .await
        .unwrap();
    assert_eq!(confirm.status(), StatusCode::BAD_REQUEST);

    let apply = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", morning.id),
            &DailyLoopOverdueApplyRequestData {
                proposal_id: "ovdp_missing".to_string(),
                idempotency_key: "ovd:test:wrong-phase".to_string(),
                confirmation_token: "confirm:ovdp_missing".to_string(),
            },
        ))
        .await
        .unwrap();
    assert_eq!(apply.status(), StatusCode::BAD_REQUEST);

    let undo = app
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/undo", morning.id),
            &DailyLoopOverdueUndoRequestData {
                action_event_id: "ovdevt_missing".to_string(),
                idempotency_key: "ovd:test:wrong-phase".to_string(),
            },
        ))
        .await
        .unwrap();
    assert_eq!(undo.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn overdue_workflow_exposes_bounded_menu_and_applies_each_action_with_run_evidence() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let overdue = now - Duration::days(1);
    let rescheduled_at = (now + Duration::days(2)).replace_nanosecond(0).unwrap();
    let rescheduled = rescheduled_at.format(&Rfc3339).unwrap();
    let mut commitment_ids = Vec::new();
    for title in ["Close me", "Reschedule me", "Back to inbox", "Tombstone me"] {
        let id = storage
            .insert_commitment(CommitmentInsert {
                text: title.to_string(),
                source_type: "todoist".to_string(),
                source_id: title.to_ascii_lowercase().replace(' ', "_"),
                status: vel_core::CommitmentStatus::Open,
                due_at: Some(overdue),
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        commitment_ids.push(id.as_ref().to_string());
    }

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
    assert_eq!(standup.status(), StatusCode::OK);
    let standup = decode_session(standup).await;

    let menu = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/menu", standup.id),
            &DailyLoopOverdueMenuRequestData {
                today: now.date().to_string(),
                include_vel_guess: true,
                limit: 10,
            },
        ))
        .await
        .unwrap();
    assert_eq!(menu.status(), StatusCode::OK);
    let menu: DailyLoopOverdueMenuResponseData = decode_api_response(menu).await;
    assert_eq!(menu.session_id, standup.id);
    assert_eq!(menu.items.len(), 4);
    for item in &menu.items {
        assert_eq!(
            item.actions,
            vec![
                DailyLoopOverdueActionData::Close,
                DailyLoopOverdueActionData::Reschedule,
                DailyLoopOverdueActionData::BackToInbox,
                DailyLoopOverdueActionData::Tombstone,
            ]
        );
        let guess = item.vel_due_guess.as_ref().expect("vel guess");
        assert_eq!(
            guess.confidence,
            DailyLoopOverdueGuessConfidenceData::Medium
        );
        assert!(!guess.reason.trim().is_empty());
    }

    let cases = [
        (
            DailyLoopOverdueActionData::Close,
            commitment_ids[0].as_str(),
            None,
            vel_core::CommitmentStatus::Done,
            true,
        ),
        (
            DailyLoopOverdueActionData::Reschedule,
            commitment_ids[1].as_str(),
            Some(DailyLoopOverdueReschedulePayloadData {
                due_at: rescheduled.clone(),
                source: "operator".to_string(),
            }),
            vel_core::CommitmentStatus::Open,
            false,
        ),
        (
            DailyLoopOverdueActionData::BackToInbox,
            commitment_ids[2].as_str(),
            None,
            vel_core::CommitmentStatus::Open,
            false,
        ),
        (
            DailyLoopOverdueActionData::Tombstone,
            commitment_ids[3].as_str(),
            None,
            vel_core::CommitmentStatus::Cancelled,
            true,
        ),
    ];

    for (idx, (action, commitment_id, payload, expected_status, keeps_due)) in
        cases.into_iter().enumerate()
    {
        let confirm = app
            .clone()
            .oneshot(authed_json_request(
                "POST",
                &format!("/v1/daily-loop/sessions/{}/overdue/confirm", standup.id),
                &DailyLoopOverdueConfirmRequestData {
                    commitment_id: commitment_id.to_string(),
                    action,
                    payload,
                    operator_reason: Some(format!("test case {idx}")),
                },
            ))
            .await
            .unwrap();
        assert_eq!(confirm.status(), StatusCode::OK);
        let confirm: DailyLoopOverdueConfirmResponseData = decode_api_response(confirm).await;
        assert!(confirm.requires_confirmation);
        let expected_scope = match action {
            DailyLoopOverdueActionData::Close | DailyLoopOverdueActionData::Tombstone => {
                format!("commitment:{commitment_id}:status")
            }
            DailyLoopOverdueActionData::Reschedule | DailyLoopOverdueActionData::BackToInbox => {
                format!("commitment:{commitment_id}:due_at")
            }
        };
        assert_eq!(confirm.write_scope, vec![expected_scope]);

        let apply = app
            .clone()
            .oneshot(authed_json_request(
                "POST",
                &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
                &DailyLoopOverdueApplyRequestData {
                    proposal_id: confirm.proposal_id.clone(),
                    idempotency_key: format!("{}:{idx}", confirm.idempotency_hint),
                    confirmation_token: confirm.confirmation_token,
                },
            ))
            .await
            .unwrap();
        assert_eq!(apply.status(), StatusCode::OK);
        let apply: DailyLoopOverdueApplyResponseData = decode_api_response(apply).await;
        assert!(apply.applied);
        assert!(apply.undo_supported);
        assert!(apply.action_event_id.starts_with("ovdevt_"));

        let updated = storage
            .get_commitment_by_id(commitment_id)
            .await
            .unwrap()
            .expect("commitment");
        assert_eq!(updated.status, expected_status);
        match action {
            DailyLoopOverdueActionData::Reschedule => {
                assert_eq!(
                    updated.due_at.unwrap().format(&Rfc3339).unwrap(),
                    rescheduled
                );
            }
            DailyLoopOverdueActionData::BackToInbox => assert!(updated.due_at.is_none()),
            _ if keeps_due => assert!(updated.due_at.is_some()),
            _ => {}
        }

        let run = storage
            .get_run_by_id(&apply.run_id)
            .await
            .unwrap()
            .expect("run");
        assert_eq!(run.status, vel_core::RunStatus::Succeeded);
        let events = storage.list_run_events(&apply.run_id).await.unwrap();
        let event_types = events
            .iter()
            .map(|event| event.event_type)
            .collect::<Vec<_>>();
        assert!(event_types.contains(&vel_core::RunEventType::MutationProposed));
        assert!(event_types.contains(&vel_core::RunEventType::MutationCommitted));
        assert!(event_types.contains(&vel_core::RunEventType::RunSucceeded));
        if action == DailyLoopOverdueActionData::Reschedule {
            let committed = events
                .iter()
                .find(|event| event.event_type == vel_core::RunEventType::MutationCommitted)
                .expect("mutation committed event");
            assert_eq!(committed.payload_json["operator_reason"], "test case 1");
            assert_eq!(committed.payload_json["payload"]["source"], "operator");
        }
    }
}

#[tokio::test]
async fn overdue_apply_requires_confirmation_and_replays_idempotently() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let due_at = now - Duration::hours(3);
    let commitment_id = storage
        .insert_commitment(CommitmentInsert {
            text: "Needs confirmation".to_string(),
            source_type: "todoist".to_string(),
            source_id: "needs_confirmation".to_string(),
            status: vel_core::CommitmentStatus::Open,
            due_at: Some(due_at),
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

    let missing_proposal = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
            &DailyLoopOverdueApplyRequestData {
                proposal_id: "ovdp_missing".to_string(),
                idempotency_key: "ovd:test:missing".to_string(),
                confirmation_token: "confirm:ovdp_missing".to_string(),
            },
        ))
        .await
        .unwrap();
    assert_eq!(missing_proposal.status(), StatusCode::NOT_FOUND);

    let confirm = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/confirm", standup.id),
            &DailyLoopOverdueConfirmRequestData {
                commitment_id: commitment_id.as_ref().to_string(),
                action: DailyLoopOverdueActionData::Close,
                payload: None,
                operator_reason: Some("done externally".to_string()),
            },
        ))
        .await
        .unwrap();
    let confirm: DailyLoopOverdueConfirmResponseData = decode_api_response(confirm).await;

    let invalid_token = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
            &DailyLoopOverdueApplyRequestData {
                proposal_id: confirm.proposal_id.clone(),
                idempotency_key: "ovd:test:invalid-token".to_string(),
                confirmation_token: "confirm:wrong".to_string(),
            },
        ))
        .await
        .unwrap();
    assert_eq!(invalid_token.status(), StatusCode::CONFLICT);
    assert_eq!(
        storage
            .get_commitment_by_id(commitment_id.as_ref())
            .await
            .unwrap()
            .unwrap()
            .status,
        vel_core::CommitmentStatus::Open
    );

    let idempotency_key = "ovd:test:close-once".to_string();
    let apply_request = DailyLoopOverdueApplyRequestData {
        proposal_id: confirm.proposal_id,
        idempotency_key: idempotency_key.clone(),
        confirmation_token: confirm.confirmation_token,
    };
    let first = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
            &apply_request,
        ))
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::OK);
    let first: DailyLoopOverdueApplyResponseData = decode_api_response(first).await;
    let replay = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
            &apply_request,
        ))
        .await
        .unwrap();
    assert_eq!(replay.status(), StatusCode::OK);
    let replay: DailyLoopOverdueApplyResponseData = decode_api_response(replay).await;
    assert_eq!(replay.action_event_id, first.action_event_id);
    assert_eq!(replay.run_id, first.run_id);
    assert_eq!(replay.before.status, first.before.status);
    assert_eq!(replay.after.status, first.after.status);

    let different_key_replay = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
            &DailyLoopOverdueApplyRequestData {
                proposal_id: apply_request.proposal_id,
                idempotency_key: "ovd:test:different-key".to_string(),
                confirmation_token: apply_request.confirmation_token,
            },
        ))
        .await
        .unwrap();
    assert_eq!(different_key_replay.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn overdue_undo_restores_before_state_and_replays_idempotently() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let due_at = now - Duration::hours(5);
    let commitment_id = storage
        .insert_commitment(CommitmentInsert {
            text: "Undo me".to_string(),
            source_type: "todoist".to_string(),
            source_id: "undo_me".to_string(),
            status: vel_core::CommitmentStatus::Open,
            due_at: Some(due_at),
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

    let confirm = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/confirm", standup.id),
            &DailyLoopOverdueConfirmRequestData {
                commitment_id: commitment_id.as_ref().to_string(),
                action: DailyLoopOverdueActionData::BackToInbox,
                payload: None,
                operator_reason: Some("not today".to_string()),
            },
        ))
        .await
        .unwrap();
    let confirm: DailyLoopOverdueConfirmResponseData = decode_api_response(confirm).await;
    let apply = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/apply", standup.id),
            &DailyLoopOverdueApplyRequestData {
                proposal_id: confirm.proposal_id,
                idempotency_key: "ovd:test:back-to-inbox".to_string(),
                confirmation_token: confirm.confirmation_token,
            },
        ))
        .await
        .unwrap();
    let apply: DailyLoopOverdueApplyResponseData = decode_api_response(apply).await;
    assert!(storage
        .get_commitment_by_id(commitment_id.as_ref())
        .await
        .unwrap()
        .unwrap()
        .due_at
        .is_none());

    let undo_request = DailyLoopOverdueUndoRequestData {
        action_event_id: apply.action_event_id.clone(),
        idempotency_key: "ovd:test:undo-back-to-inbox".to_string(),
    };
    let first = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/undo", standup.id),
            &undo_request,
        ))
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::OK);
    let first: DailyLoopOverdueUndoResponseData = decode_api_response(first).await;
    assert!(first.undone);
    assert!(first.before.due_at.is_none());
    assert_eq!(first.after.due_at, apply.before.due_at);

    let replay = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/undo", standup.id),
            &undo_request,
        ))
        .await
        .unwrap();
    assert_eq!(replay.status(), StatusCode::OK);
    let replay: DailyLoopOverdueUndoResponseData = decode_api_response(replay).await;
    assert_eq!(replay.run_id, first.run_id);
    assert_eq!(replay.before.due_at, first.before.due_at);
    assert_eq!(replay.after.due_at, first.after.due_at);
}

#[tokio::test]
async fn overdue_menu_uses_requested_today_boundary_instead_of_server_now_only() {
    let storage = test_storage().await;
    let due_after_requested_today =
        OffsetDateTime::parse("2026-03-20T12:00:00Z", &Rfc3339).unwrap();
    let commitment_id = storage
        .insert_commitment(CommitmentInsert {
            text: "Not overdue yet for requested day".to_string(),
            source_type: "todoist".to_string(),
            source_id: "future_for_requested_day".to_string(),
            status: vel_core::CommitmentStatus::Open,
            due_at: Some(due_after_requested_today),
            project: None,
            commitment_kind: Some("todo".to_string()),
            metadata_json: None,
        })
        .await
        .unwrap();
    let app = build_app(
        storage,
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

    let menu_before_due_day = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/menu", standup.id),
            &DailyLoopOverdueMenuRequestData {
                today: "2026-03-19".to_string(),
                include_vel_guess: false,
                limit: 10,
            },
        ))
        .await
        .unwrap();
    assert_eq!(menu_before_due_day.status(), StatusCode::OK);
    let menu_before_due_day: DailyLoopOverdueMenuResponseData =
        decode_api_response(menu_before_due_day).await;
    assert!(menu_before_due_day.items.is_empty());

    let menu_after_due_day = app
        .clone()
        .oneshot(authed_json_request(
            "POST",
            &format!("/v1/daily-loop/sessions/{}/overdue/menu", standup.id),
            &DailyLoopOverdueMenuRequestData {
                today: "2026-03-21".to_string(),
                include_vel_guess: false,
                limit: 10,
            },
        ))
        .await
        .unwrap();
    assert_eq!(menu_after_due_day.status(), StatusCode::OK);
    let menu_after_due_day: DailyLoopOverdueMenuResponseData =
        decode_api_response(menu_after_due_day).await;
    assert_eq!(menu_after_due_day.items.len(), 1);
    assert_eq!(
        menu_after_due_day.items[0].commitment_id,
        commitment_id.as_ref()
    );
}
