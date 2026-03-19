use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use vel_core::ProjectId;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

pub(crate) async fn upsert_execution_context(
    pool: &SqlitePool,
    project_id: &ProjectId,
    context_json: &JsonValue,
    now: OffsetDateTime,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO execution_contexts (
            project_id,
            context_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?)
        ON CONFLICT(project_id) DO UPDATE SET
            context_json = excluded.context_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(project_id.as_ref())
    .bind(serde_json::to_string(context_json)?)
    .bind(now.unix_timestamp())
    .bind(now.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub(crate) async fn get_execution_context(
    pool: &SqlitePool,
    project_id: &str,
) -> Result<Option<(JsonValue, OffsetDateTime, OffsetDateTime)>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT context_json, created_at, updated_at
        FROM execution_contexts
        WHERE project_id = ?
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_execution_context_row).transpose()
}

fn map_execution_context_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<(JsonValue, OffsetDateTime, OffsetDateTime), StorageError> {
    Ok((
        serde_json::from_str(&row.try_get::<String, _>("context_json")?)?,
        timestamp_to_datetime(row.try_get("created_at")?)?,
        timestamp_to_datetime(row.try_get("updated_at")?)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use vel_core::{
        ProjectFamily, ProjectProvisionRequest, ProjectRecord, ProjectRootRef, ProjectStatus,
    };

    fn test_project(id: &str, slug: &str) -> ProjectRecord {
        let now = OffsetDateTime::now_utc();
        ProjectRecord {
            id: ProjectId::from(id.to_string()),
            slug: slug.to_string(),
            name: format!("Project {}", slug),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: format!("/tmp/{slug}/repo"),
                label: slug.to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: format!("/tmp/{slug}/notes"),
                label: format!("{slug}-notes"),
                kind: "notes_root".to_string(),
            },
            secondary_repos: Vec::new(),
            secondary_notes_roots: Vec::new(),
            upstream_ids: BTreeMap::new(),
            pending_provision: ProjectProvisionRequest::default(),
            created_at: now,
            updated_at: now,
            archived_at: None,
        }
    }

    #[tokio::test]
    async fn execution_context_repo_is_keyed_by_project_id() {
        let storage = crate::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let first = test_project("proj_exec_repo_1", "exec-repo-1");
        let second = test_project("proj_exec_repo_2", "exec-repo-2");
        storage.create_project(first.clone()).await.unwrap();
        storage.create_project(second.clone()).await.unwrap();

        let created_at = OffsetDateTime::now_utc();
        storage
            .upsert_project_execution_context(
                &first.id,
                &serde_json::json!({
                    "objective": "persist first project context",
                    "constraints": ["keep repo local"]
                }),
                created_at,
            )
            .await
            .unwrap();

        let first_context = storage
            .get_project_execution_context(first.id.as_ref())
            .await
            .unwrap()
            .expect("context for first project");
        assert_eq!(
            first_context.0["objective"],
            "persist first project context"
        );

        assert!(storage
            .get_project_execution_context(second.id.as_ref())
            .await
            .unwrap()
            .is_none());

        let updated_at = created_at + time::Duration::seconds(30);
        storage
            .upsert_project_execution_context(
                &first.id,
                &serde_json::json!({
                    "objective": "updated objective",
                    "constraints": ["keep repo local", "sidecar only"]
                }),
                updated_at,
            )
            .await
            .unwrap();

        let updated = storage
            .get_project_execution_context(first.id.as_ref())
            .await
            .unwrap()
            .expect("updated context");
        assert_eq!(updated.0["objective"], "updated objective");
        assert_eq!(updated.2.unix_timestamp(), updated_at.unix_timestamp());
    }
}
