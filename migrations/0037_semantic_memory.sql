CREATE TABLE IF NOT EXISTS semantic_memory_records (
    record_id TEXT PRIMARY KEY,
    source_kind TEXT NOT NULL,
    source_id TEXT NOT NULL,
    chunk_id TEXT NOT NULL,
    content_text TEXT NOT NULL,
    embedding_model TEXT NOT NULL,
    embedding_revision TEXT NOT NULL,
    token_count INTEGER NOT NULL,
    metadata_json TEXT NOT NULL,
    provenance_json TEXT NOT NULL,
    capture_id TEXT,
    artifact_id TEXT,
    thread_id TEXT,
    message_id TEXT,
    run_id TEXT,
    trace_id TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_semantic_memory_source
    ON semantic_memory_records(source_kind, source_id);
CREATE INDEX IF NOT EXISTS idx_semantic_memory_trace_id
    ON semantic_memory_records(trace_id);

CREATE TABLE IF NOT EXISTS semantic_term_weights (
    record_id TEXT NOT NULL,
    term TEXT NOT NULL,
    weight REAL NOT NULL,
    PRIMARY KEY (record_id, term),
    FOREIGN KEY (record_id) REFERENCES semantic_memory_records(record_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_semantic_term_weights_term
    ON semantic_term_weights(term);
