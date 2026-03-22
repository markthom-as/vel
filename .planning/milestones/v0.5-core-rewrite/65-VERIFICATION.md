# Phase 65 Verification

**Phase:** 65 - Hard cutover, backend contract reconciliation, and milestone verification  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 65 can be considered complete as the milestone cutover and closure phase.

## Required Outputs

Phase 65 should leave behind:

- live backend wiring through the new canonical core
- retired or isolated superseded backend seams
- reconciled route/DTO/caller boundaries against the new backend authority
- milestone-spanning execution-backed integration tests
- explicit evidence notes and explicit deferred-work notes

## Verification Checks

### A. Cutover authority

- [ ] The new canonical backend path is the live path.
- [ ] Superseded paths are retired or isolated.
- [ ] No hidden dual-live authority path remains.
- [ ] No legacy write path survives the cutover.

### B. Caller reconciliation

- [ ] Remaining callers are aligned to the new backend contracts.
- [ ] Transport DTO boundaries are reconciled without reintroducing DTO leakage into the core.
- [ ] Compatibility seams are removed or explicitly bounded.

### C. Milestone proof

- [ ] End-to-end tests cover substrate, membrane, registry/bootstrap, workflow runtime, Todoist, and Google Calendar.
- [ ] Policy/audit/explain evidence is verified across the live backend path.
- [ ] The milestone proof is execution-backed rather than narrative-only.
- [ ] Field-ownership conflict proof exists for Todoist and Google Calendar against the checked-in ownership matrix.
- [ ] Identity-stub promotion proof exists for attendee/provider identity with provenance preserved.
- [ ] Dry-run proof shows runtime/audit evidence without canonical content mutation or executed provider writes.
- [ ] Module activation and invocation remain distinct in the live cutover path.
- [ ] Tag-to-`task_semantics` interpretation proof exists with raw tags preserved.
- [ ] Derived analytics proof exists for at least one reschedule metric and one rewrite metric.

### D. Closure hygiene

- [ ] Deferred post-`0.5` work is written down explicitly.
- [ ] No major `0.5` scope is still implicitly living in compatibility or cleanup code.
- [ ] `65-MILESTONE-EVIDENCE.md` exists and records flows executed, evidence summary, pass/fail posture, known deviations, and any quarantined legacy read paths.

## Suggested Command-Backed Checks

```bash
rg -n "canonical|legacy|deprecated|isolated|cutover" crates/veld/src docs/cognitive-agent-architecture/architecture
rg -n "dto|compat|legacy|service" crates/vel-api-types/src crates/veld/src/routes crates/veld/tests
rg -n "Todoist|Google|Workflow|policy_explain|audit|WriteIntent|cutover" crates/veld/tests .planning/milestones/v0.5-core-rewrite/65-MILESTONE-EVIDENCE.md
```

## Exit Standard

Phase 65 is verified when the new canonical backend is the live authority, remaining callers are reconciled, milestone behavior is proved end-to-end with execution-backed evidence, and all remaining non-`0.5` work is explicitly deferred.

---

*Verification target for the Phase 65 planning packet*
