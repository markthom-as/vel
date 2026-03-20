use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::ApiResponse;
use vel_config::AppConfig;
use veld::{app::build_app_with_state, policy_config::PolicyConfig, state::AppState};

async fn test_state() -> AppState {
    let storage = vel_storage::Storage::connect(":memory:")
        .await
        .expect("storage");
    storage.migrate().await.expect("migrations");
    let (broadcast_tx, _) = broadcast::channel(16);
    AppState::new(
        storage,
        AppConfig::default(),
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

fn request(method: &str, uri: &str, body: Option<serde_json::Value>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    builder
        .body(match body {
            Some(body) => Body::from(body.to_string()),
            None => Body::empty(),
        })
        .expect("request")
}

async fn decode_json<T: DeserializeOwned>(response: axum::response::Response) -> T {
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice::<T>(&body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for status {}: {} ({})",
            status,
            String::from_utf8_lossy(&body),
            error
        )
    })
}

#[tokio::test]
async fn planning_profile_route_reads_and_mutates_canonical_profile() {
    let state = test_state().await;
    let app = build_app_with_state(state);

    let initial_response = app
        .clone()
        .oneshot(request("GET", "/v1/planning-profile", None))
        .await
        .expect("initial response");
    assert_eq!(initial_response.status(), StatusCode::OK);
    let initial: ApiResponse<vel_api_types::PlanningProfileResponseData> =
        decode_json(initial_response).await;
    assert!(initial.ok);
    assert_eq!(
        initial
            .data
            .expect("initial profile")
            .profile
            .routine_blocks
            .len(),
        0
    );

    let patch_response = app
        .clone()
        .oneshot(request(
            "PATCH",
            "/v1/planning-profile",
            Some(json!({
                "mutation": {
                    "kind": "upsert_routine_block",
                    "data": {
                        "id": "routine_deep_work",
                        "label": "Deep work",
                        "source": "operator_declared",
                        "local_timezone": "America/Denver",
                        "start_local_time": "09:00",
                        "end_local_time": "11:00",
                        "days_of_week": [1, 2, 3, 4, 5],
                        "protected": true,
                        "active": true
                    }
                }
            })),
        ))
        .await
        .expect("patch response");
    assert_eq!(patch_response.status(), StatusCode::OK);
    let patched: ApiResponse<vel_api_types::PlanningProfileResponseData> =
        decode_json(patch_response).await;
    assert!(patched.ok);
    let patched_profile = patched.data.expect("patched profile").profile;
    assert_eq!(patched_profile.routine_blocks.len(), 1);
    assert_eq!(patched_profile.routine_blocks[0].id, "routine_deep_work");
    assert_eq!(
        patched_profile.routine_blocks[0].source,
        vel_api_types::RoutineBlockSourceKindData::OperatorDeclared
    );

    let updated_response = app
        .clone()
        .oneshot(request("GET", "/v1/planning-profile", None))
        .await
        .expect("updated response");
    assert_eq!(updated_response.status(), StatusCode::OK);
    let updated: ApiResponse<vel_api_types::PlanningProfileResponseData> =
        decode_json(updated_response).await;
    assert!(updated.ok);
    let updated_profile = updated.data.expect("updated profile").profile;
    assert_eq!(updated_profile.routine_blocks.len(), 1);
    assert_eq!(updated_profile.routine_blocks[0].label, "Deep work");
}

#[tokio::test]
async fn planning_profile_proposal_apply_route_updates_profile_and_thread_continuity() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());

    let assistant_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/api/assistant/entry",
            Some(json!({
                "text": "add a protected shutdown block from 17:00 to 17:30 in America/Denver on weekdays"
            })),
        ))
        .await
        .expect("assistant response");
    assert_eq!(assistant_response.status(), StatusCode::OK);
    let assistant_json: serde_json::Value = decode_json(assistant_response).await;
    let thread_id = assistant_json["data"]["planning_profile_proposal"]["thread_id"]
        .as_str()
        .expect("thread id");

    let apply_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/planning-profile/proposals/{thread_id}/apply"),
            None,
        ))
        .await
        .expect("apply response");
    assert_eq!(apply_response.status(), StatusCode::OK);
    let applied: ApiResponse<vel_api_types::PlanningProfileProposalApplyResponseData> =
        decode_json(apply_response).await;
    assert!(applied.ok);
    let applied_data = applied.data.expect("applied data");
    assert_eq!(
        applied_data.proposal.state,
        vel_api_types::AssistantProposalStateData::Applied
    );
    assert_eq!(applied_data.profile.routine_blocks.len(), 1);
    assert_eq!(
        applied_data.profile.routine_blocks[0].id,
        "routine_shutdown"
    );

    let thread_response = app
        .clone()
        .oneshot(request("GET", &format!("/v1/threads/{thread_id}"), None))
        .await
        .expect("thread response");
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_json: serde_json::Value = decode_json(thread_response).await;
    assert_eq!(thread_json["data"]["status"], "resolved");
    assert_eq!(thread_json["data"]["lifecycle_stage"], "applied");
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "applied");
    assert_eq!(
        thread_json["data"]["metadata"]["outcome_summary"],
        "Planning-profile proposal applied through canonical mutation seam."
    );

    let profile_response = app
        .clone()
        .oneshot(request("GET", "/v1/planning-profile", None))
        .await
        .expect("profile response");
    assert_eq!(profile_response.status(), StatusCode::OK);
    let profile_json: serde_json::Value = decode_json(profile_response).await;
    assert_eq!(profile_json["data"]["proposal_summary"]["pending_count"], 0);
    assert_eq!(
        profile_json["data"]["proposal_summary"]["latest_applied"]["thread_id"],
        thread_id
    );
}

#[tokio::test]
async fn planning_profile_proposal_apply_route_marks_failed_outcome() {
    let state = test_state().await;
    state
        .storage
        .insert_thread(
            "thr_planning_profile_edit_missing",
            "planning_profile_edit",
            "Remove missing block",
            "open",
            &json!({
                "source": "planning_profile_proposal",
                "proposal_state": "staged",
                "summary": "Remove missing block",
                "requires_confirmation": true,
                "continuity": "thread",
                "mutation": {
                    "kind": "remove_routine_block",
                    "data": {
                        "id": "missing"
                    }
                },
                "lineage": {
                    "source_surface": "assistant"
                }
            })
            .to_string(),
        )
        .await
        .expect("insert thread");

    let app = build_app_with_state(state.clone());
    let apply_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/planning-profile/proposals/thr_planning_profile_edit_missing/apply",
            None,
        ))
        .await
        .expect("apply response");
    assert_eq!(apply_response.status(), StatusCode::NOT_FOUND);

    let thread_response = app
        .clone()
        .oneshot(request(
            "GET",
            "/v1/threads/thr_planning_profile_edit_missing",
            None,
        ))
        .await
        .expect("thread response");
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_json: serde_json::Value = decode_json(thread_response).await;
    assert_eq!(thread_json["data"]["status"], "open");
    assert_eq!(thread_json["data"]["lifecycle_stage"], "failed");
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "failed");
    assert_eq!(
        thread_json["data"]["metadata"]["outcome_summary"],
        "routine block missing not found"
    );
}
