---
phase: 62-calendar-core-model-and-canonical-availability-semantics
plan: 01
work_id: 0.5.62.1
title: Native calendar and event object model
status: completed
completed_at: 2026-03-22
---

# 62-01 Summary

## What Landed

- Added native canonical `Calendar` types in `crates/vel-core/src/calendar.rs`
- Added native canonical `Event` types plus payload-level location semantics in `crates/vel-core/src/event.rs`
- Added typed `belongs_to` calendar/event relations in `crates/vel-core/src/calendar_relations.rs`
- Added `CalendarEnvelope` and `EventEnvelope` aliases and wired the new types through `crates/vel-core/src/lib.rs`
- Aligned canonical content IDs so `EventId` now emits `event_*` and introduced `CalendarId` with `calendar_*`
- Added phase proof coverage in `crates/veld/tests/phase62_calendar_objects.rs`

## Proof Coverage

- calendars are first-class canonical content objects
- events are first-class canonical content objects
- events relate canonically to calendars through a typed `belongs_to` relation
- event location remains a payload on the event object rather than becoming a first-class `Place`
- canonical event IDs now match the frozen `0.5` `event_*` content-ID contract

## Verification

- `rg -n "Calendar|timezone|visibility|default" crates/vel-core/src/calendar.rs`
- `rg -n "Event|start|end|location|transparency" crates/vel-core/src/event.rs`
- `rg -n "belongs_to|calendar|event" crates/vel-core/src/calendar_relations.rs`
- `cargo test -p vel-core --lib calendar::tests::calendar_requires_display_name_and_timezone`
- `cargo test -p vel-core --lib event::tests::event_requires_matching_time_kinds_and_location_payloads`
- `cargo test -p veld --test phase62_calendar_objects`
- `cargo check -p vel-core`
- `cargo check -p vel-storage`
- `cargo check -p veld`

## Outcome

Phase 62 now has a native calendar/event core substrate, so future recurrence, attendee, and availability work can extend canonical types instead of letting the Google adapter dictate the shape.
