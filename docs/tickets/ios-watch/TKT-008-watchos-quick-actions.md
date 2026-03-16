---
id: TKT-008
status: proposed
title: Build watchOS quick-action app for meds, reminders, and check-ins
priority: P0
estimate: 4-6 days
depends_on: [TKT-001, TKT-002, TKT-005, TKT-006]
owner: agent
---

## Goal

Create the watch app as the shortest path between prompt and action.

## Scope

Watch surfaces:

- Next action screen
- Mark med taken
- Complete current reminder
- Snooze current reminder
- Quick mood/energy check-in
- Recent status glance

## Implementation notes

- Design for interactions that complete in under 10 seconds
- Avoid dense settings/config on watch
- Cache minimal state locally for “phone not nearby” scenarios
- Support haptic feedback for successful action logging

## Acceptance criteria

- Watch app can render next actionable item from synced state
- Core actions work offline and sync later
- UX remains usable on smallest supported watch screen size
