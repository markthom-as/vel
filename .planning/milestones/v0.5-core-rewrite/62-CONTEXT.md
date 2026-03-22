# Phase 62: Calendar Core Model And Canonical Availability Semantics - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 62 is the native calendar-domain phase for milestone `0.5`.

Its job is to make calendar semantics first-class in Vel before the Google Calendar adapter lands, so that calendars, events, recurrence, attendees, locations, and availability are native canonical concepts rather than provider-led side effects.

Phase 62 is authored against the authoritative intended outputs of Phases 57 through 61 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 62 defines canonical calendar objects, recurrence semantics, attendee participation, location payloads, and governed availability as a read model over canonical calendar state. It does not define Google Calendar adapter behavior, provider sync policy, workflow automation semantics, or UI scheduling surfaces.

This phase should not widen into:

- Google Calendar adapter implementation
- provider-specific recurrence/write behavior
- UI or client scheduling surfaces
- workflow-trigger automation semantics
- first-class `Place` object expansion
- generic cross-provider merge logic

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 owns the calendar contract lock: `Calendar` and `Event` are canonical content objects; attendees use `Person` links plus participation metadata; recurrence is modeled as series plus derived/materialized occurrence views; location remains a simple canonical payload; `Availability` is a governed read model rather than a first-class content object.
- **D-02:** Phase 58 owns the canonical storage substrate for objects, relations, SyncLinks, projections, and rebuildable read models.
- **D-03:** Phase 59 owns ownership/conflict semantics and the lawful membrane that calendar actions must later reuse.
- **D-04:** Phase 60 owns registry/bootstrap/activation, which later calendar-aware workflows or modules may consume without reopening calendar ontology.
- **D-05:** Phase 61 owns manual workflow runtime; calendar semantics should be consumable by workflows, but Phase 62 does not redefine workflow execution.
- **D-06:** Availability remains a computed or materialized read model over canonical calendars, events, and policy/config, not an authored content object.
- **D-07:** Recurrence is first-class in canonical modeling, but applied synced truth still defers to the source platform once adapters arrive.

### Phase 62-specific posture

- **D-08:** Calendar semantics must be native enough that Google Calendar later maps into Vel instead of dictating it.
- **D-09:** Calendars themselves are first-class canonical objects, not just containers around events.
- **D-10:** Attendee semantics should support `Person` linkage plus participation metadata without forcing an oversized collaboration ontology.
- **D-11:** Location remains an event payload in `0.5`; first-class `Place` remains deferred.
- **D-12:** Availability should be explainable and rebuildable, whether computed on demand or materialized via projections/cache.
- **D-13:** Recurrence edit-scope complexity beyond `this occurrence` vs `entire series` remains downstream of this phase.

</decisions>

<specifics>
## Specific Ideas

- The main risk in Phase 62 is letting the future Google adapter backfill semantics into the core by omission.
- The second main risk is overbuilding scheduling/availability behavior into a product surface rather than a governed read model.
- This phase should leave behind a calendar model that is structurally rich and policy-compatible, but still narrow enough to keep provider-specific behavior in later adapter phases.
- Availability should be native enough for future scheduling logic, but still honest about being a derived interpretation of calendar state rather than a hand-authored truth object.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/58-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/60-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/61-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/61-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `docs/cognitive-agent-architecture/architecture/0.5-canonical-object-model.md`
- `docs/cognitive-agent-architecture/architecture/0.5-action-membrane-and-policy.md`
- `docs/cognitive-agent-architecture/architecture/0.5-ownership-conflict-and-write-intent.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths

- The packet already distinguishes canonical content from read models, which is exactly what calendar/availability needs.
- The workflow runtime now has typed context and manual execution posture that can later consume calendar objects cleanly.
- The storage substrate and membrane posture already make room for typed relations, projections, and refusal-aware actions.

### Main dangers

- adapter-shaped calendar semantics sneaking into the core too early
- treating availability as an authored object instead of a governed projection
- expanding location into a premature place ontology
- overloading recurrence with provider-specific quirks before canonical semantics are settled

</code_context>

<deferred>
## Deferred Ideas

- Google Calendar adapter sync/write behavior
- first-class `Place`
- advanced scheduling UX
- trigger-driven scheduling workflows
- generic cross-provider merge semantics
- rich recurrence-write scopes beyond later adapter needs

</deferred>

---

*Context gathered: 2026-03-22 for Phase 62 planning*
