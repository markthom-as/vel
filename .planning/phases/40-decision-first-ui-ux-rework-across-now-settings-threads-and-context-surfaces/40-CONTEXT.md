# Phase 40: MVP definition, canonical contracts, and architecture refinement - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 40 is the contract-and-architecture lock for milestone `v0.2`. It defines the true MVP precisely enough that later phases can implement without re-deciding scope.

The fixed MVP loop is:

`overview -> commitments -> reflow -> threads -> review`

This phase clarifies how that loop should work and how formally it should be specified. It does not widen the product beyond that loop, and it does not implement broad new capabilities.

</domain>

<decisions>
## Implementation Decisions

### Overview contract
- **D-01:** The MVP overview should use an `action + timeline` shape rather than commitments-first or flat summary-first.
- **D-02:** The overview should present one dominant current action plus a compact today timeline.
- **D-03:** Only the single highest-priority nudge should be visible by default; additional nudges live behind context affordances.
- **D-04:** Tappable icons should reveal `Why + state` context, not alternate actions or raw system detail.
- **D-05:** When there is no obvious current action, the overview should show a decision prompt with 1-3 suggested items.
- **D-06:** In the no-dominant-action case, the operator must be able to accept a suggestion, pick from the other suggestions, enter thread-based resolution, or close.

### Loop boundaries
- **D-07:** Threads are for multi-step work, not for ordinary overview/reflow interactions.
- **D-08:** The lightweight MVP loop should keep `accept / defer / choose / close` inline.
- **D-09:** A thread should be used only when the work becomes genuinely multi-step.
- **D-10:** “Multi-step” means at least two of the following are true:
  - it needs explanation
  - it needs multiple decisions
  - it needs tool/context work

### Reflow scope and local-calendar boundary
- **D-11:** `v0.2` should keep same-day reflow in scope, but local calendar work should be deferred from this milestone.
- **D-12:** Phase 40 should still define reflow contracts and provenance over the existing Rust-owned calendar inputs.
- **D-13:** Local calendar input/export, local apply behavior, and platform-specific local planning paths are out of scope for `v0.2`.

### Contract and documentation rigor
- **D-14:** Phase 40 should behave like a full spec phase, not a light pre-implementation cleanup.
- **D-15:** Canonical durable authority should live in `docs/`, not only in `.planning/`.
- **D-16:** `.planning/` should hold phase/milestone planning summaries, while durable specs belong in `docs/`.
- **D-17:** Stable examples, templates, or boundary artifacts should live close to code where that helps downstream implementation stay honest.
- **D-18:** Phase 40 must specify:
  - canonical models
  - user-visible behaviors
  - state transitions
  - failure and degraded-state behavior

### the agent's Discretion
- Exact spec document breakdown across `docs/` and code-adjacent examples/templates.
- Exact naming of the canonical overview/read-model contract as long as it stays aligned to the MVP loop.
- Exact presentation of icon affordances and compact timeline interaction details, as long as the action-first and `Why + state` rules are preserved.

</decisions>

<specifics>
## Specific Ideas

- The overview should feel like a decision surface, not a dashboard.
- Hidden depth should be behind tappable icons instead of dumped inline.
- When no dominant action exists, the product should help the operator choose rather than pretending there is already one obvious answer.
- Threads should remain bounded continuity, not become a generic chat product.
- This milestone should be substantially more spec-driven than the previous drifting cycle.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone authority
- `.planning/PROJECT.md` — v0.2 milestone goal, MVP loop, scope guardrails, and accepted decisions
- `.planning/REQUIREMENTS.md` — v0.2 requirements, MVP acceptance checklist, and non-goals
- `.planning/ROADMAP.md` — fixed phase boundary for Phase 40 and downstream phase sequencing
- `.planning/STATE.md` — current milestone state and execution position

### Durable architecture and repo rules
- `docs/MASTER_PLAN.md` — canonical implementation truth and historical lane context
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — Rust-owned cross-surface boundary guidance
- `README.md` — repo entrypoint and current product/runtime framing

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/veld/src/services/now.rs` and existing `Now` API/read-model seams: starting point for the canonical overview contract.
- `crates/veld/src/services/daily_loop.rs`: existing commitment/session behavior that can inform the MVP loop contract.
- `crates/veld/src/services/reflow.rs`: existing same-day reflow seam to formalize rather than replace wholesale.
- `crates/veld/src/routes/threads.rs` and thread-backed flows in `chat_assistant_entry` tests: existing continuation substrate to narrow and formalize.
- `crates/vel-core/src/context.rs`: existing reflow-status and current-context shape that may inform typed MVP state transitions.
- `crates/vel-api-types/src/lib.rs`: existing transport DTO surface already contains `daily_loop`, `reflow`, `review_snapshot`, and thread-related data to reconcile or tighten.

### Established Patterns
- The repo already prefers Rust-owned services and typed DTOs over shell-local policy.
- Existing roadmap and project docs now enforce a strict MVP loop and explicit non-goals.
- Phase 15/16 decisions already pushed `check_in` and `reflow` toward backend ownership.
- The current architecture expects web and Apple to consume shared transport seams rather than invent behavior locally.

### Integration Points
- Phase 40 planning should connect directly to `now`, `daily_loop`, `reflow`, `threads`, and review-related DTO/read-model seams.
- Durable spec work should land in `docs/` with planning summaries in `.planning/`, then feed later contract and service changes.
- Later client refresh work should depend on Phase 40 contracts instead of re-deriving screen behavior from existing UI.

</code_context>

<deferred>
## Deferred Ideas

- Local-first calendar input/export paths for Apple or other platforms
- Local calendar apply behavior or narrow safe local apply rules
- Broad UI polish outside the MVP loop screens
- Generic chat/tool product expansion beyond bounded thread continuation

</deferred>

---

*Phase: 40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces*
*Context gathered: 2026-03-20*
