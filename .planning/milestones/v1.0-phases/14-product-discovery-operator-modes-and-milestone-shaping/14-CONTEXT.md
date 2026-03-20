# Phase 14: Product discovery, operator modes, and milestone shaping - Context

**Gathered:** 2026-03-19
**Status:** Discovery in progress
**Source:** User thread decisions + initial Phase 14 research

<domain>
## Phase Boundary

Phase 14 defines the actual operator product shape after the cross-surface architecture is explicit.

This phase is about deciding:

- what the default daily-use experience is
- what belongs in advanced operator mode
- what belongs in internal or developer-only surfaces
- how onboarding, trust, and recovery should guide the operator
- how the roadmap should evolve so migration, logic, and shell embodiment do not collapse into one mixed phase

This phase is not broad UI implementation, not architecture migration, and not backend logic expansion. It is product classification, scope control, and milestone shaping.

</domain>

<decisions>
## Implementation Decisions

### Locked sequencing
- [locked] Architecture comes first, then product discovery, then incremental migration, then logic-first implementation, then broader UI embodiment.
- [locked] Phase 14 discovery/planning should run in parallel with early Phase 13 implementation where possible.
- [locked] Discovery may result in one or more additional future phases.

### Product-shape decisions
- [locked] Vel should continue to center the default operator experience on daily use rather than admin/runtime internals.
- [locked] The default product path should remain anchored in `Now`, `Inbox`, the daily loop, a compact context pane, unified action entry, and summary-level trust/onboarding guidance.
- [locked] Current UI surfaces likely expose too much internal state directly, especially inside Settings.
- [locked] Product boundaries should not emerge accidentally from current component layouts.
- [locked] `Inbox` is the primary work surface; `Projects` is secondary and should act more like filtering/context/configuration than a co-equal primary destination.
- [locked] `Threads` should remain a support surface for parallel work, history, search, and filtering rather than disappearing entirely.
- [locked] `Now`, `Inbox`, and `Threads` need explicit product-boundary definitions so orientation, triage, and interactive work do not collapse into one ambiguous surface.

### Mode and disclosure decisions
- [locked] Phase 14 must explicitly classify surfaces into default, advanced operator, and internal/developer buckets.
- [locked] Trust, onboarding, and recovery should be summary-first in the default path, with deeper diagnostics behind progressive disclosure.
- [locked] Advanced/runtime/developer concerns should not remain mixed into the same conceptual bucket as daily-use product surfaces.
- [locked] The product should prefer compact, contextual, icon-driven surfaces with tap/click to expand detail rather than broad equal-weight dashboards.
- [locked] A unified action entry should handle capture, voice chat, text chat, action requests, and thread starts with automatic routing plus an override affordance.
- [locked] Surface filters should be treated as views over a canonical action taxonomy, not as the core product model.
- [locked] Vel should support explicit check-in actions that query the operator to repair context or collect missing metadata when inference alone is not enough.
- [locked] Check-ins should default to inline `Now` cards with a suggested action and should escalate into `Threads` only when a longer interaction is needed.
- [locked] The action taxonomy should include both urgency and importance so visually heavier actions like `reflow` can stand apart from lighter inline prompts such as routine nudges or check-ins.
- [locked] `Now` should stay minimal: non-urgent items should usually collapse into badges/counts/deep links rather than render as direct cards.
- [locked] `Reflow` should first appear as a compact `Day changed` preview with `Accept` and `Edit`, where `Edit` escalates into `Threads`.
- [locked] Blocking `check_in` items should stay pinned, but allow bypass with a warning and operator note.
- [locked] `Threads` should usually become durable only once an interaction is meaningfully multi-step.
- [locked] Project-specific actions such as project review or project reflow may remain project-scoped even if surfaced through shared queues.
- [locked] Non-urgent indicators in `Now` should prefer a subtle context-bar style and should usually deep-link into filtered `Inbox` views.
- [locked] Check-in and bypass flows should prefer suggested responses or reasons with optional custom voice/text input.
- [locked] Reflow acceptance should depend on severity, and backend logic should eventually distinguish movable from fixed constraints.
- [locked] Project-scoped work should usually retain a visible project identity marker wherever it appears.

