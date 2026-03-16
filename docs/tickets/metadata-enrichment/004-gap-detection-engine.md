---
id: VEL-META-004
title: Gap detection engine for missing weak conflicting metadata
status: proposed
priority: P0
estimate: 3-4 days
dependencies: [VEL-META-001, VEL-META-003]
---

# Goal

Detect metadata debt in normalized snapshots.

# Scope

- Implement a rule-based gap detector.
- Initial rules:
  - Todoist task missing project
  - Todoist task missing tags
  - Todoist task missing priority when task class suggests urgency
  - Calendar event with attendees and no location/conference
  - Calendar event missing project linkage
- Add severity and downstream-impact scoring.

# Deliverables

- `gap_detector.rs`
- configurable rule pack structure
- per-source gap rules
- metrics for open/resolved gap counts

# Acceptance criteria

- Running detection produces persisted `metadata_gaps`.
- Gap score is reproducible.
- Rules can be toggled/configured without rewriting core logic.

# Notes

Keep rules interpretable. If a rule cannot explain itself, it should probably not exist yet.
