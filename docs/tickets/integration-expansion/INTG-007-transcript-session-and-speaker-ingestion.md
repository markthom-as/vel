---
id: INTG-007
title: Transcript session and speaker ingestion
status: proposed
priority: P1
estimate: 5-8 days
dependencies:
  - INTG-001
  - INTG-002
  - INTG-004
---

# Goal

Move transcript ingestion from flat assistant-message imports toward provider-aware sessions, segments, speakers, and linked recordings.

# Scope

- Define transcript session and segment types.
- Support provider contracts for:
  - Zoom
  - Google Meet
  - Apple voice recordings
  - Google Voice recordings
  - existing assistant transcript imports
- Attach speaker identity candidates for later person resolution.
- Preserve linked recording artifact provenance.

# Deliverables

- transcript session schema
- speaker identity and segment model
- adapter or import contracts for target providers
- multi-speaker fixtures and tests

# Acceptance criteria

- Transcript providers can be distinguished without leaking provider details into downstream context logic.
- Speaker identity has a structured route into person resolution.
- Recording artifacts and transcript sessions can be linked cleanly.

# Notes

“Transcript” is not just a pile of text. It is a session with speakers, timing, and provenance.
