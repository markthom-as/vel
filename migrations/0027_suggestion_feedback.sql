CREATE TABLE IF NOT EXISTS suggestion_feedback (
    id TEXT PRIMARY KEY,
    suggestion_id TEXT NOT NULL,
    outcome_type TEXT NOT NULL,
    notes TEXT,
    observed_at INTEGER NOT NULL,
    payload_json TEXT,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (suggestion_id) REFERENCES suggestions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_suggestion_feedback_suggestion_observed_at
    ON suggestion_feedback(suggestion_id, observed_at DESC);
