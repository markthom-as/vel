-- Broker events: audit trail for all capability grant/deny/execute decisions.
-- Every resolve_capability and deny_with_trace call persists a row here.
CREATE TABLE IF NOT EXISTS broker_events (
    id TEXT PRIMARY KEY NOT NULL,
    event_type TEXT NOT NULL,   -- 'grant' | 'deny' | 'execute'
    run_id TEXT NOT NULL,
    scope TEXT NOT NULL,
    resource TEXT,
    action TEXT NOT NULL,
    reason TEXT,
    occurred_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_broker_events_run_id ON broker_events (run_id);
