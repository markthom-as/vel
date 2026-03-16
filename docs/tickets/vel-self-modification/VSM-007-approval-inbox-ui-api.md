---
id: VSM-007
title: Approval Inbox UI and API
status: proposed
priority: P1
owner: fullstack
labels: [ui, api, approvals, self-modification]
---

## Summary
Create the operator review surface for medium/high-risk patch proposals.

## Why
If Vel is going to ask for permission, it should do more than dump a diff and shrug.

## Scope
- API for listing, reading, approving, rejecting, and requesting revision.
- UI showing diagnosis, confidence, target files, validations, diff summary, and rollback strategy.
- Filter by subsystem, class, status, and age.

## Implementation tasks
1. Add review endpoints and permission checks.
2. Build inbox list and detail views.
3. Add diff/artifact rendering.
4. Add decision actions with reason capture.
5. Add optimistic refresh/websocket/event updates if available.

## Acceptance criteria
- Operator can review pending proposals end-to-end.
- Approval decisions are auditable.
- UI shows risk class and protected-path warnings clearly.
- Rejections and revision requests flow back to proposal lifecycle.

## Dependencies
- VSM-002, VSM-004.

