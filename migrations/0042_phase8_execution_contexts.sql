CREATE TABLE IF NOT EXISTS execution_contexts (
    project_id TEXT PRIMARY KEY,
    context_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_execution_contexts_updated_at
    ON execution_contexts(updated_at DESC, created_at DESC);
