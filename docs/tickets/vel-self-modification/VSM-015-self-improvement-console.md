---
id: VSM-015
title: Self-Improvement Console
status: proposed
priority: P2
owner: fullstack
labels: [ui, hud, observability, self-modification]
---

## Summary
Add an operator/debug console showing pending proposals, validations, approvals, deploy status, metrics, and rollback actions.

## Why
If Vel is going to have introspection, it should be inspectable without spelunking raw logs like an archaeologist of bad decisions.

## Scope
- Pending/history views.
- Filters by subsystem/risk/status.
- Proposal detail pages with evidence and diff summary.
- Rollback controls and metrics summary.

## Implementation tasks
1. Design console information architecture.
2. Build list/detail views.
3. Integrate validation and ledger timelines.
4. Add metrics widgets and rollback actions.
5. Add links into related task/debug views if they exist.

## Acceptance criteria
- Operators can understand the self-improvement pipeline from one surface.
- Risk and confidence are visually legible.
- Rollback and approval actions are easy to find.
- Historical proposals are searchable.

## Dependencies
- VSM-004, VSM-007, VSM-019.

