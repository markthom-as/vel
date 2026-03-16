//! Run-backed weekly synthesis: creates run, gathers commitments/nudges/signals/captures, writes artifact (Phase F).

use sha2::Digest;
use time::OffsetDateTime;
use vel_core::{ArtifactId, ArtifactStorageKind, PrivacyClass, Ref, RefRelationType, RunEventType, RunId, RunKind, RunStatus, SyncClass};
use vel_core::CommitmentStatus;
use vel_storage::ArtifactInsert;

use crate::errors::AppError;
use crate::state::AppState;

/// Run weekly synthesis: create run, gather data, write synthesis_brief artifact, link refs.
pub async fn run_week_synthesis(state: &AppState) -> Result<(RunId, ArtifactId), AppError> {
    let run_id = RunId::new();
    let input_json = serde_json::json!({ "synthesis_kind": "week", "window_days": 7 });

    state.storage.create_run(&run_id, RunKind::Synthesis, &input_json).await?;
    let started_at = OffsetDateTime::now_utc().unix_timestamp();
    state.storage.update_run_status(run_id.as_ref(), RunStatus::Running, Some(started_at), None, None, None).await?;
    state.storage.append_run_event(run_id.as_ref(), 2, RunEventType::RunStarted, &serde_json::json!({})).await?;

    let now = OffsetDateTime::now_utc();
    let seven_days_ago = (now - time::Duration::days(7)).unix_timestamp();

    let open_commitments = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 200)
        .await?;
    let nudges = state.storage.list_nudges(None, 100).await?;
    let _signals = state.storage.list_signals(None, Some(seven_days_ago), 500).await?;
    let _captures = state.storage.list_captures_recent(200, false).await?;

    let week_end = now.unix_timestamp();
    let week_start = seven_days_ago;
    let all_commitments = state.storage.list_commitments(None, None, None, 500).await?;
    let completed_count = all_commitments.iter().filter(|c| c.status == CommitmentStatus::Done).count();
    let resolved_nudges = nudges.iter().filter(|n| n.state == "resolved").count();
    let open_commitment_ids: Vec<String> = open_commitments.iter().map(|c| c.id.as_ref().to_string()).collect();
    let completed_commitment_ids: Vec<String> = all_commitments
        .iter()
        .filter(|c| c.status == CommitmentStatus::Done)
        .take(50)
        .map(|c| c.id.as_ref().to_string())
        .collect();
    let thread_rows = state.storage.list_threads(Some("open"), 10).await.unwrap_or_default();
    let top_threads: Vec<serde_json::Value> = thread_rows
        .into_iter()
        .map(|(id, thread_type, title, status, _ca, _ua)| {
            serde_json::json!({ "id": id, "thread_type": thread_type, "title": title, "status": status })
        })
        .collect();

    let output = serde_json::json!({
        "week_start": week_start,
        "week_end": week_end,
        "summary": {
            "commitments_completed": completed_count,
            "commitments_open": open_commitments.len(),
            "nudges_sent": nudges.len(),
            "nudges_resolved": resolved_nudges,
            "critical_risk_events": 0
        },
        "completed_commitment_ids": completed_commitment_ids,
        "open_commitment_ids": open_commitment_ids,
        "top_commitment_patterns": [],
        "top_threads": top_threads,
        "drift_patterns": [],
        "alignment_observations": [],
        "vel_self_review": [],
        "suggested_adjustments": []
    });

    let artifact_root = &state.config.artifact_root;
    let date_str = now.date().to_string();
    let rel_path = format!("synthesis/week/{}/run_{}.json", date_str, run_id.as_ref().replace([':', '-'], "_"));
    let full_path = std::path::Path::new(artifact_root).join(&rel_path);
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::internal(format!("create_dir_all: {}", e)))?;
    }
    let content = serde_json::to_string_pretty(&output).map_err(|e| AppError::internal(e.to_string()))?;
    let content_bytes = content.as_bytes();
    let mut hasher = sha2::Sha256::new();
    hasher.update(content_bytes);
    let hash_hex = hex::encode(hasher.finalize());
    let content_hash = format!("sha256:{}", hash_hex);

    let temp_path = full_path.with_extension("tmp");
    {
        let mut f = std::fs::File::create(&temp_path).map_err(|e| AppError::internal(e.to_string()))?;
        std::io::Write::write_all(&mut f, content_bytes).map_err(|e| AppError::internal(e.to_string()))?;
        f.sync_all().map_err(|e| AppError::internal(e.to_string()))?;
    }
    std::fs::rename(&temp_path, &full_path).map_err(|e| AppError::internal(e.to_string()))?;

    let storage_uri = rel_path.clone();
    let artifact_id = state
        .storage
        .create_artifact(ArtifactInsert {
            artifact_type: "weekly_synthesis".to_string(),
            title: Some("Weekly synthesis".to_string()),
            mime_type: Some("application/json".to_string()),
            storage_uri,
            storage_kind: ArtifactStorageKind::Managed,
            privacy_class: PrivacyClass::Private,
            sync_class: SyncClass::Warm,
            content_hash: Some(content_hash),
            size_bytes: Some(content_bytes.len() as i64),
            metadata_json: Some(serde_json::json!({ "synthesis_kind": "week", "window_days": 7, "week_start": week_start, "week_end": week_end })),
        })
        .await?;
    state.storage.append_run_event(run_id.as_ref(), 3, RunEventType::ArtifactWritten, &serde_json::json!({ "artifact_id": artifact_id.to_string() })).await?;
    let ref_ = Ref::new("run", run_id.as_ref(), "artifact", artifact_id.as_ref(), RefRelationType::AttachedTo);
    state.storage.create_ref(&ref_).await?;
    state.storage.append_run_event(run_id.as_ref(), 4, RunEventType::RefsCreated, &serde_json::json!({})).await?;

    let _output_json = serde_json::to_string(&output).map_err(|e| AppError::internal(e.to_string()))?;
    let finished_at = OffsetDateTime::now_utc().unix_timestamp();
    state.storage.update_run_status(run_id.as_ref(), RunStatus::Succeeded, None, Some(finished_at), Some(&output), None).await?;
    state.storage.append_run_event(run_id.as_ref(), 5, RunEventType::RunSucceeded, &serde_json::json!({})).await?;

    Ok((run_id, artifact_id))
}

