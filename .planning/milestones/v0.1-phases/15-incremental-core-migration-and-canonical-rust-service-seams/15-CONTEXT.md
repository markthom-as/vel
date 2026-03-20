# Phase 15: Incremental core migration and canonical Rust service seams - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** Phase 13 architecture docs, Phase 14 product discovery, current backend seam analysis

<domain>
## Phase Boundary

Phase 15 owns the minimum structural migration needed so the next operator-product logic lands in canonical Rust-owned seams instead of being rederived in web, Apple, CLI, or route-layer glue.

This phase is not broad shell simplification, not a big crate shuffle, and not the final implementation of all check-in/reflow/product behavior. It should tighten domain, storage, service, and transport boundaries so Phase 16 can implement the real product logic cleanly.

The migration focus for this phase is prioritized as:

1. `check_in`
2. `reflow`
3. trust/readiness summaries
4. project-scoped actions

</domain>

<decisions>
## Implementation Decisions

### Locked scope and sequencing
- [locked] Phase 15 is migration-focused, not a broad product-logic phase.
- [locked] Structural work is justified only when it clearly improves backend ownership, transport clarity, or future portability.
- [locked] Phase 15 should create proof-bearing seams for later logic, not refactor the workspace for aesthetics.
- [locked] The priority order is `check_in`, then `reflow`, then trust/readiness summaries, then project-scoped actions.

### Locked product carry-forward from Phase 14
- [locked] `Now` stays minimal and urgent-first.
- [locked] `Inbox` remains the explicit triage surface.
- [locked] `Threads` stays archive/search-first and serves as escalation for longer flows.
- [locked] `Projects` is secondary in navigation, but project-specific actions may still remain semantically project-scoped.
- [locked] `check_in` should default to inline `Now` cards and escalate to `Threads` only when the interaction becomes meaningfully multi-step.
- [locked] `reflow` is auto-suggested, user-confirmed, heavier than normal nudges/check-ins, and should begin from a compact `Day changed` preview with `Accept` and `Edit`.
- [locked] Non-urgent items should not expand `Now`; they should usually surface as subtle indicators or deep links.
- [locked] Filters such as `Needs triage` or `Needs review` remain derived views over a canonical backend-owned action model.

### Current-code alignment
- [locked] Existing seams around `vel-core`, `vel-storage`, `vel-api-types`, `veld`, and the current HTTP-first shell clients must be extended rather than bypassed.
- [locked] Route handlers should stay thin and must not become the new home for operator policy.
- [locked] New service APIs should not return shell-specific or HTTP-shaped ad hoc payloads.
- [locked] `vel-storage` must remain independent of `vel-api-types`; transport mapping stays at the boundary.
- [locked] If new typed action or readiness shapes are introduced, they should be owned in `vel-core` and mapped to DTOs in `vel-api-types`.

### Current-repo seam observations
- [locked] `crates/vel-core/src/operator_queue.rs` already contains a useful but narrow operator action baseline (`ActionItem`, `ActionSurface`, `ActionKind`, `ActionState`, `ReviewSnapshot`).
- [locked] `crates/veld/src/services/operator_queue.rs` already synthesizes the current backend action queue from freshness, linking, writeback, conflicts, interventions, projects, commitments, and execution handoffs; Phase 15 should evolve that seam rather than bypass it.
- [locked] `crates/veld/src/services/now.rs` currently aggregates a broad operator snapshot and is the most likely near-term insertion point for new `check_in`, `reflow`, and trust/readiness read-model seams.
- [locked] `crates/veld/src/services/daily_loop.rs` currently owns daily-loop session progression and is the nearest existing seam for schedule-drift and reflow-oriented planning inputs.
- [locked] `crates/veld/src/services/backup.rs` and `crates/veld/src/services/doctor.rs` already provide part of the trust/readiness story, but that story is not yet projected through one summary-first operator seam.
- [locked] `crates/veld/src/services/projects.rs` and project transport/storage seams exist, but project-specific operator actions do not yet have a clear canonical lane.
- [locked] There is no dedicated operator orchestration service yet; Phase 15 may introduce one only if it reduces duplication rather than creating a speculative abstraction.

### What this phase should likely produce
- [auto] a current-state migration map for operator actions/read-model ownership
- [auto] a tighter core/storage/DTO boundary for canonical operator action records
- [auto] a first-class service seam for `check_in`
- [auto] a first-class service seam for `reflow` inputs/projections
- [auto] a summary-first trust/readiness projection seam that composes existing backup/review/freshness state
- [auto] an explicit project-scoped action seam that preserves project identity without flattening everything into generic global actions

### What this phase should avoid
- [auto] broad web or Apple UI rework
- [auto] final product semantics that belong in Phase 16
- [auto] forcing all operator concerns through one new mega-endpoint
- [auto] duplicating the same action semantics separately in `Now`, `Inbox`, `Threads`, and `Projects`

### Claude's Discretion
- Exact type names for new action, check-in, reflow, and readiness structures
- Whether the migration starts by extending `operator_queue.rs` or by introducing a nearby dedicated module
- Whether the first seam lands behind existing `/v1/now` and daily-loop routes or via one additional operator-focused route, as long as route thinness and backend ownership remain intact

