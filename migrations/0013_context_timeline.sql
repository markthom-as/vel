-- Append-only snapshots of meaningful context transitions over time (for debugging and temporal synthesis).
CREATE TABLE IF NOT EXISTS context_timeline (
  id TEXT PRIMARY KEY,
  timestamp INTEGER NOT NULL,
  context_json TEXT NOT NULL,
  trigger_signal_id TEXT,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_context_timeline_timestamp ON context_timeline(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_context_timeline_trigger_signal ON context_timeline(trigger_signal_id);