/// Run project-scoped synthesis: filter by project slug, produce project_synthesis artifact.
pub async fn run_project_synthesis(state: &AppState, project_slug: &str) -> Result<(RunId, ArtifactId), AppError> {
    let run_id = RunId::new();
    let input_json = serde_json::json!({ "synthesis_kind": "project", "project_slug": project_slug });

    state.storage.create_run(&run_id, RunKind::Synthesis, &input_json).await?;
    let started_at = OffsetDateTime::now_utc().unix_timestamp();
    state.storage.update_run_status(run_id.as_ref(), RunStatus::Running, Some(started_at), None, None, None).await?;
    state.storage.append_run_event(run_id.as_ref(), 2, RunEventType::RunStarted, &serde_json::json!({})).await?;

    let now = OffsetDateTime::now_utc();
    let _seven_days_ago = (now - time::Duration::days(7)).unix_timestamp();

    let open_commitments = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), Some(project_slug), None, 200)
        .await?;
    let all_project_commitments = state.storage.list_commitments(None, Some(project_slug), None, 500).await?;
    let completed_commitment_ids: Vec<String> = all_project_commitments
        .iter()
        .filter(|c| c.status == CommitmentStatus::Done)
        .take(50)
        .map(|c| c.id.as_ref().to_string())
        .collect();
    let open_commitment_ids: Vec<String> = open_commitments.iter().map(|c| c.id.as_ref().to_string()).collect();
    let thread_rows = state.storage.list_threads(Some("open"), 20).await.unwrap_or_default();
    let top_threads: Vec<serde_json::Value> = thread_rows
        .into_iter()
        .map(|(id, thread_type, title, status, _ca, _ua)| {
            serde_json::json!({ "id": id, "thread_type": thread_type, "title": title, "status": status })
        })
        .collect();

    let output = serde_json::json!({
        "project_slug": project_slug,
        "computed_at": now.unix_timestamp(),
        "open_commitments": serde_json::json!({
            "commitment_ids": open_commitment_ids,
            "evidence_refs": []
        }),
        "active_threads": top_threads,
        "repeated_drift": [],
        "ideation_without_execution": [],
        "suggested_next_actions": [],
        "completed_commitment_ids": completed_commitment_ids,
    });

    let artifact_root = &state.config.artifact_root;
    let date_str = now.date().to_string();
    let rel_path = format!("synthesis/project/{}/run_{}.json", date_str, run_id.as_ref().replace([':', '-'], "_"));
    let full_path = std::path::Path::new(artifact_root).join(&rel_path);
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError::internal(format!("create_dir_all: {}", e)))?;
    }
    let content = serde_json::to_string_pretty(&output).map_err(|e| AppError::internal(e.to_string()))?;
    let content_bytes = content.as_bytes();
    let mut hasher = sha2::Sha256::new();
    hasher.update(content_bytes);
    let hash_hex = hex::encode(hasher.finalize());
    let content_hash = format!("sha256:{}", hash_hex);

    let temp_path = full_path.with_extension("tmp");
    {
        let mut f = std::fs::File::create(&temp_path).map_err(|e| AppError::internal(e.to_string()))?;
        std::io::Write::write_all(&mut f, content_bytes).map_err(|e| AppError::internal(e.to_string()))?;
        f.sync_all().map_err(|e| AppError::internal(e.to_string()))?;
    }
    std::fs::rename(&temp_path, &full_path).map_err(|e| AppError::internal(e.to_string()))?;

    let storage_uri = rel_path.clone();
    let artifact_id = state
        .storage
        .create_artifact(ArtifactInsert {
            artifact_type: "project_synthesis".to_string(),
            title: Some(format!("Project synthesis: {}", project_slug)),
            mime_type: Some("application/json".to_string()),
            storage_uri,
            storage_kind: ArtifactStorageKind::Managed,
            privacy_class: PrivacyClass::Private,
            sync_class: SyncClass::Warm,
            content_hash: Some(content_hash),
            size_bytes: Some(content_bytes.len() as i64),
            metadata_json: Some(serde_json::json!({ "synthesis_kind": "project", "project_slug": project_slug })),
        })
        .await?;
    state.storage.append_run_event(run_id.as_ref(), 3, RunEventType::ArtifactWritten, &serde_json::json!({ "artifact_id": artifact_id.to_string() })).await?;
    let ref_ = Ref::new("run", run_id.as_ref(), "artifact", artifact_id.as_ref(), RefRelationType::AttachedTo);
    state.storage.create_ref(&ref_).await?;
    state.storage.append_run_event(run_id.as_ref(), 4, RunEventType::RefsCreated, &serde_json::json!({})).await?;

    let finished_at = OffsetDateTime::now_utc().unix_timestamp();
    state.storage.update_run_status(run_id.as_ref(), RunStatus::Succeeded, None, Some(finished_at), Some(&output), None).await?;
    state.storage.append_run_event(run_id.as_ref(), 5, RunEventType::RunSucceeded, &serde_json::json!({})).await?;

    Ok((run_id, artifact_id))
}
