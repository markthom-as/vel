---
id: TKT-006
status: proposed
title: Ship medication logging and meeting-aware reminder logic on iOS
priority: P0
estimate: 4-6 days
depends_on: [TKT-002, TKT-004, TKT-005]
owner: agent
---

## Goal

Implement the first genuinely useful Apple feature set: medication adherence flows plus meeting-aware wake/prep reminders.

## Scope

Medication:

- med due card on Today
- mark taken / snooze / skip
- log timestamp and optionally dose note
- overdue escalation treatment

Meeting-aware reminders:

- read upcoming events from Apple Calendar integration layer or Vel-synced events
- schedule prep reminders relative to meeting start
- support user rule like: `be up an hour before meeting`
- suppress or soften reminders when already completed

## Implementation notes

- Local scheduling should be deterministic and re-derived from source state
- Do not create duplicate notifications on every app launch like an overcaffeinated intern
- Build reminder policy logic in testable core code, not inside SwiftUI views
- Include quiet-hours handling and timezone-safe scheduling

## Acceptance criteria

- Med events can be logged and surfaced in history
- Meeting prep reminder appears at configured lead time in test/demo environment
- Reminder policy has unit tests for edge cases: missing travel time, already completed, overlapping meetings, timezone changes
