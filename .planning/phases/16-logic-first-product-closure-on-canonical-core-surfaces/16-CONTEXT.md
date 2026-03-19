# Phase 16: Logic-first product closure on canonical core surfaces - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** Phase 14 product discovery, Phase 15 migration completion, current backend seam analysis

<domain>
## Phase Boundary

Phase 16 owns the next wave of Rust-owned operator product behavior on top of the seams proven in Phase 15.

This phase should turn the discovered product model into canonical backend commands, queries, policies, and read models so later web, Apple, CLI, and Phase 17 shell work become embodiment rather than product-definition work.

This phase is not shell simplification, not Apple parity closure, and not a broad UI rewrite. It should implement the logic behind:

1. `check_in` handling and state transitions
2. `reflow` proposal and application behavior
3. summary-first trust/readiness behavior
4. project-scoped operator action ownership and routing

</domain>

<decisions>
## Implementation Decisions

### Locked product direction
- [locked] `Now` remains urgent-first, minimal, and contextual.
- [locked] `Inbox` remains the explicit triage/action queue.
- [locked] `Threads` remains archive/search-first and is the escalation path for longer interactive flows.
- [locked] `Projects` remains secondary in navigation, but project-specific actions stay semantically project-owned.

### Locked logic priorities
- [locked] Phase 16 should build directly on the Phase 15 seam priority outcome:
  1. `check_in`
  2. `reflow`
  3. trust/readiness summaries
  4. project-scoped actions
- [locked] Phase 16 should prefer a sequence of backend-owned logic slices instead of reopening seam-definition work that Phase 15 already settled.

### `check_in` logic rules
- [locked] `check_in` remains a typed backend-owned action, not a shell-local prompt convention.
- [locked] Default embodiment is an inline `Now` card with suggested action/answer affordances.
- [locked] `check_in` may become blocking when it gates fatal recovery, state reconciliation, morning start, end-of-day closure, or supervised transitions.
- [locked] If bypass is allowed, backend-owned logic should preserve the warning/bypass-note semantics rather than leaving them to shell interpretation.
- [locked] Escalation toward `Threads` should remain typed metadata and/or durable state transitions, not ad hoc frontend routing logic.

### `reflow` logic rules
- [locked] `reflow` is auto-suggested and user-confirmed.
- [locked] `reflow` should remain severity-aware about what can apply directly versus what requires clearer confirmation.
- [locked] `reflow` should stay tied to typed drift, stale schedule state, missed events, and adjacent day-plan signals rather than becoming a speculative general planner.
- [locked] `Edit` continues to imply escalation into a longer `Threads`-style interaction path.
- [locked] Codex-workspace-style recalculation ideas are relevant, but should be ported as canonical product behavior, not copied as scripts.

### Trust/readiness rules
- [locked] Trust/readiness remains summary-first in `Now`.
- [locked] The backend-owned readiness projection should remain the authority for summary posture, using backup trust, freshness, pending writebacks/conflicts, and supervised review pressure.
- [locked] Deeper inspection remains progressively disclosed and should not re-fragment trust posture back into multiple shell-owned heuristics.

### Project-scoped action rules
- [locked] Project-scoped actions must retain compact project identity across core, service, DTO, and shell seams.
- [locked] Project-scoped actions may surface through `Now` or `Inbox`, but must not lose their semantic project ownership.
- [locked] Phase 16 should implement project-specific action behavior on top of the preserved project seam instead of flattening project review/reflow/status work back into generic global actions.

### Scope exclusions
- [locked] No broad nav or shell simplification work belongs here; that is Phase 17.
- [locked] No Apple FFI migration or desktop/Tauri work belongs here.
- [locked] No giant crate reshuffle belongs here.
- [locked] Do not let UI concerns redefine backend action semantics now that the seams exist.

### Claude's Discretion
- Exact service/module boundaries for the first Phase 16 slices, as long as route handlers stay thin and shell logic stays out of the backend policy layer.
- Whether to introduce one additional operator-oriented service module or continue extending the existing `now`, `operator_queue`, `daily_loop`, and adjacent services.
- How to group the Phase 16 plans, as long as the grouping follows the locked logic priorities and keeps shell embodiment out of scope.

</decisions>

<specifics>
## Specific Ideas

- The highest-value Phase 16 outcome is not “more DTOs.” It is backend behavior that can answer:
  - what happens when a `check_in` is accepted?
  - what happens when it is bypassed?
  - what changes when a `reflow` is confirmed?
  - what durable review/trust states should appear next?
  - how do project-owned actions remain project-owned through those transitions?
- The product direction from Phase 14 should be treated as fixed:
  - `Now` shows immediate action or subtle indicators
  - `Inbox` handles explicit queue work
  - `Threads` handles longer continuity/edit/recovery flows
