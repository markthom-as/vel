---
id: APPLE-006
title: Add actionable notifications and intent handling
status: proposed
owner: agent
priority: p0
area: notifications
depends_on: [APPLE-005]
---

# Goal

Make reminders actually operable from the lock screen / watch without forcing full app open.

# Notification actions

Implement categories such as:

- Mark Taken
- Snooze 10m
- Snooze 30m
- Skip
- Open Vel

# Requirements

- local notification scheduling hooks
- remote/push-compatible abstraction if backend push comes later
- action handling routes into mutation queue
- action results reflected in app state after reconciliation
- duplicate tap / double-delivery handled idempotently

# Architecture

Put notification orchestration in `VelAppleNotifications`, not the iOS app target directly.

Recommended boundaries:

- `NotificationScheduler`
- `NotificationCategoryRegistrar`
- `NotificationActionHandler`
- `NotificationStateProjector`

# Acceptance criteria

- notification categories registered on launch
- tapping an action creates exactly one mutation
- successful action updates local state immediately
- on next sync, canonical state is reconciled
- tests cover duplicate action delivery
