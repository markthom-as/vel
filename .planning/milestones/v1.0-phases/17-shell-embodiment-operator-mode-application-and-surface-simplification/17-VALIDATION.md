# Phase 17 Validation

## Scope Check

- Phase 17 remains shell embodiment only.
- No plan redefines backend-owned `check_in`, `reflow`, trust/readiness, or project-action semantics.
- The approved taxonomy remains fixed: `Now`, `Inbox`, `Threads`, `Projects`, `Settings`.
- Apple, web, and CLI are all covered.
- Desktop/Tauri is referenced only as a future-compatible assumption.

## Requirement Coverage

| Requirement | Covered By | Why |
|-------------|------------|-----|
| SHELL-MODE-01 | `17-01`, `17-02` | Top-level shell classification plus daily-use surface embodiment. |
| SHELL-MODE-02 | `17-01`, `17-03`, `17-04` | Progressive disclosure is applied in web shell scaffolding, advanced web surfaces, and Apple/CLI alignment. |
| TRUST-SUMMARY-01 | `17-02`, `17-03` | `Now` summary-first treatment plus Settings/trust/onboarding disclosure cleanup. |
| APPLE-SHELL-01 | `17-04` | Apple surface regrouping and parity with the approved taxonomy. |

## Wave Structure

| Wave | Plans | Rationale |
|------|-------|-----------|
| 1 | `17-01` | Establish shared shell classification and navigation posture first. |
| 2 | `17-02`, `17-03` | Daily-use surfaces and advanced/support surfaces can proceed in parallel once shell scaffolding is fixed. |
| 3 | `17-04` | Apple and CLI should mirror the final embodied shell policy, not race it. |

## File Ownership Check

### 17-01
- `clients/web/src/App.tsx`
- `clients/web/src/components/Sidebar.tsx`
- `clients/web/src/components/MainPanel.tsx`
- `clients/web/src/components/Sidebar.test.tsx`
- `clients/web/src/components/MainPanel.test.tsx`
- `clients/web/src/data/operatorSurfaces.ts`
- `docs/user/surfaces.md`

### 17-02
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/NowView.test.tsx`
- `clients/web/src/components/InboxView.tsx`
- `clients/web/src/components/InboxView.test.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/ThreadView.test.tsx`

### 17-03
- `clients/web/src/components/ProjectsView.tsx`
- `clients/web/src/components/ProjectsView.test.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/components/SettingsPage.test.tsx`
- `clients/web/src/components/StatsView.tsx`
- `clients/web/src/components/SuggestionsView.tsx`

### 17-04
- `clients/apple/Apps/VeliOS/ContentView.swift`
- `clients/apple/Apps/VelMac/ContentView.swift`
- `clients/apple/Apps/VelWatch/ContentView.swift`
- `clients/apple/README.md`
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/commands/today.rs`
- `crates/vel-cli/src/commands/doctor.rs`
- `crates/vel-cli/src/commands/docs.rs`
- `crates/vel-cli/src/commands/threads.rs`

No execution-plan file overlap exists across waves 2 and 3.

## Goal-Backward Checks

### Observable truths required for the phase
- The default shell reads as `Now` first, `Inbox` second, with support surfaces clearly demoted.
- `Threads` is reachable and useful without becoming the default triage surface.
- Trust, onboarding, and freshness stay summary-first in everyday use.
- Projects remain visible and useful without taking equal weight with `Inbox`.
- Apple and CLI express the same product categories instead of their own legacy labels.

### Critical links that must be preserved
- Top-level shell routing must continue to point at existing typed views rather than creating new shell-only state.
- `Now` UI must continue consuming backend-owned typed action/readiness data.
- Inbox-to-thread and thread-to-history paths must remain typed and intact.
- Settings summary cards must still link to the deeper runtime/setup surfaces that already exist.
- Apple and CLI must continue using the same daemon-backed typed data they use today.

## Decision Fidelity Check

- Every locked decision from the user prompt is implemented in the phase package.
- No plan proposes backend logic changes.
- No plan replaces the approved taxonomy.
- No plan treats desktop/Tauri as an execution target.
- No plan uses vague "redesign the UI" language without file ownership.

## Ready-To-Execute Outcome

Phase 17 is now decomposed into four concrete shell plans that:

- preserve Phase 16 semantics
- keep daily-use shells simpler than current reality
- make progressive disclosure explicit
- keep Apple, web, and CLI aligned without requiring new backend work
