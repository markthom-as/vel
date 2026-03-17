//! Background workers for low-risk automation loops.
//!
//! Current responsibilities:
//! - claim and complete capture_ingest jobs
//! - promote retry-scheduled runs when their retry_at is due

use std::time::Duration;

use tracing::{debug, warn};
use vel_core::{LoopKind, RunEventType, RunKind, RunStatus};
use vel_storage::{PendingJob, RetryReadyRun};

use crate::state::AppState;

const LOOP_INTERVAL: Duration = Duration::from_secs(5);
const JOB_TYPE_CAPTURE_INGEST: &str = "capture_ingest";
const RETRY_BATCH_LIMIT: u32 = 10;

pub async fn run_background_workers(state: AppState) {
    for loop_definition in registered_loops() {
        if !loop_definition.enabled {
            debug!(loop_kind = %loop_definition.kind, "background loop disabled");
            continue;
        }

        let state = state.clone();
        tokio::spawn(async move {
            run_registered_loop(state, loop_definition).await;
        });
    }

    std::future::pending::<()>().await;
}

#[derive(Debug, Clone, Copy)]
struct LoopDefinition {
    kind: LoopKind,
    interval: Duration,
    enabled: bool,
    runner: LoopRunner,
}

type LoopRunner = fn(&AppState) -> LoopFuture<'_>;
type LoopFuture<'a> = std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<(), crate::errors::AppError>> + Send + 'a>,
>;

fn registered_loops() -> Vec<LoopDefinition> {
    vec![
        LoopDefinition {
            kind: LoopKind::CaptureIngest,
            interval: LOOP_INTERVAL,
            enabled: true,
            runner: run_capture_ingest_loop_once,
        },
        LoopDefinition {
            kind: LoopKind::RetryDueRuns,
            interval: LOOP_INTERVAL,
            enabled: true,
            runner: run_retry_due_runs_loop_once,
        },
    ]
}

async fn run_registered_loop(state: AppState, loop_definition: LoopDefinition) {
    let mut ticker = tokio::time::interval(loop_definition.interval);
    loop {
        ticker.tick().await;
        match run_claimed_loop_once(&state, loop_definition).await {
            Ok(true) | Ok(false) => {}
            Err(error) => {
                warn!(
                    error = %error,
                    loop_kind = %loop_definition.kind,
                    "background loop execution failed"
                );
            }
        }
    }
}

async fn run_registered_loops_once(state: &AppState) -> Result<(), crate::errors::AppError> {
    for loop_definition in registered_loops()
        .into_iter()
        .filter(|loop_definition| loop_definition.enabled)
    {
        let _ = run_claimed_loop_once(state, loop_definition).await?;
    }
    Ok(())
}

async fn run_claimed_loop_once(
    state: &AppState,
    loop_definition: LoopDefinition,
) -> Result<bool, crate::errors::AppError> {
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let interval_seconds = loop_definition.interval.as_secs() as i64;
    let loop_kind = loop_definition.kind.to_string();

    let claimed = state
        .storage
        .claim_due_loop(&loop_kind, interval_seconds, now_ts)
        .await?;
    if !claimed {
        debug!(loop_kind = %loop_kind, "background loop not due or already running");
        return Ok(false);
    }

    let next_due_at = now_ts + interval_seconds;
    match (loop_definition.runner)(state).await {
        Ok(()) => {
            state
                .storage
                .complete_loop(&loop_kind, "succeeded", None, next_due_at)
                .await?;
            Ok(true)
        }
        Err(error) => {
            let error_message = error.to_string();
            state
                .storage
                .complete_loop(&loop_kind, "failed", Some(&error_message), next_due_at)
                .await?;
            Err(error)
        }
    }
}

fn run_capture_ingest_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        if let Some(job) = state
            .storage
            .claim_next_pending_job(JOB_TYPE_CAPTURE_INGEST)
            .await?
        {
            process_capture_ingest_job(&state.storage, job).await?;
        }
        Ok(())
    })
}

fn run_retry_due_runs_loop_once(state: &AppState) -> LoopFuture<'_> {
    Box::pin(async move {
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let ready_runs = state
            .storage
            .list_retry_ready_runs(now_ts, RETRY_BATCH_LIMIT)
            .await?;
        for retry in ready_runs {
            if let Err(error) = process_retry_ready_run(state, retry).await {
                warn!(error = %error, "run retry execution failed");
            }
        }
        Ok(())
    })
}

#[cfg(test)]
async fn poll_once(state: &AppState) -> Result<(), crate::errors::AppError> {
    run_registered_loops_once(state).await
}

async fn process_capture_ingest_job(
    storage: &vel_storage::Storage,
    job: PendingJob,
) -> Result<(), vel_storage::StorageError> {
    debug!(job_id = %job.job_id, "processing capture_ingest job");
    storage.mark_job_succeeded(&job.job_id.to_string()).await?;
    Ok(())
}

