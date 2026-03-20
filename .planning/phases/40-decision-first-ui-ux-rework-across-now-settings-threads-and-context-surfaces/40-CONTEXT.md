# Phase 40 Context

## Phase

40 — Decision-first UI/UX rework across `Now`, `Settings`, `Threads`, and context surfaces

## Why This Phase Exists

The repaired daily-use arc made Vel materially more usable, but the operator still sees significant UI/UX problems:

- too much system state shown without clear action
- weak hierarchy across cards and panels
- conceptual overload from product and model vocabulary appearing at the same visual level
- debug/internal runtime state leaking into user-facing surfaces
- `Now`, `Threads`, and `Settings` still overlapping too much in perceived job and mental model
- meaningful web and mobile functionality still appears broken or unreliable in normal use

This phase exists to tighten the product shell around one decision-first principle:

> every primary surface should help the operator decide and act, not narrate internal state

It also requires an upfront discovery pass:

- audit what currently works
- audit what is visually present but broken/inert/misrouted
- separate genuine functionality failures from hierarchy/copy/interaction-design problems
- use that audit as the implementation baseline for the phase

## Operator-Supplied Design Spec

### Global problems to solve

1. State does not translate cleanly into action
2. Visual hierarchy is too flat
3. Too many concepts are shown at once
4. Debug/runtime model state leaks into operator-facing UI
5. Important interaction paths on web and mobile do not work reliably enough to trust

### Global corrections

- Every card should answer:
  - what should I do?
  - what happens if I click this?
- Strict hierarchy:
  - Tier 1: current action
  - Tier 2: next actions
  - Tier 3: background/system info
- Progressive disclosure by default
- Debug/internal model output moved behind explicit affordances:
  - debug mode
  - or right-panel debug tab
- Treat broken or unreliable operator interactions as part of the phase scope:
  - missing/broken buttons
  - inert affordances
  - wrong routing
  - mobile/web behavior drift
  - shell interactions that describe state but fail to complete the intended action

## Screen Intent Contract

### `Now`

Job:

- run the current day

Target structure:

1. primary action strip
2. collapsible time context
3. task stack

Required direction:

- one dominant current commitment/action
- inline continue / break down / defer controls
- next event connected to today execution, not a separate detached summary
- voice input pinned and always available
- routine card removed as its own primary surface concept
- no duplicate current/current-status blocks
- no dead empty states like “Nothing scheduled”
- primary actions on web and mobile must actually work and resolve through the intended inline or continuity path

### `Settings`

Job:

- configure the system

Target structure:

1. system profile
2. planning model
3. routines
4. recovery model

Required direction:

- no second-dashboard behavior
- categories must become explicit and operator-meaningful
- constraints vs preferences split clearly
- routines should read visually like time blocks, not raw forms
- “freshness / aging” and abstract planning/recovery summary cards should not dominate the primary settings surface
- inline editable cards preferred over full-page forms
- web/mobile configuration actions should be auditable and working, not just visually present

### `Threads`

Job:

- think through open loops

Mental model:

- `Threads` = open loops that need deeper thought

Required direction:

- clearer why/status per thread
- structured thread view with:
  - timeline
  - decisions made
  - open questions
  - next step
- remove generic chat framing and empty-chat affordances
- support:
  - promote from `Now` to thread
  - resolve thread back to action
- thread affordances should complete real continuity work rather than behaving like dead-end shell navigation

### Right panel / context surface

Job:

- provide human-readable context and explanation without leaking raw model state by default

Required direction:

- tab structure:
  - `State`
  - `Why`
  - `Debug`
- remove confidence percentages, risk scores, IDs, booleans, and similar raw internals from default display

## Cross-Screen Product Rules

1. One screen = one job
   - `Now` → act
   - `Threads` → think
   - `Settings` → configure
2. Max 3 actions rule
3. Kill dead states
4. Inline > navigation for decomposition, editing, and clarifying context
5. System confidence should affect behavior, not become raw display copy

## Visual Direction

- one dominant card per screen
- increased vertical spacing
- grouping by function rather than raw data type
- color reserved for:
  - active
  - blocked
  - urgent

## Litmus Test

For every element:

- if removed, does the operator lose the ability to act?
- if not, remove or demote it

## Risks / Open Design Questions

These are the main questions to settle during Phase 40 planning:

1. What should the top primary action strip show when there is no active commitment?
2. Should `Continue / Break down / Defer` apply to every primary item type or only commitment/task rows?
3. Is routine drag-edit in `Settings` in scope for this phase, or should this phase stop at timeline visualization plus inline edit controls?
4. Should the new `Threads` mental model be web-first in this phase, or should Apple parity be included immediately?
5. Should the right-panel `State / Why / Debug` tabs be global shell behavior or attached only to selected surfaces like `Now` and `Threads`?
6. How far should inline decomposition go in this phase:
   - lightweight checklist/subtask breakdown only
   - or broader inline planning/editing

## Recommended Planning Bias

- begin with a discovery and interaction-audit slice before redesign/implementation slices
- keep this phase focused on hierarchy, actionability, and mental-model repair
- avoid turning it into a broad visual polish pass without structural change
- treat drag-edit routines as a possible follow-on if it threatens to dominate implementation cost
- include a focused broken-interactions audit for web and mobile so obviously non-working affordances are repaired inside the same phase
- preserve backend-owned product logic; this phase is primarily shell/interaction redesign, not a new planner or shell-local policy system
