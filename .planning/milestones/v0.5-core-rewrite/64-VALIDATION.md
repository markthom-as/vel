# Phase 64 Validation

**Phase:** 64 - Google Calendar multi-account adapter and canonical calendar cut-in  
**Status:** Draft validation gate  
**Updated:** 2026-03-22

## Purpose

Validate that Phase 64 remains a Google Calendar proving-adapter phase rather than widening into generic calendar-platform design, UI scheduling surfaces, or scheduling automation behavior.

## Validation Questions

- [ ] Does Phase 64 prove Google through the canonical account, linkage, calendar-core, membrane, and registry seams rather than bespoke adapter shortcuts?
- [ ] Is multi-account support first-class?
- [ ] Does bounded import posture remain explicit?
- [ ] Do Google calendars/events/attendees/recurrence/location/availability map into native canonical calendar semantics?
- [ ] Do bidirectional writes remain conservative, config-gated, and `WriteIntent`-mediated?
- [ ] Do deletes become tombstones by default?
- [ ] Does the phase stop short of generic platform work, UI surfaces, and workflow-trigger automation?

## Google-Specific Non-Goals

Phase 64 does not include:

- full-history import by default
- provider-first calendar ontology
- direct provider writes that bypass `WriteIntent`
- background or trigger-driven scheduling behavior
- first-class `Place`
- unsupported recurring write scopes presented as if they are fully supported
- Google-specific availability truth replacing the native read model

## Validation Sources

- [64-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/64-CONTEXT.md)
- [59-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/59-CONTEXT.md)
- [62-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/62-CONTEXT.md)
- [63-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/63-CONTEXT.md)
- [64-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/64-01-PLAN.md)
- [64-02-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/64-02-PLAN.md)
- [64-03-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/64-03-PLAN.md)
- [64-04-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/64-04-PLAN.md)

## Validation Standard

Phase 64 is valid when it proves Google Calendar as a constitutional calendar-side adapter over the `0.5` core without stealing scope from generic platform design, UI scheduling surfaces, or automation behavior.

---

*Validation gate for the Phase 64 planning packet*