- The Phase 15 migration seams mean Phase 16 should optimize for real state transitions and canonical logic, not more contract invention.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and process authority
- `.planning/ROADMAP.md` — Phase 16 goal, carry-forward notes, and dependency on completed Phase 15 seams
- `.planning/STATE.md` — active lane and accumulated product decisions
- `.planning/PROJECT.md` — validated requirements and milestone decisions
- `docs/MASTER_PLAN.md` — canonical implementation truth and repo status framing
- `README.md` — repo/runtime entrypoint
- `AGENTS.md` — layering, workflow, and contract rules
- `docs/templates/agent-implementation-protocol.md` — execution protocol

### Product authority from Phase 14
- `docs/product/operator-action-taxonomy.md` — canonical action-model direction and cross-surface identity rules
- `docs/product/operator-mode-policy.md` — disclosure policy, `check_in` behavior, `reflow` posture, and project ownership rules
- `docs/product/onboarding-and-trust-journeys.md` — summary-first trust/check-in/reflow journey rules
- `docs/product/now-inbox-threads-boundaries.md` — surface-boundary decisions
- `docs/product/operator-surface-taxonomy.md` — default/advanced/internal surface classification
- `docs/product/milestone-reshaping.md` — why logic closure precedes shell embodiment

### Architecture and seam authority from Phases 13-15
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — cross-surface ownership model
- `docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md` — command/query/read-model ownership rules
- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-CONTEXT.md` — Phase 15 migration decisions
- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-01-SUMMARY.md` — canonical operator-action contract tightening
- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-02-SUMMARY.md` — `check_in` seam baseline
- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-03-SUMMARY.md` — `reflow` seam baseline
- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-04-SUMMARY.md` — trust/readiness seam baseline
- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-05-SUMMARY.md` — project-scoped action seam baseline

### Current Rust seams most likely to change
- `crates/vel-core/src/operator_queue.rs` — canonical action/review/check-in/reflow/project identity contract
- `crates/vel-core/src/daily_loop.rs` — daily-loop domain vocabulary
- `crates/vel-core/src/project.rs` — project identity semantics
- `crates/vel-api-types/src/lib.rs` — transport DTO boundary
- `crates/veld/src/services/check_in.rs` — first backend-owned `check_in` seam
- `crates/veld/src/services/reflow.rs` — first backend-owned `reflow` seam
- `crates/veld/src/services/operator_queue.rs` — canonical action queue synthesis
- `crates/veld/src/services/now.rs` — `Now` read-model composition and trust projection
- `crates/veld/src/services/daily_loop.rs` — likely home for daily-loop-linked check-in/reflow consequences
- `crates/veld/src/services/backup.rs` and `crates/veld/src/services/doctor.rs` — trust/readiness evidence sources
- `crates/veld/src/services/projects.rs` — project substrate/service seam
- `crates/veld/src/routes/now.rs` and `crates/veld/src/routes/daily_loop.rs` — thin HTTP boundaries that should remain thin

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `vel_core::ActionItem`, `CheckInCard`, `ReflowCard`, and `ReviewSnapshot` already define the canonical operator-action baseline.
- `services::check_in` and `services::reflow` now exist as backend-owned seams and should be extended rather than bypassed.
- `services::operator_queue::build_action_items` already unifies queue projections from freshness, linking, project state, commitments, writebacks, conflicts, interventions, and execution handoffs.
- `services::now::NowOutput` already composes `check_in`, `reflow`, trust/readiness, action items, and review state into one operator read model.
- `services::daily_loop` already owns session-phase progression and is the most natural existing source for morning/standup-linked `check_in` consequences.
- Project identity is now preserved in action items through compact project fields, so later logic can remain project-owned without extra shell joins.

### Gaps Phase 16 Should Close
- There is still no canonical backend handling path for accepting, bypassing, or completing `check_in`.
- There is still no backend-owned “apply reflow” logic that turns a suggestion into durable state changes or follow-up actions.
- Trust/readiness is now summarized, but the follow-up action behavior likely still needs tightening.
- Project-scoped actions preserve identity, but project-specific logic/routing beyond projection remains thin.
- `Threads` escalation is still mostly metadata-level; Phase 16 may need to define backend-owned consequences without turning this into a Phase 17 shell problem.

### Established Constraints
- Route handlers must stay thin and must not grow policy logic.
- `vel-storage` must stay independent of transport DTOs.
- Shells may render differently, but the semantic action/readiness model remains backend-owned.
- Local-first, explainable, and reviewable behavior remain core product constraints.

</code_context>

<deferred>
## Deferred Ideas

- shell/nav simplification and embodiment work (Phase 17)
- Apple parity and FFI migration
- desktop/Tauri implementation
- broad codex-workspace scheduling port beyond what is needed for canonical reflow behavior
- new provider/platform expansion not required for the core operator logic loop

</deferred>

---

*Phase: 16-logic-first-product-closure-on-canonical-core-surfaces*
*Context gathered: 2026-03-19*
