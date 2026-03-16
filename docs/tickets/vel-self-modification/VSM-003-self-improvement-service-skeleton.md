---
id: VSM-003
title: Self-Improvement Service Skeleton
status: proposed
priority: P1
owner: platform
labels: [self-modification, orchestration, backend]
---

## Summary
Implement the service boundary coordinating observe → diagnose → scope → propose → validate → approve/apply → observe.

## Why
Vel needs a dedicated subsystem rather than sprinkling self-edit logic across task execution code like glitter in a data center.

## Scope
- Add service interface and basic lifecycle state machine.
- Support proposal creation, validation dispatch, decision routing, and outcome updates.
- No direct production apply without policy check.

## Implementation tasks
1. Define service interface and internal state machine.
2. Add proposal queue/storage integration.
3. Add stub handlers for detect, diagnose, scope, and validate.
4. Add policy gate before any apply path.
5. Emit change events to ledger.

## Acceptance criteria
- Service exists behind explicit API boundary.
- Proposals can be created and transition through states.
- Apply operation is impossible without registry/policy check.
- Lifecycle events are emitted for observability.

## Dependencies
- VSM-001, VSM-002, VSM-004.

