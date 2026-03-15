-- Steerable proposed adjustments / operational suggestions (accept, reject, modify).
CREATE TABLE IF NOT EXISTS suggestions (
  id TEXT PRIMARY KEY,
  suggestion_type TEXT NOT NULL,
  state TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  resolved_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_suggestions_state_created_at ON suggestions(state, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_suggestions_type_created_at ON suggestions(suggestion_type, created_at DESC);
