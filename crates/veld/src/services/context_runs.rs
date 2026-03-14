//! Run-backed context generation: creates runs, writes artifacts, appends events.

use std::path::Path;
use time::OffsetDateTime;
use sha2::{Sha256, Digest};
use vel_core::{
    ArtifactStorageKind, PrivacyClass, Ref, RefRelationType, RunEventType, RunId, RunKind, RunStatus,
    SyncClass,
};
use vel_storage::ArtifactInsert;

use crate::errors::AppError;
use crate::state::AppState;
use crate::services::context_generation;

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
}

/// Run-backed today context: creates run, computes result, writes artifact, links refs, appends events.
pub async fn generate_today(state: &AppState) -> Result<vel_api_types::TodayData, AppError> {
    let run_id = vel_core::RunId::new();
    let kind = ContextKind::Today;
    let input_json = serde_json::json!({ "context_kind": kind.as_str() });

    state
        .storage
        .create_run(&run_id, RunKind::ContextGeneration, &input_json)
        .await?;

    let result = run_context_generation(state, &run_id, kind, |snapshot| {
        Ok(context_generation::build_today(snapshot))
    })
    .await;

    match result {
        Ok(data) => Ok(data),
        Err(e) => {
            fail_run(state, &run_id, &e).await;
            Err(e)
        }
    }
}

/// Run-backed morning context.
pub async fn generate_morning(state: &AppState) -> Result<vel_api_types::MorningData, AppError> {
    let run_id = vel_core::RunId::new();
    let kind = ContextKind::Morning;
    let input_json = serde_json::json!({ "context_kind": kind.as_str() });

    state
        .storage
        .create_run(&run_id, RunKind::ContextGeneration, &input_json)
        .await?;

    let result = run_context_generation(state, &run_id, kind, |snapshot| {
        Ok(context_generation::build_morning(snapshot))
    })
    .await;

    match result {
        Ok(data) => Ok(data),
        Err(e) => {
            fail_run(state, &run_id, &e).await;
            Err(e)
        }
    }
}

/// Run-backed end-of-day context.
pub async fn generate_end_of_day(state: &AppState) -> Result<vel_api_types::EndOfDayData, AppError> {
    let run_id = vel_core::RunId::new();
    let kind = ContextKind::EndOfDay;
    let input_json = serde_json::json!({ "context_kind": kind.as_str() });

    state
        .storage
        .create_run(&run_id, RunKind::ContextGeneration, &input_json)
        .await?;

    let result = run_context_generation(state, &run_id, kind, |snapshot| {
        Ok(context_generation::build_end_of_day(snapshot))
    })
    .await;

    match result {
        Ok(data) => Ok(data),
        Err(e) => {
            fail_run(state, &run_id, &e).await;
            Err(e)
        }
    }
}

/// Shared orchestration: transition to running, load snapshot, compute, write artifact, refs, events, succeed.
async fn run_context_generation<T, F>(
    state: &AppState,
    run_id: &RunId,
    kind: ContextKind,
    compute: F,
) -> Result<T, AppError>
where
    F: FnOnce(&vel_core::OrientationSnapshot) -> Result<T, AppError>,
{
    let now = OffsetDateTime::now_utc();
    let started_at = now.unix_timestamp();

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

    state
        .storage
        .append_run_event(
            run_id.as_ref(),
            2,
            RunEventType::RunStarted,
            &serde_json::json!({}),
        )
        .await?;

    let snapshot = state.storage.orientation_snapshot().await?;
    let data = compute(&snapshot)?;

    let body = serde_json::to_vec(&data).map_err(|e| AppError::internal(e.to_string()))?;
    let size_bytes = body.len() as i64;
    let content_hash = {
        let mut hasher = Sha256::new();
        hasher.update(&body);
        format!("sha256:{}", hex::encode(hasher.finalize()))
    };

    let date_str = OffsetDateTime::now_utc().date().to_string();
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
    std::fs::write(&temp_path, &body).map_err(|e| AppError::internal(e.to_string()))?;
    std::fs::rename(&temp_path, &full_path).map_err(|e| AppError::internal(e.to_string()))?;

    let metadata_json = serde_json::json!({
        "generator": "context-v1",
        "context_kind": kind.as_str()
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
    for capture in snapshot.recent_today.iter().chain(snapshot.recent_week.iter()) {
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

    state
        .storage
        .append_run_event(
            run_id.as_ref(),
            3,
            RunEventType::ContextGenerated,
            &serde_json::json!({ "context_kind": kind.as_str() }),
        )
        .await?;

    state
        .storage
        .append_run_event(
            run_id.as_ref(),
            4,
            RunEventType::ArtifactWritten,
            &serde_json::json!({ "artifact_id": artifact_id.to_string() }),
        )
        .await?;

    let finished_at = OffsetDateTime::now_utc().unix_timestamp();
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

    state
        .storage
        .append_run_event(
            run_id.as_ref(),
            5,
            RunEventType::RunSucceeded,
            &serde_json::json!({}),
        )
        .await?;

    Ok(data)
}

async fn fail_run(state: &AppState, run_id: &RunId, error: &AppError) {
    let finished_at = OffsetDateTime::now_utc().unix_timestamp();
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
    let _ = state
        .storage
        .append_run_event(
            run_id.as_ref(),
            3,
            RunEventType::RunFailed,
            &serde_json::json!({ "error": error.to_string() }),
        )
        .await;
}
