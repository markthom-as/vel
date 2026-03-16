---
title: Persist uncertainty ledger per task
status: todo
priority: P0
owner: data
labels: [uncertainty, persistence, tasks]
---

# Goal

Store open uncertainties, assumptions, and decision history as part of task state.

# Deliverables

- storage schema / migration for uncertainty ledger data
- task-level read/write API
- append-only decision history support
- ability to mark uncertainty items resolved and assumptions confirmed/rejected

# Requirements

- Open uncertainty items must survive task handoff and process restart.
- Decisions should be append-only for auditability.
- Assumptions must be queryable separately from uncertainty items.
- Keep the persistence model simple enough for future sync to mobile/watch surfaces.

# Acceptance criteria

- Task reload restores open items and assumptions.
- Resolution updates do not destroy historical records.
- At least one migration test verifies backwards compatibility.

# Notes

If Vel cannot remember what it was unsure about five minutes ago, then "metacognition" is just cosplay.
