---
id: SK-009
title: Add git freshness, churn, and change-hotspot intelligence
status: proposed
priority: P2
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Incorporate git history into freshness, confidence, and impact analysis.

# Tasks

1. Ingest recent commits and per-file modification timestamps.
2. Compute churn metrics and hotspot scores.
3. Expose “recently changed” and “unstable area” annotations on entities.
4. Feed freshness penalties into evidence scoring.
5. Add `vel system changed --since <window>`.

# Acceptance Criteria

- Recently changed files are queryable.
- High-churn modules are visible in reports.
- Confidence scoring can reflect stale docs versus recently changed code.
- Change summaries are scoped by time window.

