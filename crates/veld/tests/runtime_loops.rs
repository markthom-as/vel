use std::path::PathBuf;

use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_core::{RunEventType, RunId, RunKind, RunStatus};
use vel_storage::Storage;
use veld::{
    policy_config::{LoopPolicies, LoopPolicy, PolicyConfig},
    state::AppState,
    worker,
};

fn only_enabled_loop(loop_policy: Option<(&str, u64)>) -> PolicyConfig {
    let mut config = PolicyConfig::default();
    config.loops = LoopPolicies {
        queue_work_scheduler: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("queue_work_scheduler", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "queue_work_scheduler")
                .map(|(_, interval)| interval)
                .unwrap_or(30),
        }),
        evaluate_current_state: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("evaluate_current_state", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "evaluate_current_state")
                .map(|(_, interval)| interval)
                .unwrap_or(300),
        }),
        sync_calendar: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_calendar", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_calendar")
                .map(|(_, interval)| interval)
                .unwrap_or(900),
        }),
        sync_todoist: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_todoist", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_todoist")
                .map(|(_, interval)| interval)
                .unwrap_or(600),
        }),
        sync_activity: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_activity", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_activity")
                .map(|(_, interval)| interval)
                .unwrap_or(300),
        }),
        sync_health: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_health", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_health")
                .map(|(_, interval)| interval)
                .unwrap_or(900),
        }),
        sync_git: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_git", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_git")
                .map(|(_, interval)| interval)
                .unwrap_or(600),
        }),
        sync_messaging: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_messaging", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_messaging")
                .map(|(_, interval)| interval)
                .unwrap_or(300),
        }),
        sync_reminders: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_reminders", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_reminders")
                .map(|(_, interval)| interval)
                .unwrap_or(600),
        }),
        sync_notes: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_notes", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_notes")
                .map(|(_, interval)| interval)
                .unwrap_or(900),
        }),
        sync_transcripts: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("sync_transcripts", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "sync_transcripts")
                .map(|(_, interval)| interval)
                .unwrap_or(900),
        }),
        weekly_synthesis: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("weekly_synthesis", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "weekly_synthesis")
                .map(|(_, interval)| interval)
                .unwrap_or(86_400),
        }),
        stale_nudge_reconciliation: Some(LoopPolicy {
            enabled: matches!(loop_policy, Some(("stale_nudge_reconciliation", _))),
            interval_seconds: loop_policy
                .filter(|(kind, _)| *kind == "stale_nudge_reconciliation")
                .map(|(_, interval)| interval)
                .unwrap_or(1_800),
        }),
    };
    config
}

async fn test_state(config: AppConfig, policy_config: PolicyConfig) -> AppState {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let (broadcast_tx, _) = broadcast::channel(16);
    AppState::new(storage, config, policy_config, broadcast_tx, None, None)
}

fn unique_path(extension: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "vel_runtime_loops_{}.{}",
        uuid::Uuid::new_v4().simple(),
        extension
    ))
}

#[tokio::test]
async fn due_evaluate_loop_is_claimed_and_run_once() {
    let state = test_state(
        AppConfig::default(),
        only_enabled_loop(Some(("evaluate_current_state", 300))),
    )
    .await;

    worker::run_registered_loops_once(&state).await.unwrap();

    let record = state
        .storage
        .get_runtime_loop("evaluate_current_state")
        .await
        .unwrap()
        .expect("evaluate loop row should be created");
    assert_eq!(record.last_status.as_deref(), Some("succeeded"));
    assert!(record.last_started_at.is_some());
    assert!(record.last_finished_at.is_some());
    assert!(record.next_due_at.is_some());
    assert!(
        state.storage.get_current_context().await.unwrap().is_some(),
        "evaluate loop should persist current context"
    );

    let started_at = record.last_started_at;
    worker::run_registered_loops_once(&state).await.unwrap();
    let rerun_record = state
        .storage
        .get_runtime_loop("evaluate_current_state")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(rerun_record.last_started_at, started_at);
}

#[tokio::test]
async fn disabled_loop_does_not_run() {
    let state = test_state(AppConfig::default(), only_enabled_loop(None)).await;

    worker::run_registered_loops_once(&state).await.unwrap();

    assert!(
        state
            .storage
            .get_runtime_loop("evaluate_current_state")
            .await
            .unwrap()
            .is_none(),
        "disabled loops should not create or claim runtime rows"
    );
}

