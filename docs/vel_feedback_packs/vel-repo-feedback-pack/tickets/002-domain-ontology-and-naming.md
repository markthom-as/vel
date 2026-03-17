---
title: Domain Ontology and Naming Cleanup
status: proposed
priority: high
owner: codex
---

# Goal

Make the domain vocabulary explicit and enforce it across core types, docs, and route/service naming.

# Current pain

Several terms are semantically adjacent:
- capture
- signal
- event
- artifact
- thread
- suggestion
- nudge

The code mostly works, but the conceptual edges are still fuzzy enough to cause future drift.

# Concrete code changes

## Add explicit terminology module comments
Update:
- `crates/vel-core/src/lib.rs`
- `crates/vel-core/src/context.rs`
- `crates/vel-core/src/commitment.rs`
- `crates/vel-core/src/provenance.rs`
- `crates/vel-core/src/intervention.rs`

Add short module-level meaning statements.

## Introduce a typed `CurrentContext`
Create:
- `crates/vel-core/src/current_context.rs`

Suggested shape:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContext {
    pub computed_at: i64,
    pub mode: String,
    pub morning_state: String,
    pub meds_status: String,
    pub prep_window_active: bool,
    pub commute_window_active: bool,
    pub next_event_start_ts: Option<i64>,
    pub leave_by_ts: Option<i64>,
    pub global_risk_level: String,
    pub global_risk_score: f64,
    pub active_nudge_ids: Vec<String>,
    pub signals_used: Vec<String>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
}
```

This can evolve later, but the point is ownership.

## Rename ambiguous route/service names where needed
Example:
- `routes::risk::compute_and_list` should become `list_latest_risk`
- leave a compatibility comment if you want to avoid churn in one pass

# Implementation notes

Do not perform a giant rename sweep across the entire repo. Prefer:
- doc cleanup
- module comments
- obvious function-name improvements
- new typed structs at the domain boundary

# Acceptance criteria

- domain meanings are documented where the types live
- `CurrentContext` is represented as a domain type rather than only ad hoc JSON assembly
- obvious route/service misnomers are corrected
