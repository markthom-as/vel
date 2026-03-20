use serde::de::DeserializeOwned;
use serde_json::json;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use vel_core::{
    DurableRoutineBlock, PlanningConstraint, PlanningProfileMutation, PlanningProfileRemoveTarget,
    RoutinePlanningProfile,
};

use crate::db::StorageError;

pub(crate) async fn load_routine_planning_profile(
    pool: &SqlitePool,
) -> Result<RoutinePlanningProfile, StorageError> {
    let routine_blocks = sqlx::query(
        r#"
        SELECT id, label, source, local_timezone, start_local_time, end_local_time,
               days_of_week_json, protected, active
        FROM routine_blocks
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(map_routine_block_row)
    .collect::<Result<Vec<_>, _>>()?;

    let planning_constraints = sqlx::query(
        r#"
        SELECT id, label, kind, detail, time_window, minutes, max_items, active
        FROM planning_constraints
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(map_planning_constraint_row)
    .collect::<Result<Vec<_>, _>>()?;

    Ok(RoutinePlanningProfile {
        routine_blocks,
        planning_constraints,
    })
}

pub(crate) async fn replace_routine_planning_profile(
    pool: &SqlitePool,
    profile: &RoutinePlanningProfile,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query("DELETE FROM routine_blocks")
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM planning_constraints")
        .execute(&mut *tx)
        .await?;

    for (sort_order, block) in profile.routine_blocks.iter().enumerate() {
        let days_of_week_json = serde_json::to_string(&block.days_of_week)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO routine_blocks (
                id, label, source, local_timezone, start_local_time, end_local_time,
                days_of_week_json, protected, active, sort_order, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&block.id)
        .bind(&block.label)
        .bind(enum_to_storage(&block.source)?)
        .bind(&block.local_timezone)
        .bind(&block.start_local_time)
        .bind(&block.end_local_time)
        .bind(days_of_week_json)
        .bind(block.protected)
        .bind(block.active)
        .bind(sort_order as i64)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    for (sort_order, constraint) in profile.planning_constraints.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO planning_constraints (
                id, label, kind, detail, time_window, minutes, max_items,
                active, sort_order, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&constraint.id)
        .bind(&constraint.label)
        .bind(enum_to_storage(&constraint.kind)?)
        .bind(&constraint.detail)
        .bind(optional_enum_to_storage(constraint.time_window)?)
        .bind(constraint.minutes.map(i64::from))
        .bind(constraint.max_items.map(i64::from))
        .bind(constraint.active)
        .bind(sort_order as i64)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub(crate) async fn apply_routine_planning_profile_mutation(
    pool: &SqlitePool,
    mutation: &PlanningProfileMutation,
) -> Result<RoutinePlanningProfile, StorageError> {
    let mut tx = pool.begin().await?;

    match mutation {
        PlanningProfileMutation::UpsertRoutineBlock(block) => {
            upsert_routine_block(&mut tx, block).await?;
        }
        PlanningProfileMutation::RemoveRoutineBlock(target) => {
            remove_routine_block(&mut tx, target).await?;
        }
        PlanningProfileMutation::UpsertPlanningConstraint(constraint) => {
            upsert_planning_constraint(&mut tx, constraint).await?;
        }
        PlanningProfileMutation::RemovePlanningConstraint(target) => {
            remove_planning_constraint(&mut tx, target).await?;
        }
    }

    tx.commit().await?;
    load_routine_planning_profile(pool).await
}

async fn upsert_routine_block(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    block: &DurableRoutineBlock,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let days_of_week_json = serde_json::to_string(&block.days_of_week)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let sort_order = existing_sort_order(tx, "routine_blocks", &block.id)
        .await?
        .unwrap_or(next_sort_order(tx, "routine_blocks").await?);

    sqlx::query(
        r#"
        INSERT INTO routine_blocks (
            id, label, source, local_timezone, start_local_time, end_local_time,
            days_of_week_json, protected, active, sort_order, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            label = excluded.label,
            source = excluded.source,
            local_timezone = excluded.local_timezone,
            start_local_time = excluded.start_local_time,
            end_local_time = excluded.end_local_time,
            days_of_week_json = excluded.days_of_week_json,
            protected = excluded.protected,
            active = excluded.active,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&block.id)
    .bind(&block.label)
    .bind(enum_to_storage(&block.source)?)
    .bind(&block.local_timezone)
    .bind(&block.start_local_time)
    .bind(&block.end_local_time)
    .bind(days_of_week_json)
    .bind(block.protected)
    .bind(block.active)
    .bind(sort_order)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn remove_routine_block(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    target: &PlanningProfileRemoveTarget,
) -> Result<(), StorageError> {
    let rows_affected = sqlx::query("DELETE FROM routine_blocks WHERE id = ?")
        .bind(&target.id)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    if rows_affected == 0 {
        return Err(StorageError::NotFound(format!(
            "routine block {} not found",
            target.id
        )));
    }
    Ok(())
}

async fn upsert_planning_constraint(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    constraint: &PlanningConstraint,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let sort_order = existing_sort_order(tx, "planning_constraints", &constraint.id)
        .await?
        .unwrap_or(next_sort_order(tx, "planning_constraints").await?);

    sqlx::query(
        r#"
        INSERT INTO planning_constraints (
            id, label, kind, detail, time_window, minutes, max_items,
            active, sort_order, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            label = excluded.label,
            kind = excluded.kind,
            detail = excluded.detail,
            time_window = excluded.time_window,
            minutes = excluded.minutes,
            max_items = excluded.max_items,
            active = excluded.active,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&constraint.id)
    .bind(&constraint.label)
    .bind(enum_to_storage(&constraint.kind)?)
    .bind(&constraint.detail)
    .bind(optional_enum_to_storage(constraint.time_window)?)
    .bind(constraint.minutes.map(i64::from))
    .bind(constraint.max_items.map(i64::from))
    .bind(constraint.active)
    .bind(sort_order)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn remove_planning_constraint(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    target: &PlanningProfileRemoveTarget,
) -> Result<(), StorageError> {
    let rows_affected = sqlx::query("DELETE FROM planning_constraints WHERE id = ?")
        .bind(&target.id)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    if rows_affected == 0 {
        return Err(StorageError::NotFound(format!(
            "planning constraint {} not found",
            target.id
        )));
    }
    Ok(())
}

async fn existing_sort_order(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    table: &str,
    id: &str,
) -> Result<Option<i64>, StorageError> {
    let query = format!("SELECT sort_order FROM {table} WHERE id = ?");
    let row = sqlx::query(&query)
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?;
    Ok(row.map(|row| row.get::<i64, _>("sort_order")))
}

async fn next_sort_order(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    table: &str,
) -> Result<i64, StorageError> {
    let query = format!("SELECT COALESCE(MAX(sort_order), -1) + 1 AS next_sort_order FROM {table}");
    let row = sqlx::query(&query).fetch_one(&mut **tx).await?;
    Ok(row.get::<i64, _>("next_sort_order"))
}

fn map_routine_block_row(
    row: sqlx::sqlite::SqliteRow,
) -> Result<DurableRoutineBlock, StorageError> {
    let days_of_week_json: String = row.try_get("days_of_week_json")?;
    Ok(DurableRoutineBlock {
        id: row.try_get("id")?,
        label: row.try_get("label")?,
        source: enum_from_storage(&row.try_get::<String, _>("source")?)?,
        local_timezone: row.try_get("local_timezone")?,
        start_local_time: row.try_get("start_local_time")?,
        end_local_time: row.try_get("end_local_time")?,
        days_of_week: serde_json::from_str(&days_of_week_json)
            .map_err(|error| StorageError::DataCorrupted(error.to_string()))?,
        protected: row.try_get("protected")?,
        active: row.try_get("active")?,
    })
}

fn map_planning_constraint_row(
    row: sqlx::sqlite::SqliteRow,
) -> Result<PlanningConstraint, StorageError> {
    let time_window = row
        .try_get::<Option<String>, _>("time_window")?
        .as_deref()
        .map(enum_from_storage)
        .transpose()?;

    Ok(PlanningConstraint {
        id: row.try_get("id")?,
        label: row.try_get("label")?,
        kind: enum_from_storage(&row.try_get::<String, _>("kind")?)?,
        detail: row.try_get("detail")?,
        time_window,
        minutes: row
            .try_get::<Option<i64>, _>("minutes")?
            .map(|value| value as u32),
        max_items: row
            .try_get::<Option<i64>, _>("max_items")?
            .map(|value| value as u32),
        active: row.try_get("active")?,
    })
}

fn enum_to_storage<T: serde::Serialize>(value: &T) -> Result<String, StorageError> {
    serde_json::to_value(value)
        .map_err(|error| StorageError::Validation(error.to_string()))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| StorageError::Validation("enum should serialize as string".to_string()))
}

fn optional_enum_to_storage<T: serde::Serialize>(
    value: Option<T>,
) -> Result<Option<String>, StorageError> {
    value.as_ref().map(enum_to_storage).transpose()
}

fn enum_from_storage<T: DeserializeOwned>(value: &str) -> Result<T, StorageError> {
    serde_json::from_value(json!(value))
        .map_err(|error| StorageError::DataCorrupted(error.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::db::Storage;
    use vel_core::{
        DurableRoutineBlock, PlanningConstraint, PlanningConstraintKind, PlanningProfileMutation,
        PlanningProfileRemoveTarget, RoutineBlockSourceKind, RoutinePlanningProfile,
        ScheduleTimeWindow,
    };

    async fn test_storage() -> Storage {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
    }

    #[tokio::test]
    async fn replace_profile_round_trips_blocks_and_constraints() {
        let storage = test_storage().await;
        let profile = RoutinePlanningProfile {
            routine_blocks: vec![DurableRoutineBlock {
                id: "routine_focus".to_string(),
                label: "Focus block".to_string(),
                source: RoutineBlockSourceKind::OperatorDeclared,
                local_timezone: "America/Denver".to_string(),
                start_local_time: "09:00".to_string(),
                end_local_time: "11:00".to_string(),
                days_of_week: vec![1, 2, 3, 4, 5],
                protected: true,
                active: true,
            }],
            planning_constraints: vec![PlanningConstraint {
                id: "default_window".to_string(),
                label: "Use prenoon by default".to_string(),
                kind: PlanningConstraintKind::DefaultTimeWindow,
                detail: Some("Bias planning toward morning work.".to_string()),
                time_window: Some(ScheduleTimeWindow::Prenoon),
                minutes: None,
                max_items: None,
                active: true,
            }],
        };

        storage
            .replace_routine_planning_profile(&profile)
            .await
            .unwrap();

        let stored = storage.load_routine_planning_profile().await.unwrap();
        assert_eq!(stored, profile);
    }

    #[tokio::test]
    async fn replace_profile_replaces_previous_rows_atomically() {
        let storage = test_storage().await;

        storage
            .replace_routine_planning_profile(&RoutinePlanningProfile {
                routine_blocks: vec![DurableRoutineBlock {
                    id: "routine_old".to_string(),
                    label: "Old block".to_string(),
                    source: RoutineBlockSourceKind::Imported,
                    local_timezone: "UTC".to_string(),
                    start_local_time: "07:00".to_string(),
                    end_local_time: "08:00".to_string(),
                    days_of_week: vec![1],
                    protected: false,
                    active: false,
                }],
                planning_constraints: vec![],
            })
            .await
            .unwrap();

        storage
            .replace_routine_planning_profile(&RoutinePlanningProfile {
                routine_blocks: vec![],
                planning_constraints: vec![PlanningConstraint {
                    id: "overflow".to_string(),
                    label: "Require judgment".to_string(),
                    kind: PlanningConstraintKind::RequireJudgmentForOverflow,
                    detail: None,
                    time_window: None,
                    minutes: None,
                    max_items: None,
                    active: true,
                }],
            })
            .await
            .unwrap();

        let stored = storage.load_routine_planning_profile().await.unwrap();
        assert!(stored.routine_blocks.is_empty());
        assert_eq!(stored.planning_constraints.len(), 1);
        assert_eq!(
            stored.planning_constraints[0].kind,
            PlanningConstraintKind::RequireJudgmentForOverflow
        );
    }

    #[tokio::test]
    async fn apply_mutation_upserts_and_removes_profile_records() {
        let storage = test_storage().await;

        let profile = storage
            .apply_routine_planning_profile_mutation(&PlanningProfileMutation::UpsertRoutineBlock(
                DurableRoutineBlock {
                    id: "routine_focus".to_string(),
                    label: "Focus block".to_string(),
                    source: RoutineBlockSourceKind::OperatorDeclared,
                    local_timezone: "America/Denver".to_string(),
                    start_local_time: "09:00".to_string(),
                    end_local_time: "11:00".to_string(),
                    days_of_week: vec![1, 2, 3, 4, 5],
                    protected: true,
                    active: true,
                },
            ))
            .await
            .unwrap();
        assert_eq!(profile.routine_blocks.len(), 1);

        let profile = storage
            .apply_routine_planning_profile_mutation(
                &PlanningProfileMutation::UpsertPlanningConstraint(PlanningConstraint {
                    id: "default_window".to_string(),
                    label: "Prefer prenoon".to_string(),
                    kind: PlanningConstraintKind::DefaultTimeWindow,
                    detail: None,
                    time_window: Some(ScheduleTimeWindow::Prenoon),
                    minutes: None,
                    max_items: None,
                    active: true,
                }),
            )
            .await
            .unwrap();
        assert_eq!(profile.planning_constraints.len(), 1);

        let profile = storage
            .apply_routine_planning_profile_mutation(&PlanningProfileMutation::RemoveRoutineBlock(
                PlanningProfileRemoveTarget {
                    id: "routine_focus".to_string(),
                },
            ))
            .await
            .unwrap();
        assert!(profile.routine_blocks.is_empty());
        assert_eq!(profile.planning_constraints.len(), 1);
    }

    #[tokio::test]
    async fn apply_mutation_returns_not_found_for_missing_remove_target() {
        let storage = test_storage().await;

        let error = storage
            .apply_routine_planning_profile_mutation(
                &PlanningProfileMutation::RemovePlanningConstraint(PlanningProfileRemoveTarget {
                    id: "missing".to_string(),
                }),
            )
            .await
            .unwrap_err();

        match error {
            crate::db::StorageError::NotFound(message) => {
                assert!(message.contains("planning constraint missing not found"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
