CREATE TABLE routine_blocks (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    source TEXT NOT NULL,
    local_timezone TEXT NOT NULL,
    start_local_time TEXT NOT NULL,
    end_local_time TEXT NOT NULL,
    days_of_week_json TEXT NOT NULL,
    protected INTEGER NOT NULL DEFAULT 0,
    active INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_routine_blocks_active_order
    ON routine_blocks(active, sort_order, created_at);

CREATE TABLE planning_constraints (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    kind TEXT NOT NULL,
    detail TEXT,
    time_window TEXT,
    minutes INTEGER,
    max_items INTEGER,
    active INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_planning_constraints_active_order
    ON planning_constraints(active, sort_order, created_at);
