---
title: Suggestion Feedback Learning Loop
status: proposed
priority: high
owner: codex
---

# Goal

Capture what happened after a suggestion was accepted or rejected so Vel can learn from steering outcomes.

# Concrete file targets

- `migrations/0025_suggestion_feedback.sql`
- `crates/vel-storage/src/db.rs`
- `crates/veld/src/routes/suggestions.rs`
- `crates/veld/src/services/suggestions.rs`
- `docs/specs/vel-self-knowledge-system-spec.md`

# Concrete code changes

## A. Add feedback table

Suggested schema:
```sql
CREATE TABLE IF NOT EXISTS suggestion_feedback (
  id TEXT PRIMARY KEY,
  suggestion_id TEXT NOT NULL,
  outcome_type TEXT NOT NULL,
  notes TEXT,
  observed_at INTEGER NOT NULL,
  payload_json TEXT,
  FOREIGN KEY (suggestion_id) REFERENCES suggestions(id) ON DELETE CASCADE
);
```

## B. Record structured outcomes
Examples:
- `accepted_and_policy_changed`
- `accepted_no_effect`
- `rejected_not_useful`
- `rejected_incorrect`
- `deferred`

## C. Use outcomes for future suppression/ranking
At minimum:
- repeated `rejected_incorrect` on a suggestion family should lower confidence
- repeated `accepted_and_policy_changed` should increase priority of similar future candidates

Keep this deterministic; no ML required.

# Acceptance criteria

- suggestion outcomes are durably recorded
- future candidate suppression/ranking can consult outcome history
- feedback loop remains inspectable
