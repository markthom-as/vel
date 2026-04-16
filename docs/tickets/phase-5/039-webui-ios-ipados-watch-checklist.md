# Ticket 039A: Per-file implementation checklist (iOS/iPadOS/watch surface)

## Scope mapping
- Wave 1 (required): iOS-first web responsive foundation
- Wave 2 (conditional): iPad split + desktop polish
- Wave 3 (required after Wave 1): watch-reduced surface in Apple clients

## Wave 1 — iOS-first web foundation

### `clients/web/src/core/hooks/useViewportSurface.ts` (NEW)
- [x] Add viewport-size enum/type: `mobile | tablet | desktop`.
- [x] Add breakpoint constants aligned to `Theme/tokens.ts` values.
- [x] Implement SSR-safe resize subscriber and cleanup.
- [x] Expose derived flags: `isMobile`, `isTablet`, `isDesktop`, `isLandscape`.
- [x] Add small helper for safe-area mode (`supportsKeyboardInset`).
- [x] Add unit tests (`*.test.ts`) covering resize and lifecycle.

### `clients/web/src/core/Theme/tokens.ts`
- [x] Add/update compact/tablet/dense token groups for spacing, touch target size, type scale, header/nav heights, composer heights.
- [x] Add iOS safe-area constants: status bar/nav inset helpers.
- [x] Export token docs/comments describing intended surface contract.

### `clients/web/src/core/Theme/index.ts`
- [x] Re-export new surface tokens/types as needed.
- [x] Ensure old imports still resolve.

### `clients/web/src/shell/AppShell/AppShell.tsx`
- [x] Add surface-driven layout branches (`mobile | tablet | desktop`) using the App-owned `useViewportSurface` contract.
- [x] On mobile, default to single-pane shell with command composer route + content region.
- [x] On tablet, route to optional split mode with persisted preference.
- [x] Ensure right rail/nudges are not forced visible outside tablet split.
- [x] Maintain keyboard-safe bottom spacing and safe-area padding.

### `clients/web/src/shell/AppShell/AppShell.test.tsx`
- [x] Add tests for surface selection branches (mobile/tablet/desktop).
- [x] Add regression for state persistence in split-mode preference.
- [x] Ensure no desktop-only panel is required for essential iOS paths.

### `clients/web/src/shell/MainPanel/MainPanel.tsx`
- [x] Rework composition of body content to avoid desktop-only assumptions.
- [x] Add explicit mobile fast path for threads + now content composition.
- [x] Ensure mini-composer route transitions do not trap focus/scroll.

### `clients/web/src/shell/MainPanel/MainPanel.test.tsx`
- [x] Validate mobile fallback path renders expected primary route.
- [x] Validate composer mount/unmount behavior during route and orientation flips.

### `clients/web/src/shell/Navbar/Navbar.tsx`
- [x] Convert to clear primary-route bottom nav model for mobile.
- [x] Keep desktop top strip where present; hide only via surface contract.
- [x] Add accessible label/aria support for tab-like controls.
- [x] Add nudges route entry and active state.

### `clients/web/src/shell/Navbar/navbarChrome.ts`
- [x] Add/adjust theme tokens for mobile vs iPad chrome metrics.
- [x] Ensure safe-area-aware bar sizing in compact mode.

### `clients/web/src/shell/Navbar/Navbar.test.tsx`
- [x] Add mobile rendering test for bottom nav tabs.
- [x] Add mobile nudges route active-state coverage.
- [x] Add keyboard-focus/selection tests for iPad if reachable by component.

### `clients/web/src/views/threads/ThreadView.tsx`
- [x] Audit and normalize compact thread branch logic.
- [x] Improve thread list row hit area and action density.
- [x] Preserve active thread context across orientation/resizing.
- [ ] Route composer input into append-first action on mobile.

### `clients/web/src/views/threads/ConversationList/ConversationList.tsx`
- [x] Tune row rendering for mobile (avatar/icon density, one-line summaries).
- [x] Add overflow action affordances in a touch-friendly way.
- [x] Keep unread/priority cues clear without consuming horizontal space.

### `clients/web/src/views/threads/ConversationList/ConversationList.test.tsx`
- [x] Add mobile row interaction + selection tests.
- [x] Add compact rendering assertions (no overflow clipping).

### `clients/web/src/core/MessageComposer/index.ts`
- [x] Export adjusted props for keyboard/voice-first variants.
- [x] Ensure component entry points include mobile surface context.

### `clients/web/src/core/MessageComposer/MessageComposer.tsx`
- [x] Add IME-safe focus handling and keyboard inset compensation.
- [x] Add composer pinning behavior when keyboard opens.
- [x] Add distinct voice state transitions (`idle`, `listening`, `transcribing`, `error`).
- [x] Prevent accidental send/clear on iOS “return” and IME composition edge cases.

### `clients/web/src/core/MessageComposer/MessageComposer.test.tsx`
- [x] Add/extend coverage for keyboard open/close state and compose submit.
- [x] Add voice-state assertions.

### `clients/web/src/shell/NudgeZone/NudgeZone.tsx`
- [x] Make nudge entry discoverable in mobile surface without layout dominance.
- [x] Add compact drawer/badge behavior (open, dismiss, expand).
- [x] Ensure nudges can be actioned from one-tap surface path.

### `clients/web/src/shell/NudgeZone/NudgeZone.test.tsx`
- [x] Add mobile open/close tests and state assertions.
- [x] Add a11y checks for active/expanded nudge controls.

### `clients/web/src/views/now/NowView.tsx`
- [x] Add mobile route path for now task/nudge actions.
- [x] Confirm now actions route to composer or thread append without extra navigation.

