---
id: VTKT-002
title: Native Vel ticket backend
status: proposed
priority: P0
estimate: 2-4 days
dependencies:
  - VTKT-001
---

# Goal

Implement a Vel-native backend for tickets so the subsystem is useful even without external providers.

# Scope

- create/update/close/reopen native tickets
- support project assignment, labels, priority, and rich body text
- support local-only ticket lifecycle without provider dependencies

# Deliverables

- native ticket service layer
- storage operations
- API and CLI write paths for native tickets

# Acceptance criteria

- Vel can create and manage tickets without GitHub, Linear, Jira, or Todoist
- native tickets have the same inspectable shape as provider-backed tickets

# Notes

If Vel cannot own a ticket natively, the subsystem is just a mirror farm.
