use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use time::{Duration, OffsetDateTime};
use tower::util::ServiceExt;
use vel_api_types::{ApiResponse, AppleVoiceIntentData, AppleVoiceTurnRequestData};
use vel_config::AppConfig;
use vel_core::{AppleClientSurface, AppleRequestedOperation, CurrentContextV1};
use vel_storage::{CommitmentInsert, NudgeInsert, SignalInsert, Storage};
use veld::{app::build_app, policy_config::PolicyConfig};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn test_policy_config() -> PolicyConfig {
    PolicyConfig::default()
}

fn current_context_json(context: CurrentContextV1) -> String {
    serde_json::to_string(&context).unwrap()
}

async fn test_storage() -> Storage {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
}

fn build_request(payload: AppleVoiceTurnRequestData) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/apple/voice/turn")
        .header("content-type", "application/json")
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap()
}

#[tokio::test]
async fn apple_voice_query_persists_transcript_capture_and_returns_explainable_response() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    storage
        .set_current_context(
            now.unix_timestamp(),
            &current_context_json(CurrentContextV1 {
                computed_at: now.unix_timestamp(),
                mode: "day_mode".to_string(),
                morning_state: "engaged".to_string(),
                meds_status: "done".to_string(),
                global_risk_level: "low".to_string(),
                global_risk_score: Some(0.15),
                attention_state: "on_task".to_string(),
                drift_type: Some("none".to_string()),
                drift_severity: Some("none".to_string()),
                attention_confidence: Some(0.95),
                attention_reasons: vec!["Calendar and commitments are current.".to_string()],
                ..CurrentContextV1::default()
            }),
        )
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
        .oneshot(build_request(AppleVoiceTurnRequestData {
            transcript: "What matters now?".to_string(),
            surface: AppleClientSurface::IosVoice.into(),
            operation: AppleRequestedOperation::CaptureAndQuery.into(),
            intents: vec![AppleVoiceIntentData::ExplainWhy],
            provenance: None,
        }))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<vel_api_types::AppleVoiceTurnResponseData> =
        serde_json::from_slice(&body).unwrap();
    let data = payload.data.expect("voice response data");

    let capture_id = data.capture_id.expect("capture id");
    assert!(!data.summary.trim().is_empty());
    assert!(
        !data.reasons.is_empty(),
        "backend-owned voice response should explain itself"
    );
    assert!(
        !data.evidence.is_empty(),
        "backend-owned voice response should cite evidence"
    );

    let capture = storage
        .get_capture_by_id(capture_id.as_ref())
        .await
        .unwrap()
        .expect("capture should persist before response");
    assert!(
        capture.content_text.contains("What matters now?"),
        "persisted capture should preserve transcript provenance"
    );
}

#[tokio::test]
async fn apple_voice_schedule_query_uses_backend_now_state_not_client_heuristics() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let event_start = (now + Duration::minutes(30)).unix_timestamp();
    storage
        .insert_signal(SignalInsert {
            signal_type: "calendar_event".to_string(),
            source: "google_calendar".to_string(),
            source_ref: Some("evt_backend_schedule".to_string()),
            timestamp: event_start,
            payload_json: Some(serde_json::json!({
                "event_id": "evt_backend_schedule",
                "title": "Backend Planning Review",
                "start": event_start,
                "end": event_start + 1800,
                "location": "Studio"
            })),
        })
        .await
        .unwrap();
    storage
        .set_current_context(
            now.unix_timestamp(),
            &current_context_json(CurrentContextV1 {
                computed_at: now.unix_timestamp(),
                mode: "day_mode".to_string(),
                morning_state: "engaged".to_string(),
                meds_status: "done".to_string(),
                global_risk_level: "low".to_string(),
                global_risk_score: Some(0.12),
                attention_state: "on_task".to_string(),
                drift_type: Some("none".to_string()),
                drift_severity: Some("none".to_string()),
                attention_confidence: Some(0.91),
                signals_used: vec!["evt_backend_schedule".to_string()],
                ..CurrentContextV1::default()
            }),
        )
        .await
        .unwrap();

    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let response = app
        .oneshot(build_request(AppleVoiceTurnRequestData {
            transcript: "What's next on my schedule? My local note says lunch.".to_string(),
            surface: AppleClientSurface::IosVoice.into(),
            operation: AppleRequestedOperation::CaptureAndQuery.into(),
            intents: vec![AppleVoiceIntentData::CurrentSchedule],
            provenance: None,
        }))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<vel_api_types::AppleVoiceTurnResponseData> =
        serde_json::from_slice(&body).unwrap();
    let data = payload.data.expect("voice response data");

    let schedule = data.schedule.expect("schedule snapshot");
    let next_event = schedule.next_event.expect("next event");
    assert_eq!(next_event.title, "Backend Planning Review");
    assert_eq!(next_event.location.as_deref(), Some("Studio"));
    assert!(
        data.evidence
            .iter()
            .any(|item| item.label.contains("Backend Planning Review")),
        "response should cite backend-derived schedule evidence"
    );
}

