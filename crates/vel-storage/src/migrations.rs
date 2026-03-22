use sqlx::{migrate::Migrator, SqlitePool};

use crate::{db::StorageError, infra};

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

pub async fn migrate_storage(pool: &SqlitePool) -> Result<u32, StorageError> {
    infra::run_migrations(pool, &MIGRATOR).await?;
    infra::schema_version(pool).await
}
