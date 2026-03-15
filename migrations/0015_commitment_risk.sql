-- Risk score snapshots for commitments over time (consequence, proximity, dependency pressure).
CREATE TABLE IF NOT EXISTS commitment_risk (
  id TEXT PRIMARY KEY,
  commitment_id TEXT NOT NULL,
  risk_score REAL NOT NULL,
  risk_level TEXT NOT NULL,
  factors_json TEXT NOT NULL,
  computed_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_commitment_risk_commitment_time ON commitment_risk(commitment_id, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_commitment_risk_level_time ON commitment_risk(risk_level, computed_at DESC);
