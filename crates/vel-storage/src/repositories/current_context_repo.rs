use sqlx::{Sqlite, SqlitePool, Transaction};
use vel_core::context::{ContextMigrator, CurrentContextV1};

use crate::db::StorageError;

pub(crate) async fn get_current_context(
    pool: &SqlitePool,
) -> Result<Option<(i64, CurrentContextV1)>, StorageError> {
    let row = sqlx::query_as::<_, (i64, String)>(
        r#"SELECT computed_at, context_json FROM current_context WHERE id = 1"#,
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some((computed_at, context_json)) => {
            let context = ContextMigrator::from_json_str(&context_json)
                .map_err(|e| StorageError::Validation(e.to_string()))?;
            Ok(Some((computed_at, context)))
        }
        None => Ok(None),
    }
}

pub(crate) async fn set_current_context(
    pool: &SqlitePool,
    computed_at: i64,
    context_json: &str,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    set_current_context_in_tx(&mut tx, computed_at, context_json).await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn set_current_context_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    computed_at: i64,
    context_json: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"INSERT OR REPLACE INTO current_context (id, computed_at, context_json) VALUES (1, ?, ?)"#,
    )
    .bind(computed_at)
    .bind(context_json)
    .execute(&mut **tx)
    .await?;
    Ok(())
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
    async fn set_and_get_current_context_round_trip() {
        let pool = test_pool().await;

        set_current_context(&pool, 1_700_000_123, r#"{"mode":"morning"}"#)
            .await
            .unwrap();

        let row = get_current_context(&pool).await.unwrap();
        let (computed_at, context) = row.expect("should have context");
        assert_eq!(computed_at, 1_700_000_123);
        assert_eq!(context.mode, "morning");
    }

    #[tokio::test]
    async fn set_current_context_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;
        let before = get_current_context(&pool).await.unwrap();

        {
            let mut tx = pool.begin().await.unwrap();
            set_current_context_in_tx(&mut tx, 1_700_000_555, r#"{"mode":"today"}"#)
                .await
                .unwrap();
            tx.rollback().await.unwrap();
        }

        let after = get_current_context(&pool).await.unwrap();
        assert_eq!(after, before);
    }
}
