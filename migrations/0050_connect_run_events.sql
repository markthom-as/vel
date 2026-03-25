-- Connect runtime event stream for interactive delegated sessions.
-- Stores bounded stdout/stderr/system/stdin events for inspectable replay.
CREATE TABLE IF NOT EXISTS connect_run_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id TEXT NOT NULL,
    stream TEXT NOT NULL,
    chunk TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_connect_run_events_run_id_id
    ON connect_run_events (run_id, id);
