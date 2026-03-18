-- Connect agent run lifecycle records.
-- Each supervised agent launch creates a connect_run. Agents must heartbeat before
-- lease_expires_at to stay in 'running' state; expired leases transition to 'expired'.
CREATE TABLE IF NOT EXISTS connect_runs (
    id TEXT PRIMARY KEY NOT NULL,
    agent_id TEXT NOT NULL,
    node_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    capabilities_json TEXT NOT NULL DEFAULT '[]',
    lease_expires_at INTEGER NOT NULL,
    started_at INTEGER NOT NULL,
    terminated_at INTEGER,
    terminal_reason TEXT
);

CREATE INDEX IF NOT EXISTS idx_connect_runs_status ON connect_runs (status);
CREATE INDEX IF NOT EXISTS idx_connect_runs_lease ON connect_runs (lease_expires_at) WHERE status = 'running';
