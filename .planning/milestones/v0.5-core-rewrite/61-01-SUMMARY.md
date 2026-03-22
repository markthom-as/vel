---
phase: 61-workflow-and-skill-primitives-over-canonical-objects
plan: 01
work_id: 0.5.61.1
title: Typed workflow context binding and minimal step taxonomy
status: completed
completed_at: 2026-03-22
---

# 61-01 Summary

## What Landed

- Added typed workflow context contracts in `crates/vel-core/src/workflow_context.rs`
- Added the minimal workflow-step taxonomy in `crates/vel-core/src/workflow_steps.rs`
- Added runtime context binding over canonical objects in `crates/veld/src/services/workflow_context_binding.rs`
- Added focused phase proof in `crates/veld/tests/phase61_context_binding.rs`

## Contract Shape

- Workflow context now binds canonical objects through explicit `object_ref`, `object_type`, and optional `expected_revision`
- Runtime values remain typed and separate from canonical object bindings
- Workflow steps are explicitly limited to `action`, `skill`, `approval`, `sync`, and `condition`
- Trigger, loop, hook, and background-automation semantics remain deferred outside the `0.5` runtime core

## Verification

- `cargo test -p vel-core workflow_context --lib`
- `cargo test -p vel-core workflow_steps --lib`
- `cargo test -p veld --test phase61_context_binding`
- `cargo check -p vel-core && cargo check -p veld`
- `rg -n "WorkflowContext|context binding|object_ref|canonical object|action|skill|approval|sync|condition" crates/vel-core/src/workflow_context.rs crates/vel-core/src/workflow_steps.rs crates/veld/src/services/workflow_context_binding.rs crates/veld/tests/phase61_context_binding.rs`

## Outcome

Phase 61 now starts from typed, canonical-object-based workflow context law instead of ad hoc runtime maps, and the workflow-step vocabulary is constrained enough to support later skill mediation and manual invocation without widening into speculative orchestration behavior.
