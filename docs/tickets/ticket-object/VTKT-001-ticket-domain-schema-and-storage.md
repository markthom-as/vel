---
id: VTKT-001
title: Ticket domain schema and storage
status: proposed
priority: P0
estimate: 3-5 days
dependencies: []
---

# Goal

Add `ticket` as a first-class core and storage type with canonical identity, backend metadata, state, and provenance.

# Scope

- add `Ticket`, `TicketId`, `TicketState`, `TicketType`, `TicketBackend`, and related types in `vel-core`
- add `tickets` storage schema and repository methods
- add typed provider/backend fields
- add canonical link and provenance fields

# Deliverables

- core domain types
- storage migrations
- repository CRUD
- JSON fixtures

# Acceptance criteria

- ticket exists as a first-class domain object
- tickets and commitments are distinct core types
- native and provider-backed tickets fit the same canonical schema

# Notes

This is the foundation. Do not start by cramming provider records into commitments and calling it abstraction.
