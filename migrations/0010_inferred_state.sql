-- Inferred context state from signals + commitments + time (inference engine output).
CREATE TABLE IF NOT EXISTS inferred_state (
  state_id TEXT PRIMARY KEY,
  state_name TEXT NOT NULL,
  confidence TEXT,
  timestamp INTEGER NOT NULL,
  context_json TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_inferred_state_name ON inferred_state(state_name);
CREATE INDEX IF NOT EXISTS idx_inferred_state_timestamp ON inferred_state(timestamp DESC);
