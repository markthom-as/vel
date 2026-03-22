# Phase 62 Verification

**Phase:** 62 - Calendar core model and canonical availability semantics  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 62 can be considered complete as the native calendar-domain phase.

## Required Outputs

Phase 62 should leave behind:

- canonical `Calendar` and `Event` object implementations
- typed calendar/event relations
- canonical recurrence and attendee participation semantics
- availability read-model types and projection/explainability services
- focused tests proving calendar-native behavior independent of Google adapter code

## Verification Checks

### A. Core calendar shape

- [ ] Calendars are first-class canonical objects.
- [ ] Events carry native time, location, and transparency semantics.
- [ ] Typed relations connect events and calendars lawfully.

### B. Recurrence and participation

- [ ] Recurrence is representable as series plus derived/materialized occurrences.
- [ ] Exceptions are representable.
- [ ] Attendees use `Person` linkage plus participation metadata.

### C. Availability posture

- [ ] Availability is derived from canonical calendar state and policy/config inputs.
- [ ] Availability remains rebuildable and explainable.
- [ ] Availability is not treated as an authored content object.

### D. Adapter independence

- [ ] Google adapter behavior is not required for the calendar core tests to pass.
- [ ] The resulting model is rich enough that future Google mapping is a translation problem, not an ontology rewrite.

## Suggested Command-Backed Checks

```bash
rg -n "Calendar|Event|location|transparency|timezone" crates/vel-core/src crates/veld/src/services
rg -n "Series|Occurrence|Exception|Participation|Person" crates/vel-core/src crates/veld/src/services
rg -n "AvailabilityWindow|Projection|rebuild|explain" crates/vel-core/src crates/veld/src/services crates/veld/tests
```

## Exit Standard

Phase 62 is verified when Vel has a native canonical calendar model with recurrence, participation, and availability semantics that later adapters and workflows can consume without reopening the ontology.

---

*Verification target for the Phase 62 planning packet*
