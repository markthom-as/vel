---
phase: 63-todoist-multi-account-adapter-and-canonical-task-cut-in
plan: 04
work_id: 0.5.63.4
title: Todoist black-box adapter and error-surface proof
status: completed
completed_at: 2026-03-23
---

# 63-04 Summary

## What Landed

- Added Todoist black-box proving coverage in `crates/veld/tests/phase63_todoist_black_box.rs`
- Added Todoist hostile-path and error-surface coverage in `crates/veld/tests/phase63_todoist_error_surface.rs`

## Proof Coverage

- Todoist account linking, backlog import, canonical task/tag semantics, project mapping, attached-comment mapping, tombstones, and mediated writes all run through the same canonical substrate and membrane surfaces
- Todoist remains visibly registered as `module.integration.todoist`, not a bespoke shortcut lane
- hostile paths keep `UnsupportedCapability`, `OwnershipConflict`, `PendingReconciliation`, `ReadOnlyViolation`, and `PolicyDenied` distinct instead of collapsing into generic adapter failure
- Todoist therefore proves the task-side `0.5` core rather than redefining it

## Verification

- `rg -n "account|import|Task|Project|Tag|AttachedCommentRecord|tombstone|write|PolicyDenied|ReadOnlyViolation|OwnershipConflict|PendingReconciliation|UnsupportedCapability" crates/veld/tests/phase63_todoist_black_box.rs crates/veld/tests/phase63_todoist_error_surface.rs`
- `cargo test -p veld --test phase63_todoist_black_box`
- `cargo test -p veld --test phase63_todoist_error_surface`
- `cargo check -p veld`

## Outcome

Phase 63 is now complete. Todoist is proven as a constitutional task-side adapter over the `0.5` core:

- multi-account import is canonical-first
- mapping preserves Vel-owned task semantics
- sync and deletes obey ownership and tombstone law
- outward writes stay policy-mediated and `WriteIntent`-backed
- black-box and hostile-path tests both confirm there is no adapter bypass lane
