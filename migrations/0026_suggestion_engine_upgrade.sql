ALTER TABLE suggestions ADD COLUMN title TEXT;
ALTER TABLE suggestions ADD COLUMN summary TEXT;
ALTER TABLE suggestions ADD COLUMN priority INTEGER NOT NULL DEFAULT 50;
ALTER TABLE suggestions ADD COLUMN confidence TEXT;
ALTER TABLE suggestions ADD COLUMN dedupe_key TEXT;
ALTER TABLE suggestions ADD COLUMN decision_context_json TEXT;

CREATE TABLE IF NOT EXISTS suggestion_evidence (
  id TEXT PRIMARY KEY,
  suggestion_id TEXT NOT NULL,
  evidence_type TEXT NOT NULL,
  ref_id TEXT NOT NULL,
  evidence_json TEXT,
  weight REAL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (suggestion_id) REFERENCES suggestions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_suggestions_dedupe_key
  ON suggestions(dedupe_key, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_suggestion_evidence_suggestion
  ON suggestion_evidence(suggestion_id, created_at DESC);
