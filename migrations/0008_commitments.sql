-- First-class commitments: actionable, reviewable, statusful (distinct from raw captures).
CREATE TABLE IF NOT EXISTS commitments (
  id TEXT PRIMARY KEY,
  text TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_id TEXT,
  status TEXT NOT NULL,
  due_at INTEGER,
  project TEXT,
  commitment_kind TEXT,
  created_at INTEGER NOT NULL,
  resolved_at INTEGER,
  metadata_json TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_commitments_status ON commitments(status);
CREATE INDEX IF NOT EXISTS idx_commitments_due_at ON commitments(due_at);
CREATE INDEX IF NOT EXISTS idx_commitments_project ON commitments(project);
CREATE INDEX IF NOT EXISTS idx_commitments_source ON commitments(source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_commitments_created_at ON commitments(created_at);
