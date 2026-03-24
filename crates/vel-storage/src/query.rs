use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::{
    db::StorageError,
    repositories::{
        canonical_objects_repo::{map_canonical_object_row_for_query, CanonicalObjectRecord},
        relations_repo::{map_relation_row_for_query, CanonicalRelationRecord},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuerySortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalObjectSortField {
    UpdatedAt,
    CreatedAt,
    Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalObjectSort {
    pub field: CanonicalObjectSortField,
    pub direction: QuerySortDirection,
}

impl Default for CanonicalObjectSort {
    fn default() -> Self {
        Self {
            field: CanonicalObjectSortField::UpdatedAt,
            direction: QuerySortDirection::Desc,
        }
    }
}

/// Storage-neutral canonical object query with pagination, sort, and visibility flags.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CanonicalObjectQuery {
    pub object_class: Option<String>,
    pub object_type: Option<String>,
    pub include_archived: bool,
    pub include_deleted: bool,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort: CanonicalObjectSort,
}

/// Typed relation traversal request instead of ad hoc SQL joins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationTraversal {
    pub from_id: String,
    pub relation_type: Option<String>,
    pub direction: Option<String>,
    pub active_only: bool,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort: QuerySortDirection,
}

impl Default for RelationTraversal {
    fn default() -> Self {
        Self {
            from_id: String::new(),
            relation_type: None,
            direction: None,
            active_only: true,
            cursor: None,
            limit: Some(50),
            sort: QuerySortDirection::Asc,
        }
    }
}

pub async fn query_canonical_objects(
    pool: &SqlitePool,
    query: &CanonicalObjectQuery,
) -> Result<Vec<CanonicalObjectRecord>, StorageError> {
    let mut builder = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT
            id,
            object_type,
            object_class,
            schema_version,
            revision,
            status,
            provenance_json,
            facets_json,
            source_summary_json,
            deleted_at,
            archived_at,
            created_at,
            updated_at
        FROM canonical_objects
        WHERE 1 = 1
        "#,
    );

    if let Some(object_class) = &query.object_class {
        builder.push(" AND object_class = ").push_bind(object_class);
    }
    if let Some(object_type) = &query.object_type {
        builder.push(" AND object_type = ").push_bind(object_type);
    }
    if !query.include_archived {
        builder.push(" AND archived_at IS NULL");
    }
    if !query.include_deleted {
        builder.push(" AND deleted_at IS NULL");
    }
    if let Some(cursor) = &query.cursor {
        builder.push(" AND id > ").push_bind(cursor);
    }

    builder.push(" ORDER BY ");
    match query.sort.field {
        CanonicalObjectSortField::UpdatedAt => builder.push("updated_at"),
        CanonicalObjectSortField::CreatedAt => builder.push("created_at"),
        CanonicalObjectSortField::Id => builder.push("id"),
    };
    builder.push(match query.sort.direction {
        QuerySortDirection::Asc => " ASC, id ASC",
        QuerySortDirection::Desc => " DESC, id DESC",
    });
    builder
        .push(" LIMIT ")
        .push_bind(i64::from(query.limit.unwrap_or(50)));

    let rows = builder.build().fetch_all(pool).await?;
    rows.iter()
        .map(map_canonical_object_row_for_query)
        .collect()
}

pub async fn traverse_relations(
    pool: &SqlitePool,
    traversal: &RelationTraversal,
) -> Result<Vec<CanonicalRelationRecord>, StorageError> {
    let mut builder = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT
            id,
            relation_type,
            from_id,
            to_id,
            direction,
            active,
            source_json,
            confidence,
            revision,
            created_at,
            updated_at
        FROM canonical_relations
        WHERE from_id = 
        "#,
    );
    builder.push_bind(&traversal.from_id);

    if let Some(relation_type) = &traversal.relation_type {
        builder
            .push(" AND relation_type = ")
            .push_bind(relation_type);
    }
    if let Some(direction) = &traversal.direction {
        builder.push(" AND direction = ").push_bind(direction);
    }
    if traversal.active_only {
        builder.push(" AND active = 1");
    }
    if let Some(cursor) = &traversal.cursor {
        builder.push(" AND id > ").push_bind(cursor);
    }

    builder.push(" ORDER BY updated_at ");
    builder.push(match traversal.sort {
        QuerySortDirection::Asc => "ASC, id ASC",
        QuerySortDirection::Desc => "DESC, id DESC",
    });
    builder
        .push(" LIMIT ")
        .push_bind(i64::from(traversal.limit.unwrap_or(50)));

    let rows = builder.build().fetch_all(pool).await?;
    rows.iter().map(map_relation_row_for_query).collect()
}
