---
phase: 63-todoist-multi-account-adapter-and-canonical-task-cut-in
plan: 03
work_id: 0.5.63.3
title: Ownership-aware Todoist sync, tombstones, and mediated outward writes
status: completed
completed_at: 2026-03-22
---

# 63-03 Summary

## What Landed

- Added ownership-aware Todoist reconciliation in `crates/vel-adapters-todoist/src/ownership_sync.rs`
- Added upstream-delete tombstone and restore handling in `crates/vel-adapters-todoist/src/tombstones.rs`
- Added conservative Todoist outward-write bridge in `crates/veld/src/services/todoist_write_bridge.rs`
- Added hostile-path and continuity proof coverage in `crates/veld/tests/phase63_sync_and_conflicts.rs`
- Wired the new Todoist adapter and service modules through `crates/vel-adapters-todoist/src/lib.rs` and `crates/veld/src/services/mod.rs`

## Proof Coverage

- source-owned Todoist fields win during reconcile and emit explicit ownership conflicts instead of adapter-local guesswork
- local Vel-originated writes emit normalized `TaskEvent` history alongside provider-originated change events
- upstream deletes become tombstones with `pending_reconcile` posture and explicit restore transitions
- outward Todoist writes remain conservative, respect read-only and denied policy paths, and dispatch through `WriteIntent`
- dry-run stays non-dispatching while preserving explicit local write-intent history

## Verification

- `rg -n "source-owned|shared|Vel-only|conflict|reconcile|tombstone|deleted_upstream|pending_reconcile|restored|WriteIntent|read_only|PolicyDenied|ask_if_external_write" crates/vel-adapters-todoist/src/ownership_sync.rs crates/vel-adapters-todoist/src/tombstones.rs crates/veld/src/services/todoist_write_bridge.rs crates/veld/tests/phase63_sync_and_conflicts.rs`
- `cargo test -p vel-adapters-todoist --lib`
- `cargo test -p veld --test phase63_sync_and_conflicts`
- `cargo check -p vel-adapters-todoist`
- `cargo check -p veld`

## Outcome

Phase 63 now proves that Todoist bidirectional behavior stays constitutional:

- sync honors canonical ownership law
- deletes default to tombstones instead of silent disappearance
- outward writes remain policy-mediated and `WriteIntent`-backed
- task behavior history stays continuous across provider and local changes
