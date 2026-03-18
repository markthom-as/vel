//! Run-backed context generation: creates runs, writes artifacts, appends events.

use sha2::{Digest, Sha256};
use std::path::Path;
use time::OffsetDateTime;
use vel_core::{
    ArtifactId, ArtifactStorageKind, Clock, PrivacyClass, Ref, RefRelationType, RunEventType,
    RunId, RunKind, RunStatus, SyncClass, SystemClock,
};
use vel_storage::ArtifactInsert;

use crate::errors::AppError;
use crate::services::context_generation::{
    self, EndOfDayContextData, MorningContextData, TodayContextData,
};
use crate::state::AppState;

/// Service-level result of a context run: run identity, artifact, and the computed payload.
/// Routes map `.data` to the API response; run_id/artifact_id/context_kind for logging or future use.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ContextRunOutput<T> {
    pub run_id: RunId,
    pub artifact_id: ArtifactId,
    pub context_kind: &'static str,
    pub data: T,
}

/// Context kind for run input and artifact naming.
#[derive(Debug, Clone, Copy)]
pub enum ContextKind {
    Today,
    Morning,
    EndOfDay,
}

impl ContextKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::Morning => "morning",
            Self::EndOfDay => "end_of_day",
        }
    }

    fn from_input_json(input_json: &serde_json::Value) -> Result<Self, AppError> {
        match input_json
            .get("context_kind")
            .and_then(serde_json::Value::as_str)
        {
            Some("today") => Ok(Self::Today),
            Some("morning") => Ok(Self::Morning),
            Some("end_of_day") => Ok(Self::EndOfDay),
            Some(other) => Err(AppError::bad_request(format!(
                "unsupported context_kind for retry: {}",
                other
            ))),
            None => Err(AppError::bad_request(
                "retry input_json missing context_kind",
            )),
        }
    }
}

struct RunEventSequencer {
    next_seq: u32,
}

impl RunEventSequencer {
    async fn for_run(state: &AppState, run_id: &RunId) -> Result<Self, AppError> {
        let next_seq = state
            .storage
            .list_run_events(run_id.as_ref())
            .await?
            .last()
            .map(|e| e.seq.saturating_add(1))
            .unwrap_or(1);
        Ok(Self { next_seq })
    }

    async fn append(
        &mut self,
        state: &AppState,
        run_id: &RunId,
        event_type: RunEventType,
        payload: &serde_json::Value,
    ) -> Result<(), AppError> {
        state
            .storage
            .append_run_event(run_id.as_ref(), self.next_seq, event_type, payload)
            .await?;
        self.next_seq = self.next_seq.saturating_add(1);
        Ok(())
    }
}

/// Run-backed today context: creates run, computes result, writes artifact, links refs, appends events.
pub async fn generate_today(
    state: &AppState,
) -> Result<ContextRunOutput<TodayContextData>, AppError> {
    generate_today_at(state, SystemClock.now()).await
}

pub async fn generate_today_at(
    state: &AppState,
    now: OffsetDateTime,
) -> Result<ContextRunOutput<TodayContextData>, AppError> {
    let run_id = RunId::new();
    let kind = ContextKind::Today;
    let input_json = serde_json::json!({ "context_kind": kind.as_str() });

    state
        .storage
        .create_run(&run_id, RunKind::ContextGeneration, &input_json)
        .await?;

    let result = run_context_generation(state, &run_id, kind, now, |snapshot| {
        Ok(context_generation::build_today_at(snapshot, now))
    })
    .await;

    match result {
        Ok((artifact_id, data)) => Ok(ContextRunOutput {
            run_id: run_id.clone(),
            artifact_id,
            context_kind: kind.as_str(),
            data,
        }),
        Err(e) => {
            fail_run_at(state, &run_id, &e, now).await;
            Err(e)
        }
    }
}

/// Retry an existing run ID for context generation without creating a new run row.
/// The run input must include `context_kind` with one of: today, morning, end_of_day.
pub async fn retry_existing_run(
    state: &AppState,
    run_id: &RunId,
    input_json: &serde_json::Value,
) -> Result<(), AppError> {
    retry_existing_run_at(state, run_id, input_json, SystemClock.now()).await
}

