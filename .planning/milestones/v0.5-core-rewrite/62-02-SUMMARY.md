---
phase: 62-calendar-core-model-and-canonical-availability-semantics
plan: 02
work_id: 0.5.62.2
title: Canonical recurrence and attendee participation semantics
status: completed
completed_at: 2026-03-22
---

# 62-02 Summary

## What Landed

- Added canonical recurrence contracts in `crates/vel-core/src/recurrence.rs`
- Added attendee participation and `Person` / provider-stub linkage types in `crates/vel-core/src/attendees.rs`
- Added a bounded derived occurrence materializer in `crates/veld/src/services/recurrence_materialization.rs`
- Added phase proof coverage in `crates/veld/tests/phase62_recurrence_and_attendees.rs`
- Wired the new recurrence and attendee contracts through `crates/vel-core/src/lib.rs` and `crates/veld/src/services/mod.rs`

## Proof Coverage

- recurrence is represented as canonical `Series` plus derived/materialized `Occurrence` records
- modified and cancelled exceptions are explicit recurrence records, not implicit omissions
- attendee participation is canonical `Person` linkage plus typed participation metadata
- unresolved attendees are represented as provider-scoped stubs instead of opaque blobs
- the proof materializer stays bounded and adapter-independent rather than pretending to implement provider-grade recurrence surgery

## Verification

- `rg -n "Series|Occurrence|Exception|RRULE|materialized" crates/vel-core/src/recurrence.rs crates/veld/src/services/recurrence_materialization.rs`
- `rg -n "Person|Participation|response_status|organizer|resource" crates/vel-core/src/attendees.rs`
- `cargo test -p vel-core --lib recurrence::tests::weekly_series_requires_weekday_and_modified_exception_requires_replacements`
- `cargo test -p vel-core --lib attendees::tests::participation_supports_person_links_and_provider_stubs`
- `cargo test -p veld --test phase62_recurrence_and_attendees`
- `cargo check -p veld`

## Outcome

Phase 62 now has native recurrence and attendee participation semantics, so Google Calendar later has to map into canonical series/occurrence and `Person`/participation shapes instead of inventing them.
