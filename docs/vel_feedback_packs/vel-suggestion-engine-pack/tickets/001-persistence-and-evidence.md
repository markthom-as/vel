---
title: Suggestion Persistence and Evidence Model
status: proposed
priority: critical
owner: codex
---

# Goal

Move suggestions from a minimal table to a proper inspectable steering record.

# Concrete file targets

- `migrations/0017_suggestions.sql` (do not edit in place; add a new migration)
- `migrations/0024_suggestion_engine_upgrade.sql`
- `crates/vel-storage/src/db.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/vel-core/src/lib.rs`

# Concrete code changes

## A. Add upgrade migration

Create migration:
- `migrations/0024_suggestion_engine_upgrade.sql`

Suggested DDL:
```sql
ALTER TABLE suggestions ADD COLUMN title TEXT;
ALTER TABLE suggestions ADD COLUMN summary TEXT;
ALTER TABLE suggestions ADD COLUMN priority INTEGER DEFAULT 50;
ALTER TABLE suggestions ADD COLUMN confidence TEXT;
ALTER TABLE suggestions ADD COLUMN dedupe_key TEXT;
ALTER TABLE suggestions ADD COLUMN decision_context_json TEXT;

CREATE TABLE IF NOT EXISTS suggestion_evidence (
  id TEXT PRIMARY KEY,
  suggestion_id TEXT NOT NULL,
  evidence_type TEXT NOT NULL,
  ref_id TEXT NOT NULL,
  evidence_json TEXT,
  weight REAL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (suggestion_id) REFERENCES suggestions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_suggestions_dedupe_key
  ON suggestions(dedupe_key, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_suggestion_evidence_suggestion
  ON suggestion_evidence(suggestion_id, created_at DESC);
```

## B. Add storage methods

Implement:
- `insert_suggestion_v2(...)`
- `insert_suggestion_evidence(...)`
- `list_suggestion_evidence(...)`
- `find_recent_suggestion_by_dedupe_key(...)`

Avoid positional tuple soup where practical. Introduce typed row structs for suggestions if needed.

## C. Expand API payload shape

Suggestion responses should include:
- title
- summary
- priority
- confidence
- evidence_count
- decision_context summary

# Acceptance criteria

- suggestions can carry operator-readable summaries
- evidence is stored durably outside raw payload JSON
- dedupe can be based on a stable key instead of only type/state
