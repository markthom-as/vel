---
id: VTKT-005
title: Todoist ticket backend and migration
status: proposed
priority: P1
estimate: 3-5 days
dependencies:
  - VTKT-001
  - VTKT-003
---

# Goal

Move Todoist integration from commitment-only assumptions toward a policy-based ticket and commitment mapping model.

# Scope

- add Todoist-to-ticket mapping
- preserve existing commitment flows during migration
- add policy for commitment-only vs ticket-only vs dual materialization

# Deliverables

- Todoist ticket adapter
- migration strategy for current mirrored commitments
- policy documentation

# Acceptance criteria

- Todoist items can become canonical tickets
- existing dogfooding flows do not break abruptly
- provider provenance remains explicit

# Notes

Todoist is exactly where the old “everything is a commitment” model starts to crack.
