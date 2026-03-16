---
id: SK-003
title: Implement evidence-backed claim model and confidence scoring
status: proposed
priority: P0
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Ensure Vel can explain architectural claims with evidence, freshness, and confidence rather than vibes and wishful synthesis.

# Tasks

1. Define a `claim` model with subject, predicate, object/value.
2. Link claims to one or more evidence items.
3. Implement confidence scoring inputs:
   - source canonicality
   - source freshness
   - code/test/doc agreement
   - symbol resolution quality
4. Implement contradiction state when sources disagree.
5. Add an explanation formatter for human-readable output.

# Acceptance Criteria

- `vel system explain <component>` returns claims with cited evidence items.
- Confidence score is present for every explanation claim.
- Contradictions are surfaced instead of averaged into nonsense.
- Unknown / insufficient evidence state is supported.

# Notes

Confidence should be a summary of support, not a vanity metric.

