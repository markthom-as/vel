---
title: Loop Registry and LoopKind Introduction
status: proposed
priority: critical
owner: codex
---

# Goal

Create explicit named loop kinds and a registry rather than continuing with a single ad hoc worker poller.

# Concrete file targets

- `crates/vel-core/src/run.rs`
- `crates/vel-core/src/lib.rs`
- `crates/veld/src/worker.rs`
- `crates/veld/src/state.rs`
- `crates/veld/src/policy_config.rs`

# Concrete code changes

## Add loop kind enum
Create a new domain file if preferred:
- `crates/vel-core/src/loops.rs`

Suggested enum:
```rust
pub enum LoopKind {
    CaptureIngest,
    RetryDueRuns,
    EvaluateCurrentState,
    SyncCalendar,
    SyncTodoist,
    SyncActivity,
    SyncMessaging,
    WeeklySynthesis,
    StaleNudgeReconciliation,
}
```

## Add registry
In `worker.rs`, replace one big `poll_once()` with:
- `registered_loops() -> Vec<LoopDefinition>`
- each loop has interval, enabled flag, and runner fn

# Acceptance criteria

- loop kinds are explicit
- worker code reads as a registry, not a pile of special cases
