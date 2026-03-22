---
phase: 61-workflow-and-skill-primitives-over-canonical-objects
plan: 03
work_id: 0.5.61.3
title: Manual workflow invocation, run records, and approval-aware execution
status: completed
completed_at: 2026-03-22
---

# 61-03 Summary

## What Landed

- Added workflow run-record contracts in `crates/vel-core/src/workflow_runs.rs`
- Added approval-runtime contracts in `crates/vel-core/src/approvals.rs`
- Added manual workflow execution in `crates/veld/src/services/workflow_runner.rs`
- Added focused invocation proof in `crates/veld/tests/phase61_manual_invocation.rs`
- Tightened runtime-record ordering determinism in `crates/vel-storage/src/repositories/runtime_records_repo.rs`

## Contract Shape

- Manual invocation is now the proving posture for the minimal `0.5` workflow runtime
- Run records remain runtime/control evidence, not canonical content
- Approval records remain distinct from run records, audit entries, and `WriteIntent` records
- Dry-run can evaluate, bind context, invoke mediated skill paths, and emit runtime evidence, but does not mutate canonical content or perform external mutation
- Approval steps pause execution lawfully through explicit pending approval records and `awaiting_approval` run state

## Verification

- `cargo test -p vel-core workflow_runs --lib`
- `cargo test -p vel-core approvals --lib`
- `cargo test -p vel-storage runtime_records_repo --lib`
- `cargo test -p veld --test phase61_manual_invocation`
- `cargo check -p vel-storage && cargo check -p veld`
- `rg -n "RunRecord|created|ready|running|awaiting_approval|dry_run_complete|completed|failed|refused|cancelled|ApprovalRequired|approved|rejected|pending|AuditEntry|ApprovalRecord|WriteIntent|manual|dry_run|no canonical mutation|no external mutation|WorkflowRunner|run record" crates/vel-core/src/workflow_runs.rs crates/vel-core/src/approvals.rs crates/veld/src/services/workflow_runner.rs crates/veld/tests/phase61_manual_invocation.rs`

## Outcome

Phase 61 now proves a minimal manual workflow runtime over canonical objects: typed context binds, action and skill steps execute through lawful seams, approval pauses are explicit, and dry-run remains runtime-evidence-only instead of mutating durable content.