async fn process_retry_ready_run(
    state: &AppState,
    retry: RetryReadyRun,
) -> Result<(), crate::errors::AppError> {
    let run_id = retry.run.id.clone();
    debug!(
        run_id = %run_id,
        retry_at = retry.retry_at,
        kind = %retry.run.kind,
        "processing retry-scheduled run"
    );

    state.storage.reset_run_for_retry(run_id.as_ref()).await?;
    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunRequeued,
            &serde_json::json!({
                "retry_at": retry.retry_at,
                "reason": retry.retry_reason,
                "source": "worker",
            }),
        )
        .await?;

    match retry.run.kind {
        RunKind::ContextGeneration => {
            crate::services::context_runs::retry_existing_run(
                state,
                &run_id,
                &retry.run.input_json,
            )
            .await?;
        }
        RunKind::Synthesis => {
            crate::services::synthesis::retry_existing_run(state, &run_id, &retry.run.input_json)
                .await?;
        }
        unsupported_kind => {
            let blocked_reason = unsupported_kind
                .retry_policy()
                .automatic_retry_reason
                .map(ToString::to_string)
                .unwrap_or_else(|| {
                    format!(
                        "automatic retry unsupported for run kind {}",
                        unsupported_kind
                    )
                });
            let error_json = serde_json::json!({
                "message": blocked_reason,
            });
            let output_json = serde_json::json!({
                "blocked_reason": blocked_reason,
            });
            state
                .storage
                .append_run_event_auto(
                    run_id.as_ref(),
                    RunEventType::RunRetryBlocked,
                    &serde_json::json!({
                        "reason": blocked_reason,
                        "kind": unsupported_kind.to_string(),
                    }),
                )
                .await?;
            state
                .storage
                .update_run_status(
                    run_id.as_ref(),
                    RunStatus::Blocked,
                    None,
                    Some(time::OffsetDateTime::now_utc().unix_timestamp()),
                    Some(&output_json),
                    Some(&error_json),
                )
                .await?;
        }
    }

    let _ = crate::routes::runs::broadcast_run_updated(state, run_id.as_ref()).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{poll_once, registered_loops};
    use crate::state::AppState;
    use vel_config::AppConfig;
    use vel_core::{LoopKind, RunEventType, RunId, RunKind, RunStatus};
    use vel_storage::Storage;

    #[test]
    fn registered_loops_are_explicit_and_enabled() {
        let loops = registered_loops();
        assert_eq!(loops.len(), 2);
        assert_eq!(loops[0].kind, LoopKind::CaptureIngest);
        assert_eq!(loops[1].kind, LoopKind::RetryDueRuns);
        assert!(loops.iter().all(|loop_definition| loop_definition.enabled));
    }

    #[tokio::test]
    async fn poll_once_records_runtime_loop_status() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        poll_once(&state).await.unwrap();

        let loops = storage.list_runtime_loops().await.unwrap();
        assert_eq!(loops.len(), 2);
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.last_started_at.is_some()));
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.last_finished_at.is_some()));
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.last_status.as_deref() == Some("succeeded")));
        assert!(loops
            .iter()
            .all(|loop_record| loop_record.next_due_at.is_some()));
    }

    #[tokio::test]
    async fn poll_once_retries_due_context_run() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let mut config = AppConfig::default();
        let artifact_root = std::env::temp_dir().join(format!(
            "vel-worker-retry-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&artifact_root).unwrap();
        config.artifact_root = artifact_root.to_string_lossy().to_string();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            config,
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let run_id = RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::ContextGeneration,
                &serde_json::json!({ "context_kind": "today" }),
            )
            .await
            .unwrap();
        storage
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
        storage
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
        storage
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

        assert!(poll_once(&state).await.is_ok());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, RunStatus::Succeeded);

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRequeued));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunSucceeded));
    }

    #[tokio::test]
    async fn poll_once_retries_due_synthesis_run() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let mut config = AppConfig::default();
        let artifact_root = std::env::temp_dir().join(format!(
            "vel-worker-synthesis-retry-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&artifact_root).unwrap();
        config.artifact_root = artifact_root.to_string_lossy().to_string();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            config,
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let run_id = RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();
        storage
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
        storage
            .append_run_event_auto(
                run_id.as_ref(),
                RunEventType::RunRetryScheduled,
                &serde_json::json!({
                    "retry_at": time::OffsetDateTime::now_utc().unix_timestamp() - 1,
                    "reason": "retry_weekly_synthesis",
                }),
            )
            .await
            .unwrap();
        storage
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

        assert!(poll_once(&state).await.is_ok());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, RunStatus::Succeeded);
        assert!(run.output_json.is_some());

        let refs = storage
            .list_refs_from("run", run_id.as_ref())
            .await
            .unwrap();
        assert!(
            !refs.is_empty(),
            "retried synthesis run should relink an artifact"
        );

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRequeued));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::ArtifactWritten));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunSucceeded));
    }

    #[tokio::test]
    async fn poll_once_blocks_unsupported_retry_kind() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let run_id = RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();
        storage
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
        storage
            .append_run_event_auto(
                run_id.as_ref(),
                RunEventType::RunRetryScheduled,
                &serde_json::json!({
                    "retry_at": time::OffsetDateTime::now_utc().unix_timestamp() - 1,
                    "reason": "retry_search",
                }),
            )
            .await
            .unwrap();
        storage
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

        assert!(poll_once(&state).await.is_ok());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, RunStatus::Blocked);
        assert_eq!(
            run.error_json
                .as_ref()
                .and_then(|json| json.get("message"))
                .and_then(serde_json::Value::as_str),
            Some("search runs do not have an automatic retry executor")
        );
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|json| json.get("blocked_reason"))
                .and_then(serde_json::Value::as_str),
            Some("search runs do not have an automatic retry executor")
        );

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRequeued));
        assert!(events
            .iter()
            .any(|event| event.event_type == RunEventType::RunRetryBlocked));
    }
}
