use std::collections::BTreeMap;
use std::path::Path;

use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use vel_core::{ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord, ProjectRootRef};

use crate::{db::StorageError, mapping::timestamp_to_datetime, repositories::semantic_memory_repo};

pub(crate) async fn create_project(
    pool: &SqlitePool,
    project: ProjectRecord,
) -> Result<ProjectRecord, StorageError> {
    let mut tx = pool.begin().await?;
    let result = create_project_in_tx(&mut tx, project).await?;
    tx.commit().await?;
    Ok(result)
}

pub(crate) async fn create_project_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    project: ProjectRecord,
) -> Result<ProjectRecord, StorageError> {
    let secondary_repo_paths = project
        .secondary_repos
        .iter()
        .map(|root| root.path.clone())
        .collect::<Vec<_>>();
    let secondary_notes_roots = project
        .secondary_notes_roots
        .iter()
        .map(|root| root.path.clone())
        .collect::<Vec<_>>();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(project.id.as_ref())
    .bind(&project.slug)
    .bind(&project.name)
    .bind(project.family.to_string())
    .bind(project.status.to_string())
    .bind(&project.primary_repo.path)
    .bind(&project.primary_notes_root.path)
    .bind(serde_json::to_string(&secondary_repo_paths)?)
    .bind(serde_json::to_string(&secondary_notes_roots)?)
    .bind(serde_json::to_string(&project.upstream_ids)?)
    .bind(serde_json::to_string(&project.pending_provision)?)
    .bind(project.created_at.unix_timestamp())
    .bind(project.updated_at.unix_timestamp())
    .bind(project.archived_at.map(|value| value.unix_timestamp()))
    .execute(&mut **tx)
    .await?;

    upsert_project_alias_in_tx(tx, &project.slug, &project.id, "slug").await?;
    upsert_project_alias_in_tx(tx, &project.name, &project.id, "name").await?;
    semantic_memory_repo::upsert_project_record_in_tx(tx, &project).await?;

    Ok(project)
}

pub(crate) async fn list_projects(pool: &SqlitePool) -> Result<Vec<ProjectRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        FROM projects
        ORDER BY updated_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_project_row).collect()
}

pub(crate) async fn get_project(
    pool: &SqlitePool,
    project_id: &str,
) -> Result<Option<ProjectRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        FROM projects
        WHERE id = ?
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_project_row).transpose()
}

pub(crate) async fn get_project_by_slug_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    slug: &str,
) -> Result<Option<ProjectRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        FROM projects
        WHERE slug = ?
        "#,
    )
    .bind(slug)
    .fetch_optional(&mut **tx)
    .await?;

    row.as_ref().map(map_project_row).transpose()
}

pub(crate) async fn get_project_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<ProjectRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        FROM projects
        WHERE slug = ?
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_project_row).transpose()
}

pub(crate) async fn get_project_by_upstream_id(
    pool: &SqlitePool,
    provider_key: &str,
    upstream_id: &str,
) -> Result<Option<ProjectRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        FROM projects
        WHERE json_extract(upstream_ids_json, ?) = ?
        LIMIT 1
        "#,
    )
    .bind(format!("$.{}", provider_key.trim()))
    .bind(upstream_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_project_row).transpose()
}

pub(crate) async fn upsert_project_alias(
    pool: &SqlitePool,
    alias: &str,
    project_id: &ProjectId,
    source: &str,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    upsert_project_alias_in_tx(&mut tx, alias, project_id, source).await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn list_project_families(
    pool: &SqlitePool,
) -> Result<Vec<ProjectFamily>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT family
        FROM projects
        ORDER BY family ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            let family: String = row.try_get("family")?;
            family.parse().map_err(|error: vel_core::VelCoreError| {
                StorageError::Validation(error.to_string())
            })
        })
        .collect()
}

async fn upsert_project_alias_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    alias: &str,
    project_id: &ProjectId,
    source: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO project_aliases (alias, project_id, source)
        VALUES (?, ?, ?)
        ON CONFLICT(alias) DO UPDATE SET
            project_id = excluded.project_id,
            source = excluded.source
        "#,
    )
    .bind(alias)
    .bind(project_id.as_ref())
    .bind(source)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

