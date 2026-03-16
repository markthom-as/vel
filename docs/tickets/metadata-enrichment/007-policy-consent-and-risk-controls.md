---
id: VEL-META-007
title: Policy consent and risk controls for enrichment actions
status: proposed
priority: P0
estimate: 3-4 days
dependencies: [VEL-META-005, VEL-META-006]
---

# Goal

Decide whether an enrichment should auto-apply, queue for review, ask inline, or be blocked.

# Scope

- Implement policy evaluation engine.
- Support rule dimensions:
  - source
  - field
  - confidence threshold
  - risk level
  - learned user preference
  - external/internal visibility
- Add per-source and per-field consent controls.

# Deliverables

- policy evaluator
- config schema for rules
- API to read/update preferences
- audit reason for policy decisions

# Acceptance criteria

- Same candidate evaluated twice under same policy yields same decision.
- Unsafe actions are blocked before writeback.
- User can override defaults.
- Policy reason visible in UI/API.

# Notes

This is where autonomy stops being sexy and starts being governable.
