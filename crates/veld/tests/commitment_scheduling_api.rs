use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use time::OffsetDateTime;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::ApiResponse;
use vel_config::AppConfig;
use vel_storage::CommitmentInsert;
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
async fn commitment_scheduling_apply_route_updates_commitment_and_thread_continuity() {
    let state = test_state().await;
    let due_at = OffsetDateTime::from_unix_timestamp(1_774_039_200).expect("due_at");
    let next_due_at = OffsetDateTime::from_unix_timestamp(1_774_046_400).expect("next_due_at");
    let commitment_id = state
        .storage
        .insert_commitment(CommitmentInsert {
            text: "Finish phase 33 contract slice".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todoist_phase33".to_string(),
            status: vel_core::CommitmentStatus::Open,
            due_at: Some(due_at),
            project: Some("Vel".to_string()),
            commitment_kind: Some("todo".to_string()),
            metadata_json: Some(json!({})),
        })
        .await
        .expect("insert commitment")
        .to_string();

    state
        .storage
        .insert_thread(
            "thr_day_plan_apply_1",
            "day_plan_apply",
            "Apply day plan",
            "open",
            &json!({
                "source_kind": "day_plan",
                "proposal_state": "staged",
                "summary": "Apply one bounded same-day scheduling change",
                "requires_confirmation": true,
                "continuity": "thread",
                "mutations": [
                    {
                        "commitment_id": commitment_id,
                        "kind": "set_due_at",
                        "title": "Finish phase 33 contract slice",
                        "summary": "Move this work into the next free same-day window.",
                        "project_label": "Vel",
                        "previous_due_at_ts": 1774039200,
                        "next_due_at_ts": 1774046400
                    }
                ]
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
            "/v1/commitment-scheduling/proposals/thr_day_plan_apply_1/apply",
            None,
        ))
        .await
        .expect("apply response");
    assert_eq!(apply_response.status(), StatusCode::OK);
    let applied: ApiResponse<vel_api_types::CommitmentSchedulingProposalApplyResponseData> =
        decode_json(apply_response).await;
    assert!(applied.ok);
    assert_eq!(
        applied.data.expect("applied data").proposal.state,
        vel_api_types::AssistantProposalStateData::Applied
    );
    let commitment = state
        .storage
        .get_commitment_by_id(&commitment_id)
        .await
        .expect("lookup commitment")
        .expect("commitment exists");
    assert_eq!(commitment.due_at, Some(next_due_at));

    let thread_response = app
        .clone()
        .oneshot(request("GET", "/v1/threads/thr_day_plan_apply_1", None))
        .await
        .expect("thread response");
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_json: serde_json::Value = decode_json(thread_response).await;
    assert_eq!(thread_json["data"]["status"], "resolved");
    assert_eq!(thread_json["data"]["lifecycle_stage"], "applied");
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "applied");
    assert_eq!(
        thread_json["data"]["metadata"]["outcome_summary"],
        "Commitment scheduling proposal applied through canonical mutation seam."
    );
}

#[tokio::test]
async fn commitment_scheduling_apply_route_marks_failed_outcome() {
    let state = test_state().await;
    state
        .storage
        .insert_thread(
            "thr_reflow_apply_missing",
            "reflow_edit",
            "Apply reflow",
            "open",
            &json!({
                "source_kind": "reflow",
                "proposal_state": "staged",
                "summary": "Apply one bounded same-day scheduling repair",
                "requires_confirmation": true,
                "continuity": "thread",
                "mutations": [
                    {
                        "commitment_id": "cmt_missing",
                        "kind": "clear_due_at",
                        "title": "Missing commitment",
                        "summary": "Clear a stale due time."
                    }
                ]
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
            "/v1/commitment-scheduling/proposals/thr_reflow_apply_missing/apply",
            None,
        ))
        .await
        .expect("apply response");
    assert_eq!(apply_response.status(), StatusCode::NOT_FOUND);

    let thread_response = app
        .clone()
        .oneshot(request("GET", "/v1/threads/thr_reflow_apply_missing", None))
        .await
        .expect("thread response");
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_json: serde_json::Value = decode_json(thread_response).await;
    assert_eq!(thread_json["data"]["status"], "open");
    assert_eq!(thread_json["data"]["lifecycle_stage"], "failed");
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "failed");
    assert_eq!(
        thread_json["data"]["metadata"]["failed_via"],
        "commitment_scheduling_apply"
    );
}
