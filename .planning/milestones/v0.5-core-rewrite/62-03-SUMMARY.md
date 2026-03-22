---
phase: 62-calendar-core-model-and-canonical-availability-semantics
plan: 03
work_id: 0.5.62.3
title: Governed availability read model and explainability
status: completed
completed_at: 2026-03-22
---

# 62-03 Summary

## What Landed

- Added availability read-model contracts in `crates/vel-core/src/availability.rs`
- Added availability projection/materialization service in `crates/veld/src/services/availability_projection.rs`
- Added availability explain service in `crates/veld/src/services/calendar_explain.rs`
- Added phase proof coverage in `crates/veld/tests/phase62_availability.rs`
- Wired availability types through `crates/vel-core/src/lib.rs` and the new services through `crates/veld/src/services/mod.rs`

## Proof Coverage

- availability derives from canonical calendar/event state plus the fixed `0.5` policy/config inputs
- declined events are ignored by default when policy says so
- transparent events, including transparent all-day events, do not block availability
- availability persists as a projection, not as canonical content
- explain output includes basis, consulted sources, applied filters, blocking intervals, and a clear acceptance/rejection reason

## Verification

- `rg -n "AvailabilityWindow|busy|free|policy|config" crates/vel-core/src/availability.rs`
- `rg -n "Projection|rebuild|materialize|explain" crates/veld/src/services/availability_projection.rs crates/veld/src/services/calendar_explain.rs`
- `cargo test -p vel-core --lib availability::tests::availability_policy_and_window_capture_governed_read_model_shape`
- `cargo test -p veld --test phase62_availability`
- `cargo check -p veld`

## Outcome

Phase 62 is now complete with a native calendar-domain core: calendars and events are canonical objects, recurrence and participation are canonical semantics, and availability is an explainable, rebuildable read model rather than a content-object mistake.
