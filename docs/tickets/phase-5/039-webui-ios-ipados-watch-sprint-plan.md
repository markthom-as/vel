# Ticket 039B: Sprint Board — iOS/iPadOS/watch surface work

## Sprint 1 — Wave 1 foundation (mandatory)

|Item|Task|Files|Estimate (SP)|Definition of Done|
|---|---|---|---:|---|
|VEL-UI-601|Create viewport surface contract and hook with safe resize handling|`clients/web/src/core/hooks/useViewportSurface.ts` (new), `clients/web/src/core/Theme/tokens.ts`, `clients/web/src/core/Theme/index.ts`|3|Surface type available as `mobile | tablet | desktop`; SSR-safe resize + cleanup; derived flags for platform; safe-area and keyboard-inset helper exported; tests cover mount/unmount and branch changes.
|VEL-UI-602|Refactor AppShell for surface-driven layout branching|`clients/web/src/shell/AppShell/AppShell.tsx`|4|Mobile uses single-pane by default; tablet can split; desktop path unchanged; right rail behavior no longer hard-gated by only `lg`; safe-area and keyboard padding applied consistently.
|VEL-UI-603|Ship mobile bottom-nav primary routes|`clients/web/src/shell/Navbar/Navbar.tsx`, `clients/web/src/shell/Navbar/navbarChrome.ts`, `clients/web/src/shell/Navbar/Navbar.test.tsx`|3|Now/Threads/Nudges/Settings route affordances present and labeled; tab navigation reachable in mobile; desktop top controls preserved where applicable; a11y roles/labels pass spot checks.
|VEL-UI-604|Stabilize mobile thread path and actions|`clients/web/src/views/threads/ThreadView.tsx`, `clients/web/src/views/threads/ConversationList/ConversationList.tsx`, `clients/web/src/views/threads/ConversationList/ConversationList.test.tsx`|5|Compact thread UI stays selectable and scroll-stable on mobile; row taps reliably open threads; unread/priority visuals remain visible; selection persists through resize.
|VEL-UI-605|Harden composer for iOS/IME/voice ergonomics|`clients/web/src/core/MessageComposer/MessageComposer.tsx`, `clients/web/src/core/MessageComposer/index.ts`, `clients/web/src/core/MessageComposer/MessageComposer.test.tsx`|5|Keyboard open/close does not jitter layout; IME and Enter behavior do not drop messages; voice state lifecycle visible; composer remains reachable in primary thread paths.
|VEL-UI-606|Rework nudge discoverability on mobile|`clients/web/src/shell/NudgeZone/NudgeZone.tsx`, `clients/web/src/shell/NudgeZone/NudgeZone.test.tsx`|3|Nudges open from one predictable mobile path; compact drawer can open/collapse; nudges never permanently block content.
|VEL-UI-607|Fast append path for thread + now|`clients/web/src/views/threads/ThreadView.tsx`, `clients/web/src/views/now/NowView.tsx`, `clients/web/src/views/now/nowNudgePresentation.tsx`|3|One-tap/quick input route to append into thread from Now and thread contexts; mobile submit path works without deep navigation.
|VEL-UI-608|Now surface mobile hardening and spacing coherence|`clients/web/src/views/now/NowView.tsx`, `clients/web/src/views/now/components/NowNudgeStrip.tsx`, `clients/web/src/views/now/components/NowTasksSection.tsx`, `clients/web/src/views/now/components/CompactTaskLaneRow.tsx`|3|Task/nudge rows remain one-handed and readable; action buttons avoid clipping; safe-area spacing consistent with shell composer and nav.
|VEL-UI-609|Global shell bootstrap for safe areas|`clients/web/src/main.tsx`, relevant CSS entry (`clients/web/src/index.css` if used)|1|Viewport and iOS meta behavior updated for safe-area rendering; shell mount uses surface context without warnings.

**Sprint 1 subtotal: 30 SP**

---

