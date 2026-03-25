use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{CaptureId, PrivacyClass, ProjectId, ProjectProvisionRequest, ProjectRecord, ProjectRootRef, ProjectStatus, ProjectFamily};

use crate::db::{CaptureInsert, SignalInsert, StorageError};
use crate::repositories::{captures_repo, projects_repo, signals_repo};

#[derive(Debug, Clone)]
pub enum BatchImportStorageItem {
    Capture {
        capture_id: String,
        content_text: String,
        capture_type: String,
        source_device: Option<String>,
    },
    Signal {
        signal_type: String,
        source: String,
        source_ref: Option<String>,
        timestamp: i64,
        payload: JsonValue,
    },
    Project {
        slug: String,
        name: String,
        family: ProjectFamily,
        status: ProjectStatus,
        primary_repo: ProjectRootRef,
        primary_notes_root: ProjectRootRef,
        secondary_repos: Vec<ProjectRootRef>,
        secondary_notes_roots: Vec<ProjectRootRef>,
        upstream_ids: std::collections::BTreeMap<String, String>,
        pending_provision: ProjectProvisionRequest,
    },
}

#[derive(Debug, Clone)]
pub struct BatchImportStorageResult {
    pub index: usize,
    pub kind: String,
    pub status: BatchImportStorageStatus,
    pub id: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchImportStorageStatus {
    Created,
    Skipped,
    Error,
}

pub(crate) async fn import_batch(
    pool: &SqlitePool,
    items: Vec<BatchImportStorageItem>,
) -> Result<Vec<BatchImportStorageResult>, StorageError> {
    let mut tx = pool.begin().await?;
    let mut results = Vec::with_capacity(items.len());

    for (index, item) in items.into_iter().enumerate() {
        let result = match item {
            BatchImportStorageItem::Capture {
                capture_id,
                content_text,
                capture_type,
                source_device,
            } => {
                let cid = CaptureId::from(capture_id.clone());
                let now = OffsetDateTime::now_utc().unix_timestamp();
                let inserted = captures_repo::insert_capture_with_id_in_tx(
                    &mut tx,
                    cid,
                    CaptureInsert {
                        content_text,
                        capture_type,
                        source_device,
                        privacy_class: PrivacyClass::Private,
                    },
                    now,
                )
                .await;

                match inserted {
                    Ok(true) => BatchImportStorageResult {
                        index,
                        kind: "capture".to_string(),
                        status: BatchImportStorageStatus::Created,
                        id: Some(capture_id),
                        message: None,
                    },
                    Ok(false) => BatchImportStorageResult {
                        index,
                        kind: "capture".to_string(),
                        status: BatchImportStorageStatus::Skipped,
                        id: Some(capture_id),
                        message: Some("already exists".to_string()),
                    },
                    Err(e) => BatchImportStorageResult {
                        index,
                        kind: "capture".to_string(),
                        status: BatchImportStorageStatus::Error,
                        id: Some(capture_id),
                        message: Some(e.to_string()),
                    },
                }
            }
            BatchImportStorageItem::Signal {
                signal_type,
                source,
                source_ref,
                timestamp,
                payload,
            } => {
                let signal_id = signals_repo::insert_signal_in_tx(
                    &mut tx,
                    &SignalInsert {
                        signal_type,
                        source,
                        source_ref,
                        timestamp,
                        payload_json: Some(payload),
                    },
                )
                .await;

                match signal_id {
                    Ok(id) => BatchImportStorageResult {
                        index,
                        kind: "signal".to_string(),
                        status: BatchImportStorageStatus::Created,
                        id: Some(id),
                        message: None,
                    },
                    Err(e) => BatchImportStorageResult {
                        index,
                        kind: "signal".to_string(),
                        status: BatchImportStorageStatus::Error,
                        id: None,
                        message: Some(e.to_string()),
                    },
                }
            }
            BatchImportStorageItem::Project {
                slug,
                name,
                family,
                status,
                primary_repo,
                primary_notes_root,
                secondary_repos,
                secondary_notes_roots,
                upstream_ids,
                pending_provision,
            } => {
                let existing =
                    projects_repo::get_project_by_slug_in_tx(&mut tx, &slug).await;
                match existing {
                    Ok(Some(_)) => BatchImportStorageResult {
                        index,
                        kind: "project".to_string(),
                        status: BatchImportStorageStatus::Skipped,
                        id: None,
                        message: Some("slug already exists".to_string()),
                    },
                    Ok(None) => {
                        let now = OffsetDateTime::now_utc();
                        let project_id = ProjectId::new();
                        let record = ProjectRecord {
                            id: project_id.clone(),
                            slug,
                            name,
                            family,
                            status,
                            primary_repo,
                            primary_notes_root,
                            secondary_repos,
                            secondary_notes_roots,
                            upstream_ids,
                            pending_provision,
                            created_at: now,
                            updated_at: now,
                            archived_at: None,
                        };
                        match projects_repo::create_project_in_tx(&mut tx, record).await {
                            Ok(created) => BatchImportStorageResult {
                                index,
                                kind: "project".to_string(),
                                status: BatchImportStorageStatus::Created,
                                id: Some(created.id.to_string()),
                                message: None,
                            },
                            Err(e) => BatchImportStorageResult {
                                index,
                                kind: "project".to_string(),
                                status: BatchImportStorageStatus::Error,
                                id: None,
                                message: Some(e.to_string()),
                            },
                        }
                    }
                    Err(e) => BatchImportStorageResult {
                        index,
                        kind: "project".to_string(),
                        status: BatchImportStorageStatus::Error,
                        id: None,
                        message: Some(e.to_string()),
                    },
                }
            }
        };
        results.push(result);
    }

    tx.commit().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::SqlitePool;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn import_batch_mixed_items() {
        let pool = test_pool().await;

        let items = vec![
            BatchImportStorageItem::Capture {
                capture_id: "cap_test_001".to_string(),
                content_text: "hello world".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("test".to_string()),
            },
            BatchImportStorageItem::Signal {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: Some("evt-1".to_string()),
                timestamp: 1_700_000_000,
                payload: json!({"title": "standup"}),
            },
            BatchImportStorageItem::Project {
                slug: "test-project".to_string(),
                name: "Test Project".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/test".to_string(),
                    label: "test".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/notes/test".to_string(),
                    label: "test".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: std::collections::BTreeMap::new(),
                pending_provision: ProjectProvisionRequest {
                    create_repo: false,
                    create_notes_root: false,
                },
            },
        ];

        let results = import_batch(&pool, items).await.unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].status, BatchImportStorageStatus::Created);
        assert_eq!(results[0].kind, "capture");
        assert_eq!(results[1].status, BatchImportStorageStatus::Created);
        assert_eq!(results[1].kind, "signal");
        assert_eq!(results[2].status, BatchImportStorageStatus::Created);
        assert_eq!(results[2].kind, "project");
    }

    #[tokio::test]
    async fn import_batch_idempotent() {
        let pool = test_pool().await;

        let items = vec![
            BatchImportStorageItem::Capture {
                capture_id: "cap_idem_001".to_string(),
                content_text: "idempotent".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: None,
            },
            BatchImportStorageItem::Project {
                slug: "idem-proj".to_string(),
                name: "Idem Project".to_string(),
                family: ProjectFamily::Personal,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/idem".to_string(),
                    label: "idem".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/notes/idem".to_string(),
                    label: "idem".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: std::collections::BTreeMap::new(),
                pending_provision: ProjectProvisionRequest {
                    create_repo: false,
                    create_notes_root: false,
                },
            },
        ];

        let first = import_batch(&pool, items.clone()).await.unwrap();
        assert_eq!(first[0].status, BatchImportStorageStatus::Created);
        assert_eq!(first[1].status, BatchImportStorageStatus::Created);

        let second = import_batch(&pool, items).await.unwrap();
        assert_eq!(second[0].status, BatchImportStorageStatus::Skipped);
        assert_eq!(second[1].status, BatchImportStorageStatus::Skipped);
    }
}
