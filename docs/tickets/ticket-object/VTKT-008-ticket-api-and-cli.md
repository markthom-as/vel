---
id: VTKT-008
title: Ticket API and CLI
status: proposed
priority: P0
estimate: 3-5 days
dependencies:
  - VTKT-001
  - VTKT-002
---

# Goal

Expose tickets as first-class operator surfaces in API and CLI.

# Scope

- list/detail/create/update endpoints
- list/detail/create/update CLI commands
- ticket-to-commitment linking commands
- provider sync triggers

# Deliverables

- API DTOs
- routes
- CLI commands
- tests

# Acceptance criteria

- operator can inspect and manipulate native tickets
- provider-backed tickets are readable through the same canonical contract

# Notes

If tickets only exist in storage and not in operator surfaces, they are not actually first-class.
