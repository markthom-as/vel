use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{PersonAlias, PersonId, PersonRecord};

use crate::{db::StorageError, mapping::timestamp_to_datetime, repositories::semantic_memory_repo};

pub(crate) async fn create_person(
    pool: &SqlitePool,
    person: PersonRecord,
) -> Result<PersonRecord, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO people (
            id,
            display_name,
            given_name,
            family_name,
            relationship_context,
            birthday,
            last_contacted_at,
            metadata_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(person.id.as_ref())
    .bind(&person.display_name)
    .bind(&person.given_name)
    .bind(&person.family_name)
    .bind(&person.relationship_context)
    .bind(&person.birthday)
    .bind(person.last_contacted_at.map(|value| value.unix_timestamp()))
    .bind("{}")
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    for alias in &person.aliases {
        upsert_person_alias_in_tx(&mut tx, &person.id, alias).await?;
    }

    tx.commit().await?;
    let person = get_person(pool, person.id.as_ref()).await?.ok_or_else(|| {
        StorageError::NotFound(format!("person {} missing after insert", person.id))
    })?;
    semantic_memory_repo::upsert_person_record(pool, &person).await?;
    Ok(person)
}

pub(crate) async fn list_people(pool: &SqlitePool) -> Result<Vec<PersonRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            display_name,
            given_name,
            family_name,
            relationship_context,
            birthday,
            last_contacted_at
        FROM people
        ORDER BY updated_at DESC, created_at DESC, display_name ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut people = Vec::with_capacity(rows.len());
    for row in rows {
        let person_id = row.try_get::<String, _>("id")?;
        people.push(map_person_row(pool, row, person_id).await?);
    }
    Ok(people)
}

pub(crate) async fn get_person(
    pool: &SqlitePool,
    person_id: &str,
) -> Result<Option<PersonRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            display_name,
            given_name,
            family_name,
            relationship_context,
            birthday,
            last_contacted_at
        FROM people
        WHERE id = ?
        "#,
    )
    .bind(person_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(
            map_person_row(pool, row, person_id.to_string()).await?,
        )),
        None => Ok(None),
    }
}

pub(crate) async fn upsert_person_alias(
    pool: &SqlitePool,
    person_id: &PersonId,
    alias: &PersonAlias,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    upsert_person_alias_in_tx(&mut tx, person_id, alias).await?;
    sqlx::query(r#"UPDATE people SET updated_at = ? WHERE id = ?"#)
        .bind(OffsetDateTime::now_utc().unix_timestamp())
        .bind(person_id.as_ref())
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    if let Some(person) = get_person(pool, person_id.as_ref()).await? {
        semantic_memory_repo::upsert_person_record(pool, &person).await?;
    }
    Ok(())
}

pub(crate) async fn list_person_aliases(
    pool: &SqlitePool,
    person_id: &str,
) -> Result<Vec<PersonAlias>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT platform, handle, display, source_ref_json
        FROM person_aliases
        WHERE person_id = ?
        ORDER BY created_at ASC, platform ASC, handle ASC
        "#,
    )
    .bind(person_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(map_alias_row).collect()
}

async fn map_person_row(
    pool: &SqlitePool,
    row: sqlx::sqlite::SqliteRow,
    person_id: String,
) -> Result<PersonRecord, StorageError> {
    Ok(PersonRecord {
        id: PersonId::from(person_id.clone()),
        display_name: row.try_get("display_name")?,
        given_name: row.try_get("given_name")?,
        family_name: row.try_get("family_name")?,
        relationship_context: row.try_get("relationship_context")?,
        birthday: row.try_get("birthday")?,
        last_contacted_at: row
            .try_get::<Option<i64>, _>("last_contacted_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        aliases: list_person_aliases(pool, &person_id).await?,
        links: Vec::new(),
    })
}

fn map_alias_row(row: sqlx::sqlite::SqliteRow) -> Result<PersonAlias, StorageError> {
    let source_ref_json = row.try_get::<String, _>("source_ref_json")?;
    let source_ref = match source_ref_json.trim() {
        "" | "{}" | "null" => None,
        value => Some(serde_json::from_str(value)?),
    };

    Ok(PersonAlias {
        platform: row.try_get("platform")?,
        handle: row.try_get("handle")?,
        display: row
            .try_get::<Option<String>, _>("display")?
            .unwrap_or_default(),
        source_ref,
    })
}

async fn upsert_person_alias_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    person_id: &PersonId,
    alias: &PersonAlias,
) -> Result<(), StorageError> {
    let alias_id = format!("pal_{}", Uuid::new_v4().simple());
    let source_ref_json = alias
        .source_ref
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?
        .unwrap_or_else(|| "null".to_string());
    let display = if alias.display.trim().is_empty() {
        None
    } else {
        Some(alias.display.trim().to_string())
    };

    sqlx::query(
        r#"
        INSERT INTO person_aliases (
            id,
            person_id,
            platform,
            handle,
            display,
            source_ref_json,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(platform, handle) DO UPDATE SET
            person_id = excluded.person_id,
            display = excluded.display,
            source_ref_json = excluded.source_ref_json
        "#,
    )
    .bind(alias_id)
    .bind(person_id.as_ref())
    .bind(alias.platform.trim())
    .bind(alias.handle.trim())
    .bind(display)
    .bind(source_ref_json)
    .bind(OffsetDateTime::now_utc().unix_timestamp())
    .execute(&mut **tx)
    .await?;
    Ok(())
}
