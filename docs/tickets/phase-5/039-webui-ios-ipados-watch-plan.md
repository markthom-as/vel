# Ticket 039: Responsive Web UI for iOS/iPadOS with Watch Edge-Client Strategy

## Owner
- TBD

## Status
- Proposed

## Context
The current web UI has partial mobile behavior but remains desktop-first in layout, nav hierarchy, and surface density. This ticket defines a targeted plan to deliver:

1) responsive iOS-first experience,
2) iPad-optimized split experience,
3) optional desktop polish if near-free,
4) and a watch edge-client surface focused on nudges, haptics, quick capture, and bounded voice/keyboard append into threads.

Priority order follows repo guidance: capture + recall + contextual flows before speculative platform features.

## Goals
- Make iOS and iPad primary web interactions feel native-like and reliable under touch/virtual keyboard/viewport-change.
- Keep watch surface intentionally narrow: nudges, compact state, quick capture, and quick voice/keyboard append into existing thread flows.
- Treat watch as an edge client of `veld`, with iPhone as the preferred bridge/cache/reconciliation proxy.
- Preserve layering contracts and boundary rules (no transport DTO leakage into domain/services).

## Non-goals
- Full parity of desktop admin/monitoring workflows in first pass.
- New domain data models beyond what is required for surface config state.
- Rewriting existing components in one pass; this is a staged migration.

## Proposed implementation waves

### Wave 1: iPhone/iOS first (mandatory)

#### 1. [VEL-UI-601] Surface model + viewport contract
- Add shared surface detection hook/type in:
  - `clients/web/src/core/hooks/useViewportSurface.ts`
- Normalize token semantics for mobile/tablet/desktop in:
  - `clients/web/src/core/Theme/tokens.ts`
- Add safe-area spacing helpers for iOS status bar / keyboard overlays.

**Acceptance criteria**
- All major app shell code paths can branch on one canonical surface value.
- Safe-area variables exist and are used where fixed-position chrome is rendered.

#### 2. [VEL-UI-602] App shell responsive orchestration
- Introduce surface branches in:
  - `clients/web/src/shell/AppShell.tsx`
  - `clients/web/src/shell/MainPanel.tsx`
- Remove desktop-first assumptions for mobile (sidebar visibility, right rail always-on, fixed controls).

**Acceptance criteria**
- Mobile uses single-pane default with no hidden critical actions.
- iPad can route to split mode when available.

#### 3. [VEL-UI-603] Bottom nav and primary routes
- Replace mobile action-only assumptions with explicit bottom navigation in:
  - `clients/web/src/shell/Navbar.tsx`
- Routes: `Now`, `Threads`, `Nudges`, `Settings/Context`.

**Acceptance criteria**
- Every core thread/nudge path is one tap away on mobile.
- Voice/compose actions remain visible and accessible from thread context and now context.

#### 4. [VEL-UI-604] Thread mobile hardening
- Refactor compact thread branch in:
  - `clients/web/src/views/thread/ThreadView.tsx`
- Improve list item density, action discoverability, tap targets, and focus transitions.

**Acceptance criteria**
- Scrolling and thread switching remain stable with large message lists.
- Backward navigation does not lose active thread context.

#### 5. [VEL-UI-605] Message composer iOS ergonomics
- Update:
  - `clients/web/src/core/MessageComposer/*`
- Handle IME, focus, composer pinning on keyboard, and compact state transitions for low-height viewports.
- Strengthen voice state UX states (`listening`, `transcribing`, etc.).

**Acceptance criteria**
- No severe layout jank during keyboard open/close.
- Typing and voice flows are consistently reachable in mobile shell.

#### 6. [VEL-UI-606] Nudges discoverability on iOS
- Update:
  - `clients/web/src/shell/NudgeZone.tsx`
- Provide explicit mobile entry and compact drawer semantics.

**Acceptance criteria**
- Nudges are reachable from a predictable route in any mobile surface.
- Nudges never consume entire content area unless explicitly opened.

#### 7. [VEL-UI-607] Fast append input path
- Surface lightweight keyboard-first append behavior in thread views and now flow.

**Acceptance criteria**
- Users can append to active thread without deep navigation.
- Keyboard submit behavior is stable with no dropped messages.

---

### Wave 2: iPad and near-free desktop (optional/conditional)

#### 8. [VEL-UI-608] iPad split-shell mode
- Add optional split mode in:
  - `clients/web/src/shell/AppShell.tsx`
- Modes: `auto | single | split` with persisted user preference.

**Acceptance criteria**
- iPad can show compact dual-pane list + reader in split mode.
- Landscape/portrait transitions preserve state and selection.

#### 9. [VEL-UI-609] iPad nudge/rail ergonomics
- Tune:
  - `clients/web/src/shell/NudgeZone.tsx`
  - `clients/web/src/views/thread/ThreadView.tsx`

**Acceptance criteria**
- Nudge rail is collapsible/pinnable and non-blocking.
- No duplicate command surfaces with desktop route for the same action.

#### 10. [VEL-UI-610] Desktop polish (if near-free)
- Non-breaking refinements only:
  - `clients/web/src/shell/Navbar.tsx`
  - `clients/web/src/core/Theme/tokens.ts`

**Acceptance criteria**
- Existing desktop flows remain intact.
- iOS/tablet overrides do not regress desktop behavior.

---

### Wave 3: watch edge-client surface (native-first)

#### 11. [VEL-APP-701] Native watch reduced surface
- Implement watch route in `clients/apple` that supports:
  - active nudges
  - compact `Now` or risk snapshot
  - quick capture
  - voice capture
  - keyboard-to-thread append
- Do not expand to full thread management in Wave 3.
- Keep watch policy-free: no watch-local synthesis, no heavy planner logic, no broad thread browsing.

**Acceptance criteria**
- Watch user can always reach nudge actions and append to a thread.
- No complex list management in watch surface.
- iPhone is treated as the preferred bridge for cache, offline replay, and remote transport.

#### 12. [VEL-APP-702] Boundary-safe action mapping
- Wire watch actions to existing services/contracts and typed event/log lanes; keep boundaries clean.

**Acceptance criteria**
- Watch actions map to existing APIs without domain-layer transport coupling.
- Error handling remains deterministic with explainable provenance.
- Sensor or haptic-oriented expansions must remain event-first rather than watch-specific RPC sprawl.

#### 13. [VEL-DOC-703] Surface contract documentation
- Update affected docs and ticket references in `docs/` and Apple docs.

**Acceptance criteria**
- Behavior docs explicitly call out surface scope and limitations per platform.

---

## Effort estimate

- Wave 1: 16–22 story points
- Wave 2: 10–15 story points (reduced to 5–8 if scoped to UI refinements only)
- Wave 3: 8–13 story points
- Total: 34–50 story points

## Suggested execution PR sequence

1. **PR-1**: Surface foundation + tokens (`useViewportSurface`, token updates)
2. **PR-2**: Shell + navbar + route mode branching
3. **PR-3**: Thread + composer stabilization on iOS
4. **PR-4**: Nudge parity + a11y polish
5. **PR-5**: Wave 1 validation + docs update
6. **PR-6**: Wave 2 conditional split-mode (if budget)
7. **PR-7**: Watch reduced surface + contract mapping

## QA / verification notes
- Manual matrix:
  - iPhone portrait/landscape,
  - iPad portrait/landscape,
  - keyboard + IME stress,
  - low-memory/slow render regression checks.
- Manual checks for run/event provenance and action routing from UI gestures to backend.
- Accessibility checks: touch target size, focus states, voice state announcements.
