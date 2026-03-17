---
title: Docs Canonicalization and Status Hygiene
status: proposed
priority: high
owner: codex
---

# Goal

Reduce epistemic drift across `docs/status.md`, specs, review notes, and implementation docs.

# Why this matters

The repo now has enough documentation that contradictions are becoming structural, not incidental. Vel needs a docs hierarchy, otherwise future agent work will keep reintroducing mismatches.

# Concrete code and doc changes

## Create new canonical docs
Create:
- `docs/specs/vel-canonical-system-map.md`
- `docs/specs/vel-terminology.md`
- `docs/specs/vel-service-boundary-contract.md`

## Update existing docs
Update:
- `README.md`
- `AGENTS.md`
- `docs/status.md`
- `docs/api.md`
- `docs/runtime-concepts.md`

## Add doc classification headers
For all docs under `docs/specs/`, `docs/reviews/`, and `docs/tickets/`, add a tiny header convention:
- `doc_type: spec | status | review | ticket | archive`
- `canonicality: canonical | supporting | historical`

Do not do a repo-wide perfect cleanup in one shot. Start with the docs above plus any doc touched by this ticket's changes.

# Implementation steps

1. Create a short terminology table.
2. Create a short system map.
3. In `docs/status.md`, add:
   - "canonical for implementation status only"
   - "not canonical for design rationale or future plans"
4. In review docs that are now historical, add a note saying they are advisory, not implementation truth.
5. Update README so new contributors know exactly which docs to read first.

# Acceptance criteria

- a new contributor can identify the canonical status doc, system map, and terminology doc in under 60 seconds
- `docs/status.md` no longer tries to carry every design rationale
- review docs are clearly marked as non-canonical
