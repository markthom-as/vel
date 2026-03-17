---
title: Hybrid Logical Clocks (HLC) & LWW Synchronization
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
labels:
  - vel-core
  - distributed
  - sync
---

Implement Hybrid Logical Clocks (HLC) in `vel-core` to provide a mathematically deterministic Last-Write-Wins (LWW) conflict resolution strategy for distributed synchronization.

## Technical Details
- **HLC Implementation**: Build a `Clock` struct in `vel-core` following the HLC algorithm (combining physical time and a counter).
- **Metadata Integration**: Update all major data models (Commitment, Signal, RunEvent) to include an `hlc_timestamp` field.
- **Conflict Resolution**: Implement a comparison function that prefers higher physical time, then higher counters, then node IDs.
- **Sync Reconciliation**: Update the sync service to use this comparison for all merge operations.

## Acceptance Criteria
- `vel-core` provides a thread-safe HLC clock.
- Data records carry a sortable HLC timestamp.
- Conflict resolution is deterministic across different nodes.
- All sync-related tests pass.