fn map_project_row(row: &sqlx::sqlite::SqliteRow) -> Result<ProjectRecord, StorageError> {
    let family: String = row.try_get("family")?;
    let status: String = row.try_get("status")?;
    let secondary_repo_paths: Vec<String> =
        serde_json::from_str(&row.try_get::<String, _>("secondary_repo_paths_json")?)?;
    let secondary_notes_roots: Vec<String> =
        serde_json::from_str(&row.try_get::<String, _>("secondary_notes_roots_json")?)?;
    let upstream_ids: BTreeMap<String, String> =
        serde_json::from_str(&row.try_get::<String, _>("upstream_ids_json")?)?;
    let pending_provision: ProjectProvisionRequest =
        serde_json::from_str(&row.try_get::<String, _>("pending_provision_json")?)?;

    let primary_repo_path: String = row.try_get("primary_repo_path")?;
    let primary_notes_root: String = row.try_get("primary_notes_root")?;
    let created_at: i64 = row.try_get("created_at")?;
    let updated_at: i64 = row.try_get("updated_at")?;
    let archived_at = row
        .try_get::<Option<i64>, _>("archived_at")?
        .map(timestamp_to_datetime)
        .transpose()?;

    Ok(ProjectRecord {
        id: ProjectId::from(row.try_get::<String, _>("id")?),
        slug: row.try_get("slug")?,
        name: row.try_get("name")?,
        family: family
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        status: status
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        primary_repo: path_root_ref(&primary_repo_path, "repo"),
        primary_notes_root: path_root_ref(&primary_notes_root, "notes_root"),
        secondary_repos: secondary_repo_paths
            .into_iter()
            .map(|path| path_root_ref(&path, "repo"))
            .collect(),
        secondary_notes_roots: secondary_notes_roots
            .into_iter()
            .map(|path| path_root_ref(&path, "notes_root"))
            .collect(),
        upstream_ids,
        pending_provision,
        created_at: timestamp_to_datetime(created_at)?,
        updated_at: timestamp_to_datetime(updated_at)?,
        archived_at,
    })
}

fn path_root_ref(path: &str, kind: &str) -> ProjectRootRef {
    let label = Path::new(path)
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(path)
        .to_string();

    ProjectRootRef {
        path: path.to_string(),
        label,
        kind: kind.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{migrate::Migrator, SqlitePool};
    use time::OffsetDateTime;
    use vel_core::ProjectStatus;

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_project() -> ProjectRecord {
        let now = OffsetDateTime::now_utc();
        ProjectRecord {
            id: ProjectId::new(),
            slug: "vel-runtime".to_string(),
            name: "Vel Runtime".to_string(),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: "/tmp/vel-runtime".to_string(),
                label: "vel-runtime".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: "/tmp/notes/vel-runtime".to_string(),
                label: "vel-runtime".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![ProjectRootRef {
                path: "/tmp/vel-runtime-web".to_string(),
                label: "vel-runtime-web".to_string(),
                kind: "repo".to_string(),
            }],
            secondary_notes_roots: vec![],
            upstream_ids: BTreeMap::from([("todoist".to_string(), "proj_123".to_string())]),
            pending_provision: ProjectProvisionRequest {
                create_repo: true,
                create_notes_root: false,
            },
            created_at: now,
            updated_at: now,
            archived_at: None,
        }
    }

    #[tokio::test]
    async fn projects_repo_persists_and_reads_project_records() {
        let pool = test_pool().await;

        let created = create_project(&pool, sample_project()).await.unwrap();
        let fetched = get_project(&pool, created.id.as_ref())
            .await
            .unwrap()
            .expect("project should exist");

        assert_eq!(fetched.slug, "vel-runtime");
        assert_eq!(fetched.family, ProjectFamily::Work);
        assert_eq!(fetched.primary_repo.path, "/tmp/vel-runtime");
        assert!(fetched.pending_provision.create_repo);

        let by_slug = get_project_by_slug(&pool, "vel-runtime")
            .await
            .unwrap()
            .expect("slug lookup should work");
        assert_eq!(by_slug.id, created.id);

        let by_upstream = get_project_by_upstream_id(&pool, "todoist", "proj_123")
            .await
            .unwrap()
            .expect("upstream lookup should work");
        assert_eq!(by_upstream.id, created.id);

        let listed = list_projects(&pool).await.unwrap();
        assert_eq!(listed.len(), 1);
    }

    #[tokio::test]
    async fn projects_repo_tracks_aliases_and_families() {
        let pool = test_pool().await;

        let created = create_project(&pool, sample_project()).await.unwrap();
        upsert_project_alias(
            &pool,
            "runtime-core",
            &created.id,
            "legacy_commitment_project",
        )
        .await
        .unwrap();

        let alias_row =
            sqlx::query("SELECT project_id, source FROM project_aliases WHERE alias = ?")
                .bind("runtime-core")
                .fetch_one(&pool)
                .await
                .unwrap();
        let project_id: String = alias_row.try_get("project_id").unwrap();
        let source: String = alias_row.try_get("source").unwrap();
        assert_eq!(project_id, created.id.as_ref());
        assert_eq!(source, "legacy_commitment_project");

        let families = list_project_families(&pool).await.unwrap();
        assert_eq!(families, vec![ProjectFamily::Work]);
    }
}
