use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use vel_core::{
    LinkScope, LinkStatus, LinkedNodeRecord, PairingTokenRecord, TrustedNodeEndpointKind,
    TrustedNodeEndpointRecord, TrustedNodeReachability,
};

use crate::{db::StorageError, mapping::timestamp_to_datetime};

pub(crate) async fn issue_pairing_token(
    pool: &SqlitePool,
    record: &PairingTokenRecord,
) -> Result<PairingTokenRecord, StorageError> {
    sqlx::query(
        r#"
        INSERT INTO pairing_tokens (
            token_id,
            token_code,
            issued_at,
            expires_at,
            issued_by_node_id,
            scopes_json,
            redeemed_at
        )
        VALUES (?, ?, ?, ?, ?, ?, NULL)
        "#,
    )
    .bind(&record.token_id)
    .bind(&record.token_code)
    .bind(record.issued_at.unix_timestamp())
    .bind(record.expires_at.unix_timestamp())
    .bind(&record.issued_by_node_id)
    .bind(serde_json::to_string(&record.scopes)?)
    .execute(pool)
    .await?;

    Ok(record.clone())
}

pub(crate) async fn get_pairing_token_by_code(
    pool: &SqlitePool,
    token_code: &str,
) -> Result<Option<(PairingTokenRecord, Option<OffsetDateTime>)>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT token_id, token_code, issued_at, expires_at, issued_by_node_id, scopes_json, redeemed_at
        FROM pairing_tokens
        WHERE token_code = ?
        "#,
    )
    .bind(token_code)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_pairing_token_row).transpose()
}

pub(crate) async fn mark_pairing_token_redeemed(
    pool: &SqlitePool,
    token_code: &str,
    redeemed_at: OffsetDateTime,
) -> Result<bool, StorageError> {
    let result = sqlx::query(
        r#"
        UPDATE pairing_tokens
        SET redeemed_at = ?
        WHERE token_code = ? AND redeemed_at IS NULL
        "#,
    )
    .bind(redeemed_at.unix_timestamp())
    .bind(token_code)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub(crate) async fn upsert_linked_node(
    pool: &SqlitePool,
    record: &LinkedNodeRecord,
) -> Result<LinkedNodeRecord, StorageError> {
    let revoked_at = (record.status == LinkStatus::Revoked).then_some(record.linked_at);

    sqlx::query(
        r#"
        INSERT INTO linked_nodes (
            node_id,
            node_display_name,
            status,
            scopes_json,
            linked_at,
            last_seen_at,
            transport_hint,
            sync_base_url,
            tailscale_base_url,
            lan_base_url,
            localhost_base_url,
            public_base_url,
            revoked_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(node_id) DO UPDATE SET
            node_display_name = excluded.node_display_name,
            status = excluded.status,
            scopes_json = excluded.scopes_json,
            linked_at = excluded.linked_at,
            last_seen_at = excluded.last_seen_at,
            transport_hint = excluded.transport_hint,
            sync_base_url = excluded.sync_base_url,
            tailscale_base_url = excluded.tailscale_base_url,
            lan_base_url = excluded.lan_base_url,
            localhost_base_url = excluded.localhost_base_url,
            public_base_url = excluded.public_base_url,
            revoked_at = excluded.revoked_at
        "#,
    )
    .bind(&record.node_id)
    .bind(&record.node_display_name)
    .bind(record.status.to_string())
    .bind(serde_json::to_string(&record.scopes)?)
    .bind(record.linked_at.unix_timestamp())
    .bind(record.last_seen_at.map(|value| value.unix_timestamp()))
    .bind(&record.transport_hint)
    .bind(&record.sync_base_url)
    .bind(&record.tailscale_base_url)
    .bind(&record.lan_base_url)
    .bind(&record.localhost_base_url)
    .bind(&record.public_base_url)
    .bind(revoked_at.map(|value| value.unix_timestamp()))
    .execute(pool)
    .await?;

    Ok(record.clone())
}

pub(crate) async fn list_linked_nodes(
    pool: &SqlitePool,
) -> Result<Vec<LinkedNodeRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT node_id, node_display_name, status, scopes_json, linked_at, last_seen_at, transport_hint,
               sync_base_url, tailscale_base_url, lan_base_url, localhost_base_url, public_base_url
        FROM linked_nodes
        ORDER BY linked_at DESC, node_id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_linked_node_row).collect()
}

