---
id: VSM-009
title: Rollback Controller
status: proposed
priority: P0
owner: platform
labels: [rollback, safety, deploy]
---

## Summary
Implement standardized rollback for self-authored changes.

## Why
Autonomy without reversibility is how you end up explaining to future-you why the smoke was technically policy-compliant.

## Scope
- Roll back by proposal ID / commit hash.
- Support automatic rollback on threshold breach.
- Run post-rollback verification.

## Implementation tasks
1. Define rollback targets and metadata.
2. Implement revert flow for supported apply modes.
3. Add verification hooks after rollback.
4. Emit ledger/metrics events.
5. Add operator-triggered rollback action.

## Acceptance criteria
- Every autonomous apply has a rollback path.
- Rollback can be triggered manually or by policy.
- Rollback outcomes are visible in UI/ledger.
- Failed rollback attempts surface actionable errors.

## Dependencies
- VSM-004, VSM-006.

