# Phase 52 Context

## Phase

**52: Full Now/UI conformance implementation chunk**

## Objective

Implement the full operator correction memo in one execution chunk. No requested product, IA, shell, or data-truth item is deferred beyond this phase.

## Product Authority

Authority order for this phase:

1. latest operator clarification memo
2. `/home/jove/Downloads/vel-now-surface-contract-codex-final.md`
3. current shipped UI

## Locked Decisions

### Now

- top area must be containerless
- top area must use stacked micro-rows, not a horizontal strip
- top area structure:
  - title
  - timing + active description
  - nudges row if present
- timing is minimal:
  - include current time
  - include minimal context only when useful
  - exclude elapsed time and verbose metadata
- active description is single-line and truncated with ellipsis
- nudges render as compact styled info boxes, not inline text
- each nudge includes:
  - icon
  - severity/type color
  - project tags
- project tag colors should be auto-generated for now; operators can override later in project settings
- hide empty-state controls entirely when value is `0` or `null`
- remove `More Context and Controls` from the Now page body
- input must be a floating bottom-center overlay
- input stays visible while on `Now`
- MVP input controls are text + voice only
- input behavior is inline-first and escalates to thread only when needed
- helper text should be removed across the surface
- buttons should be small and ALL CAPS across the affected surfaces, using shared components rather than one-off styling

### Tasks

- tasks are the only dominant visual container on `Now`
- tasks must be grouped, not flat
- strict group order:
  1. `NOW`
  2. `TODAY`
  3. `AT RISK`
  4. `NEXT`
- keep the task container even in zero state
- zero state should be a terse neutral line
- project reviews should be removed from `Now` unless explicitly due today

### Navigation And Shell

- replace the current left sidebar navigation with compact top navigation
- top nav must include:
  - context/sync indicators
  - `Now`
  - `Inbox`
  - `Threads`
  - `Settings`
  - `Documentation`
- right sidebar should be collapsible context/documentation
- right sidebar should be collapsed by default even on desktop
- collapsed state must still show an open affordance including arrow + info icon
- on mobile/responsive layouts, the sidebar affordance becomes a top-level info-nav item
- remove `Daily Use`
- remove sidebar explanations, helper blurbs, and shell noise
- improve icons for compactness and semantic clarity

### Inbox

- `Inbox` must contain the same underlying actionable objects shown in `Now`
- `Inbox` remains the superset queue
- empty inbox while `Now` has actionable items is a data-model/query bug, not a display bug

### Threads

- threads layout:
  - global top nav
  - left thread list
  - main thread content panel
- thread row must include:
  - title
  - truncated last message
  - unread indicator
  - unread count when applicable
  - optional lower-priority tags
- priority under tight space:
  1. unread
  2. message preview
  3. tags
- thread title fallback should be generated from metadata/context/user first message until a later LLM-backed title lane exists

### Settings

- settings must be compact
- settings must use a left tab rail, not top tabs
- settings must use minimal prose
- settings grouping:
  - `Profile / Onboarding`
  - `Device / Sync`
  - `Agent Grounding / State`
  - `Backups`
- documentation must be removed from Settings and exposed in top nav / right sidebar access

### Parity And Verification

- web is the reference implementation for the phase
- parity-sensitive client behavior should follow the corrected web reference in this phase
- if Apple app-target execution is not available in this environment, source-level parity plus shared-contract alignment is acceptable in implementation, with execution-backed parity evidence recorded separately during verification
- verification order remains:
  1. manual checklist
  2. contract/DTO tests
  3. UI tests

## Codebase Reality Observed

- current app shell is still three-column with a left sidebar and optional right context panel
- `Sidebar.tsx` still renders `Daily Use`, `Support`, and helper copy
- `NowView.tsx` still uses:
  - a large boxed top card
  - elapsed time in the status row
  - inline-style nudge bars without project-tag treatment
  - a `More context and controls` expandable section
  - non-floating composer behavior
- `InboxView.tsx` still contains multiple explanatory paragraphs and large card treatment
- `ThreadView.tsx` still treats the thread list as a chip row inside the content header rather than a left column
- `SettingsPage.tsx` still uses top tabs, verbose prose, and includes documentation directly inside settings

## Execution Guardrails

- do not touch unrelated in-flight repository changes outside the conformance slice
- preserve Rust-owned semantics from `v0.3`; only change backend/shared data where necessary to fix the inbox/now shared-object truth or support the corrected surface
- prefer shared UI components for repeated button, tag, and info-box styles so the phase does not hardcode one-off variants