pub(crate) async fn revoke_linked_node(
    pool: &SqlitePool,
    node_id: &str,
    revoked_at: OffsetDateTime,
) -> Result<Option<LinkedNodeRecord>, StorageError> {
    let existing = sqlx::query(
        r#"
        SELECT node_id, node_display_name, status, scopes_json, linked_at, last_seen_at, transport_hint,
               sync_base_url, tailscale_base_url, lan_base_url, localhost_base_url, public_base_url
        FROM linked_nodes
        WHERE node_id = ?
        "#,
    )
    .bind(node_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = existing else {
        return Ok(None);
    };

    let mut record = map_linked_node_row(&row)?;
    record.status = LinkStatus::Revoked;
    record.last_seen_at = Some(revoked_at);

    sqlx::query(
        r#"
        UPDATE linked_nodes
        SET status = ?, revoked_at = ?, last_seen_at = ?
        WHERE node_id = ?
        "#,
    )
    .bind(record.status.to_string())
    .bind(revoked_at.unix_timestamp())
    .bind(revoked_at.unix_timestamp())
    .bind(node_id)
    .execute(pool)
    .await?;

    Ok(Some(record))
}

fn map_pairing_token_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<(PairingTokenRecord, Option<OffsetDateTime>), StorageError> {
    let scopes_json: String = row.try_get("scopes_json")?;
    let scopes: LinkScope = serde_json::from_str(&scopes_json)?;
    let redeemed_at = row
        .try_get::<Option<i64>, _>("redeemed_at")?
        .map(timestamp_to_datetime)
        .transpose()?;

    Ok((
        PairingTokenRecord {
            token_id: row.try_get("token_id")?,
            token_code: row.try_get("token_code")?,
            issued_at: timestamp_to_datetime(row.try_get("issued_at")?)?,
            expires_at: timestamp_to_datetime(row.try_get("expires_at")?)?,
            issued_by_node_id: row.try_get("issued_by_node_id")?,
            scopes,
        },
        redeemed_at,
    ))
}

fn map_linked_node_row(row: &sqlx::sqlite::SqliteRow) -> Result<LinkedNodeRecord, StorageError> {
    let scopes_json: String = row.try_get("scopes_json")?;
    let scopes: LinkScope = serde_json::from_str(&scopes_json)?;
    let status: String = row.try_get("status")?;
    let last_seen_at = row
        .try_get::<Option<i64>, _>("last_seen_at")?
        .map(timestamp_to_datetime)
        .transpose()?;
    let sync_base_url: Option<String> = row.try_get("sync_base_url")?;
    let tailscale_base_url: Option<String> = row.try_get("tailscale_base_url")?;
    let lan_base_url: Option<String> = row.try_get("lan_base_url")?;
    let localhost_base_url: Option<String> = row.try_get("localhost_base_url")?;
    let public_base_url: Option<String> = row.try_get("public_base_url")?;

    Ok(LinkedNodeRecord {
        node_id: row.try_get("node_id")?,
        node_display_name: row.try_get("node_display_name")?,
        status: status
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        scopes,
        linked_at: timestamp_to_datetime(row.try_get("linked_at")?)?,
        last_seen_at,
        transport_hint: row.try_get("transport_hint")?,
        sync_base_url: sync_base_url.clone(),
        tailscale_base_url: tailscale_base_url.clone(),
        lan_base_url: lan_base_url.clone(),
        localhost_base_url: localhost_base_url.clone(),
        public_base_url: public_base_url.clone(),
        endpoint_inventory: build_endpoint_inventory(
            sync_base_url,
            tailscale_base_url,
            lan_base_url,
            localhost_base_url,
            public_base_url,
            last_seen_at,
        ),
        reachability: TrustedNodeReachability::Unknown,
    })
}

fn build_endpoint_inventory(
    sync_base_url: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    localhost_base_url: Option<String>,
    public_base_url: Option<String>,
    last_seen_at: Option<OffsetDateTime>,
) -> Vec<TrustedNodeEndpointRecord> {
    let mut endpoints = Vec::new();
    let mut push_endpoint = |kind: TrustedNodeEndpointKind, base_url: Option<String>| {
        let Some(base_url) = base_url.map(|value| value.trim().to_string()) else {
            return;
        };
        if base_url.is_empty()
            || endpoints
                .iter()
                .any(|record: &TrustedNodeEndpointRecord| record.base_url == base_url)
        {
            return;
        }
        endpoints.push(TrustedNodeEndpointRecord {
            kind,
            base_url,
            last_seen_at,
            advertised: true,
        });
    };

    push_endpoint(TrustedNodeEndpointKind::Sync, sync_base_url);
    push_endpoint(TrustedNodeEndpointKind::Tailscale, tailscale_base_url);
    push_endpoint(TrustedNodeEndpointKind::Lan, lan_base_url);
    push_endpoint(TrustedNodeEndpointKind::Localhost, localhost_base_url);
    push_endpoint(TrustedNodeEndpointKind::Public, public_base_url);
    endpoints
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{migrate::Migrator, SqlitePool};

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_scopes() -> LinkScope {
        LinkScope {
            read_context: true,
            write_safe_actions: true,
            execute_repo_tasks: false,
        }
    }

    #[tokio::test]
    async fn linking_repo_issue_and_redeem_pairing_token() {
        let pool = test_pool().await;
        let issued_at = OffsetDateTime::now_utc();
        let record = PairingTokenRecord {
            token_id: "pair_tok_1".to_string(),
            token_code: "paircode_123".to_string(),
            issued_at,
            expires_at: issued_at + time::Duration::seconds(900),
            issued_by_node_id: "node_alpha".to_string(),
            scopes: sample_scopes(),
        };

        issue_pairing_token(&pool, &record).await.unwrap();

        let (stored, redeemed_at) = get_pairing_token_by_code(&pool, "paircode_123")
            .await
            .unwrap()
            .expect("pairing token should exist");
        assert_eq!(stored.issued_by_node_id, "node_alpha");
        assert!(redeemed_at.is_none());

        let marked = mark_pairing_token_redeemed(&pool, "paircode_123", issued_at)
            .await
            .unwrap();
        assert!(marked);
    }

    #[tokio::test]
    async fn linking_repo_lists_and_revokes_linked_nodes() {
        let pool = test_pool().await;
        let linked_at = OffsetDateTime::now_utc();
        let record = LinkedNodeRecord {
            node_id: "node_beta".to_string(),
            node_display_name: "Beta".to_string(),
            status: LinkStatus::Linked,
            scopes: sample_scopes(),
            linked_at,
            last_seen_at: None,
            transport_hint: Some("tailscale".to_string()),
            sync_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
            tailscale_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
            lan_base_url: Some("http://192.168.1.55:4130".to_string()),
            localhost_base_url: None,
            public_base_url: None,
            endpoint_inventory: Vec::new(),
            reachability: TrustedNodeReachability::Unknown,
        };

        upsert_linked_node(&pool, &record).await.unwrap();
        let listed = list_linked_nodes(&pool).await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].node_id, "node_beta");
        assert_eq!(
            listed[0].sync_base_url.as_deref(),
            Some("http://node-beta.tailnet.ts.net:4130")
        );
        assert_eq!(
            listed[0].lan_base_url.as_deref(),
            Some("http://192.168.1.55:4130")
        );

        let revoked = revoke_linked_node(&pool, "node_beta", linked_at)
            .await
            .unwrap()
            .expect("linked node should exist");
        assert_eq!(revoked.status, LinkStatus::Revoked);
    }
}
