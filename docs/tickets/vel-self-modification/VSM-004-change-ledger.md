---
id: VSM-004
title: Change Ledger
status: proposed
priority: P0
owner: platform
labels: [audit, ledger, self-modification, observability]
---

## Summary
Add an append-only audit ledger for every self-modification attempt, decision, apply, rollback, and block.

## Why
If Vel edits itself without a durable record, debugging becomes theology.

## Scope
- Append-only storage for proposal lifecycle events.
- Query by proposal ID, subsystem, status, actor, and time range.
- Store hashes of diffs and validation artifacts.

## Event types
- `proposal_created`
- `proposal_blocked`
- `validation_started`
- `validation_completed`
- `approval_requested`
- `approval_granted`
- `approval_rejected`
- `apply_started`
- `apply_completed`
- `rollback_started`
- `rollback_completed`
- `post_deploy_regression_detected`

## Implementation tasks
1. Define event schema.
2. Add append-only persistence table/log.
3. Add query API.
4. Add artifact/hash fields.
5. Add tests for immutability and event ordering.

## Acceptance criteria
- Every lifecycle step writes a ledger event.
- Ledger cannot be silently overwritten through normal API paths.
- Query API supports debugging and UI needs.
- Blocked attempts on protected paths are captured.

## Dependencies
- VSM-002.

