use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use time::macros::datetime;
use time::{Duration, OffsetDateTime};
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::{ApiResponse, AppleVoiceIntentData, AppleVoiceTurnRequestData};
use vel_config::AppConfig;
use vel_core::{
    AppleClientSurface, AppleRequestedOperation, CurrentContextV1, DailyLoopPhase, DailyLoopStatus,
    DailyLoopSurface,
};
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
    let thread_id = data.thread_id.expect("thread id");
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

    let signals = storage
        .list_signals(Some("capture_created"), None, 10)
        .await
        .unwrap();
    let capture_signal = signals
        .into_iter()
        .find(|signal| signal.source_ref.as_deref() == Some(capture_id.as_ref()))
        .expect("capture_created signal");
    let payload = capture_signal.payload_json;
    assert_eq!(payload["provenance"]["surface"], "apple_ios_voice");
    assert_eq!(payload["provenance"]["source_device"], "apple_ios_voice");

    let thread_messages = storage
        .list_messages_by_conversation(&thread_id, 10)
        .await
        .unwrap();
    assert_eq!(thread_messages.len(), 1);
    let thread_content: serde_json::Value =
        serde_json::from_str(&thread_messages[0].content_json).unwrap();
    assert_eq!(thread_content["input_mode"], "voice");
    assert_eq!(thread_content["entry_route"], "threads");
    assert_eq!(
        thread_content["voice_provenance"]["surface"],
        "apple_ios_voice"
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

    assert!(
        data.thread_id.is_some(),
        "backend schedule query should preserve thread continuity"
    );
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
async fn apple_voice_query_reuses_shared_voice_provenance_shape_when_metadata_is_present() {
    let storage = test_storage().await;
    let (broadcast_tx, _) = broadcast::channel(8);
    let state = veld::state::AppState::new(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        broadcast_tx,
        None,
        None,
    );

    let response = veld::services::apple_voice::apple_voice_turn(
        &state,
        AppleVoiceTurnRequestData {
            transcript: "What matters now?".to_string(),
            surface: AppleClientSurface::IosVoice.into(),
            operation: AppleRequestedOperation::CaptureOnly.into(),
            intents: vec![AppleVoiceIntentData::Capture],
            provenance: Some(vel_api_types::AppleTurnProvenanceData {
                source_device: Some("iphone".to_string()),
                locale: Some("en-US".to_string()),
                transcript_origin: Some("speech_recognition".to_string()),
                recorded_at: Some(datetime!(2026-03-20 03:10:00 UTC)),
                offline_captured_at: None,
                queued_at: None,
            }),
        }
        .into(),
    )
    .await
    .unwrap();

    assert_eq!(response.capture_id.is_some(), true);
    let signals = storage
        .list_signals(Some("capture_created"), None, 10)
        .await
        .unwrap();
    let payload = signals[0].payload_json.clone();
    assert_eq!(payload["provenance"]["surface"], "apple_ios_voice");
    assert_eq!(payload["provenance"]["source_device"], "iphone");
    assert_eq!(payload["provenance"]["locale"], "en-US");
    assert_eq!(
        payload["provenance"]["transcript_origin"],
        "speech_recognition"
    );
    assert_eq!(
        payload["provenance"]["recorded_at"],
        datetime!(2026-03-20 03:10:00 UTC).unix_timestamp()
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

#[tokio::test]
async fn apple_voice_morning_briefing_starts_shared_daily_loop_session() {
    let storage = test_storage().await;
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let response = app
        .oneshot(build_request(AppleVoiceTurnRequestData {
            transcript: "Good morning, start my day.".to_string(),
            surface: AppleClientSurface::IosVoice.into(),
            operation: AppleRequestedOperation::QueryOnly.into(),
            intents: vec![AppleVoiceIntentData::MorningBriefing],
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

    assert!(
        data.summary.contains("Morning overview ready"),
        "morning voice intent should route into the daily-loop engine"
    );
    assert!(
        data.reasons
            .iter()
            .any(|reason| reason.contains("daily-loop")),
        "voice response should explain that the backend daily-loop engine answered it"
    );

    let session_date = time::OffsetDateTime::now_utc().date();
    let session_date = format!(
        "{:04}-{:02}-{:02}",
        session_date.year(),
        u8::from(session_date.month()),
        session_date.day()
    );
    let record = storage
        .get_active_daily_session_for_date(&session_date, DailyLoopPhase::MorningOverview)
        .await
        .unwrap()
        .expect("active morning session");

    assert_eq!(record.session.phase, DailyLoopPhase::MorningOverview);
    assert_eq!(record.session.status, DailyLoopStatus::WaitingForInput);
    assert_eq!(record.session.start.surface, DailyLoopSurface::AppleVoice);
}

#[tokio::test]
async fn apple_voice_standup_resume_uses_shared_standup_session_flow() {
    let storage = test_storage().await;
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );

    let response = app
        .oneshot(build_request(AppleVoiceTurnRequestData {
            transcript: "Resume standup.".to_string(),
            surface: AppleClientSurface::IosVoice.into(),
            operation: AppleRequestedOperation::QueryOnly.into(),
            intents: vec![AppleVoiceIntentData::MorningBriefing],
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

    assert!(
        data.summary.to_lowercase().contains("standup"),
        "standup transcript should start or resume the standup phase"
    );

    let session_date = time::OffsetDateTime::now_utc().date();
    let session_date = format!(
        "{:04}-{:02}-{:02}",
        session_date.year(),
        u8::from(session_date.month()),
        session_date.day()
    );
    let record = storage
        .get_active_daily_session_for_date(&session_date, DailyLoopPhase::Standup)
        .await
        .unwrap()
        .expect("active standup session");

    assert_eq!(record.session.phase, DailyLoopPhase::Standup);
    assert_eq!(record.session.start.surface, DailyLoopSurface::AppleVoice);
}
