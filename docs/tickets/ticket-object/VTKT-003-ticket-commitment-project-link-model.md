---
id: VTKT-003
title: Ticket, commitment, and project link model
status: proposed
priority: P0
estimate: 3-5 days
dependencies:
  - VTKT-001
---

# Goal

Define and implement the durable relationship model between tickets, commitments, and projects.

# Scope

- add ticket-to-commitment links
- add ticket-to-project links
- define when provider imports create:
  - ticket only
  - commitment only
  - both
- update project-facing read models to understand both layers

# Deliverables

- link schema
- services for mapping and link maintenance
- docs for canonical decision rules

# Acceptance criteria

- commitments no longer need to impersonate all external work objects
- one ticket can link to many commitments
- project views can distinguish backlog objects from personal obligations

# Notes

This is the conceptual cleanup ticket. Without it, the rest becomes a semantics swamp.
