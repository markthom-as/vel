# Phase 44: Minimal fresh web and Apple shells - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 44 rebuilds the shipped web and Apple MVP surfaces around the already-locked Rust-owned loop:

`overview -> commitments -> reflow -> threads -> review`

This phase is responsible for shell embodiment, screen boundaries, and removal or deprecation of remaining shell-owned MVP behavior.

It does not reopen MVP contracts, broaden provider/platform scope, or add new product lanes outside the strict MVP loop.

</domain>

<decisions>
## Implementation Decisions

### Shell authority and layout
- **D-01:** Web and Apple must behave as thin shells over the backend-owned `Now`, daily-loop, reflow, thread continuation, and review contracts already locked in Phases 40-43.
- **D-02:** The primary shipped MVP surfaces should stay narrow: `Now`, `Inbox`, `Threads`, and `Settings`, with `Projects` remaining secondary and non-MVP detail surfaces demoted or removed from the main path.
- **D-03:** One-screen-one-job remains the core UI rule. `Now` should orient and commit, `Inbox` should triage, `Threads` should continue bounded multi-step work, and `Settings` should handle support/setup/deeper detail.

### Web shell priorities
- **D-04:** Web should stop presenting non-MVP detail surfaces like `Suggestions` and `Stats` as first-class daily-use destinations when those surfaces do not own the loop.
- **D-05:** Web navigation, sidebar behavior, and surface transitions should reinforce the strict MVP loop instead of preserving legacy surface sprawl for completeness.
- **D-06:** Web should continue to render backend-owned statuses, proposals, review gates, and routing hints directly rather than adding local policy or ranking.

### Apple shell priorities
- **D-07:** Apple should align iPhone, iPad, macOS, and shared Swift transport around the same MVP surface hierarchy instead of preserving platform-local section taxonomies that predate the v0.2 MVP.
- **D-08:** Apple may keep platform-native affordances like quick capture, voice entry, and offline presentation, but those affordances must remain wrappers over shared backend-owned state and routing.
- **D-09:** Apple should not keep parallel summary/triage/history semantics that differ materially from web for MVP behavior.

### Removal and parity
- **D-10:** Remaining shell-owned MVP behavior should be removed, migrated onto typed backend contracts, or explicitly deprecated during this phase.
- **D-11:** Verification for this phase must prove parity at the shell boundary, not just visual resemblance.
- **D-12:** Any broader redesign, contextual-help work, analytics work, or non-MVP surface polish stays deferred to Phase 45+ or later roadmap work.

### the agent's Discretion
- Exact screen composition and component split, as long as surface ownership stays clear and backend-owned contracts remain the source of truth.
- Exact web vs Apple rollout order, as long as the phase ends with both shells aligned to the same MVP hierarchy and authority model.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/components/NowView.tsx`, `InboxView.tsx`, and `ThreadView.tsx` already embody the backend-owned MVP loop more closely after Phases 41-43.
- `clients/web/src/App.tsx`, `MainPanel.tsx`, `Sidebar.tsx`, and `data/operatorSurfaces.ts` still preserve broader shell taxonomy and legacy surface slots that can be simplified.
- `clients/apple/Apps/VeliOS/ContentView.swift` already distinguishes `Now`, `Inbox`, `Threads`, `Projects`, and `Settings`, but still uses older section/tab names like `activity` and `planning`.
- `clients/apple/Apps/VelMac/ContentView.swift` is still a summary-heavy split view with mixed sections rather than a tighter MVP shell over one clear surface hierarchy.
- `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift` and `clients/apple/VelAPI/Sources/VelAPI/Models.swift` already provide the shared typed transport foundation for Apple embodiment.

### Established Patterns
- Backend-owned daily-loop, reflow, and thread continuation contracts are already verified in Phases 41-43.
- `docs/product/operator-surface-taxonomy.md` and `docs/user/daily-use.md` already define the intended summary-first / triage-first / continuity-first surface hierarchy.
- The web shell already uses dedicated view components and typed query/data helpers, which should be reused rather than replaced with a new frontend architecture.

### Integration Points
- `clients/web/src/App.tsx`
- `clients/web/src/components/AppShell.tsx`
- `clients/web/src/components/MainPanel.tsx`
- `clients/web/src/components/Sidebar.tsx`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/InboxView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/apple/Apps/VeliOS/ContentView.swift`
- `clients/apple/Apps/VelMac/ContentView.swift`
- `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`
- `docs/user/daily-use.md`
- `docs/product/operator-surface-taxonomy.md`

</code_context>

<specifics>
## Specific Ideas

- The most credible Phase 44 path is: simplify web surface hierarchy first, align Apple surface hierarchy second, then remove shell-owned leftovers and verify parity.
- The shell work should be bold enough to feel intentionally minimal, but it must stay within the already-locked MVP loop rather than becoming a fresh product discovery phase.
- The phase should leave Phase 45 focused on end-to-end MVP verification and review closure, not still arguing about which screens matter.

</specifics>

<deferred>
## Deferred Ideas

- contextual-help systems
- broad shell polish beyond MVP screens
- non-MVP detail surface expansion
- broad Apple platform/runtime migration beyond thin-shell needs
- new backend product features not already locked by Phases 40-43

</deferred>

---

*Phase: 44-minimal-fresh-web-and-apple-shells*
*Context gathered: 2026-03-20*
