-- Provenance / generic refs (run->artifact, artifact->capture, etc.)
CREATE TABLE IF NOT EXISTS refs (
    ref_id TEXT PRIMARY KEY,
    from_type TEXT NOT NULL,
    from_id TEXT NOT NULL,
    to_type TEXT NOT NULL,
    to_id TEXT NOT NULL,
    relation_type TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_refs_from ON refs(from_type, from_id);
CREATE INDEX IF NOT EXISTS idx_refs_to ON refs(to_type, to_id);
CREATE INDEX IF NOT EXISTS idx_refs_relation_type ON refs(relation_type);

-- Artifact metadata: size_bytes (existing table has artifact_type, mime_type, content_hash, metadata_json)
ALTER TABLE artifacts ADD COLUMN size_bytes INTEGER;

CREATE INDEX IF NOT EXISTS idx_artifacts_artifact_type ON artifacts(artifact_type);
CREATE INDEX IF NOT EXISTS idx_artifacts_content_hash ON artifacts(content_hash);
