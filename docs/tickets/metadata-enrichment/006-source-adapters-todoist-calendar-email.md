---
id: VEL-META-006
title: Source writeback adapters for Todoist Calendar and Email-linked actions
status: proposed
priority: P0
estimate: 4-6 days
dependencies: [VEL-META-002, VEL-META-005]
---

# Goal

Implement safe writeback for the first enrichment actions.

# Scope

- Todoist adapter actions:
  - add/remove tags or labels as supported by actual integration model
  - move task to project/section if supported
  - update priority if supported
- Calendar adapter actions:
  - set location
  - set/add conferencing metadata if allowed
  - attach project linkage in Vel-side metadata if source writeback is unavailable
- Email-linked actions:
  - create or update Vel-side project linkage / follow-up association
  - defer source mutation if Gmail field writeback is not meaningful

# Deliverables

- adapter methods with idempotency keys
- apply result model
- rollback token handling where possible
- integration tests against mocks/sandboxes

# Acceptance criteria

- Writeback actions consult capabilities registry.
- Writes are idempotent.
- External mutation conflicts are surfaced cleanly.
- Action record persisted for every attempt.

# Notes

Prefer boring reliability over clever abstraction. Source APIs have a way of punishing idealists.
