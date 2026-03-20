use crate::db::{AssistantTranscriptInsert, StorageError};
use serde_json::{json, Value};
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool, Transaction};
use std::collections::{HashMap, HashSet};
use vel_core::{
    HybridRetrievalPolicy, PersonAlias, PersonRecord, ProjectRecord, RetrievalStrategy,
    SemanticHit, SemanticMemoryRecord, SemanticProvenance, SemanticQuery, SemanticRecordId,
    SemanticSourceKind,
};

const EMBEDDING_MODEL: &str = "local_token_overlap_v1";
const EMBEDDING_REVISION: &str = "2026-03-18";
const MAX_TERMS: usize = 32;

trait SemanticIndexBackend {
    fn embedding_model(&self) -> &'static str;
    fn embedding_revision(&self) -> &'static str;
    fn embed(&self, text: &str) -> Vec<(String, f32)>;
    fn snippet(&self, text: &str, query: &str) -> String;
}

struct LocalTokenOverlapBackend;

impl SemanticIndexBackend for LocalTokenOverlapBackend {
    fn embedding_model(&self) -> &'static str {
        EMBEDDING_MODEL
    }

    fn embedding_revision(&self) -> &'static str {
        EMBEDDING_REVISION
    }

    fn embed(&self, text: &str) -> Vec<(String, f32)> {
        embed_text(text)
    }

    fn snippet(&self, text: &str, query: &str) -> String {
        snippet_for(text, query)
    }
}

#[derive(Debug, Clone)]
struct SemanticRow {
    record_id: String,
    source_kind: String,
    source_id: String,
    content_text: String,
    provenance_json: String,
}

fn backend() -> LocalTokenOverlapBackend {
    LocalTokenOverlapBackend
}

pub(crate) async fn upsert_capture_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    capture_id: &str,
    content_text: &str,
    occurred_at: i64,
) -> Result<(), StorageError> {
    let record_id = format!("sem_cap_{capture_id}");
    let provenance = json!({
        "capture_id": capture_id,
    });
    upsert_record_in_tx(
        tx,
        SemanticRecordUpsert {
            record_id,
            source_kind: SemanticSourceKind::Capture,
            source_id: capture_id.to_string(),
            chunk_id: capture_id.to_string(),
            content_text: content_text.to_string(),
            metadata_json: json!({ "backend": EMBEDDING_MODEL }),
            provenance,
            occurred_at,
        },
    )
    .await
}

pub(crate) async fn upsert_project_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    project: &ProjectRecord,
) -> Result<(), StorageError> {
    let content_text = vec![
        project.name.clone(),
        project.slug.clone(),
        project.family.to_string(),
        project.primary_repo.path.clone(),
        project.primary_notes_root.path.clone(),
    ]
    .join(" ");
    let first_upstream_id = project.upstream_ids.values().next().cloned();
    upsert_record_in_tx(
        tx,
        SemanticRecordUpsert {
            record_id: stable_record_id("proj", project.id.as_ref()),
            source_kind: SemanticSourceKind::Project,
            source_id: project.id.to_string(),
            chunk_id: project.slug.clone(),
            content_text,
            metadata_json: json!({
                "slug": project.slug,
                "family": project.family.to_string(),
                "status": project.status.to_string(),
            }),
            provenance: serde_json::to_value(SemanticProvenance {
                project_id: Some(project.id.to_string()),
                external_id: first_upstream_id,
                ..Default::default()
            })?,
            occurred_at: project.updated_at.unix_timestamp(),
        },
    )
    .await
}

pub(crate) async fn upsert_transcript_note_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &AssistantTranscriptInsert,
) -> Result<(), StorageError> {
    upsert_record_in_tx(
        tx,
        SemanticRecordUpsert {
            record_id: stable_record_id("transcript", &input.id),
            source_kind: SemanticSourceKind::TranscriptNote,
            source_id: input.id.clone(),
            chunk_id: input.message_id.clone().unwrap_or_else(|| input.id.clone()),
            content_text: input.content.clone(),
            metadata_json: json!({
                "source": input.source,
                "conversation_id": input.conversation_id,
                "role": input.role,
                "notes": input.metadata_json.get("notes").cloned().unwrap_or_else(|| json!({})),
            }),
            provenance: serde_json::to_value(SemanticProvenance {
                transcript_id: Some(input.id.clone()),
                message_id: input.message_id.clone(),
                external_id: input.message_id.clone(),
                ..Default::default()
            })?,
            occurred_at: input.timestamp,
        },
    )
    .await
}