### Roadmap-shaping decisions
- [locked] Phase 15 should stay focused on seam migration.
- [locked] Phase 16 should stay focused on canonical Rust logic closure.
- [auto] Discovery should evaluate whether a dedicated post-16 shell embodiment/simplification phase is needed.

### Claude's Discretion
- Exact names for operator modes and disclosure levels
- Whether Stats becomes the main advanced read-only observability surface
- Where agent grounding belongs in the default-vs-advanced hierarchy
- How much advanced/runtime detail Apple clients should ever expose directly

</decisions>

<specifics>
## Specific Ideas

- Current research indicates the main problem is classification, not missing features.
- `SettingsPage.tsx` is the clearest signal of product sprawl: onboarding, trust, runtime controls, agent grounding, and execution review all accumulate there.
- The sidebar and daily-use docs already imply the right center of gravity, but the operator clarified that `Projects` should likely be demoted behind `Inbox` and contextual filtering.
- An early Phase 14 recommendation is to add a future post-16 shell embodiment phase rather than forcing UI simplification into Phase 16.
- The operator wants eventual iOS parity, but mobile should remain summary-first, with grounding buried for now and advanced/runtime detail web-first.
- The current boundary draft is: `Now` for orientation and immediate pressure, `Inbox` for triage and actionable queue work, `Threads` for parallel interactive work and searchable history.
- Discovery should define an action taxonomy covering action kind, actor, permission mode, urgency, state, and surface affinity before filter labels harden into product semantics.
- Check-ins should become a first-class operator action for context repair, missing metadata collection, and confirmation of ambiguous real-world state.
- `Reflow` should be treated as more important and more notification-heavy than routine nudges or normal check-ins, even when all three can surface in `Now`.
- `Now` should prefer ADHD-friendly minimal actionable context: direct cards for urgent issues, lighter indicators for the rest.
- Morning start and end-of-day closure can remain named flows, but should ride on the same action/check-in/review model rather than becoming a separate semantics system.
- The preferred subtle surface for non-urgent indicators is a compact context bar rather than another loud panel.
- The Todoist user-story export in `/home/jove/Downloads/Vel.csv` reinforces several directionally important choices: subtle top status/context, icon-driven and collapsible shell affordances, strong onboarding, route-aware contextual help, integration hygiene, and richer project drill-down without promoting Projects back to the main daily-use surface.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Architecture and roadmap authority
- `.planning/ROADMAP.md` — Phase 13-16 ordering and future-phase descriptions
- `.planning/STATE.md` — active lane, accumulated decisions, and discovery history
- `.planning/PROJECT.md` — product-direction decisions that already constrain discovery
- `docs/MASTER_PLAN.md` — canonical implementation truth
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — architecture lane authority from Phase 13
- `docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md` — command/query/read-model vocabulary and shell-boundary rules

### Current product and shell evidence
- `docs/user/daily-use.md` — current operator workflow and daily-use framing
- `docs/product/operator-surface-taxonomy.md` — active surface classification authority
- `docs/product/now-inbox-threads-boundaries.md` — working boundary draft for the primary daily-use surfaces
- `docs/product/operator-action-taxonomy.md` — working action-model draft behind filters, permissions, and cross-surface routing
- `docs/user/setup.md` — onboarding and setup framing
- `clients/web/src/components/NowView.tsx` — current primary daily-use surface
- `clients/web/src/components/SettingsPage.tsx` — current trust/runtime/onboarding sprawl and disclosure boundary
- `clients/web/src/components/Sidebar.tsx` — current primary/support navigation grouping
- `clients/web/src/App.tsx` — current top-level shell routing
- `crates/vel-cli/src/main.rs` — current CLI operator surface scope
- `clients/apple/README.md` — current Apple product framing and boundary assumptions

</canonical_refs>

<deferred>
## Deferred Ideas

- UI implementation and shell restructuring
- crate or service migration work
- full Apple FFI migration
- desktop/Tauri implementation
- broad provider/platform expansion not directly tied to operator product shape

</deferred>

---

*Phase: 14-product-discovery-operator-modes-and-milestone-shaping*
*Context gathered: 2026-03-19*
