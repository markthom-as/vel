CREATE TABLE IF NOT EXISTS execution_handoffs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    handoff_json TEXT NOT NULL,
    task_kind TEXT NOT NULL,
    agent_profile TEXT NOT NULL,
    token_budget TEXT NOT NULL,
    review_gate TEXT NOT NULL,
    origin_kind TEXT NOT NULL,
    review_state TEXT NOT NULL,
    routing_json TEXT NOT NULL DEFAULT '{}',
    manifest_id TEXT,
    requested_by TEXT NOT NULL,
    reviewed_by TEXT,
    decision_reason TEXT,
    reviewed_at INTEGER,
    launched_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_execution_handoffs_project_state
    ON execution_handoffs(project_id, review_state, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_execution_handoffs_state_updated
    ON execution_handoffs(review_state, updated_at DESC, created_at DESC);
