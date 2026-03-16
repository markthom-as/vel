# Ticket 005 — Delegation Engine

## Purpose
Core orchestration mechanism routing tasks to Navs.

## Deliverables
Delegation engine with:
- delegate(task)
- select_nav()
- score_nav()

Scoring criteria:
- capability match
- trust score
- cost
- latency estimate

## Acceptance Criteria
Engine selects Nav and dispatches execution.

