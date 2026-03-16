---
id: APPLE-007
title: Build watchOS quick-action surface
status: proposed
owner: agent
priority: p1
area: apps/watch
depends_on: [APPLE-006]
---

# Goal

Implement the watch surface as a fast actuator for tiny decisions, not a miniature clone of the phone app.

# MVP screens

- Next due item
- Med confirmation action sheet
- Snooze choices
- Recent action confirmation
- Optional simple risk/status glance

# Requirements

- watch target under `Apps/VelWatch`
- shared view models from packages where feasible
- concise flows optimized for 1-3 taps
- resilient behavior when phone unreachable

# UX principles

- ruthless brevity
- giant tap targets
- no narrative overload
- clear post-action feedback
- avoid making the user litigate metaphysics on a 41mm screen

# Acceptance criteria

- watch can show next due item from synced/local state
- watch actions enqueue mutations locally or relay to phone
- app handles connectivity loss gracefully
- at least one end-to-end meds action works from watch
