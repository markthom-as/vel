-- Runs: first-class execution records (context generation, synthesis, agent, etc.)
CREATE TABLE IF NOT EXISTS runs (
    run_id TEXT PRIMARY KEY,
    run_kind TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    started_at INTEGER,
    finished_at INTEGER,
    input_json TEXT NOT NULL DEFAULT '{}',
    output_json TEXT,
    error_json TEXT
);

CREATE INDEX IF NOT EXISTS idx_runs_status ON runs(status);
CREATE INDEX IF NOT EXISTS idx_runs_kind ON runs(run_kind);
CREATE INDEX IF NOT EXISTS idx_runs_created_at ON runs(created_at DESC);

-- Run events: append-only log per run (STEP_STARTED, TOOL_INVOKED, RUN_SUCCEEDED, etc.)
CREATE TABLE IF NOT EXISTS run_events (
    event_id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    seq INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    payload_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL,
    FOREIGN KEY (run_id) REFERENCES runs(run_id)
);

CREATE INDEX IF NOT EXISTS idx_run_events_run_id ON run_events(run_id);

-- Events: global append-only runtime event log (CAPTURE_CREATED, SEARCH_EXECUTED, etc.)
CREATE TABLE IF NOT EXISTS events (
    event_id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    subject_type TEXT NOT NULL,
    subject_id TEXT,
    payload_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_subject ON events(subject_type, subject_id);
CREATE INDEX IF NOT EXISTS idx_events_created_at ON events(created_at DESC);
