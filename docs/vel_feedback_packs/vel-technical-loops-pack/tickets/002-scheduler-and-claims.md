---
title: Scheduled Loop Claims and Safe Execution
status: proposed
priority: critical
owner: codex
---

# Goal

Give loops safe claim/execute semantics so they cannot stampede or overlap unpredictably.

# Concrete file targets

- `migrations/0024_runtime_loops.sql`
- `crates/vel-storage/src/db.rs`
- `crates/veld/src/worker.rs`

# Suggested schema

```sql
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
```

## Storage methods to add
- `claim_due_loop(loop_kind, now_ts) -> bool`
- `complete_loop(loop_kind, status, error, next_due_at)`
- `list_runtime_loops()`

## Implementation notes
You do not need distributed locking sophistication yet. SQLite-safe compare-and-set style updates are enough.

# Acceptance criteria

- loop executions are tracked
- next due times are inspectable
- accidental overlap is prevented
