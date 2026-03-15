-- Nudges: actionable prompts from inference engine; support done/snooze only.
CREATE TABLE IF NOT EXISTS nudges (
  nudge_id TEXT PRIMARY KEY,
  nudge_type TEXT NOT NULL,
  level TEXT NOT NULL,
  state TEXT NOT NULL,
  related_commitment_id TEXT,
  message TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  snoozed_until INTEGER,
  resolved_at INTEGER,
  signals_snapshot_json TEXT,
  inference_snapshot_json TEXT,
  metadata_json TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nudges_state ON nudges(state);
CREATE INDEX IF NOT EXISTS idx_nudges_type ON nudges(nudge_type);
CREATE INDEX IF NOT EXISTS idx_nudges_created_at ON nudges(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_nudges_snoozed_until ON nudges(snoozed_until);
