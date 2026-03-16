---
status: todo
owner: agent
priority: medium
---

# 008 — Add QA gates for uncanny valley and performance

## Goal
Prevent the implementation from drifting into cursed territory.

## Deliverables
- QA checklist
- automated lint/test hooks where feasible
- manual review rubric

## Instructions
1. Add review checklist for:
   - accidental gaze
   - visible eye structures
   - realistic lips
   - dead symmetry
   - cartoon emotion logic
   - watch battery-hostile rendering
2. Add performance checks for mobile/watch tiers.
3. Add screenshots or short captures for preset states.

## Acceptance criteria
- Regressions toward surveillant or mascot-like rendering are caught early.
