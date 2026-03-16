---
id: TKT-009
status: proposed
title: Add iOS widgets, optional Live Activities, and watch complications
priority: P1
estimate: 4-5 days
depends_on: [TKT-004, TKT-008]
owner: agent
---

## Goal

Turn Vel into ambient infrastructure instead of an app you must remember to remember.

## Scope

iOS widgets:

- Next up widget
- Med due widget
- Focus / urgent widget

Optional Live Activity:

- active countdown to next critical reminder or meeting-prep window

watch complications:

- next due item
- med due indicator
- streak/compliance glance

## Implementation notes

- Drive widget timelines from compact shared state snapshots
- Avoid over-promising real-time updates beyond platform guarantees
- Complications should privilege one bit of truth, not three bits of mush

## Acceptance criteria

- At least one lock screen/widget surface and one complication compile and display realistic data
- Widget refresh path handles missing auth/offline state gracefully
