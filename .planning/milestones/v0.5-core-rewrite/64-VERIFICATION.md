# Phase 64 Verification

**Phase:** 64 - Google Calendar multi-account adapter and canonical calendar cut-in  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 64 can be considered complete as the calendar-side proving adapter phase.

## Required Outputs

Phase 64 should leave behind:

- multi-account Google Calendar linking over canonical `IntegrationAccount`
- bounded-window import and `SyncLink`-based cut-in
- canonical calendar/event/attendee mapping for Google data
- recurrence fidelity, availability integration, tombstones, and conservative outward-write bridging
- black-box tests proving Google Calendar acts constitutionally over the calendar core

## Verification Checks

### A. Multi-account and bounded linkage

- [ ] Multiple Google accounts can coexist lawfully.
- [ ] Google linkage uses canonical `SyncLink` posture.
- [ ] Import honors bounded-window posture by default.

### B. Canonical calendar mapping

- [ ] Calendars and events map into canonical Vel objects.
- [ ] Attendees map through `Person` linkage plus participation metadata.
- [ ] Location remains a canonical event payload.
- [ ] Provider-specific extras remain bounded facets rather than ontology overrides.

### C. Calendar fidelity and write posture

- [ ] Recurrence maps into canonical series/occurrence semantics.
- [ ] Availability integrates through the native read-model contract.
- [ ] Upstream deletes become tombstones with reconciliation state.
- [ ] Outward writes remain mediated by policy and `WriteIntent`.
- [ ] Read-only and denial paths are verifiable.
- [ ] Unsupported recurrence scopes fail honestly rather than implying fake support.

### D. Adapter proof

- [ ] Google black-box tests pass.
- [ ] Refusal/error-surface tests pass.
- [ ] The adapter proves the calendar-side MVP without reopening calendar ontology or membrane law.
- [ ] The adapter proves constitutional subordination to the native calendar core, bounded import posture, and read-model availability law.

## Suggested Command-Backed Checks

```bash
rg -n "IntegrationAccount|SyncLink|remote_id|window|bounded" crates/vel-adapters-google-calendar/src crates/veld/tests
rg -n "Calendar|Event|Participation|Person|Occurrence|Availability|Projection" crates/vel-adapters-google-calendar/src crates/veld/tests
rg -n "source-owned|tombstone|WriteIntent|PolicyDenied|ReadOnlyViolation|OwnershipConflict" crates/vel-adapters-google-calendar/src crates/veld/src/services crates/veld/tests
```

## Exit Standard

Phase 64 is verified when Google Calendar proves the calendar-side MVP through canonical mapping, bounded multi-account sync, recurrence and availability integration, tombstones, and mediated writes, while remaining a constitutional adapter over the `0.5` core.

---

*Verification target for the Phase 64 planning packet*
