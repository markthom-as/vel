use sqlx::SqlitePool;
use time::OffsetDateTime;

use crate::{
    db::StorageError,
    get_registry_object, upsert_registry_object, CanonicalRegistryRecord,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BootstrapReport {
    pub seeded: usize,
    pub unchanged: usize,
}

pub async fn bootstrap_canonical_registry(
    pool: &SqlitePool,
) -> Result<BootstrapReport, StorageError> {
    let now = OffsetDateTime::now_utc();
    let seeds = vec![
        CanonicalRegistryRecord {
            id: "module.integration.todoist".to_string(),
            registry_type: "module".to_string(),
            namespace: "integration".to_string(),
            slug: "todoist".to_string(),
            display_name: "Todoist".to_string(),
            version: "0.5".to_string(),
            status: "active".to_string(),
            manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
            overlay_json: serde_json::json!({"bootstrap":"deterministic"}),
            created_at: now,
            updated_at: now,
        },
        CanonicalRegistryRecord {
            id: "module.integration.google-calendar".to_string(),
            registry_type: "module".to_string(),
            namespace: "integration".to_string(),
            slug: "google-calendar".to_string(),
            display_name: "Google Calendar".to_string(),
            version: "0.5".to_string(),
            status: "active".to_string(),
            manifest_ref: "modules/integration/google-calendar/module.yaml".to_string(),
            overlay_json: serde_json::json!({"bootstrap":"deterministic"}),
            created_at: now,
            updated_at: now,
        },
    ];

    let mut report = BootstrapReport::default();
    for seed in seeds {
        reconcile_registry_seed(pool, seed, &mut report).await?;
    }

    Ok(report)
}

async fn reconcile_registry_seed(
    pool: &SqlitePool,
    seed: CanonicalRegistryRecord,
    report: &mut BootstrapReport,
) -> Result<(), StorageError> {
    match get_registry_object(pool, &seed.id).await? {
        Some(existing)
            if existing.registry_type == seed.registry_type
                && existing.namespace == seed.namespace
                && existing.slug == seed.slug
                && existing.display_name == seed.display_name
                && existing.version == seed.version
                && existing.status == seed.status
                && existing.manifest_ref == seed.manifest_ref
                && existing.overlay_json == seed.overlay_json =>
        {
            report.unchanged += 1;
            Ok(())
        }
        _ => {
            upsert_registry_object(pool, &seed).await?;
            report.seeded += 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    use crate::{list_registry_objects, migrate_storage};

    #[tokio::test]
    async fn bootstrap_is_idempotent() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();

        let first = bootstrap_canonical_registry(&pool).await.unwrap();
        let second = bootstrap_canonical_registry(&pool).await.unwrap();
        let records = list_registry_objects(&pool).await.unwrap();

        assert_eq!(first.seeded, 2);
        assert_eq!(second.unchanged, 2);
        assert_eq!(records.len(), 2);
    }
}
