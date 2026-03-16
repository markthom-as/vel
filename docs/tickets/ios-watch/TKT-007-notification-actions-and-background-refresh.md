---
id: TKT-007
status: proposed
title: Add actionable notifications, background refresh, and rescheduling engine
priority: P0
estimate: 4-5 days
depends_on: [TKT-006]
owner: agent
---

## Goal

Make notifications actually actionable instead of decorative guilt confetti.

## Scope

- Local notification categories with actions:
  - complete
  - snooze 10m
  - snooze 30m
  - skip
  - open app
- Background refresh pipeline to:
  - fetch deltas
  - recalculate local schedules
  - clear stale notifications
  - queue failed writes for retry

## Implementation notes

- Use `BGAppRefreshTask` / background mechanisms where appropriate
- Build a notification scheduler abstraction with diffing
- Notifications should be keyed by stable item IDs + schedule slot IDs
- Track last successful refresh and expose it in settings/debug UI

## Acceptance criteria

- User can complete/snooze from notification without opening the app when platform allows
- Duplicate or stale notifications are removed during resync
- Background refresh path is testable and observable through logs/debug screen
