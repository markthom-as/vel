---
id: INTG-009
title: Steam activity adapter
status: proposed
priority: P2
estimate: 2-4 days
dependencies:
  - INTG-001
  - INTG-003
  - INTG-004
---

# Goal

Add Steam as a first-class activity source for game sessions, time allocation, and attention/drift evidence.

# Scope

- Define canonical `game_activity` payload.
- Support at least one practical ingest mode:
  - local export
  - API pull
  - bridge snapshot
- Preserve app id, title, timestamps, and session duration.
- Keep provider-specific multiplayer/social details in structured metadata.

# Deliverables

- Steam provider registration
- game activity adapter
- fixtures and tests for replay-safe ingest
- explain-friendly provenance fields

# Acceptance criteria

- Steam sessions are ingestible without pretending they are workstation shell events.
- Downstream systems can distinguish `game_activity` from `computer_activity`.
- Source provenance remains explicit and replay-safe.

# Notes

Game time is real time. Vel should be able to see it without moralizing or flattening it into nonsense.