pub async fn retry_existing_run_at(
    state: &AppState,
    run_id: &RunId,
    input_json: &serde_json::Value,
    now: OffsetDateTime,
) -> Result<(), AppError> {
    let kind = ContextKind::from_input_json(input_json)?;
    let existing = state.storage.get_run_by_id(run_id.as_ref()).await?;
    if existing.is_none() {
        return Err(AppError::not_found("run not found"));
    }

    let result = match kind {
        ContextKind::Today => run_context_generation(state, run_id, kind, now, |snapshot| {
            Ok(context_generation::build_today_at(snapshot, now))
        })
        .await
        .map(|_| ()),
        ContextKind::Morning => run_context_generation(state, run_id, kind, now, |snapshot| {
            Ok(context_generation::build_morning_at(snapshot, now))
        })
        .await
        .map(|_| ()),
        ContextKind::EndOfDay => run_context_generation(state, run_id, kind, now, |snapshot| {
            Ok(context_generation::build_end_of_day_at(snapshot, now))
        })
        .await
        .map(|_| ()),
    };

    if let Err(e) = result {
        fail_run_at(state, run_id, &e, now).await;
        return Err(e);
    }

    Ok(())
}

/// Run-backed morning context.
pub async fn generate_morning(
    state: &AppState,
) -> Result<ContextRunOutput<MorningContextData>, AppError> {
    generate_morning_at(state, SystemClock.now()).await
}

pub async fn generate_morning_at(
    state: &AppState,
    now: OffsetDateTime,
) -> Result<ContextRunOutput<MorningContextData>, AppError> {
    let run_id = RunId::new();
    let kind = ContextKind::Morning;
    let input_json = serde_json::json!({ "context_kind": kind.as_str() });

    state
        .storage
        .create_run(&run_id, RunKind::ContextGeneration, &input_json)
        .await?;

    let result = run_context_generation(state, &run_id, kind, now, |snapshot| {
        Ok(context_generation::build_morning_at(snapshot, now))
    })
    .await;

    match result {
        Ok((artifact_id, data)) => Ok(ContextRunOutput {
            run_id: run_id.clone(),
            artifact_id,
            context_kind: kind.as_str(),
            data,
        }),
        Err(e) => {
            fail_run_at(state, &run_id, &e, now).await;
            Err(e)
        }
    }
}

/// Run-backed end-of-day context.
pub async fn generate_end_of_day(
    state: &AppState,
) -> Result<ContextRunOutput<EndOfDayContextData>, AppError> {
    generate_end_of_day_at(state, SystemClock.now()).await
}

pub async fn generate_end_of_day_at(
    state: &AppState,
    now: OffsetDateTime,
) -> Result<ContextRunOutput<EndOfDayContextData>, AppError> {
    let run_id = RunId::new();
    let kind = ContextKind::EndOfDay;
    let input_json = serde_json::json!({ "context_kind": kind.as_str() });

    state
        .storage
        .create_run(&run_id, RunKind::ContextGeneration, &input_json)
        .await?;

    let result = run_context_generation(state, &run_id, kind, now, |snapshot| {
        Ok(context_generation::build_end_of_day_at(snapshot, now))
    })
    .await;

    match result {
        Ok((artifact_id, data)) => Ok(ContextRunOutput {
            run_id: run_id.clone(),
            artifact_id,
            context_kind: kind.as_str(),
            data,
        }),
        Err(e) => {
            fail_run_at(state, &run_id, &e, now).await;
            Err(e)
        }
    }
}