#[tokio::test]
async fn failed_loop_records_error_and_next_due_time() {
    let calendar_path = unique_path("ics");
    let config = AppConfig {
        calendar_ics_path: Some(calendar_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let state = test_state(config, only_enabled_loop(Some(("sync_calendar", 900)))).await;

    let error = worker::run_registered_loops_once(&state)
        .await
        .expect_err("missing sync input should fail the loop");
    assert!(error.to_string().contains("read ics path"));

    let record = state
        .storage
        .get_runtime_loop("sync_calendar")
        .await
        .unwrap()
        .expect("failed loop should still persist runtime status");
    assert_eq!(record.last_status.as_deref(), Some("failed"));
    assert!(record
        .last_error
        .as_deref()
        .is_some_and(|message| message.contains("read ics path")));
    assert!(record.next_due_at.is_some());
}

#[tokio::test]
async fn sync_loop_can_trigger_evaluate_follow_up() {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let snapshot_path = unique_path("json");
    let snapshot = serde_json::json!({
        "source": "messaging",
        "account_id": "local-default",
        "threads": [
            {
                "thread_id": "thr_ops",
                "platform": "sms",
                "title": "Review reschedule",
                "participants": [
                    { "id": "me", "name": "Me", "is_me": true },
                    { "id": "+15551234567", "name": "Sam", "is_me": false }
                ],
                "latest_timestamp": now,
                "waiting_state": "me",
                "scheduling_related": true,
                "urgent": true,
                "summary": "Need to answer the review reschedule request.",
                "snippet": "Can we move the review to 3?"
            }
        ]
    });
    std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

    let config = AppConfig {
        messaging_snapshot_path: Some(snapshot_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let state = test_state(config, only_enabled_loop(Some(("sync_messaging", 300)))).await;

    worker::run_registered_loops_once(&state).await.unwrap();

    let (_, context_json) = state
        .storage
        .get_current_context()
        .await
        .unwrap()
        .expect("sync loop should trigger evaluate follow-up");
    let context: serde_json::Value = serde_json::to_value(context_json).unwrap();
    assert_eq!(context["message_waiting_on_me_count"], 1);
    assert_eq!(context["message_scheduling_thread_count"], 1);
    assert_eq!(context["message_urgent_thread_count"], 1);

    let loop_record = state
        .storage
        .get_runtime_loop("sync_messaging")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loop_record.last_status.as_deref(), Some("succeeded"));

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn git_sync_loop_can_trigger_evaluate_follow_up() {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let snapshot_path = unique_path("json");
    let snapshot = serde_json::json!({
        "source": "git",
        "captured_at": now,
        "events": [
            {
                "id": "evt_git_review",
                "timestamp": now,
                "repo": "/home/jove/code/vel",
                "branch": "main",
                "operation": "commit",
                "message": "Tighten context ranking"
            }
        ]
    });
    std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

    let config = AppConfig {
        git_snapshot_path: Some(snapshot_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let state = test_state(config, only_enabled_loop(Some(("sync_git", 600)))).await;

    worker::run_registered_loops_once(&state).await.unwrap();

    let (_, context_json) = state
        .storage
        .get_current_context()
        .await
        .unwrap()
        .expect("git sync loop should trigger evaluate follow-up");
    let context: serde_json::Value = serde_json::to_value(context_json).unwrap();
    assert_eq!(context["git_activity_summary"]["repo"], "vel");
    assert_eq!(context["git_activity_summary"]["branch"], "main");
    assert_eq!(context["git_activity_summary"]["operation"], "commit");

    let loop_record = state
        .storage
        .get_runtime_loop("sync_git")
        .await
        .unwrap()
        .expect("git loop should persist runtime status");
    assert_eq!(loop_record.last_status.as_deref(), Some("succeeded"));

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn retry_loop_remains_functional_after_worker_refactor() {
    let artifact_root = unique_path("artifacts");
    std::fs::create_dir_all(&artifact_root).unwrap();
    let config = AppConfig {
        artifact_root: artifact_root.to_string_lossy().to_string(),
        ..Default::default()
    };
    let state = test_state(config, only_enabled_loop(None)).await;

    let run_id = RunId::new();
    state
        .storage
        .create_run(
            &run_id,
            RunKind::ContextGeneration,
            &serde_json::json!({ "context_kind": "today" }),
        )
        .await
        .unwrap();
    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Failed,
            Some(1),
            Some(2),
            None,
            Some(&serde_json::json!({ "message": "boom" })),
        )
        .await
        .unwrap();
    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunRetryScheduled,
            &serde_json::json!({
                "retry_at": time::OffsetDateTime::now_utc().unix_timestamp() - 1,
                "reason": "test",
            }),
        )
        .await
        .unwrap();
    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::RetryScheduled,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    worker::run_registered_loops_once(&state).await.unwrap();

    let run = state
        .storage
        .get_run_by_id(run_id.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(run.status, RunStatus::Succeeded);

    let events = state
        .storage
        .list_run_events(run_id.as_ref())
        .await
        .unwrap();
    assert!(events
        .iter()
        .any(|event| event.event_type == RunEventType::RunRequeued));
    assert!(events
        .iter()
        .any(|event| event.event_type == RunEventType::RunSucceeded));
}
