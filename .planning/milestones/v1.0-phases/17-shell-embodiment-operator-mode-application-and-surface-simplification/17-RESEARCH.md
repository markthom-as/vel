# Phase 17: Shell embodiment, operator-mode application, and surface simplification - Research

**Researched:** 2026-03-19
**Domain:** Web, Apple, and CLI shell embodiment over fixed Phase 14-16 product policy
**Confidence:** HIGH
**Discovery level:** Level 0 - existing patterns only

<user_constraints>
## User Constraints

### Locked Decisions
- Phase 17 is shell embodiment only; it must not redefine backend semantics owned by Phase 16.
- Preserve the agreed taxonomy: `Now`, `Inbox`, `Threads`, `Projects`, `Settings`.
- Keep Apple, web, and CLI embodiment in scope.
- Treat Tauri/desktop only as a future-compatible assumption, not an implementation target.
- Avoid broad UI slop; plans must name concrete repo surfaces and concrete current files.

### Claude's Discretion
- Exact wave breakdown.
- Whether web shell scaffolding should be split from default-surface embodiment.
- Which CLI commands best carry the shell taxonomy.

### Deferred Ideas
- Desktop implementation.
- Backend action-model changes.
- New taxonomy discovery.
- Broad provider/platform expansion.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| SHELL-MODE-01 | Apply the approved default-shell taxonomy to current shell entrypoints. | `Sidebar.tsx`, `MainPanel.tsx`, Apple `ContentView.swift`, and CLI command inventory still expose pre-taxonomy or peer-level surfaces that need classification. |
| SHELL-MODE-02 | Apply progressive disclosure so default, advanced, and internal/runtime surfaces stop blending together. | `SettingsPage.tsx` and Apple Settings/runtime sections still mix summary-first operator guidance with deeper runtime detail. |
| TRUST-SUMMARY-01 | Keep trust, onboarding, and freshness summary-first in shell embodiment. | Phase 14 policy already locks this; current `NowView.tsx` and Apple shells need more consistent embodiment rather than new semantics. |
| APPLE-SHELL-01 | Align Apple shell posture with the same product-mode rules used by web and CLI. | iPhone/iPad tabs and macOS/watch sections still use older labels like `Today`, `Nudges`, and `Activity` rather than the approved taxonomy and disclosure model. |

</phase_requirements>

## Summary

Phase 17 does not need new architecture, new product research, or new backend contracts. It needs disciplined embodiment over shells that already exist.

The web shell is the clearest implementation target. `Sidebar.tsx` still treats `Projects` as primary and still keeps `Suggestions` and `Stats` available like peer surfaces. `MainPanel.tsx` reinforces that by routing those views directly instead of expressing them as contextual or advanced drill-downs. That makes the shell feel broader than the approved product taxonomy.

The supporting web surfaces are uneven rather than wrong. `InboxView.tsx` already reads like a triage queue. `ThreadView.tsx` already has latest-thread fallback and looks like a support surface. `ProjectsView.tsx` and `SettingsPage.tsx` are the places where the shell still most obviously teaches implementation categories or spreads product attention too widely.

Apple is not a blank slate either. The watch already behaves summary-first. The issue is iPhone and iPad labeling and grouping: `Today`, `Nudges`, `Activity`, and `Chat scaffold` are practical implementation-era names, not the approved Phase 14 product vocabulary. The right move is to regroup those surfaces under the same policy as web, not to chase full parity or desktop work.

CLI should also be treated as a shell, not a diagnostic sidecar. The existing commands already map well enough: `today` and daily-loop commands can embody `Now`, `thread` can embody the archive/search lane, `doctor` and `docs` can embody advanced trust/setup paths. Phase 17 should make that mapping explicit in help text and output structure rather than inventing a new CLI paradigm.

## Current Surface Evidence

### Web
- `clients/web/src/components/Sidebar.tsx`
  - Primary: `Now`, `Inbox`, `Projects`
  - Support: `Threads`, `Suggestions`, `Stats`, `Settings`
  - Evidence: the shell still exposes extra peer destinations that the product taxonomy now treats as contextual or advanced.
- `clients/web/src/components/MainPanel.tsx`
  - Routes `suggestions` and `stats` directly as peer views.
  - Evidence: current shell routing still preserves implementation-era peer surfaces.
- `clients/web/src/components/NowView.tsx`
  - Already consumes backend-owned `Now` state and filters action items for `surface === 'now'`.
  - Evidence: Phase 17 should sharpen presentation and routing, not redefine action semantics.
- `clients/web/src/components/InboxView.tsx`
  - Already embodies triage and thread handoff well.
  - Evidence: good target for role-tightening instead of large redesign.
