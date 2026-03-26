use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::json;
use time::OffsetDateTime;
use tower::util::ServiceExt;
use vel_config::AppConfig;
use vel_core::{context::CurrentContextTaskLanes, CommitmentStatus, CurrentContextV1};
use vel_storage::{CommitmentInsert, Storage};
use veld::{app::build_app, policy_config::PolicyConfig};

fn request(uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("PATCH")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn todoist_lane_completion_fails_closed_when_writeback_is_unavailable() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let commitment_id = storage
        .insert_commitment(CommitmentInsert {
            text: "Ship patch".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todoist_task_1".to_string(),
            status: CommitmentStatus::Open,
            due_at: None,
            project: Some("vel".to_string()),
            commitment_kind: Some("todo".to_string()),
            metadata_json: Some(json!({ "todoist_id": "task_1" })),
        })
        .await
        .unwrap();
    storage
        .set_current_context(
            OffsetDateTime::now_utc().unix_timestamp(),
            &serde_json::to_string(&CurrentContextV1 {
                task_lanes: CurrentContextTaskLanes {
                    active_commitment_ids: vec![commitment_id.as_ref().to_string()],
                    ..Default::default()
                },
                ..Default::default()
            })
            .unwrap(),
        )
        .await
        .unwrap();

    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let response = app
        .oneshot(request(
            "/v1/now/task-lane",
            json!({
                "commitment_id": commitment_id,
                "lane": "completed",
                "position": null
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let payload: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert!(payload["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("writeback is disabled by default"));

    let stored = storage
        .get_commitment_by_id(commitment_id.as_ref())
        .await
        .unwrap()
        .expect("commitment should still exist");
    assert_eq!(stored.status, CommitmentStatus::Open);

    let (_, context) = storage
        .get_current_context()
        .await
        .unwrap()
        .expect("context should still exist");
    assert_eq!(
        context.task_lanes.active_commitment_ids,
        vec![commitment_id.as_ref().to_string()]
    );
    assert!(context.task_lanes.completed_commitment_ids.is_empty());
}