</decisions>

<specifics>
## Specific Ideas

- The current code already has enough building blocks to avoid speculative architecture:
  - operator action baseline in `vel-core::operator_queue`
  - broad operator aggregation in `services::now`
  - daily-loop session state in `services::daily_loop`
  - trust/backup freshness in `services::backup`
  - project substrate in `services::projects`
- The main migration job is to stop letting these remain adjacent but semantically separate when Phase 16 needs to treat them as one operator action/readiness system.
- The most useful Phase 15 outcome is a set of backend-owned seams that let later product logic say:
  - "this is a `check_in`"
  - "this is a `reflow` candidate"
  - "this is readiness state"
  - "this action is project-scoped"
  without shell-local interpretation.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and process authority
- `.planning/ROADMAP.md` — Phase 15 goal, carry-forward notes, and sequencing into Phases 16-17
- `.planning/STATE.md` — active lane and accumulated product decisions
- `docs/MASTER_PLAN.md` — canonical implementation truth and historical phase context
- `README.md` — repo entrypoint and current runtime/shell framing
- `AGENTS.md` — repository layering and workflow rules
- `docs/templates/agent-implementation-protocol.md` — planning/execution protocol

### Architecture and product contracts
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — Phase 13 architecture authority
- `docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md` — command/query/read-model ownership rules
- `docs/product/operator-surface-taxonomy.md` — canonical default/advanced/internal surface classification
- `docs/product/now-inbox-threads-boundaries.md` — `Now`/`Inbox`/`Threads` boundary decisions
- `docs/product/operator-action-taxonomy.md` — canonical action-model direction and cross-surface identity rules
- `docs/product/operator-mode-policy.md` — disclosure, `check_in`, `reflow`, and presentation policy
- `docs/product/onboarding-and-trust-journeys.md` — summary-first trust/readiness/routing decisions
- `docs/product/milestone-reshaping.md` — why Phase 15 remains migration-focused

### Current Rust seams most likely to change
- `crates/vel-core/src/operator_queue.rs` — existing action/review domain baseline
- `crates/vel-core/src/daily_loop.rs` — daily-loop domain vocabulary
- `crates/vel-core/src/project.rs` — project identity and status semantics
- `crates/vel-api-types/src/lib.rs` — transport DTO boundary for `Now`, action items, review snapshot, backup status, daily loop, and project records
- `crates/vel-storage/src/lib.rs` and `crates/vel-storage/src/repositories/` — storage seam ownership
- `crates/veld/src/services/operator_queue.rs` — current synthesized backend action queue
- `crates/veld/src/services/now.rs` — current broad operator snapshot aggregation
- `crates/veld/src/services/daily_loop.rs` — current daily-loop session logic
- `crates/veld/src/services/backup.rs` — current trust/backup readiness source
- `crates/veld/src/services/doctor.rs` — current backup-trust classification and readiness-adjacent diagnostics
- `crates/veld/src/services/projects.rs` — project substrate service seam
- `crates/veld/src/routes/now.rs`, `crates/veld/src/routes/daily_loop.rs`, `crates/veld/src/routes/backup.rs`, `crates/veld/src/routes/projects.rs`, `crates/veld/src/routes/threads.rs` — current HTTP boundaries that must remain thin

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ActionItem` / `ReviewSnapshot` already exist and can likely be evolved instead of replaced wholesale.
- `services::operator_queue::build_action_items` already provides the main projection seam for operator actions and review counts, even though it is currently rebuild-on-read rather than canonical persisted source of truth.
- `NowOutput` already centralizes multiple operator-facing data sources and can host migration-backed read-model composition while route handlers stay simple.
- `DailyLoopSession` and related state types already provide a bounded seam that later `reflow` logic can build from.
- `BackupStatusData` and persisted backup runs already provide real trust/readiness evidence instead of requiring a new trust subsystem.
- Projects already have typed identities and routes, which should make project-scoped action ownership easier to preserve.

### Missing or Thin Areas
- There is no canonical `check_in` type or service seam yet.
- There is no backend-owned `reflow` contract or drift/readiness projection yet.
- Trust/readiness is still spread across backup, freshness, review, and linking/onboarding signals instead of being projected intentionally.
- Project-scoped actions are implied by tags and project IDs, but not yet modeled as first-class operator actions.
- Chat/inbox reads still recover action semantics indirectly from intervention evidence, which is too brittle to serve as the long-term canonical action seam.
- `Threads` exists as a route/storage surface, but there is no canonical link from `check_in` or `reflow` escalation into thread creation/reuse semantics yet.

</code_context>

<deferred>
## Deferred Ideas

- broad shell simplification and navigation changes (Phase 17)
- full product-logic closure for `check_in`, `reflow`, or operator-mode policy (Phase 16)
- full codex-workspace scheduling port
- Apple FFI migration
- desktop/Tauri implementation

</deferred>

---

*Phase: 15-incremental-core-migration-and-canonical-rust-service-seams*
*Context gathered: 2026-03-19*
