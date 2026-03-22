use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::SqlitePool;

use crate::{
    db::StorageError,
    get_canonical_object, insert_canonical_object, update_canonical_object, CanonicalObjectRecord,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationArtifactRecord {
    pub id: String,
    pub version: String,
    pub snapshot_ref: String,
    pub validation: JsonValue,
    pub objects: Vec<CanonicalObjectRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationValidationReport {
    pub valid: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReplayReport {
    pub inserted: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub idempotent: bool,
}

pub fn validate_migration_artifact(
    artifact: &MigrationArtifactRecord,
) -> MigrationValidationReport {
    let mut issues = Vec::new();

    if artifact.snapshot_ref.trim().is_empty() {
        issues.push("snapshot_ref must not be empty".to_string());
    }
    if artifact.version.trim().is_empty() {
        issues.push("version must not be empty".to_string());
    }

    MigrationValidationReport {
        valid: issues.is_empty(),
        issues,
    }
}

pub async fn replay_migration_artifact(
    pool: &SqlitePool,
    artifact: &MigrationArtifactRecord,
) -> Result<MigrationReplayReport, StorageError> {
    let validation = validate_migration_artifact(artifact);
    if !validation.valid {
        return Err(StorageError::Validation(format!(
            "migration artifact validation failed: {}",
            validation.issues.join(", ")
        )));
    }

    let mut report = MigrationReplayReport {
        inserted: 0,
        updated: 0,
        unchanged: 0,
        idempotent: true,
    };

    for object in &artifact.objects {
        match get_canonical_object(pool, &object.id).await? {
            None => {
                insert_canonical_object(pool, object).await?;
                report.inserted += 1;
            }
            Some(existing)
                if existing.object_type == object.object_type
                    && existing.object_class == object.object_class
                    && existing.schema_version == object.schema_version
                    && existing.status == object.status
                    && existing.provenance_json == object.provenance_json
                    && existing.facets_json == object.facets_json
                    && existing.source_summary_json == object.source_summary_json
                    && existing.archived_at == object.archived_at
                    && existing.deleted_at == object.deleted_at =>
            {
                report.unchanged += 1;
            }
            Some(existing) => {
                update_canonical_object(
                    pool,
                    &existing.id,
                    existing.revision,
                    &object.status,
                    &object.facets_json,
                    object.source_summary_json.as_ref(),
                    object.archived_at,
                )
                .await?;
                report.updated += 1;
            }
        }
    }

    Ok(report)
}
