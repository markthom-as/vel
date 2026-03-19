CREATE TABLE daily_sessions (
    session_id TEXT PRIMARY KEY,
    session_date TEXT NOT NULL,
    phase TEXT NOT NULL,
    status TEXT NOT NULL,
    start_json TEXT NOT NULL,
    turn_state TEXT NOT NULL,
    current_prompt_json TEXT,
    state_json TEXT NOT NULL,
    outcome_json TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    completed_at INTEGER,
    cancelled_at INTEGER
);

CREATE INDEX idx_daily_sessions_active_lookup
    ON daily_sessions(session_date, phase, updated_at DESC)
    WHERE status IN ('active', 'waiting_for_input');
