-- Append-only event history for nudges (nudge_created, nudge_sent, nudge_snoozed, nudge_resolved, etc.).
CREATE TABLE IF NOT EXISTS nudge_events (
  id TEXT PRIMARY KEY,
  nudge_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  timestamp INTEGER NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nudge_events_nudge_time ON nudge_events(nudge_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_nudge_events_type_time ON nudge_events(event_type, timestamp DESC);