pub(crate) async fn upsert_thread_record(
    pool: &SqlitePool,
    thread_id: &str,
    thread_type: &str,
    title: &str,
    status: &str,
    updated_at: i64,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    upsert_record_in_tx(
        &mut tx,
        SemanticRecordUpsert {
            record_id: stable_record_id("thread", thread_id),
            source_kind: SemanticSourceKind::Thread,
            source_id: thread_id.to_string(),
            chunk_id: thread_id.to_string(),
            content_text: format!("{title} {thread_type} {status}"),
            metadata_json: json!({
                "thread_type": thread_type,
                "status": status,
            }),
            provenance: serde_json::to_value(SemanticProvenance {
                thread_id: Some(thread_id.to_string()),
                ..Default::default()
            })?,
            occurred_at: updated_at,
        },
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn upsert_person_record(
    pool: &SqlitePool,
    person: &PersonRecord,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    upsert_person_record_in_tx(&mut tx, person).await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn upsert_note_record(
    pool: &SqlitePool,
    note_path: &str,
    title: &str,
    content_text: &str,
    capture_id: &str,
    modified_at: i64,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    upsert_record_in_tx(
        &mut tx,
        SemanticRecordUpsert {
            record_id: stable_record_id("note", note_path),
            source_kind: SemanticSourceKind::Note,
            source_id: note_path.to_string(),
            chunk_id: capture_id.to_string(),
            content_text: format!("{title}\n{content_text}"),
            metadata_json: json!({
                "title": title,
                "capture_id": capture_id,
            }),
            provenance: serde_json::to_value(SemanticProvenance {
                capture_id: Some(capture_id.to_string()),
                note_path: Some(note_path.to_string()),
                ..Default::default()
            })?,
            occurred_at: modified_at,
        },
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn semantic_record_count(pool: &SqlitePool) -> Result<i64, StorageError> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM semantic_memory_records")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub(crate) async fn rebuild_index(pool: &SqlitePool) -> Result<u64, StorageError> {
    let captures = sqlx::query(
        r#"SELECT capture_id, content_text, occurred_at FROM captures ORDER BY occurred_at ASC"#,
    )
    .fetch_all(pool)
    .await?;

    let mut tx = pool.begin().await?;
    sqlx::query(r#"DELETE FROM semantic_term_weights WHERE record_id LIKE 'sem_cap_%'"#)
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        r#"
        DELETE FROM semantic_term_weights
        WHERE record_id IN (
            SELECT record_id
            FROM semantic_memory_records
            WHERE source_kind IN ('project', 'note', 'transcript_note', 'thread', 'person')
        )
        "#,
    )
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        DELETE FROM semantic_memory_records
        WHERE source_kind IN ('capture', 'project', 'note', 'transcript_note', 'thread', 'person')
        "#,
    )
    .execute(&mut *tx)
    .await?;

    for row in &captures {
        upsert_capture_record_in_tx(
            &mut tx,
            &row.try_get::<String, _>("capture_id")?,
            &row.try_get::<String, _>("content_text")?,
            row.try_get::<i64, _>("occurred_at")?,
        )
        .await?;
    }

    let projects = sqlx::query(
        r#"
        SELECT
            id,
            slug,
            name,
            family,
            status,
            primary_repo_path,
            primary_notes_root,
            secondary_repo_paths_json,
            secondary_notes_roots_json,
            upstream_ids_json,
            pending_provision_json,
            created_at,
            updated_at,
            archived_at
        FROM projects
        ORDER BY updated_at ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let projects_len = projects.len();
    for row in projects {
        let project = map_project_record_row(row)?;
        upsert_project_record_in_tx(&mut tx, &project).await?;
    }

    let notes = sqlx::query(
        r#"
        SELECT
            json_extract(s.payload_json, '$.path') AS note_path,
            json_extract(s.payload_json, '$.title') AS title,
            json_extract(s.payload_json, '$.capture_id') AS capture_id,
            c.content_text AS content_text,
            s.timestamp AS modified_at
        FROM signals s
        JOIN captures c
            ON c.capture_id = json_extract(s.payload_json, '$.capture_id')
        WHERE s.source = 'notes'
          AND s.signal_type = 'note_document'
        ORDER BY s.timestamp ASC, s.created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let notes_len = notes.len();
    for row in notes {
        upsert_record_in_tx(
            &mut tx,
            SemanticRecordUpsert {
                record_id: stable_record_id("note", &row.try_get::<String, _>("note_path")?),
                source_kind: SemanticSourceKind::Note,
                source_id: row.try_get("note_path")?,
                chunk_id: row.try_get("capture_id")?,
                content_text: format!(
                    "{}\n{}",
                    row.try_get::<String, _>("title")?,
                    row.try_get::<String, _>("content_text")?
                ),
                metadata_json: json!({
                    "title": row.try_get::<String, _>("title")?,
                    "capture_id": row.try_get::<String, _>("capture_id")?,
                }),
                provenance: serde_json::to_value(SemanticProvenance {
                    capture_id: Some(row.try_get("capture_id")?),
                    note_path: Some(row.try_get("note_path")?),
                    ..Default::default()
                })?,
                occurred_at: row.try_get("modified_at")?,
            },
        )
        .await?;
    }

    let transcripts = sqlx::query(
        r#"
        SELECT id, source, conversation_id, message_id, timestamp, role, content, metadata_json
        FROM assistant_transcripts
        ORDER BY timestamp ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let transcripts_len = transcripts.len();
    for row in transcripts {
        let input = AssistantTranscriptInsert {
            id: row.try_get("id")?,
            source: row.try_get("source")?,
            conversation_id: row.try_get("conversation_id")?,
            message_id: row.try_get("message_id")?,
            timestamp: row.try_get("timestamp")?,
            role: row.try_get("role")?,
            content: row.try_get("content")?,
            metadata_json: serde_json::from_str(&row.try_get::<String, _>("metadata_json")?)?,
        };
        upsert_transcript_note_record_in_tx(&mut tx, &input).await?;
    }

    let threads = sqlx::query(
        r#"SELECT id, thread_type, title, status, updated_at FROM threads ORDER BY updated_at ASC"#,
    )
    .fetch_all(pool)
    .await?;
    let threads_len = threads.len();
    for row in threads {
        upsert_record_in_tx(
            &mut tx,
            SemanticRecordUpsert {
                record_id: stable_record_id("thread", &row.try_get::<String, _>("id")?),
                source_kind: SemanticSourceKind::Thread,
                source_id: row.try_get("id")?,
                chunk_id: row.try_get("id")?,
                content_text: format!(
                    "{} {} {}",
                    row.try_get::<String, _>("title")?,
                    row.try_get::<String, _>("thread_type")?,
                    row.try_get::<String, _>("status")?
                ),
                metadata_json: json!({
                    "thread_type": row.try_get::<String, _>("thread_type")?,
                    "status": row.try_get::<String, _>("status")?,
                }),
                provenance: serde_json::to_value(SemanticProvenance {
                    thread_id: Some(row.try_get("id")?),
                    ..Default::default()
                })?,
                occurred_at: row.try_get("updated_at")?,
            },
        )
        .await?;
    }

    let people = load_people_for_index(pool).await?;
    let people_len = people.len();
    for person in &people {
        upsert_person_record_in_tx(&mut tx, person).await?;
    }

    tx.commit().await?;
    Ok(
        (captures.len() + projects_len + notes_len + transcripts_len + threads_len + people_len)
            as u64,
    )
}

pub(crate) async fn semantic_query(
    pool: &SqlitePool,
    query: &SemanticQuery,
) -> Result<Vec<SemanticHit>, StorageError> {
    let backend = backend();
    let query_terms = backend.embed(&query.query_text);
    if query_terms.is_empty() {
        return Ok(Vec::new());
    }

    let semantic_scores = load_semantic_scores(pool, query, &query_terms).await?;
    let lexical_scores = match query.strategy {
        RetrievalStrategy::SemanticOnly => HashMap::new(),
        RetrievalStrategy::LexicalOnly | RetrievalStrategy::Hybrid => {
            load_lexical_scores(pool, query, query.top_k.max(5) * 4).await?
        }
    };

    let policy = query.policy.clone().unwrap_or(HybridRetrievalPolicy {
        lexical_weight: 0.35,
        semantic_weight: 0.65,
        rerank_window: query.top_k.max(5) * 4,
        min_combined_score: 0.05,
    });

    let mut candidate_ids = semantic_scores.keys().cloned().collect::<HashSet<_>>();
    if matches!(
        query.strategy,
        RetrievalStrategy::LexicalOnly | RetrievalStrategy::Hybrid
    ) {
        candidate_ids.extend(lexical_scores.keys().cloned());
    }
    if candidate_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut candidate_ids = candidate_ids
        .into_iter()
        .map(|record_id| {
            let semantic_score = semantic_scores.get(&record_id).copied().unwrap_or_default();
            let lexical_score = lexical_scores.get(&record_id).copied().unwrap_or_default();
            let preliminary =
                combine_scores(&query.strategy, &policy, lexical_score, semantic_score);
            (record_id, preliminary)
        })
        .collect::<Vec<_>>();
    candidate_ids.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    candidate_ids.truncate(policy.rerank_window.clamp(query.top_k.max(1), 100) as usize);
    let candidate_ids = candidate_ids
        .into_iter()
        .map(|(record_id, _)| record_id)
        .collect::<Vec<_>>();
    let rows = load_semantic_rows(pool, &candidate_ids, query).await?;

    let mut hits = rows
        .into_iter()
        .map(|row| {
            let semantic_score = semantic_scores
                .get(&row.record_id)
                .copied()
                .unwrap_or_default();
            let lexical_score = lexical_scores
                .get(&row.record_id)
                .copied()
                .unwrap_or_default();
            let combined_score =
                combine_scores(&query.strategy, &policy, lexical_score, semantic_score);
            let provenance = serde_json::from_str::<SemanticProvenance>(&row.provenance_json)?;

            Ok(SemanticHit {
                record_id: SemanticRecordId::new(row.record_id),
                source_kind: parse_source_kind(&row.source_kind)?,
                source_id: row.source_id,
                snippet: backend.snippet(&row.content_text, &query.query_text),
                lexical_score,
                semantic_score,
                combined_score,
                provenance,
            })
        })
        .collect::<Result<Vec<_>, StorageError>>()?;

    hits.retain(|hit| hit.combined_score >= policy.min_combined_score);
    hits.sort_by(|a, b| {
        b.combined_score
            .total_cmp(&a.combined_score)
            .then_with(|| b.semantic_score.total_cmp(&a.semantic_score))
            .then_with(|| a.source_id.cmp(&b.source_id))
    });
    hits.truncate(query.top_k.clamp(1, 50) as usize);

    if matches!(query.strategy, RetrievalStrategy::LexicalOnly) {
        for hit in &mut hits {
            hit.semantic_score = 0.0;
            hit.combined_score = hit.lexical_score;
        }
    }

    if !query.include_provenance {
        for hit in &mut hits {
            hit.provenance = SemanticProvenance::default();
        }
    }

    Ok(hits)
}

pub(crate) async fn get_semantic_record(
    pool: &SqlitePool,
    record_id: &str,
) -> Result<Option<SemanticMemoryRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            record_id,
            source_kind,
            source_id,
            chunk_id,
            content_text,
            embedding_model,
            embedding_revision,
            token_count,
            metadata_json,
            provenance_json
        FROM semantic_memory_records
        WHERE record_id = ?
        "#,
    )
    .bind(record_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    Ok(Some(SemanticMemoryRecord {
        record_id: SemanticRecordId::new(row.try_get::<String, _>("record_id")?),
        source_kind: parse_source_kind(&row.try_get::<String, _>("source_kind")?)?,
        source_id: row.try_get("source_id")?,
        chunk_id: row.try_get("chunk_id")?,
        content_text: row.try_get("content_text")?,
        embedding_model: row.try_get("embedding_model")?,
        embedding_revision: row.try_get("embedding_revision")?,
        token_count: row.try_get::<i64, _>("token_count")? as u32,
        metadata_json: serde_json::from_str::<Value>(&row.try_get::<String, _>("metadata_json")?)?,
        provenance: serde_json::from_str::<SemanticProvenance>(
            &row.try_get::<String, _>("provenance_json")?,
        )?,
    }))
}

async fn load_semantic_scores(
    pool: &SqlitePool,
    query: &SemanticQuery,
    query_terms: &[(String, f32)],
) -> Result<HashMap<String, f32>, StorageError> {
    let mut builder = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT st.record_id, st.term, st.weight
        FROM semantic_term_weights st
        JOIN semantic_memory_records smr ON smr.record_id = st.record_id
        WHERE st.term IN (
        "#,
    );
    let mut separated = builder.separated(", ");
    for (term, _) in query_terms {
        separated.push_bind(term);
    }
    separated.push_unseparated(")");
    apply_filters(&mut builder, query);

    let rows = builder.build().fetch_all(pool).await?;
    let query_weights = query_terms.iter().cloned().collect::<HashMap<_, _>>();
    let mut scores = HashMap::<String, f32>::new();

    for row in rows {
        let record_id = row.try_get::<String, _>("record_id")?;
        let term = row.try_get::<String, _>("term")?;
        let weight = row.try_get::<f64, _>("weight")? as f32;
        let query_weight = query_weights.get(&term).copied().unwrap_or_default();
        *scores.entry(record_id).or_default() += weight * query_weight;
    }

    Ok(scores)
}

async fn load_lexical_scores(
    pool: &SqlitePool,
    query: &SemanticQuery,
    limit: u32,
) -> Result<HashMap<String, f32>, StorageError> {
    let mut scores = load_semantic_record_lexical_scores(pool, query, limit).await?;
    let Some(capture_fts_query) = capture_fts_query_text(&query.query_text) else {
        return Ok(scores);
    };

    let rows = sqlx::query(
        r#"
        SELECT c.capture_id, bm25(captures_fts) AS rank
        FROM captures_fts
        JOIN captures c ON c.capture_id = captures_fts.capture_id
        WHERE captures_fts MATCH ?
        ORDER BY rank ASC, c.occurred_at DESC, c.created_at DESC
        LIMIT ?
        "#,
    )
    .bind(capture_fts_query)
    .bind(i64::from(limit.clamp(1, 100)))
    .fetch_all(pool)
    .await?;

    for row in rows {
        let capture_id = row.try_get::<String, _>("capture_id")?;
        let rank = row.try_get::<f64, _>("rank")? as f32;
        let record_id = format!("sem_cap_{capture_id}");
        let score = lexical_score(rank);
        scores
            .entry(record_id)
            .and_modify(|existing| *existing = existing.max(score))
            .or_insert(score);
    }
    Ok(scores)
}

async fn load_semantic_record_lexical_scores(
    pool: &SqlitePool,
    query: &SemanticQuery,
    limit: u32,
) -> Result<HashMap<String, f32>, StorageError> {
    let query_terms = tokenize(&query.query_text);
    if query_terms.is_empty() {
        return Ok(HashMap::new());
    }

    let mut builder = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT smr.record_id, smr.content_text, smr.source_id
        FROM semantic_memory_records smr
        WHERE 1 = 1
        "#,
    );
    apply_filters(&mut builder, query);
    let rows = builder.build().fetch_all(pool).await?;

    let mut scored = rows
        .into_iter()
        .filter_map(|row| {
            let record_id = row.try_get::<String, _>("record_id").ok()?;
            let content_text = row.try_get::<String, _>("content_text").ok()?;
            let source_id = row.try_get::<String, _>("source_id").ok()?;
            let score = semantic_record_lexical_score(&query_terms, &content_text, &source_id);
            (score > 0.0).then_some((record_id, score))
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    scored.truncate(limit.clamp(1, 100) as usize);
    Ok(scored.into_iter().collect())
}

async fn load_semantic_rows(
    pool: &SqlitePool,
    record_ids: &[String],
    query: &SemanticQuery,
) -> Result<Vec<SemanticRow>, StorageError> {
    let mut builder = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT smr.record_id, smr.source_kind, smr.source_id, smr.content_text, smr.provenance_json
        FROM semantic_memory_records smr
        WHERE record_id IN (
        "#,
    );
    let mut separated = builder.separated(", ");
    for record_id in record_ids {
        separated.push_bind(record_id);
    }
    separated.push_unseparated(")");
    apply_filters(&mut builder, query);

    let rows = builder.build().fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| {
            Ok(SemanticRow {
                record_id: row.try_get("record_id")?,
                source_kind: row.try_get("source_kind")?,
                source_id: row.try_get("source_id")?,
                content_text: row.try_get("content_text")?,
                provenance_json: row.try_get("provenance_json")?,
            })
        })
        .collect()
}

fn apply_filters<'a>(builder: &mut QueryBuilder<'a, Sqlite>, query: &'a SemanticQuery) {
    if !query.filters.source_kinds.is_empty() {
        builder.push(" AND smr.source_kind IN (");
        let mut separated = builder.separated(", ");
        for kind in &query.filters.source_kinds {
            separated.push_bind(source_kind_str(*kind));
        }
        separated.push_unseparated(")");
    }

    if !query.filters.source_ids.is_empty() {
        builder.push(" AND smr.source_id IN (");
        let mut separated = builder.separated(", ");
        for source_id in &query.filters.source_ids {
            separated.push_bind(source_id);
        }
        separated.push_unseparated(")");
    }

    if let Some(trace_id) = query.filters.trace_id.as_deref() {
        builder.push(" AND smr.trace_id = ");
        builder.push_bind(trace_id);
    }
}

fn source_kind_str(kind: SemanticSourceKind) -> &'static str {
    match kind {
        SemanticSourceKind::Capture => "capture",
        SemanticSourceKind::Artifact => "artifact",
        SemanticSourceKind::Project => "project",
        SemanticSourceKind::Note => "note",
        SemanticSourceKind::TranscriptNote => "transcript_note",
        SemanticSourceKind::Thread => "thread",
        SemanticSourceKind::Message => "message",
        SemanticSourceKind::Person => "person",
    }
}

fn parse_source_kind(value: &str) -> Result<SemanticSourceKind, StorageError> {
    match value {
        "capture" => Ok(SemanticSourceKind::Capture),
        "artifact" => Ok(SemanticSourceKind::Artifact),
        "project" => Ok(SemanticSourceKind::Project),
        "note" => Ok(SemanticSourceKind::Note),
        "transcript_note" => Ok(SemanticSourceKind::TranscriptNote),
        "thread" => Ok(SemanticSourceKind::Thread),
        "message" => Ok(SemanticSourceKind::Message),
        "person" => Ok(SemanticSourceKind::Person),
        other => Err(StorageError::DataCorrupted(format!(
            "unknown semantic source kind: {other}"
        ))),
    }
}

fn combine_scores(
    strategy: &RetrievalStrategy,
    policy: &HybridRetrievalPolicy,
    lexical_score: f32,
    semantic_score: f32,
) -> f32 {
    match strategy {
        RetrievalStrategy::LexicalOnly => lexical_score,
        RetrievalStrategy::SemanticOnly => semantic_score,
        RetrievalStrategy::Hybrid => {
            (lexical_score * policy.lexical_weight) + (semantic_score * policy.semantic_weight)
        }
    }
}

fn lexical_score(rank: f32) -> f32 {
    1.0 / (1.0 + rank.abs())
}

fn semantic_record_lexical_score(
    query_terms: &[String],
    content_text: &str,
    source_id: &str,
) -> f32 {
    let haystack = format!(
        "{} {}",
        content_text.to_lowercase(),
        source_id.to_lowercase()
    );
    let matched = query_terms
        .iter()
        .filter(|term| haystack.contains(term.as_str()))
        .count();
    if matched == 0 {
        return 0.0;
    }

    let coverage = matched as f32 / query_terms.len().max(1) as f32;
    let exact_phrase_bonus = if haystack.contains(&query_terms.join(" ")) {
        0.15
    } else {
        0.0
    };
    (coverage + exact_phrase_bonus).min(1.0)
}

fn embed_text(text: &str) -> Vec<(String, f32)> {
    let mut counts = HashMap::<String, usize>::new();
    for token in tokenize(text) {
        *counts.entry(token).or_default() += 1;
    }

    let total = counts.values().sum::<usize>().max(1) as f32;
    let mut weighted = counts
        .into_iter()
        .map(|(term, count)| (term, count as f32 / total))
        .collect::<Vec<_>>();
    weighted.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    weighted.truncate(MAX_TERMS);
    weighted
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter_map(|token| {
            let token = token.trim().to_lowercase();
            if token.len() < 3 || stopwords().contains(token.as_str()) {
                None
            } else {
                Some(token)
            }
        })
        .collect()
}

fn capture_fts_query_text(input: &str) -> Option<String> {
    let tokens = tokenize(input);
    (!tokens.is_empty()).then(|| tokens.join(" "))
}

fn stopwords() -> HashSet<&'static str> {
    [
        "about", "after", "again", "also", "been", "from", "have", "into", "just", "memo", "more",
        "note", "notes", "quick", "some", "that", "this", "today", "with", "work", "would",
    ]
    .into_iter()
    .collect()
}

fn snippet_for(text: &str, query: &str) -> String {
    let lower = text.to_lowercase();
    for term in tokenize(query) {
        if let Some(index) = lower.find(&term) {
            let start = index.saturating_sub(24);
            let end = (index + term.len() + 48).min(text.len());
            let snippet = text[start..end].trim();
            if start > 0 || end < text.len() {
                return format!("...{snippet}...");
            }
            return snippet.to_string();
        }
    }
    text.chars().take(96).collect()
}

struct SemanticRecordUpsert {
    record_id: String,
    source_kind: SemanticSourceKind,
    source_id: String,
    chunk_id: String,
    content_text: String,
    metadata_json: Value,
    provenance: Value,
    occurred_at: i64,
}

async fn upsert_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    record: SemanticRecordUpsert,
) -> Result<(), StorageError> {
    let backend = backend();
    let terms = backend.embed(&record.content_text);

    sqlx::query(
        r#"
        INSERT INTO semantic_memory_records (
            record_id,
            source_kind,
            source_id,
            chunk_id,
            content_text,
            embedding_model,
            embedding_revision,
            token_count,
            metadata_json,
            provenance_json,
            capture_id,
            artifact_id,
            thread_id,
            message_id,
            run_id,
            trace_id,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(record_id) DO UPDATE SET
            source_kind = excluded.source_kind,
            source_id = excluded.source_id,
            chunk_id = excluded.chunk_id,
            content_text = excluded.content_text,
            embedding_model = excluded.embedding_model,
            embedding_revision = excluded.embedding_revision,
            token_count = excluded.token_count,
            metadata_json = excluded.metadata_json,
            provenance_json = excluded.provenance_json,
            capture_id = excluded.capture_id,
            artifact_id = excluded.artifact_id,
            thread_id = excluded.thread_id,
            message_id = excluded.message_id,
            run_id = excluded.run_id,
            trace_id = excluded.trace_id,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&record.record_id)
    .bind(source_kind_str(record.source_kind))
    .bind(&record.source_id)
    .bind(&record.chunk_id)
    .bind(&record.content_text)
    .bind(backend.embedding_model())
    .bind(backend.embedding_revision())
    .bind(terms.len() as i64)
    .bind(record.metadata_json.to_string())
    .bind(record.provenance.to_string())
    .bind(record.provenance.get("capture_id").and_then(Value::as_str))
    .bind(record.provenance.get("artifact_id").and_then(Value::as_str))
    .bind(record.provenance.get("thread_id").and_then(Value::as_str))
    .bind(record.provenance.get("message_id").and_then(Value::as_str))
    .bind(record.provenance.get("run_id").and_then(Value::as_str))
    .bind(record.provenance.get("trace_id").and_then(Value::as_str))
    .bind(record.occurred_at)
    .bind(record.occurred_at)
    .execute(&mut **tx)
    .await?;

    sqlx::query("DELETE FROM semantic_term_weights WHERE record_id = ?")
        .bind(&record.record_id)
        .execute(&mut **tx)
        .await?;

    for (term, weight) in terms {
        sqlx::query(
            r#"INSERT INTO semantic_term_weights (record_id, term, weight) VALUES (?, ?, ?)"#,
        )
        .bind(&record.record_id)
        .bind(term)
        .bind(weight)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

async fn upsert_person_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    person: &PersonRecord,
) -> Result<(), StorageError> {
    let content_text = person_index_text(person);
    let external_id = person.aliases.iter().find_map(|alias| {
        alias
            .source_ref
            .as_ref()
            .map(|value| value.external_id.clone())
    });
    upsert_record_in_tx(
        tx,
        SemanticRecordUpsert {
            record_id: stable_record_id("person", person.id.as_ref()),
            source_kind: SemanticSourceKind::Person,
            source_id: person.id.to_string(),
            chunk_id: person.id.to_string(),
            content_text,
            metadata_json: json!({
                "alias_count": person.aliases.len(),
                "relationship_context": person.relationship_context,
            }),
            provenance: serde_json::to_value(SemanticProvenance {
                person_id: Some(person.id.to_string()),
                external_id,
                ..Default::default()
            })?,
            occurred_at: person
                .last_contacted_at
                .map(|value| value.unix_timestamp())
                .unwrap_or_else(|| time::OffsetDateTime::now_utc().unix_timestamp()),
        },
    )
    .await
}

fn stable_record_id(prefix: &str, source_id: &str) -> String {
    let normalized = source_id
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    format!("sem_{prefix}_{normalized}")
}

fn person_index_text(person: &PersonRecord) -> String {
    let mut parts = vec![person.display_name.clone()];
    if let Some(value) = person.given_name.as_ref() {
        parts.push(value.clone());
    }
    if let Some(value) = person.family_name.as_ref() {
        parts.push(value.clone());
    }
    if let Some(value) = person.relationship_context.as_ref() {
        parts.push(value.clone());
    }
    for PersonAlias {
        platform,
        handle,
        display,
        ..
    } in &person.aliases
    {
        parts.push(platform.clone());
        parts.push(handle.clone());
        if !display.trim().is_empty() {
            parts.push(display.clone());
        }
    }
    parts.join(" ")
}

async fn load_people_for_index(pool: &SqlitePool) -> Result<Vec<PersonRecord>, StorageError> {
    let people_rows = sqlx::query(
        r#"
        SELECT id, display_name, given_name, family_name, relationship_context, birthday, last_contacted_at
        FROM people
        ORDER BY updated_at ASC, created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    let alias_rows = sqlx::query(
        r#"
        SELECT person_id, platform, handle, display, source_ref_json
        FROM person_aliases
        ORDER BY created_at ASC, platform ASC, handle ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut aliases_by_person = HashMap::<String, Vec<PersonAlias>>::new();
    for row in alias_rows {
        let source_ref_json = row.try_get::<String, _>("source_ref_json")?;
        let source_ref = match source_ref_json.trim() {
            "" | "{}" | "null" => None,
            value => Some(serde_json::from_str(value)?),
        };
        aliases_by_person
            .entry(row.try_get("person_id")?)
            .or_default()
            .push(PersonAlias {
                platform: row.try_get("platform")?,
                handle: row.try_get("handle")?,
                display: row
                    .try_get::<Option<String>, _>("display")?
                    .unwrap_or_default(),
                source_ref,
            });
    }

    let mut people = Vec::with_capacity(people_rows.len());
    for row in people_rows {
        let person_id = row.try_get::<String, _>("id")?;
        people.push(PersonRecord {
            id: person_id.clone().into(),
            display_name: row.try_get("display_name")?,
            given_name: row.try_get("given_name")?,
            family_name: row.try_get("family_name")?,
            relationship_context: row.try_get("relationship_context")?,
            birthday: row.try_get("birthday")?,
            last_contacted_at: row
                .try_get::<Option<i64>, _>("last_contacted_at")?
                .map(crate::mapping::timestamp_to_datetime)
                .transpose()?,
            aliases: aliases_by_person.remove(&person_id).unwrap_or_default(),
            links: Vec::new(),
        });
    }
    Ok(people)
}

fn map_project_record_row(row: sqlx::sqlite::SqliteRow) -> Result<ProjectRecord, StorageError> {
    let secondary_repo_paths: Vec<String> =
        serde_json::from_str(&row.try_get::<String, _>("secondary_repo_paths_json")?)?;
    let secondary_notes_roots: Vec<String> =
        serde_json::from_str(&row.try_get::<String, _>("secondary_notes_roots_json")?)?;
    let upstream_ids = serde_json::from_str(&row.try_get::<String, _>("upstream_ids_json")?)?;
    let pending_provision =
        serde_json::from_str(&row.try_get::<String, _>("pending_provision_json")?)?;
    let created_at = crate::mapping::timestamp_to_datetime(row.try_get("created_at")?)?;
    let updated_at = crate::mapping::timestamp_to_datetime(row.try_get("updated_at")?)?;
    let archived_at = row
        .try_get::<Option<i64>, _>("archived_at")?
        .map(crate::mapping::timestamp_to_datetime)
        .transpose()?;
    let primary_repo: String = row.try_get("primary_repo_path")?;
    let primary_notes_root: String = row.try_get("primary_notes_root")?;

    Ok(ProjectRecord {
        id: row.try_get::<String, _>("id")?.into(),
        slug: row.try_get("slug")?,
        name: row.try_get("name")?,
        family: row
            .try_get::<String, _>("family")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        status: row
            .try_get::<String, _>("status")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        primary_repo: vel_core::ProjectRootRef {
            label: root_label(&primary_repo),
            path: primary_repo,
            kind: "repo".to_string(),
        },
        primary_notes_root: vel_core::ProjectRootRef {
            label: root_label(&primary_notes_root),
            path: primary_notes_root,
            kind: "notes_root".to_string(),
        },
        secondary_repos: secondary_repo_paths
            .into_iter()
            .map(|path| vel_core::ProjectRootRef {
                label: root_label(&path),
                path,
                kind: "repo".to_string(),
            })
            .collect(),
        secondary_notes_roots: secondary_notes_roots
            .into_iter()
            .map(|path| vel_core::ProjectRootRef {
                label: root_label(&path),
                path,
                kind: "notes_root".to_string(),
            })
            .collect(),
        upstream_ids,
        pending_provision,
        created_at,
        updated_at,
        archived_at,
    })
}

fn root_label(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(path)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{AssistantTranscriptInsert, CaptureInsert, Storage};
    use std::collections::BTreeMap;
    use time::OffsetDateTime;
    use vel_core::{
        HybridRetrievalPolicy, IntegrationConnectionId, IntegrationFamily, IntegrationSourceRef,
        PersonAlias, PersonId, PersonRecord, PrivacyClass, ProjectFamily, ProjectId,
        ProjectProvisionRequest, ProjectRecord, ProjectRootRef, ProjectStatus, RetrievalStrategy,
        SemanticQueryFilters,
    };

    #[tokio::test]
    async fn insert_capture_indexes_semantic_record() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let capture_id = storage
            .insert_capture_at(
                CaptureInsert {
                    content_text: "Quarterly tax estimate review with accountant".to_string(),
                    capture_type: "quick_note".to_string(),
                    source_device: Some("test-phone".to_string()),
                    privacy_class: PrivacyClass::Private,
                },
                1_742_000_000,
            )
            .await
            .unwrap();

        let record = storage
            .get_semantic_record(&format!("sem_cap_{capture_id}"))
            .await
            .unwrap()
            .expect("semantic record should exist");
        assert_eq!(record.source_kind, SemanticSourceKind::Capture);
        assert_eq!(
            record.provenance.capture_id.as_deref(),
            Some(capture_id.as_ref())
        );
        assert_eq!(storage.semantic_record_count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn hybrid_query_returns_related_capture_with_provenance() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        storage
            .insert_capture_at(
                CaptureInsert {
                    content_text: "Remember to finish quarterly tax estimate with accountant"
                        .to_string(),
                    capture_type: "quick_note".to_string(),
                    source_device: None,
                    privacy_class: PrivacyClass::Private,
                },
                1_742_000_100,
            )
            .await
            .unwrap();
        storage
            .insert_capture_at(
                CaptureInsert {
                    content_text: "Buy coffee beans and oat milk".to_string(),
                    capture_type: "quick_note".to_string(),
                    source_device: None,
                    privacy_class: PrivacyClass::Private,
                },
                1_742_000_200,
            )
            .await
            .unwrap();

        let hits = storage
            .semantic_query(&SemanticQuery {
                query_text: "tax accountant follow up".to_string(),
                top_k: 3,
                strategy: RetrievalStrategy::Hybrid,
                include_provenance: true,
                filters: SemanticQueryFilters {
                    source_kinds: vec![SemanticSourceKind::Capture],
                    ..Default::default()
                },
                policy: Some(HybridRetrievalPolicy {
                    lexical_weight: 0.25,
                    semantic_weight: 0.75,
                    rerank_window: 8,
                    min_combined_score: 0.01,
                }),
            })
            .await
            .unwrap();

        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].source_kind, SemanticSourceKind::Capture);
        assert!(hits[0].combined_score > 0.0);
        assert!(hits[0].snippet.to_lowercase().contains("tax"));
        assert!(hits[0].provenance.capture_id.is_some());
    }

    #[test]
    fn capture_fts_query_text_strips_conversational_punctuation() {
        assert_eq!(
            capture_fts_query_text("What do I need to remember about the accountant?"),
            Some("need remember accountant".to_string())
        );
        assert_eq!(
            capture_fts_query_text("Help me close out today."),
            Some("help close".to_string())
        );
    }

    #[tokio::test]
    async fn hybrid_query_assigns_lexical_score_to_non_capture_records() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();

        storage
            .create_project(ProjectRecord {
                id: ProjectId::from("proj_tax_ops".to_string()),
                slug: "tax-ops".to_string(),
                name: "Tax Ops".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/tax-ops".to_string(),
                    label: "tax-ops".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/notes/tax-ops".to_string(),
                    label: "tax-ops".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::new(),
                pending_provision: ProjectProvisionRequest {
                    create_repo: false,
                    create_notes_root: false,
                },
                created_at: now,
                updated_at: now,
                archived_at: None,
            })
            .await
            .unwrap();
        storage
            .upsert_note_semantic_record(
                "projects/tax-ops/accountant.md",
                "Accountant follow up",
                "Need accountant follow up on quarterly estimate.",
                "cap_note_tax_ops",
                now.unix_timestamp(),
            )
            .await
            .unwrap();

        let hits = storage
            .semantic_query(&SemanticQuery {
                query_text: "accountant follow up tax ops".to_string(),
                top_k: 5,
                strategy: RetrievalStrategy::Hybrid,
                include_provenance: true,
                filters: SemanticQueryFilters {
                    source_kinds: vec![SemanticSourceKind::Project, SemanticSourceKind::Note],
                    ..Default::default()
                },
                policy: Some(HybridRetrievalPolicy {
                    lexical_weight: 0.45,
                    semantic_weight: 0.55,
                    rerank_window: 16,
                    min_combined_score: 0.01,
                }),
            })
            .await
            .unwrap();

        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::Project && hit.lexical_score > 0.0));
        assert!(hits
            .iter()
            .any(|hit| hit.source_kind == SemanticSourceKind::Note && hit.lexical_score > 0.0));
    }

    #[tokio::test]
    async fn semantic_memory_repo_indexes_phase6_entities_with_provenance() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();

        storage
            .create_project(ProjectRecord {
                id: ProjectId::from("proj_semantic_phase6".to_string()),
                slug: "accountant-ops".to_string(),
                name: "Accountant Ops".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/accountant-ops".to_string(),
                    label: "accountant-ops".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/notes/accountant-ops".to_string(),
                    label: "accountant-ops".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::from([(
                    "todoist_project_id".to_string(),
                    "todo_proj_1".to_string(),
                )]),
                pending_provision: ProjectProvisionRequest {
                    create_repo: false,
                    create_notes_root: false,
                },
                created_at: now,
                updated_at: now,
                archived_at: None,
            })
            .await
            .unwrap();
        storage
            .create_person(PersonRecord {
                id: PersonId::from("per_semantic_phase6".to_string()),
                display_name: "Annie Accountant".to_string(),
                given_name: Some("Annie".to_string()),
                family_name: None,
                relationship_context: Some("tax accountant".to_string()),
                birthday: None,
                last_contacted_at: Some(now),
                aliases: vec![PersonAlias {
                    platform: "email".to_string(),
                    handle: "annie@example.com".to_string(),
                    display: "Annie".to_string(),
                    source_ref: Some(IntegrationSourceRef {
                        family: IntegrationFamily::Messaging,
                        provider_key: "gmail".to_string(),
                        connection_id: IntegrationConnectionId::from("icn_semantic".to_string()),
                        external_id: "msg_annie".to_string(),
                    }),
                }],
                links: vec![],
            })
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_semantic_phase6",
                "planning_execution",
                "Accountant follow-up thread",
                "open",
                "{}",
            )
            .await
            .unwrap();
        storage
            .upsert_note_semantic_record(
                "projects/accountant-ops/follow-up.md",
                "Accountant follow-up",
                "Need accountant follow up on the quarterly estimate.",
                "cap_semantic_note",
                now.unix_timestamp(),
            )
            .await
            .unwrap();
        storage
            .insert_assistant_transcript(AssistantTranscriptInsert {
                id: "tr_semantic_phase6".to_string(),
                source: "chat".to_string(),
                conversation_id: "conv_semantic_phase6".to_string(),
                message_id: Some("msg_semantic_phase6".to_string()),
                timestamp: now.unix_timestamp(),
                role: "assistant".to_string(),
                content: "Draft the accountant follow up note.".to_string(),
                metadata_json: serde_json::json!({
                    "notes": { "source_subtype": "transcript" }
                }),
            })
            .await
            .unwrap();

        let hits = storage
            .semantic_query(&SemanticQuery {
                query_text: "accountant follow up ops".to_string(),
                top_k: 8,
                strategy: RetrievalStrategy::Hybrid,
                include_provenance: true,
                filters: SemanticQueryFilters {
                    source_kinds: vec![
                        SemanticSourceKind::Project,
                        SemanticSourceKind::Note,
                        SemanticSourceKind::TranscriptNote,
                        SemanticSourceKind::Thread,
                        SemanticSourceKind::Person,
                    ],
                    ..Default::default()
                },
                policy: Some(HybridRetrievalPolicy {
                    lexical_weight: 0.25,
                    semantic_weight: 0.75,
                    rerank_window: 16,
                    min_combined_score: 0.01,
                }),
            })
            .await
            .unwrap();

        assert!(hits.iter().any(|hit| {
            hit.source_kind == SemanticSourceKind::Project
                && hit.provenance.project_id.as_deref() == Some("proj_semantic_phase6")
                && hit.provenance.external_id.as_deref() == Some("todo_proj_1")
        }));
        assert!(hits.iter().any(|hit| {
            hit.source_kind == SemanticSourceKind::Note
                && hit.provenance.note_path.as_deref()
                    == Some("projects/accountant-ops/follow-up.md")
        }));
        assert!(hits.iter().any(|hit| {
            hit.source_kind == SemanticSourceKind::TranscriptNote
                && hit.provenance.transcript_id.as_deref() == Some("tr_semantic_phase6")
        }));
        assert!(hits.iter().any(|hit| {
            hit.source_kind == SemanticSourceKind::Thread
                && hit.provenance.thread_id.as_deref() == Some("thr_semantic_phase6")
        }));
        assert!(hits.iter().any(|hit| {
            hit.source_kind == SemanticSourceKind::Person
                && hit.provenance.person_id.as_deref() == Some("per_semantic_phase6")
                && hit.provenance.external_id.as_deref() == Some("msg_annie")
        }));
    }
}
