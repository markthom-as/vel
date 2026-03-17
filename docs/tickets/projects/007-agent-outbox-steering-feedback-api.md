---
id: VEL-PROJ-007
title: Implement queued message, steering, and feedback control plane
status: proposed
priority: P0
estimate: 3-5 days
dependencies:
  - VEL-PROJ-002
  - VEL-PROJ-006
labels:
  - outbox
  - steering
  - feedback
  - api
---

# Goal

Give project-linked agent sessions a real control plane: queue messages, attach steering, record feedback, and update session settings.

# Scope

Add APIs and services for:

- queue outbound message
- mark message for manual dispatch vs adapter dispatch
- create steering record
- create feedback record
- patch session settings
- update session status or project linkage

Support capability-aware behavior:

- if a source is queue-only, store the message and mark `needs_manual_dispatch`
- if a source later supports dispatch, the same outbox model should still work

# Deliverables

- route module(s) for agent session control
- storage/service methods for outbox, steering, feedback, settings patch
- DTOs for queued message and control records
- backend tests covering:
  - queue message to queue-only source
  - add steering
  - add feedback
  - change per-session settings

# Acceptance criteria

- A queued message can be created for a project-linked session.
- Steering is stored as a structured record, not only as raw text.
- Feedback can be attached and read back through the session/project APIs.
- Capability flags gate the available actions cleanly.
- Manual-dispatch state is explicit in API responses.

# Notes for agent

Treat steering like policy, not chat confetti.
