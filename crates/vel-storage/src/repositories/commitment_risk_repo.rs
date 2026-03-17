use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use crate::db::StorageError;

pub(crate) async fn insert_commitment_risk(
    pool: &SqlitePool,
    commitment_id: &str,
    risk_score: f64,
    risk_level: &str,
    factors_json: &str,
    computed_at: i64,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_commitment_risk_in_tx(
        &mut tx,
        commitment_id,
        risk_score,
        risk_level,
        factors_json,
        computed_at,
    )
    .await?;
    tx.commit().await?;
    Ok(id)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn insert_commitment_risk_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    commitment_id: &str,
    risk_score: f64,
    risk_level: &str,
    factors_json: &str,
    computed_at: i64,
) -> Result<String, StorageError> {
    let id = format!("risk_{}", Uuid::new_v4().simple());
    sqlx::query(
        r#"INSERT INTO commitment_risk (id, commitment_id, risk_score, risk_level, factors_json, computed_at) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(commitment_id)
    .bind(risk_score)
    .bind(risk_level)
    .bind(factors_json)
    .bind(computed_at)
    .execute(&mut **tx)
    .await?;
    Ok(id)
}

pub(crate) async fn list_commitment_risk_recent(
    pool: &SqlitePool,
    commitment_id: &str,
    limit: u32,
) -> Result<Vec<(String, f64, String, String, i64)>, StorageError> {
    let limit = limit.min(50) as i64;
    let rows = sqlx::query_as::<_, (String, f64, String, String, i64)>(
        r#"SELECT id, risk_score, risk_level, factors_json, computed_at FROM commitment_risk WHERE commitment_id = ? ORDER BY computed_at DESC LIMIT ?"#,
    )
    .bind(commitment_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub(crate) async fn list_commitment_risk_latest_all(
    pool: &SqlitePool,
) -> Result<Vec<(String, String, f64, String, String, i64)>, StorageError> {
    let rows = sqlx::query_as::<_, (String, String, f64, String, String, i64)>(
        r#"SELECT cr.id, cr.commitment_id, cr.risk_score, cr.risk_level, cr.factors_json, cr.computed_at
           FROM commitment_risk cr
           INNER JOIN (
             SELECT commitment_id, MAX(computed_at) AS max_at FROM commitment_risk GROUP BY commitment_id
           ) latest ON cr.commitment_id = latest.commitment_id AND cr.computed_at = latest.max_at
           ORDER BY cr.risk_score DESC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub(crate) async fn count_commitment_risk(pool: &SqlitePool) -> Result<i64, StorageError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM commitment_risk")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn insert_list_count_and_latest_commitment_risk() {
        let pool = test_pool().await;

        let older =
            insert_commitment_risk(&pool, "com_1", 0.2, "low", r#"{"factors":["none"]}"#, 100)
                .await
                .unwrap();
        let newer = insert_commitment_risk(
            &pool,
            "com_1",
            0.8,
            "high",
            r#"{"factors":["deadline"]}"#,
            200,
        )
        .await
        .unwrap();
        let other = insert_commitment_risk(
            &pool,
            "com_2",
            0.9,
            "high",
            r#"{"factors":["dependency"]}"#,
            150,
        )
        .await
        .unwrap();

        let recent = list_commitment_risk_recent(&pool, "com_1", 10)
            .await
            .unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].0, newer);
        assert_eq!(recent[0].1, 0.8);
        assert_eq!(recent[1].0, older);
        assert_eq!(recent[1].1, 0.2);

        let latest = list_commitment_risk_latest_all(&pool).await.unwrap();
        assert_eq!(latest.len(), 2);
        assert_eq!(latest[0].0, other);
        assert_eq!(latest[0].1, "com_2");
        assert_eq!(latest[0].2, 0.9);
        assert_eq!(latest[1].0, newer);
        assert_eq!(latest[1].1, "com_1");
        assert_eq!(latest[1].2, 0.8);

        let count = count_commitment_risk(&pool).await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn insert_commitment_risk_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;

        {
            let mut tx = pool.begin().await.unwrap();
            let inserted = insert_commitment_risk_in_tx(
                &mut tx,
                "com_tx",
                0.55,
                "medium",
                r#"{"factors":["timing"]}"#,
                300,
            )
            .await
            .unwrap();
            assert!(inserted.starts_with("risk_"));
            tx.rollback().await.unwrap();
        }

        let count = count_commitment_risk(&pool).await.unwrap();
        assert_eq!(count, 0);
    }
}
