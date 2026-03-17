---
title: Uncertainty Calibration and Learning
status: proposed
priority: high
owner: codex
---

# Goal

Let Vel compare predicted confidence with eventual outcomes.

# Concrete file targets

- `migrations/0025_uncertainty_outcomes.sql`
- `crates/vel-storage/src/db.rs`
- `crates/veld/src/services/uncertainty.rs`

# Concrete code changes

Add outcome tracking:
```sql
CREATE TABLE IF NOT EXISTS uncertainty_outcomes (
  id TEXT PRIMARY KEY,
  uncertainty_id TEXT NOT NULL,
  outcome_type TEXT NOT NULL,
  payload_json TEXT,
  observed_at INTEGER NOT NULL,
  FOREIGN KEY (uncertainty_id) REFERENCES uncertainty_records(id) ON DELETE CASCADE
);
```

Use this to support:
- calibration reports
- threshold tuning
- "we keep asking the user about this because our signal quality is bad"

Do not build full statistics infrastructure yet. Start with durable outcomes and a simple report.

# Acceptance criteria

- uncertainty records can be paired with outcomes
- future calibration work has durable historical data
