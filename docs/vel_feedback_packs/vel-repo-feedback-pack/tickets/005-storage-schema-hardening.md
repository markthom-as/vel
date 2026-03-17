---
title: Storage and Schema Hardening
status: proposed
priority: high
owner: codex
---

# Goal

Tighten persistence invariants around suggestions, context, and run/state inspection.

# Concrete file targets

- `migrations/*.sql`
- `crates/vel-storage/src/db.rs`
- `docs/specs/vel-migrations-and-schema-spec.md`

# Concrete code changes

## A. Add indexes for likely hot paths
Audit and add indexes for:
- `suggestions(state, suggestion_type, created_at DESC)`
- `run_events(run_id, created_at)` if not already effectively covered for inspection
- `context_timeline(timestamp DESC)` is already present; keep it

## B. Introduce a suggestion evidence table
Create migration:
- `migrations/0024_suggestion_evidence.sql`

Suggested schema:
```sql
CREATE TABLE IF NOT EXISTS suggestion_evidence (
  id TEXT PRIMARY KEY,
  suggestion_id TEXT NOT NULL,
  evidence_type TEXT NOT NULL,
  ref_id TEXT NOT NULL,
  weight REAL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (suggestion_id) REFERENCES suggestions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_suggestion_evidence_suggestion
  ON suggestion_evidence(suggestion_id, created_at DESC);
```

This is needed because stuffing all evidence into `payload_json` makes explainability and dedupe brittle.

## C. Introduce optional `dedupe_key` on suggestions
Migration:
- add nullable `dedupe_key TEXT`

Use it to suppress repeated suggestions of the same effective recommendation.

## D. Add storage methods
Update `crates/vel-storage/src/db.rs` with:
- `insert_suggestion_with_dedupe_key(...)`
- `list_suggestion_evidence(suggestion_id)`
- `insert_suggestion_evidence(...)`
- `find_recent_suggestion_by_dedupe_key(...)`

# Acceptance criteria

- suggestion persistence supports durable evidence
- dedupe is not only "any pending suggestion of same type"
- storage exposes targeted methods rather than route-level ad hoc SQL
