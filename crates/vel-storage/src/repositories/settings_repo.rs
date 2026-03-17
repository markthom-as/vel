use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::db::StorageError;

pub(crate) async fn get_all_settings(
    pool: &SqlitePool,
) -> Result<HashMap<String, JsonValue>, StorageError> {
    let rows = sqlx::query_as::<_, (String, String)>("SELECT key, value_json FROM settings")
        .fetch_all(pool)
        .await?;
    let mut out = HashMap::new();
    for (k, v) in rows {
        let val: JsonValue = serde_json::from_str(&v).unwrap_or(JsonValue::Null);
        out.insert(k, val);
    }
    Ok(out)
}

pub(crate) async fn set_setting(
    pool: &SqlitePool,
    key: &str,
    value: &JsonValue,
) -> Result<(), StorageError> {
    let json = serde_json::to_string(value).map_err(|e| StorageError::Validation(e.to_string()))?;
    sqlx::query("INSERT INTO settings (key, value_json) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json")
        .bind(key)
        .bind(&json)
        .execute(pool)
        .await?;
    Ok(())
}