## Sprint 2 — Wave 2 iPad split + optional desktop polish (conditional)

|Item|Task|Files|Estimate (SP)|Definition of Done|
|---|---|---|---:|---|
|VEL-UI-610|Persisted split-mode shell behavior|`clients/web/src/shell/AppShell/AppShell.tsx`, `clients/web/src/shell/AppShell/AppShell.test.tsx`|4|Adds `layoutMode` (`auto|single|split`), persists preference, and preserves selection across orientation changes.
|VEL-UI-611|Split-mode thread UX in tablet flow|`clients/web/src/views/threads/ThreadView.tsx`, `clients/web/src/views/threads/ThreadView.test.tsx`, `clients/web/src/views/threads/ConversationList/ConversationList.tsx`|4|List/detail/composer triage can render in split mode without regressions; no state loss during width jumps.
|VEL-UI-612|iPad nudge rail and keyboard/overlay behavior|`clients/web/src/shell/NudgeZone/NudgeZone.tsx`, `clients/web/src/shell/Navbar/Navbar.tsx`, `clients/web/src/core/Theme/tokens.ts`|3|Docks/collapses correctly in iPad split; focus does not get trapped between rail and composer.
|VEL-UI-613|Optional desktop polish pass|`clients/web/src/shell/Navbar/Navbar.tsx`, `clients/web/src/core/Theme/tokens.ts`, `clients/web/src/core/Theme/navbarChrome.ts`|2|No behavior regressions in existing desktop baseline; iOS/tablet overrides remain isolated.
|VEL-UI-614|Wave 1 + Wave 2 verification pack|`clients/web/src/**/*.test.tsx` for changed files|2|Targeted tests updated and run/recorded for mobile and iPad split scenarios.

**Sprint 2 subtotal: 15 SP**

---

## Sprint 3 — Wave 3 watch reduced surface (Apple, native)

|Item|Task|Files|Estimate (SP)|Definition of Done|
|---|---|---|---:|---|
|VEL-APP-701|Define watch minimal scope in docs|`clients/apple/Docs/feature-capability-matrix.md`, `clients/apple/AGENTS.md`, `clients/apple/Docs/apple-architecture.md`|2|Watch scope explicitly limited to nudges + voice + keyboard append; unsupported paths listed.
|VEL-APP-702|Implement watch minimal UI surface|`clients/apple/Apps/VelWatch/ContentView.swift`, `clients/apple/Apps/VelWatch/VelWatchApp.swift`|5|Watch surface supports nudge list, voice capture entry, and thread append input; no full thread management added.
|VEL-APP-703|Bridge watch actions to existing services safely|`clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`, `clients/apple/Packages/VelAppleModules/Sources/VelApplication/Services.swift`, `clients/apple/Packages/VelAppleModules/Sources/VelInfrastructure/Infrastructure.swift`|4|Existing domain/service contracts reused; no transport DTO leakage into domain; failure states are explainable and user-visible.
|VEL-APP-704|Watch regression checks|`clients/apple/Apps/VelWatch/*` tests and any new shared test doubles|2|Happy-path append and nudge actions pass; failure path surfaces clear message; no auth or capability overreach.

**Sprint 3 subtotal: 13 SP**

---

## Total estimate

- Sprint 1: 30 SP
- Sprint 2: 15 SP (conditional)
- Sprint 3: 13 SP
- Total: 43 SP (+/- 6 SP variance from optional scope)

## Execution order recommendation

1. Run Sprint 1 first, end-to-end gate before any iPad split work.
2. If approved, run Sprint 2 as optional polish and split-mode expansion.
3. Run Sprint 3 in parallel with final Sprint 2 polish only if shared API mappings are stable.

## Quick sprint-level exit checks

- Wave 1 gate: no critical mobile path blocked by hidden desktop controls; composer and nudges available.
- Wave 2 gate (if run): iPad split state transitions are stable and stateful.
- Wave 3 gate: watch route remains intentionally minimal and stable in capability scope.
