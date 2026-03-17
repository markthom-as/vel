---
id: CAL-004
title: Launch service and Connect launch API
status: todo
priority: P0
dependencies:
  - CAL-001
  - CAL-002
  - CAL-003
---

# Goal

Implement the service and API path that launches an external runtime on a compatible Connect instance and creates or updates the linked Vel session record.

# Scope

- define launch request contract
- add route/service boundary for launch
- validate instance/runtime compatibility and policy constraints
- persist launch request, launch result, and failure semantics
- reconcile returned external session refs into session state

# Deliverables

- launch DTOs and route handlers
- service-layer launch orchestration
- durable launch-state transitions
- run/event emission for launch attempts and outcomes

# Acceptance criteria

- Vel can launch at least one supported runtime through a durable API path.
- Failed launches are explicit and inspectable rather than silent.
- Successful launches create a session visible to downstream projections.

# Notes

Routes should remain thin; launch logic belongs in services.
