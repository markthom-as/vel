---
id: INTG-011
title: Explainability and debug surfaces for provenance and people
status: proposed
priority: P1
estimate: 3-5 days
dependencies:
  - INTG-002
  - INTG-004
  - INTG-005
  - INTG-006
  - INTG-007
---

# Goal

Make the expanded integration layer inspectable so operator trust increases instead of collapsing under hidden identity merges and vendor ambiguity.

# Scope

- Add explain endpoints and views for:
  - connection provenance
  - source object refs
  - person identity matches
  - unresolved identities
  - merge decisions
- Add sample payload inspection for provider debug.

# Deliverables

- explain API additions
- operator UI surfaces or drawers
- test fixtures for ambiguous and resolved identities

# Acceptance criteria

- Operator can inspect why two external identities became one person.
- Operator can inspect which provider and connection produced a note, message, or transcript.
- Unresolved identities are visible instead of silently dropped.

# Notes

If identity merges are not explainable, they will become a trust-destroying ghost in the machine.
