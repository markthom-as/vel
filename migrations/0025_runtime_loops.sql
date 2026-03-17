CREATE TABLE IF NOT EXISTS runtime_loops (
  loop_kind TEXT PRIMARY KEY,
  enabled INTEGER NOT NULL DEFAULT 1,
  interval_seconds INTEGER NOT NULL,
  last_started_at INTEGER,
  last_finished_at INTEGER,
  last_status TEXT,
  last_error TEXT,
  next_due_at INTEGER
);
