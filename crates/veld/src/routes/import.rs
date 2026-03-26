use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, BatchImportItem, BatchImportItemResult, BatchImportItemStatus, BatchImportRequest,
    BatchImportResponse, BatchImportSummary,
};
use vel_core::{ProjectFamily, ProjectProvisionRequest, ProjectRootRef, ProjectStatus};
use vel_storage::{BatchImportStorageItem, BatchImportStorageStatus};

use crate::{errors::AppError, state::AppState};

pub async fn import_batch(
    State(state): State<AppState>,
    Json(payload): Json<BatchImportRequest>,
) -> Result<Json<ApiResponse<BatchImportResponse>>, AppError> {
    if payload.items.is_empty() {
        return Err(AppError::bad_request("items must not be empty"));
    }
    if payload.items.len() > 5000 {
        return Err(AppError::bad_request(
            "batch size exceeds maximum of 5000 items",
        ));
    }

    let storage_items: Vec<BatchImportStorageItem> =
        payload.items.into_iter().map(map_to_storage_item).collect();

    let storage_results = state.storage.import_batch(storage_items).await?;

    let mut summary = BatchImportSummary {
        total: storage_results.len(),
        created: 0,
        skipped: 0,
        errors: 0,
    };

    let results: Vec<BatchImportItemResult> = storage_results
        .into_iter()
        .map(|r| {
            let status = match r.status {
                BatchImportStorageStatus::Created => {
                    summary.created += 1;
                    BatchImportItemStatus::Created
                }
                BatchImportStorageStatus::Skipped => {
                    summary.skipped += 1;
                    BatchImportItemStatus::Skipped
                }
                BatchImportStorageStatus::Error => {
                    summary.errors += 1;
                    BatchImportItemStatus::Error
                }
            };
            BatchImportItemResult {
                index: r.index,
                kind: r.kind,
                status,
                id: r.id,
                message: r.message,
            }
        })
        .collect();

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        BatchImportResponse { results, summary },
        request_id,
    )))
}

fn map_to_storage_item(item: BatchImportItem) -> BatchImportStorageItem {
    match item {
        BatchImportItem::Capture(c) => BatchImportStorageItem::Capture {
            capture_id: c.capture_id,
            content_text: c.content_text,
            capture_type: c.capture_type,
            source_device: c.source_device,
        },
        BatchImportItem::Signal(s) => BatchImportStorageItem::Signal {
            signal_type: s.signal_type,
            source: s.source,
            source_ref: s.source_ref,
            timestamp: s.timestamp.unwrap_or(0),
            payload: s.payload,
        },
        BatchImportItem::Project(p) => {
            let family = match p.family {
                vel_api_types::ProjectFamilyData::Personal => ProjectFamily::Personal,
                vel_api_types::ProjectFamilyData::Creative => ProjectFamily::Creative,
                vel_api_types::ProjectFamilyData::Work => ProjectFamily::Work,
            };
            let status = match p.status {
                Some(vel_api_types::ProjectStatusData::Paused) => ProjectStatus::Paused,
                Some(vel_api_types::ProjectStatusData::Archived) => ProjectStatus::Archived,
                _ => ProjectStatus::Active,
            };
            BatchImportStorageItem::Project {
                slug: p.slug,
                name: p.name,
                family,
                status,
                primary_repo: map_root_ref(p.primary_repo),
                primary_notes_root: map_root_ref(p.primary_notes_root),
                secondary_repos: p.secondary_repos.into_iter().map(map_root_ref).collect(),
                secondary_notes_roots: p
                    .secondary_notes_roots
                    .into_iter()
                    .map(map_root_ref)
                    .collect(),
                upstream_ids: p.upstream_ids,
                pending_provision: ProjectProvisionRequest {
                    create_repo: p.pending_provision.create_repo,
                    create_notes_root: p.pending_provision.create_notes_root,
                },
            }
        }
    }
}

fn map_root_ref(data: vel_api_types::ProjectRootRefData) -> ProjectRootRef {
    ProjectRootRef {
        path: data.path,
        label: data.label,
        kind: data.kind,
    }
}