/// Shared orchestration: transition to running, load snapshot, compute, write artifact, refs, events, succeed.
/// Returns (artifact_id, data) on success.
async fn run_context_generation<T, F>(
    state: &AppState,
    run_id: &RunId,
    kind: ContextKind,
    now: OffsetDateTime,
    compute: F,
) -> Result<(ArtifactId, T), AppError>
where
    T: serde::Serialize,
    F: FnOnce(&vel_core::OrientationSnapshot) -> Result<T, AppError>,
{
    let started_at = now.unix_timestamp();
    let mut event_seq = RunEventSequencer::for_run(state, run_id).await?;

    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Running,
            Some(started_at),
            None,
            None,
            None,
        )
        .await?;

    event_seq
        .append(
            state,
            run_id,
            RunEventType::RunStarted,
            &serde_json::json!({}),
        )
        .await?;

    let snapshot = state.storage.orientation_snapshot_at(now).await?;
    let semantic_hits = if let Some(query) =
        context_generation::semantic_query_for_snapshot(&snapshot)
    {
        let hits = state.storage.semantic_query(&query).await?;
        event_seq
            .append(
                state,
                run_id,
                RunEventType::SearchExecuted,
                &serde_json::json!({
                    "query_text": query.query_text,
                    "strategy": query.strategy,
                    "hit_count": hits.len(),
                    "hits": hits.iter().map(|hit| serde_json::json!({
                        "record_id": serde_json::to_value(&hit.record_id).unwrap_or(serde_json::Value::Null),
                        "source_kind": hit.source_kind,
                        "source_id": hit.source_id,
                        "combined_score": hit.combined_score,
                        "lexical_score": hit.lexical_score,
                        "semantic_score": hit.semantic_score,
                        "provenance": hit.provenance,
                    })).collect::<Vec<_>>(),
                }),
            )
            .await?;
        hits
    } else {
        Vec::new()
    };
    let data = compute(&snapshot)?;

    let body = serde_json::to_vec(&data).map_err(|e| AppError::internal(e.to_string()))?;
    let size_bytes = body.len() as i64;
    let content_hash = {
        let mut hasher = Sha256::new();
        hasher.update(&body);
        format!("sha256:{}", hex::encode(hasher.finalize()))
    };

    let date_str = now.date().to_string();
    let storage_uri = format!(
        "context/{}/{}/{}.json",
        kind.as_str(),
        date_str,
        run_id.as_ref()
    );
    let full_path = Path::new(&state.config.artifact_root).join(&storage_uri);
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::internal(e.to_string()))?;
    }
    let temp_path = full_path.with_extension("json.tmp");
    let mut f = std::fs::File::create(&temp_path).map_err(|e| AppError::internal(e.to_string()))?;
    std::io::Write::write_all(&mut f, &body).map_err(|e| AppError::internal(e.to_string()))?;
    f.sync_all()
        .map_err(|e| AppError::internal(e.to_string()))?;
    drop(f);
    std::fs::rename(&temp_path, &full_path).map_err(|e| AppError::internal(e.to_string()))?;

    let metadata_json = serde_json::json!({
        "generator": "context-v1",
        "context_kind": kind.as_str(),
        "snapshot_window": "7d",
        "semantic_hit_count": semantic_hits.len()
    });

    let artifact_id = state
        .storage
        .create_artifact(ArtifactInsert {
            artifact_type: "context_brief".to_string(),
            title: Some(format!("{} context", kind.as_str())),
            mime_type: Some("application/json".to_string()),
            storage_uri,
            storage_kind: ArtifactStorageKind::Managed,
            privacy_class: PrivacyClass::Private,
            sync_class: SyncClass::Warm,
            content_hash: Some(content_hash),
            size_bytes: Some(size_bytes),
            metadata_json: Some(metadata_json),
        })
        .await?;

    let ref_ = Ref::new(
        "run",
        run_id.as_ref(),
        "artifact",
        artifact_id.as_ref(),
        RefRelationType::AttachedTo,
    );
    state.storage.create_ref(&ref_).await?;

    let mut seen_captures = std::collections::HashSet::new();
    for capture in snapshot
        .recent_today
        .iter()
        .chain(snapshot.recent_week.iter())
    {
        if seen_captures.insert(capture.capture_id.as_ref()) {
            let art_ref = Ref::new(
                "artifact",
                artifact_id.as_ref(),
                "capture",
                capture.capture_id.as_ref(),
                RefRelationType::DerivedFrom,
            );
            let _ = state.storage.create_ref(&art_ref).await;
        }
    }

    event_seq
        .append(
            state,
            run_id,
            RunEventType::ContextGenerated,
            &serde_json::json!({ "context_kind": kind.as_str() }),
        )
        .await?;

    event_seq
        .append(
            state,
            run_id,
            RunEventType::ArtifactWritten,
            &serde_json::json!({ "artifact_id": artifact_id.to_string() }),
        )
        .await?;

    event_seq
        .append(
            state,
            run_id,
            RunEventType::RefsCreated,
            &serde_json::json!({}),
        )
        .await?;

    let finished_at = now.unix_timestamp();
    let output_json = serde_json::json!({
        "artifact_id": artifact_id.to_string(),
        "context_kind": kind.as_str()
    });

    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Succeeded,
            None,
            Some(finished_at),
            Some(&output_json),
            None,
        )
        .await?;

    event_seq
        .append(
            state,
            run_id,
            RunEventType::RunSucceeded,
            &serde_json::json!({}),
        )
        .await?;

    Ok((artifact_id, data))
}

async fn fail_run(state: &AppState, run_id: &RunId, error: &AppError) {
    fail_run_at(state, run_id, error, SystemClock.now()).await;
}

async fn fail_run_at(state: &AppState, run_id: &RunId, error: &AppError, now: OffsetDateTime) {
    let finished_at = now.unix_timestamp();
    let error_json = serde_json::json!({ "message": error.to_string() });
    let _ = state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Failed,
            None,
            Some(finished_at),
            None,
            Some(&error_json),
        )
        .await;
    let mut event_seq = match RunEventSequencer::for_run(state, run_id).await {
        Ok(v) => v,
        Err(_) => return,
    };
    let _ = event_seq
        .append(
            state,
            run_id,
            RunEventType::RunFailed,
            &serde_json::json!({ "error": error.to_string() }),
        )
        .await;
}
