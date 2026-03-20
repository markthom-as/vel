---
phase: 03-deterministic-verification-continuous-alignment
plan: 01
subsystem: runtime-api, trace-contracts, docs, web-types
tags: [phase-3, trace, runs, docs, rust, typescript]

requires:
  - phase: 02-distributed-state-offline-clients-system-of-systems
    provides: stable run persistence, run inspection routes, operator DTO patterns

provides:
  - Shared `TraceId`, `TraceLink`, and `HandoffEnvelope` types in `vel-core`
  - Trace-aware run summary/detail/event DTO fields in `vel-api-types`
  - `/v1/runs` and `/v1/runs/:id` responses exposing `trace_id` and `parent_run_id`
  - API and architecture docs aligned on fallback trace behavior for older runs
  - Web decoder support for trace-linked run summaries

affects:
  - 03-02-PLAN.md (CLI/web inspect surfaces build on the new DTO contract)
  - 03-03-PLAN.md (docs/support slice now has explicit trace terminology to reference)
  - 03-04-PLAN.md (simulation replay can target a stable trace contract)
  - 03-05-PLAN.md (eval reporting can link verdicts to stable traces)

requirements-completed:
  - TRACE-01 (partial)
  - TRACE-02 (partial)

duration: 47min
completed: 2026-03-18
---

# Phase 3 Plan 01: Trace Contract and Run Inspection Summary

Implemented the Phase 3 entry slice by introducing explicit trace contract types, surfacing trace linkage in run operator APIs, and aligning the runtime/handoff docs with the same contract.

## Accomplishments

- Added `TraceId`, `TraceLink`, and `HandoffEnvelope` to [`crates/vel-core/src/run.rs`](/home/jove/code/vel/crates/vel-core/src/run.rs)
- Extended run summary/detail/event DTOs in [`crates/vel-api-types/src/lib.rs`](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with trace-aware fields
- Updated [`crates/veld/src/routes/runs.rs`](/home/jove/code/vel/crates/veld/src/routes/runs.rs) so run list/detail responses resolve `trace_id` from persisted structured metadata, falling back to `run_id` for older runs
- Added focused route tests in [`crates/veld/src/app.rs`](/home/jove/code/vel/crates/veld/src/app.rs) for default and explicit trace linkage behavior
- Updated [`clients/web/src/types.ts`](/home/jove/code/vel/clients/web/src/types.ts) and [`clients/web/src/types.test.ts`](/home/jove/code/vel/clients/web/src/types.test.ts) so the web decoder accepts the new fields while remaining compatible with older payloads
- Documented the trace inspection contract in [`docs/api/runtime.md`](/home/jove/code/vel/docs/api/runtime.md) and [`docs/cognitive-agent-architecture/agents/handoffs.md`](/home/jove/code/vel/docs/cognitive-agent-architecture/agents/handoffs.md)

## Verification

- `cargo test -p vel-core run::tests -- --nocapture`
- `cargo test -p veld get_run_includes_automatic_retry_policy -- --nocapture`
- `cargo test -p veld get_run_prefers_explicit_trace_metadata -- --nocapture`
- `cargo test -p veld list_runs_includes_unsupported_automatic_retry_policy -- --nocapture`
- `npm test -- --run src/types.test.ts -t "run summary"`

## Notes

- Full `npm test -- --run src/types.test.ts` still reports an unrelated existing failure in the integrations decoder path (`local integration.configured` boolean expectation). The targeted run-summary decoder check passed.
- This slice exposes trace linkage in operator surfaces but does not yet add dedicated trace persistence tables or richer handoff inspection UI; those remain for later Phase 3 plans.
