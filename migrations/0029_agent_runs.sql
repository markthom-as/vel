CREATE TABLE IF NOT EXISTS agent_runs (
  run_id TEXT PRIMARY KEY,
  agent_id TEXT NOT NULL,
  parent_run_id TEXT,
  status TEXT NOT NULL,
  mission_input_json TEXT NOT NULL,
  deadline_ts INTEGER,
  ttl_seconds INTEGER NOT NULL,
  expires_at INTEGER NOT NULL,
  waiting_reason TEXT,
  return_contract TEXT NOT NULL,
  max_tool_calls INTEGER NOT NULL,
  max_tokens INTEGER NOT NULL,
  allowed_tools_json TEXT NOT NULL,
  memory_scope_json TEXT NOT NULL,
  summary TEXT,
  confidence REAL,
  structured_output_json TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY (run_id) REFERENCES runs(run_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_agent_runs_agent_id
  ON agent_runs(agent_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_agent_runs_status
  ON agent_runs(status, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_agent_runs_parent_run_id
  ON agent_runs(parent_run_id, created_at DESC);
