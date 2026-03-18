use crate::db::StorageError;
use serde_json::{json, Value};
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool, Transaction};
use std::collections::{HashMap, HashSet};
use vel_core::{
    HybridRetrievalPolicy, RetrievalStrategy, SemanticHit, SemanticMemoryRecord,
    SemanticProvenance, SemanticQuery, SemanticRecordId, SemanticSourceKind,
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
    let backend = backend();
    let record_id = format!("sem_cap_{capture_id}");
    let terms = backend.embed(content_text);
    let provenance = json!({
        "capture_id": capture_id,
    });

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
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, NULL, ?, ?)
        ON CONFLICT(record_id) DO UPDATE SET
            content_text = excluded.content_text,
            embedding_model = excluded.embedding_model,
            embedding_revision = excluded.embedding_revision,
            token_count = excluded.token_count,
            metadata_json = excluded.metadata_json,
            provenance_json = excluded.provenance_json,
            capture_id = excluded.capture_id,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&record_id)
    .bind("capture")
    .bind(capture_id)
    .bind(capture_id)
    .bind(content_text)
    .bind(backend.embedding_model())
    .bind(backend.embedding_revision())
    .bind(terms.len() as i64)
    .bind(json!({ "backend": backend.embedding_model() }).to_string())
    .bind(provenance.to_string())
    .bind(capture_id)
    .bind(occurred_at)
    .bind(occurred_at)
    .execute(&mut **tx)
    .await?;

    sqlx::query("DELETE FROM semantic_term_weights WHERE record_id = ?")
        .bind(&record_id)
        .execute(&mut **tx)
        .await?;

    for (term, weight) in terms {
        sqlx::query(
            r#"INSERT INTO semantic_term_weights (record_id, term, weight) VALUES (?, ?, ?)"#,
        )
        .bind(&record_id)
        .bind(term)
        .bind(weight)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

pub(crate) async fn semantic_record_count(pool: &SqlitePool) -> Result<i64, StorageError> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM semantic_memory_records")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub(crate) async fn rebuild_capture_index(pool: &SqlitePool) -> Result<u64, StorageError> {
    let captures = sqlx::query(
        r#"SELECT capture_id, content_text, occurred_at FROM captures ORDER BY occurred_at ASC"#,
    )
    .fetch_all(pool)
    .await?;

    let mut tx = pool.begin().await?;
    sqlx::query(r#"DELETE FROM semantic_term_weights WHERE record_id LIKE 'sem_cap_%'"#)
        .execute(&mut *tx)
        .await?;
    sqlx::query(r#"DELETE FROM semantic_memory_records WHERE source_kind = 'capture'"#)
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

    tx.commit().await?;
    Ok(captures.len() as u64)
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
            load_lexical_scores(pool, &query.query_text, query.top_k.max(5) * 4).await?
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
            let preliminary = semantic_scores.get(&record_id).copied().unwrap_or_default();
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
                .get(&row.source_id)
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
    query_text: &str,
    limit: u32,
) -> Result<HashMap<String, f32>, StorageError> {
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
    .bind(query_text)
    .bind(i64::from(limit.clamp(1, 100)))
    .fetch_all(pool)
    .await?;

    let mut scores = HashMap::new();
    for row in rows {
        let capture_id = row.try_get::<String, _>("capture_id")?;
        let rank = row.try_get::<f64, _>("rank")? as f32;
        scores.insert(capture_id, lexical_score(rank));
    }
    Ok(scores)
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
        SemanticSourceKind::Thread => "thread",
        SemanticSourceKind::Message => "message",
    }
}

fn parse_source_kind(value: &str) -> Result<SemanticSourceKind, StorageError> {
    match value {
        "capture" => Ok(SemanticSourceKind::Capture),
        "artifact" => Ok(SemanticSourceKind::Artifact),
        "thread" => Ok(SemanticSourceKind::Thread),
        "message" => Ok(SemanticSourceKind::Message),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{CaptureInsert, Storage};
    use vel_core::{PrivacyClass, SemanticQueryFilters};

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
}
