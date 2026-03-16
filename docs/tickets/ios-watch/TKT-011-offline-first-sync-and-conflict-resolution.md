---
id: TKT-011
status: proposed
title: Add offline-first local store, operation queue, and conflict reconciliation
priority: P0
estimate: 5-7 days
depends_on: [TKT-002, TKT-005, TKT-007, TKT-008]
owner: agent
---

## Goal

Make the Apple clients robust under flaky connectivity, background constraints, and the banal cruelty of mobile networks.

## Scope

- Local persistent store for snapshots + pending ops
- Outbound operation queue with retry/backoff
- Incremental sync via cursor
- Conflict policies for:
  - duplicate completion
  - stale snooze
  - med logged on watch and phone near-simultaneously
  - server-cancelled reminder still present locally

## Implementation notes

- Use append-only operation log where feasible
- Distinguish optimistic local state from confirmed server state
- Expose sync diagnostics in debug/settings UI
- Add deterministic tests for reconciliation logic

## Acceptance criteria

- Core actions succeed offline and reconcile when connectivity returns
- Duplicate writes remain idempotent
- Conflict outcomes are documented and covered by tests
