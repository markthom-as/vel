---
title: Uncertainty Persistence Schema
status: proposed
priority: critical
owner: codex
---

# Goal

Add a durable table for uncertainty records.

# Concrete file targets

- `migrations/0024_uncertainty_records.sql`
- `crates/vel-storage/src/db.rs`

# Suggested schema

```sql
CREATE TABLE IF NOT EXISTS uncertainty_records (
  id TEXT PRIMARY KEY,
  subject_type TEXT NOT NULL,
  subject_id TEXT,
  decision_kind TEXT NOT NULL,
  confidence_band TEXT NOT NULL,
  confidence_score REAL,
  reasons_json TEXT NOT NULL,
  missing_evidence_json TEXT,
  resolution_mode TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'open',
  created_at INTEGER NOT NULL,
  resolved_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_uncertainty_subject
  ON uncertainty_records(subject_type, subject_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_uncertainty_open
  ON uncertainty_records(status, created_at DESC);
```

# Storage methods to add

- `insert_uncertainty_record(...)`
- `list_uncertainty_records(...)`
- `get_uncertainty_record(...)`
- `resolve_uncertainty_record(...)`

# Acceptance criteria

- uncertainty can be persisted and queried
- open uncertainty can be listed for operator inspection
