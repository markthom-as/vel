CREATE TABLE IF NOT EXISTS captures (
    capture_id TEXT PRIMARY KEY,
    capture_type TEXT NOT NULL,
    content_text TEXT NOT NULL,
    occurred_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    source_device TEXT,
    privacy_class TEXT NOT NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS artifacts (
    artifact_id TEXT PRIMARY KEY,
    artifact_type TEXT NOT NULL,
    title TEXT,
    mime_type TEXT,
    storage_uri TEXT NOT NULL,
    privacy_class TEXT NOT NULL,
    sync_class TEXT NOT NULL,
    content_hash TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS processing_jobs (
    job_id TEXT PRIMARY KEY,
    job_type TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    started_at INTEGER,
    finished_at INTEGER,
    payload_json TEXT NOT NULL DEFAULT '{}',
    error_text TEXT
);
