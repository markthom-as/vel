---
title: Vel Technical Loops Spec
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Purpose

Vel needs explicit temporal loops so that important behavior does not depend on a human remembering to call `POST /v1/evaluate`.

# Existing repo anchors

- `crates/veld/src/worker.rs`
- `crates/veld/src/services/evaluate.rs`
- run kinds and retry policies in `vel-core`
- current context / risk / nudge / suggestion subsystems

# Target concept

A loop is a scheduled deterministic system activity with:
- kind
- schedule
- claim/run semantics
- bounded work
- observability
- disable/enable control

# Proposed loop kinds

- `sync_adapters`
- `evaluate_current_state`
- `retry_due_runs`
- `weekly_synthesis`
- `stale_nudge_reconciliation`
- `uncertainty_review`

# Concrete code changes

## A. Introduce loop domain types

Add to `vel-core`:
```rust
pub enum LoopKind {
    SyncAdapters,
    EvaluateCurrentState,
    RetryDueRuns,
    WeeklySynthesis,
    StaleNudgeReconciliation,
    UncertaintyReview,
}
```

## B. Introduce loop registry/config
Add config-driven schedule and enable flags.

## C. Upgrade worker into loop runner
Refactor `worker.rs` so it runs registered loops rather than hard-coding two tasks in one polling function.

# Acceptance criteria

- loops are explicit named runtime concepts
- loop scheduling and claiming are inspectable
- evaluation can run automatically on a bounded schedule
