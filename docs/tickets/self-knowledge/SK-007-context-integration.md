---
id: SK-007
title: Integrate self-knowledge retrieval into Vel reasoning context
status: proposed
priority: P1
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Use the self-knowledge system to ground engineering answers and agent planning.

# Tasks

1. Add retrieval policy for engineering / repo-related prompts.
2. Retrieve code, docs, tests, and recent changes for relevant entities.
3. Inject evidence summaries into reasoning context.
4. Gate direct assertions on minimum evidence/confidence thresholds.
5. Trigger uncertainty behavior when confidence is low or contradictions exist.

# Acceptance Criteria

- Vel uses indexed evidence when answering repo questions.
- Low-confidence engineering answers explicitly say so.
- Contradictory sources are surfaced rather than flattened.
- Retrieved context is provenance-preserving.

# Notes

The point is not to stuff more text into the prompt. The point is to ground the model's attention in the right artifacts.