#[tokio::test]
async fn apple_voice_low_risk_action_reuses_safe_backend_mutation_path() {
    let storage = test_storage().await;
    let commitment_id = storage
        .insert_commitment(CommitmentInsert {
            text: "Reply to Alex".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todo_1".to_string(),
            status: vel_core::CommitmentStatus::Open,
            due_at: None,
            project: None,
            commitment_kind: Some("todo".to_string()),
            metadata_json: Some(serde_json::json!({ "priority": 1 })),
        })
        .await
        .unwrap();
    let nudge_id = storage
        .insert_nudge(NudgeInsert {
            nudge_type: "response_debt".to_string(),
            level: "warning".to_string(),
            state: "active".to_string(),
            related_commitment_id: Some(commitment_id.as_ref().to_string()),
            message: "Reply to Alex".to_string(),
            snoozed_until: None,
            resolved_at: None,
            signals_snapshot_json: None,
            inference_snapshot_json: None,
            metadata_json: Some(serde_json::json!({ "source": "test" })),
        })
        .await
        .unwrap();
    let now = OffsetDateTime::now_utc();
    storage
        .set_current_context(
            now.unix_timestamp(),
            &current_context_json(CurrentContextV1 {
                computed_at: now.unix_timestamp(),
                mode: "day_mode".to_string(),
                morning_state: "engaged".to_string(),
                meds_status: "done".to_string(),
                global_risk_level: "low".to_string(),
                global_risk_score: Some(0.2),
                attention_state: "needs_attention".to_string(),
                drift_type: Some("none".to_string()),
                drift_severity: Some("none".to_string()),
                attention_confidence: Some(0.88),
                attention_reasons: vec!["One active nudge needs response.".to_string()],
                ..CurrentContextV1::default()
            }),
        )
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
        .oneshot(build_request(AppleVoiceTurnRequestData {
            transcript: "Snooze that nudge for ten minutes.".to_string(),
            surface: AppleClientSurface::IosVoice.into(),
            operation: AppleRequestedOperation::Mutation.into(),
            intents: vec![AppleVoiceIntentData::SnoozeNudge],
            provenance: None,
        }))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<vel_api_types::AppleVoiceTurnResponseData> =
        serde_json::from_slice(&body).unwrap();
    let data = payload.data.expect("voice response data");

    let queued = data.queued_mutation.expect("queued mutation summary");
    assert_eq!(queued.mutation_kind, "nudge_snooze");
    assert!(
        queued.queued || data.summary.to_lowercase().contains("applied"),
        "response should reflect queued or applied backend mutation state"
    );

    let nudges = storage.list_nudges(Some("snoozed"), 10).await.unwrap();
    assert!(
        nudges.iter().any(|nudge| nudge.nudge_id == nudge_id),
        "low-risk voice action should reuse the existing backend nudge path"
    );
}
