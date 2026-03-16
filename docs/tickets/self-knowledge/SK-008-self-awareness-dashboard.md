---
id: SK-008
title: Build self-awareness dashboard for docs, code, and confidence
status: proposed
priority: P2
owner: nav-ui
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Expose Vel's internal self-model in a visible UI surface.

# Panels

- system map
- documentation coverage
- drift report
- confidence / contradictions
- canonical docs by subsystem
- change hotspots

# Tasks

1. Define dashboard information architecture.
2. Build endpoints for graph summaries and drift reports.
3. Add visual affordances for confidence and staleness.
4. Support drill-down from subsystem to evidence sources.

# Acceptance Criteria

- A developer can inspect what Vel believes about a subsystem.
- Coverage gaps and stale docs are visually obvious.
- Contradictory evidence is visible rather than hidden.
- Dashboard supports drill-down to source artifacts.

