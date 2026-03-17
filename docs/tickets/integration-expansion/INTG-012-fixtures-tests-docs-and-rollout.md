---
id: INTG-012
title: Fixtures, tests, docs, and rollout for integration expansion
status: proposed
priority: P0
estimate: 3-5 days
dependencies:
  - INTG-001
  - INTG-002
  - INTG-003
  - INTG-004
---

# Goal

Create the canonical fixtures, test coverage, and rollout docs needed to keep the new integration architecture coherent as provider count increases.

# Scope

- Add multi-provider fixtures for:
  - same person across messaging/calendar/transcript/doc sources
  - Apple reminders + Todoist side by side
  - Obsidian + Apple Notes side by side
  - Zoom + Google Meet transcripts
  - Steam activity alongside workstation activity
- Add docs for provider onboarding and capability registration.
- Add migration notes from current family-only settings/config.

# Deliverables

- fixture directory and generator notes
- integration tests
- operator docs
- migration / rollout checklist

# Acceptance criteria

- New providers can be added against canonical fixtures instead of bespoke hand-made examples.
- Regression tests cover multi-provider identity and provenance.
- Docs clearly separate implemented behavior from planned provider support.

# Notes

Without fixtures, every adapter author will quietly reinvent the world and call it a standard.