- `clients/web/src/components/ThreadView.tsx`
  - Already latest-thread-falls-back and reads like continuity/history.
  - Evidence: good base for archive/search-first posture.
- `clients/web/src/components/ProjectsView.tsx`
  - Still reads like a rich co-equal destination.
  - Evidence: needs secondary/context posture, not removal.
- `clients/web/src/components/SettingsPage.tsx`
  - Still strongest sprawl point for trust, onboarding, linking, runtime, and diagnostics.
  - Evidence: main progressive-disclosure target.

### Apple
- `clients/apple/Apps/VeliOS/ContentView.swift`
  - iPhone tabs: `Today`, `Nudges`, `Activity`, `Capture`, `Voice`, `Settings`
  - iPad sections: `Now`, `Plan`, `Projects`, `Chat`, `Capture`, `Settings`
  - Evidence: pre-taxonomy labels remain live; phase should align naming/grouping to current product policy.
- `clients/apple/Apps/VelMac/ContentView.swift`
  - Sidebar sections still mix status, context, nudges, commitments, projects, capture, docs.
  - Evidence: needs clearer daily-use vs support vs setup organization.
- `clients/apple/Apps/VelWatch/ContentView.swift`
  - Already summary-first with `Status`, `Now`, `Behavior`, quick actions, and docs.
  - Evidence: likely a light-touch alignment lane, not a broad rewrite.

### CLI
- `crates/vel-cli/src/main.rs`
  - Command list is broad and flat.
- `crates/vel-cli/src/commands/today.rs`
  - Good candidate for `Now` embodiment.
- `crates/vel-cli/src/commands/doctor.rs`
  - Good candidate for advanced trust/setup embodiment.
- `crates/vel-cli/src/commands/docs.rs`
  - Good candidate for guided operator docs entry.
- `crates/vel-cli/src/commands/threads.rs`
  - Natural match for `Threads` archive/search posture.

## Recommended Plan Breakdown

### Plan 17-01
Create shared web shell scaffolding for the approved taxonomy and remove extra peer surfaces from first-contact navigation without deleting the underlying views.

Why first:
- Later web plans need a stable top-level shell model.
- This keeps `Now` / `Inbox` / `Threads` / `Projects` / `Settings` explicit before content tuning widens.

### Plan 17-02
Apply the default-mode policy across `Now`, `Inbox`, and `Threads`.

Why separate:
- These three surfaces form one coherent daily-use slice.
- They share backend-owned semantics from Phase 16 and should be embodied together.

### Plan 17-03
Apply secondary and advanced disclosure rules to `Projects`, `Settings`, and the remaining support/detail views.

Why parallel with 17-02 after 17-01:
- Different file ownership.
- Both depend on stable top-level shell scaffolding, but not on each other.

### Plan 17-04
Align Apple and CLI shells to the same taxonomy and disclosure rules after the web shell model is explicit.

Why last:
- Cross-surface alignment should follow the locked web-facing embodiment model.
- Apple and CLI should copy the stable product policy, not race the web shell.

## Dependency Graph

### Plan-level dependencies
- `17-01` creates the shared shell classification and top-level navigation posture.
- `17-02` needs `17-01` and embodies default daily-use surfaces.
- `17-03` needs `17-01` and embodies secondary/advanced surfaces.
- `17-04` needs `17-01`, `17-02`, and `17-03` so Apple/CLI alignment follows the final embodied shell policy.

### Wave structure
- Wave 1: `17-01`
- Wave 2: `17-02`, `17-03`
- Wave 3: `17-04`

## Risks And Guardrails

### Risk 1: Shell work quietly changes product semantics
- Guardrail: every plan must consume existing Phase 16 typed state and must not invent new shell-local action rules.

### Risk 2: Web simplification removes useful access paths
- Guardrail: demote extra peer surfaces, but keep real drill-down entry points through contextual or advanced paths.

### Risk 3: Apple alignment turns into platform-specific redesign
- Guardrail: adjust labels, grouping, and disclosure posture inside existing `ContentView.swift` seams first.

### Risk 4: CLI gets ignored because it is not visual
- Guardrail: plan explicit operator-facing help and output changes so CLI stays part of the same product model.

## Recommendation

Proceed with four execute plans:

1. shared web shell scaffolding and route classification
2. default-surface embodiment (`Now`, `Inbox`, `Threads`)
3. advanced/support embodiment (`Projects`, `Settings`, trust/setup detail)
4. Apple and CLI alignment

That is the smallest breakdown that preserves current repo seams, maximizes parallelism after the shell scaffold lands, and keeps Phase 17 squarely in shell embodiment rather than logic reinvention.
