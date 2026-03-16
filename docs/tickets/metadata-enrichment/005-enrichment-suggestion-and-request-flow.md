---
id: VEL-META-005
title: Candidate generation and enrichment request flow
status: proposed
priority: P0
estimate: 4-5 days
dependencies: [VEL-META-004]
---

# Goal

Generate candidate enrichments from gaps and expose them for review or inline requests.

# Scope

- Candidate generator for initial fields:
  - Todoist tags
  - Todoist project
  - Calendar location
  - Calendar project linkage
- Candidate record storage.
- Structured reasons/provenance model.
- Inline question payload format for chat/UI.

# Deliverables

- candidate inference service
- candidate persistence
- reason/provenance serializer
- APIs to fetch queue and object candidates

# Acceptance criteria

- Each open gap can produce zero or more candidates.
- Candidates include confidence, reasons, provenance, and approval mode.
- System can ask for clarification when confidence is insufficient.

# Notes

No hidden vibes. Reasons need to be legible to a skeptical operator.
