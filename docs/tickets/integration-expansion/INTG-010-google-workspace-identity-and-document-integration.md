---
id: INTG-010
title: Google Workspace identity and document integration
status: proposed
priority: P1
estimate: 5-8 days
dependencies:
  - INTG-001
  - INTG-002
  - INTG-003
  - INTG-004
---

# Goal

Unify Google-origin identity, calendar, document, and collaboration data under a shared provider model instead of treating each endpoint as an island.

# Scope

- Define Google Workspace provider family relationships.
- Support identity convergence across:
  - Calendar attendees
  - Contacts
  - Drive owners/collaborators
  - Docs collaborators
  - Meet-adjacent transcript metadata where accessible
- Prefer API + export-aware contracts.

# Deliverables

- provider contracts for Google workspace sources
- identity mapping fixtures
- document metadata ingest model
- shared provenance conventions for Google objects

# Acceptance criteria

- Google-origin people and document identities can converge into the person graph.
- Workspace documents have a place in the files/documents family.
- Calendar and document collaborator data do not fork the identity model.

# Notes

Google Workspace matters because real life is full of “the same person across five Google surfaces.”
