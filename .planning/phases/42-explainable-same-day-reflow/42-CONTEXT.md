# Phase 42: Explainable same-day reflow - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 42 turns reflow from an already-present seam into the canonical `v0.2` same-day recovery lane.

This phase is responsible for making reflow outcomes, proposal state, escalation, and degraded behavior genuinely Rust-owned and explainable across shells.

It does not widen into multi-day planning, new calendar ingestion work, or broad calendar write-back automation.

</domain>

<decisions>
## Implementation Decisions

### Reflow scope
- **D-01:** Reflow remains current-day only and operates over already-ingested commitments, schedule state, and planning-profile constraints.
- **D-02:** Reflow should reuse the existing `reflow` service and `Now` seam rather than introducing a second planner endpoint.
- **D-03:** Reflow output must make `moved`, `unscheduled`, and `needs_judgment` outcomes explicit and typed.

### Explainability and provenance
- **D-04:** Reflow decisions must stay explainable from persisted context, commitments, normalized rule facets, and existing schedule signals.
- **D-05:** Proposal state and application state should remain visible through Rust-owned status/provenance, not hidden in shell-local state.
- **D-06:** Degraded behavior must fail explicit and reviewable rather than fabricating certainty when schedule truth or inputs are weak.

### Escalation and supervision
- **D-07:** Straightforward actionable reflow may stay in the bounded inline path when the result is typed and review-gated.
- **D-08:** Ambiguous, conservative, or manual-shaping cases should escalate into `Threads` instead of widening shell-side planning logic.
- **D-09:** Reflow apply/edit behavior should preserve explicit thread and proposal continuity so shells only render, not invent workflow state.

### Shell parity and non-goals
- **D-10:** Web and Apple should consume the same typed reflow semantics as thin shells.
- **D-11:** Phase 42 must not add local-calendar work back into `v0.2`.
- **D-12:** Phase 42 must not widen into multi-day planning or broad scheduling automation.

### the agent's Discretion
- Exact split between contract tightening, engine behavior, and verification files as long as the resulting reflow lane stays Rust-owned and current-day only.
- Exact test selection as long as it gives direct evidence for proposal shape, escalation, and degraded-state behavior.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/veld/src/services/reflow.rs` already holds the candidate derivation, proposal building, apply/edit transitions, and several focused tests.
- `crates/veld/src/services/commitment_scheduling.rs` already provides the supervised application seam for actionable reflow proposals.
- `crates/veld/src/services/now.rs` already projects `reflow`, `reflow_status`, and commitment-scheduling summary into the backend-owned `Now` model.
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md` already records the current contract and limits from earlier slices.

### Established Patterns
- Reflow already escalates into `Threads` through typed `reflow_edit` continuation rather than shell-owned planner state.
- Current planning-profile and scheduler-facet work gives reflow a normalized rule vocabulary that should be reused instead of reparsing upstream labels.
- MVP docs already define reflow as same-day only and explicitly bounded inside `overview -> commitments -> reflow -> threads -> review`.

### Integration Points
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/commitment_scheduling.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/app.rs`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/NowView.test.tsx`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`

</code_context>

<specifics>
## Specific Ideas

- Phase 42 should tighten the reflow contract first so later implementation work stops relying on historical placeholder wording.
- The most credible slice is to treat reflow as one bounded recovery engine over the same planning substrate already used for day-plan output.
- This phase should leave Phase 43 free to focus on bounded thread/tool continuation instead of still resolving what counts as reflow versus thread work.

</specifics>

<deferred>
## Deferred Ideas

- Multi-day planning
- New calendar ingestion paths or local-calendar work
- Broad automatic write-back beyond the existing supervised proposal/apply lane
- Generic planner UI or shell-side schedule logic

</deferred>

---

*Phase: 42-explainable-same-day-reflow*
*Context gathered: 2026-03-20*
