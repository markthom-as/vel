use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;

use crate::{
    db::StorageError, get_canonical_object, list_sync_links_for_object, rebuild_projection,
    upsert_projection, ProjectionRecord,
};

pub async fn rebuild_source_summary_projection(
    pool: &SqlitePool,
    object_id: &str,
) -> Result<ProjectionRecord, StorageError> {
    let object = get_canonical_object(pool, object_id)
        .await?
        .ok_or_else(|| {
            StorageError::NotFound(format!(
                "canonical object {object_id} missing for projection rebuild"
            ))
        })?;
    let links = list_sync_links_for_object(pool, object_id).await?;

    let active_links: Vec<_> = links
        .iter()
        .filter(|link| !matches!(link.state.as_str(), "deleted_upstream" | "superseded"))
        .collect();
    let providers: Vec<String> = active_links
        .iter()
        .map(|link| link.provider.clone())
        .collect();
    let primary_provider = providers.first().cloned();
    let sync_state = if active_links.iter().any(|link| link.state == "conflicted") {
        "conflicted"
    } else if active_links.iter().any(|link| link.state == "stale") {
        "stale"
    } else if active_links.is_empty() {
        "unlinked"
    } else {
        "healthy"
    };
    let last_sync_at = active_links.iter().map(|link| link.last_seen_at).max();

    let source_summary = json!({
        "active_link_count": active_links.len(),
        "providers": providers,
        "primary_provider": primary_provider,
        "sync_state": sync_state,
        "last_sync_at": last_sync_at.map(|value| value.unix_timestamp()),
        "basis": "derived"
    });
    let projection_id = format!("projection.source_summary.{object_id}");
    let now = OffsetDateTime::now_utc();

    let existing = crate::get_projection(pool, &projection_id).await?;
    if existing.is_none() {
        upsert_projection(
            pool,
            &ProjectionRecord {
                id: projection_id.clone(),
                projection_type: "source_summary".to_string(),
                object_id: Some(object_id.to_string()),
                source_summary_json: Some(source_summary.clone()),
                projection_json: json!({
                    "object_id": object.id,
                    "object_status": object.status,
                    "source_summary": source_summary
                }),
                rebuild_token: Some(format!("rebuild:{}", now.unix_timestamp())),
                created_at: now,
                updated_at: now,
            },
        )
        .await?;
    }

    rebuild_projection(
        pool,
        &projection_id,
        Some(&source_summary),
        &json!({
            "object_id": object.id,
            "object_status": object.status,
            "source_summary": source_summary
        }),
        &format!("rebuild:{}", now.unix_timestamp()),
    )
    .await
}