### `clients/web/src/views/now/components/NowNudgeStrip.tsx`
- [x] Ensure compact display and touch targets for iPhone.
- [x] Avoid overflow in long nudge labels; keep row actions tappable.

### `clients/web/src/views/now/components/NowTasksSection.tsx`
- [x] Verify task action buttons and completion controls are thumb-friendly.
- [x] Reduce text density and spacing in mobile surfaces.

### `clients/web/src/views/now/components/CompactTaskLaneRow.tsx`
- [x] Audit row height and tap area for one-handed interactions.

### `clients/web/src/views/now/nowNudgePresentation.tsx`
- [x] Ensure mapping logic has mobile-safe fallback labels and priorities.

### `clients/web/src/main.tsx`
- [x] Ensure viewport meta tags are iOS-safe (viewport-fit=cover as needed).
- [x] Confirm app shell mount sequence supports surface-driven conditional renders.

### `clients/web/src/core/Theme/tokens.ts` + related css entry point
- [x] Reconcile classnames used by shell/chrome with updated tokens (single source of spacing truth).

## Wave 2 — iPad split + optional desktop polish

### `clients/web/src/shell/AppShell/AppShell.tsx`
- [x] Add persisted `layoutMode` user setting (`auto | single | split`).
- [x] Implement split activation thresholds independent of generic `lg` breakpoint.
- [x] Handle orientation changes without dropping active thread state.

### `clients/web/src/views/threads/ThreadView.tsx`
- [x] Add optional three-region flow (list/detail/composer) in split mode.
- [x] Preserve thread selection on rapid width changes.

### `clients/web/src/views/threads/ConversationList/ConversationList.tsx`
- [ ] Add collapsible/compact list mode variant for split surfaces.
- [x] Keep action affordances for pin/archive/mute in iPad list width.

### `clients/web/src/shell/NudgeZone/NudgeZone.tsx`
- [x] Add iPad docked rail option and close/collapse control.
- [x] Avoid overlapping focus with thread composer in split layouts.

### `clients/web/src/shell/Navbar/Navbar.tsx`
- [x] Keep tab row for phone and transition to adaptive control set for tablet/desktop.

### `clients/web/src/core/Theme/tokens.ts`
- [x] Add compact/iPad density variants where split and one-hand/three-panel flows diverge.

### `clients/web/src/core/Theme/navbarChrome.ts`
- [x] Tune toolbar heights and horizontal rhythm for iPad split.

### `clients/web/src/views/threads/ThreadView.test.tsx`
- [x] Add split-mode render tests (desktop width but compact list/detail assertions).

### `clients/web/src/main.tsx`
- [x] Confirm root route container and viewport handling for iPad landscape/portrait.

## Wave 3 — Watch-reduced surface (Apple)

### `clients/apple/Docs/feature-capability-matrix.md`
- [x] Add/update explicit watch scope note: nudges, voice, keyboard append only.
- [x] Record blocked/unsupported features to set expectations.

### `clients/apple/Docs/apple-architecture.md`
- [x] Align watch boundary and data flow with reduced-surface contract.

### `clients/apple/AGENTS.md`
- [x] Update instructions if needed for explicit watch scope and limits.

### `clients/apple/Apps/VelWatch/ContentView.swift`
- [x] Implement minimal watch entry surface.
- [x] Ensure route set includes active nudges and one-thread append composer.
- [x] Add quick voice capture path and keyboard append fallback.

### `clients/apple/Apps/VelWatch/VelWatchApp.swift`
- [x] Ensure watch app lifecycle and capabilities are scoped to reduced surface.
- [x] Confirm target surfaces are minimal and secure by default.

### `clients/apple/Apps/VeliOS/ContentView.swift` and related iOS scenes
- [ ] Ensure iPhone/iPad app routes can point to the same reduced surface primitives where appropriate.
- [ ] Keep watch-only flows out of non-watch targets.

### `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`
- [x] Confirm no web DTO transport types leak into domain/service boundaries.
- [x] Verify thread append/nudge actions reuse stable API boundaries.

### `clients/apple/Packages/VelAppleModules/Sources/VelInfrastructure/Infrastructure.swift`
- [ ] Validate secret/tool capability mediation for voice/nudge actions stays boundary-safe.

### `clients/apple/Packages/VelAppleModules/Sources/VelApplication/Services.swift`
- [ ] Add/adjust service mappings for watch append/nudge actions.

### `clients/apple/Apps/VelWatch/` tests (or new tests)
- [x] Add watch-surface tests for nudge + voice + keyboard append flows.

## Cross-cutting docs/contract updates

### `docs/MASTER_PLAN.md`
- [x] Add/update short section describing platform-surface migration priority and watch limits.

### `docs/tickets/phase-5/039-webui-ios-ipados-watch-plan.md`
- [x] Link to this checklist and mark status updates as each file block completes.

### `README.md`
- [x] Verify references still point to active source of authority for UI surface targets.

## Suggested execution order with file-level ownership

1. `useViewportSurface` + `tokens` + `main.tsx` (foundation)
2. `AppShell` + `MainPanel` + `Navbar`
3. `ThreadView` + `ConversationList` + `MessageComposer`
4. `NudgeZone` + `Now` compact surfaces
5. Cross-surface docs + tests
6. Wave 2 split-mode files
7. Wave 3 Apple watch and API bridge files

## Sprint board format
For implementation planning with estimates and ownership, use:
- [phase-5/039-sprint-board-ios-ipados-watch.md](/home/jove/code/vel/docs/tickets/phase-5/039-webui-ios-ipados-watch-sprint-plan.md)
