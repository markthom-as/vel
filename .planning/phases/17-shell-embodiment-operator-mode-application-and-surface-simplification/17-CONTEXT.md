# Phase 17: Shell embodiment, operator-mode application, and surface simplification - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning
**Source:** Phase 14 product policy, Phase 16 logic boundary, current web/Apple/CLI shell evidence

<domain>
## Phase Boundary

Phase 17 owns shell embodiment only.

Its job is to apply already-decided product policy across the operator shells so the shipped surfaces feel simpler, calmer, and more consistent without redefining backend semantics.

This phase should:

- embody the approved surface taxonomy in the web shell
- apply progressive disclosure across default, advanced, and internal/runtime surfaces
- keep `Now`, `Inbox`, `Threads`, `Projects`, and `Settings` as the agreed taxonomy
- align Apple and CLI embodiment to the same product-mode rules
- stay future-compatible with a later desktop/Tauri shell without making desktop the implementation target now

This phase must not:

- redefine `check_in`, `reflow`, trust/readiness, or project-action semantics owned by Phase 16
- reopen the `Now` / `Inbox` / `Threads` / `Projects` taxonomy
- treat current UI drift as product truth
- expand backend scope because a shell surface is awkward

</domain>

<decisions>
## Implementation Decisions

### Locked phase constraints
- [locked] Phase 17 is shell embodiment only; backend semantics remain owned by Phase 16.
- [locked] Preserve the current agreed taxonomy: `Now`, `Inbox`, `Threads`, `Projects`, `Settings`.
- [locked] Keep Apple, web, and CLI embodiment in scope.
- [locked] Treat Tauri/desktop only as a future-compatible assumption, not an implementation target.
- [locked] Plans must stay concrete and aligned to current repo surfaces rather than broad redesign language.

### Locked product carry-forward
- [locked] `Now` remains minimal, urgent-first, and summary-capable.
- [locked] `Inbox` remains the explicit triage surface.
- [locked] `Threads` remains archive/search-first and the escalation path for longer interaction.
- [locked] `Projects` remains secondary in navigation, but project-owned work stays visibly project-owned.
- [locked] `Settings` remains real, but progressive disclosure must separate advanced operator flows from internal/runtime detail.
- [locked] `check_in` should render inline by default and `reflow` should feel heavier; Phase 17 only embodies those typed semantics.

### Shell-shaping decisions
- [locked] Default shells should foreground daily use before runtime internals.
- [locked] Non-urgent work should usually leave `Now` and deep-link into `Inbox` or `Threads`.
- [locked] Trust, onboarding, and freshness should stay summary-first, with deeper inspection behind deliberate navigation.
- [locked] Apple remains summary-first and web remains the richest embodiment.
- [locked] CLI is a real operator shell and should use the same product classification, not a separate internal model.

### Claude's Discretion
- Exact plan boundaries between shared shell scaffolding, default-surface embodiment, and advanced-surface embodiment.
- Which existing web components should absorb Stats and Suggestions access after they stop behaving like peer primary destinations.
- Which Apple files should be treated as the practical embodiment seam for iPhone, iPad, watch, and macOS.
- Which CLI commands best express `Now`, `Threads`, trust, and docs without inventing a TUI.

</decisions>

<specifics>
## Specific Ideas

- The current web shell still leaks old categories: `Sidebar.tsx` keeps `Projects` primary and still exposes `Suggestions` and `Stats` as support peers, while `MainPanel.tsx` routes them like first-class destinations.
- `NowView.tsx` already consumes backend-owned `Now` data, but it still needs stronger shell rules so non-urgent work stays summarized and heavier items stay visually distinct.
- `InboxView.tsx` and `ThreadView.tsx` are already close to the desired roles; Phase 17 should sharpen their posture instead of re-platforming them.
- `ProjectsView.tsx` is useful but should read more like contextual drill-down than another main daily destination.
- `SettingsPage.tsx` still carries the strongest product-sprawl risk and needs the clearest advanced-vs-internal disclosure treatment.
- Apple shells still use pre-taxonomy labels like `Today`, `Nudges`, and `Activity`; Phase 17 should align those surfaces to the approved product model without adding backend logic.
- The CLI already has strong operator commands (`today`, `doctor`, `docs`, `thread`), but its presentation still reads more like command inventory than product-mode embodiment.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Planning and roadmap authority
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- `.planning/PROJECT.md`
- `README.md`
- `docs/MASTER_PLAN.md`
- `docs/templates/agent-implementation-protocol.md`
- `AGENTS.md`

### Product authority from Phase 14
- `docs/product/operator-surface-taxonomy.md`
- `docs/product/now-inbox-threads-boundaries.md`
- `docs/product/operator-mode-policy.md`
- `docs/product/onboarding-and-trust-journeys.md`
- `docs/product/milestone-reshaping.md`
- `.planning/phases/14-product-discovery-operator-modes-and-milestone-shaping/14-CONTEXT.md`
- `.planning/phases/14-product-discovery-operator-modes-and-milestone-shaping/14-RESEARCH.md`

### Logic-boundary authority from Phase 16
- `.planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces/16-CONTEXT.md`
- `.planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces/16-01-PLAN.md`
- `.planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces/16-05-PLAN.md`

### Current shell evidence
- `clients/web/src/App.tsx`
- `clients/web/src/components/Sidebar.tsx`
- `clients/web/src/components/MainPanel.tsx`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/InboxView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/ProjectsView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/apple/Apps/VeliOS/ContentView.swift`
- `clients/apple/Apps/VelMac/ContentView.swift`
- `clients/apple/Apps/VelWatch/ContentView.swift`
- `clients/apple/README.md`
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/commands/today.rs`
- `crates/vel-cli/src/commands/doctor.rs`
- `crates/vel-cli/src/commands/docs.rs`
- `crates/vel-cli/src/commands/threads.rs`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable assets
- Web already has stable top-level shell seams in `App.tsx`, `Sidebar.tsx`, and `MainPanel.tsx`.
- `NowView.tsx`, `InboxView.tsx`, and `ThreadView.tsx` already consume typed runtime state and should be sharpened, not replaced.
- `ProjectsView.tsx` and `SettingsPage.tsx` already hold the secondary/advanced shell lanes that Phase 17 needs to simplify.
- Apple shells are concentrated in three files: `VeliOS/ContentView.swift`, `VelMac/ContentView.swift`, and `VelWatch/ContentView.swift`.
- CLI embodiment can stay within `main.rs` plus a small set of operator-facing command modules.

### Gaps Phase 17 should close
- Web navigation and panel routing still preserve older peer surfaces that blur the approved taxonomy.
- Default-mode shells do not yet apply the `minimal Now / triage Inbox / archive-search Threads` policy consistently.
- Settings and related trust/detail surfaces still need a clearer advanced/internal split.
- Apple still exposes pre-taxonomy labels and activity slices that read as separate product surfaces.
- CLI output still needs more deliberate product-mode framing.

### Established constraints
- Do not add shell-local action semantics.
- Do not expand backend contracts just to make UI easier.
- Keep route/service ownership in Rust; clients embody typed state.
- Keep implementation targets to existing web, Apple, and CLI surfaces.

</code_context>

<deferred>
## Deferred Ideas

- backend logic or transport changes
- desktop/Tauri implementation
- broad Apple FFI migration
- new top-level taxonomy changes
- broad visual redesign detached from the current repo surfaces

</deferred>

---

*Phase: 17-shell-embodiment-operator-mode-application-and-surface-simplification*
*Context gathered: 2026-03-19*
