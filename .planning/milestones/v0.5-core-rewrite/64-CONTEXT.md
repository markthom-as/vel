# Phase 64: Google Calendar Multi-Account Adapter And Canonical Calendar Cut-In - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 64 is the calendar-side proving adapter phase for milestone `0.5`.

Its job is to prove the new canonical calendar domain, lawful membrane, multi-account integration substrate, and governed workflow/runtime environment against the operator's second MVP system: Google Calendar.

Phase 64 is authored against the authoritative intended outputs of Phases 57 through 63 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 64 defines the Google Calendar adapter boundary, multi-account calendar/event sync behavior, canonical mapping for calendars/events/attendees/recurrence/location/availability, conservative bidirectional writes, and ownership-aware reconciliation. It does not define generalized calendar-provider expansion, UI scheduling surfaces, or broad scheduling automation behavior.

Google Calendar is mapped into the native canonical calendar, event, attendee, recurrence, and availability model. Provider semantics may enrich, constrain, or facet canonical objects, but they do not replace the native calendar core or redefine availability as provider-owned truth.

This phase should not widen into:

- generic calendar-provider framework work beyond what Google needs to prove the core
- UI/client scheduling embodiment
- workflow-trigger or background scheduling automation
- first-class `Place` expansion
- generalized cross-provider merge logic
- cutover and cleanup work reserved for Phase 65

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 locked the calendar-side core shape: `Calendar` and `Event` are canonical content objects; attendees use `Person` links plus participation metadata; recurrence is represented as series plus derived/materialized occurrence views; location remains a simple event payload; `Availability` is a governed read model; Google sync defaults to a bounded window of past 90 days / future 365 days.
- **D-02:** Phase 58 owns canonical storage for accounts, SyncLinks, projections, tombstones, and runtime/control records.
- **D-03:** Phase 59 owns ownership/conflict semantics, read-only posture, `WriteIntent`, audit, and policy mediation for adapter writes.
- **D-04:** Phase 60 owns `module.integration.google-calendar` registration and governed activation path.
- **D-05:** Phase 61 owns manual workflow runtime; calendar objects may later be consumed by workflows, but this phase does not widen into workflow-trigger scheduling behavior.
- **D-06:** Phase 62 owns native calendar semantics; Google must map into that core rather than backfilling ontology by omission.
- **D-07:** Sync is bidirectional, but field conflict resolution follows explicit ownership; source-owned fields win.
- **D-08:** Upstream deletes create local tombstones with reconciliation state by default.
- **D-08a:** [0.5-CLARIFICATION-ADDENDUM.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md) locks Google attendee stub promotion, recurrence write-scope support, availability input set, tombstone restore law, dry-run law, and shared Google provider-account auth posture.

### Phase 64-specific posture

- **D-09:** Multi-account support is required from the start rather than treated as later cleanup.
- **D-10:** Calendar sync defaults to a bounded import window with explicit expansion rather than infinite history mirroring.
- **D-11:** Google calendars are first-class canonical `Calendar` mappings, not just provider containers.
- **D-12:** Recurrence fidelity matters here, but provider-specific quirks must still map into canonical series/occurrence semantics rather than redefining them.
- **D-13:** Availability integration must consume the native read-model contract from Phase 62 rather than re-inventing scheduling semantics inside the adapter.
- **D-14:** Conservative bidirectional writes exist, but external mutation remains config-gated and mediated through policy and `WriteIntent`.
- **D-15:** Provider modules must not bypass registry, membrane, calendar-core, or policy law just because this is the second proving adapter.
- **D-16:** Attendee identity resolution follows the earlier rule: normalized email first, then provider-scoped stub when no stable canonical identity is available.
- **D-17:** Tombstones remain hidden from default query posture, retain audit lineage, and support restored/reappearing upstream reconciliation.
- **D-18:** Recurrence fidelity means no silent semantic flattening, no fake round-trip promises, and no provider-shaped ontology rollback in the core.
- **D-19:** Conservative writes mean only bounded supported field sets, explicit recurrence scope handling, no bypass of `WriteIntent`, and no broad destructive mutations by default.
- **D-20:** Google event and attendee identity ownership behavior must conform to the checked-in [0.5-FIELD-OWNERSHIP-MATRIX.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md) artifact rather than adapter-local convention.

</decisions>

<specifics>
## Specific Ideas

- The main risk in Phase 64 is letting Google Calendar complexity reopen the calendar ontology that Phase 62 was supposed to settle.
- The second main risk is letting recurrence, attendee, or availability behavior leak provider-specific assumptions back into the core.
- Attendee mapping should stay mature but honest: canonical `Person` when identity is stable, provider stub when it is not, and participation metadata carried explicitly.
- This phase should prove that Google Calendar can be translated honestly into canonical Vel calendars/events while still preserving source-owned truth, bounded sync posture, and conservative outward writes.
- The adapter should remain boringly constitutional: same registry path, same membrane, same `WriteIntent` law, same multi-account substrate, same read-model availability contract.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/58-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/60-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/60-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/61-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/62-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/62-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/63-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/64-ACCOUNT-WINDOW-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/64-RECURRENCE-SCOPE-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md`
- `.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `docs/cognitive-agent-architecture/architecture/0.5-action-membrane-and-policy.md`
- `docs/cognitive-agent-architecture/architecture/0.5-ownership-conflict-and-write-intent.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths

- The packet already has lawful multi-account, linkage, tombstone, membrane, and registry seams that Google Calendar should consume instead of re-inventing.
- The native calendar model and availability read model already exist as core contracts that the adapter must map into.
- The workflow/runtime packet already keeps calendar consumers separate from adapter sync concerns.

### Main dangers

- provider-first calendar semantics flattening canonical recurrence or participation shape
- sync logic bypassing `WriteIntent` because event writes feel operationally urgent
- availability being recomputed in adapter space instead of through the native read-model contract
- letting Google recurrence quirks force an ontology rollback in the core

</code_context>

<deferred>
## Deferred Ideas

- broader calendar-provider platform work
- UI/client scheduling surfaces
- workflow-trigger behavior over calendar changes
- first-class `Place`
- generalized cross-provider merge semantics
- cutover cleanup reserved for Phase 65
- full-history import by default
- direct provider write path bypassing `WriteIntent`
- Google-specific availability truth model
- unsupported recurring write scopes presented as if they are complete

</deferred>

---

*Context gathered: 2026-03-22 for Phase 64 planning*
