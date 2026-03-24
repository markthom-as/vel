---
phase: 82
slug: milestone-lock
status: draft
created: 2026-03-23
---

# v0.5.3 UI System Design — Milestone Lock

This document compresses the current milestone into:

- resolved decisions
- remaining decisions worth locking
- recommended defaults
- explicit bans
- implementation-ready next steps

It exists to prevent the design-definition milestone from dissolving into endless pseudo-open questions.

## Resolved Decisions

### Product Law

- `Now` is strictly bounded and must not become inbox-like.
- `Now` allows only:
  - active task
  - one or two next items
  - current event
  - next event
  - nudges
  - trust state
- `Now` forbids:
  - long queues
  - scrolling task lists
  - project grouping
  - generic today dumping
  - messages
  - threads
  - artifacts
  - runs/logs
  - people
  - raw integrations
  - config
- `Now` follows one dominant slot plus one subordinate slot.
- `Threads` is object/context first and chronology second.
- `System` is hybrid:
  - read-first in `Overview` and `Integrations`
  - more operational in `Operations` and `Control`
- Projects are:
  - tag-only on `Now`
  - stronger contextual identity in `Threads`
  - first-class in `System`

### Shell Law

- Shell chrome is instrument-like and spatially consistent across surfaces.
- Top orientation band remains stable across surfaces.
- Nudge zone is always present, compressing outside `Now`.
- Bottom action bar is always visible except in extreme focus modes where it must be instantly recallable.
- Bottom action bar includes:
  - voice
  - capture
  - ask
  - command
  - one brokered contextual quick slot
- Mobile uses a docked action bar, not a floating desktop-style overlay.
- Breadcrumbs appear only in focused subviews when needed.

### Surface Behavior

#### `Now`

- active task wins visually
- event wins behaviorally as a constraint
- events visible: max two
- nudges render in a dedicated lane
- completed items disappear immediately, allowing at most a brief transient acknowledgement
- trust/status appears only when degraded or critical

#### `Threads`

- default open state is the continuity stream
- ordering is hybrid: recency first with relevance/pinned influence
- provenance is collapsed by default
- filters are sticky per thread
- run/action blocks are visually distinct from messages
- bounded config editing is allowed inline

#### `System`

- `Integrations` is highly prominent
- `Control` should feel dense but readable
- logs are summary-first with drill-down
- `Preferences` includes visual and accessibility settings in this milestone

### Interaction Law

- optimistic by default:
  - complete task
  - dismiss nudge
  - defer nudge
  - toggle preference
- confirmation required:
  - delete
  - disconnect
  - revoke auth
  - destructive resets
  - high-risk external actions
- destructive means:
  - delete
  - revoke
  - reset
  - disconnect
- not destructive by default:
  - archive
  - dismiss
  - resolve
- inline feedback plus persistent review path is the trust model
- retry/review affordances should be standardized everywhere relevant
- critical actions must never be hover-only

### Foundation Direction

- keep the copper/orange direction and refine it
- dark-first is canonical for this milestone
- typography should feel technical-instrumental with a slight editorial layer
- mono is reserved for timestamps, IDs, logs, and provenance
- object colors should be selective, not universal
- provider/client identities should be recognizable but subdued
- provider colors should be normalized into Vel’s palette rather than raw brand colors

### Component Law

- row-first component system
- cards are reserved for:
  - nudges
  - run/action blocks
  - media/artifact blocks
  - config blocks
- metric strips should be reduced in prominence
- one canonical density for MVP
- drawers are a sparing primitive
- one base row skeleton with surface-specific subclasses

### Accessibility

- high contrast
- keyboard-first support
- clear focus states
- reduced motion support
- color never stands alone
- minimum touch targets are enforced

### Deliverables

- docs
- clickable mockups
- browser proofs / screenshots / fixtures
- primitive-first follow-on implementation plan

## Remaining Decisions Worth Locking

These are the real unresolved items. Everything else can wait for token pass or implementation detail.

### 1. Typography Stack

Need to lock explicitly:

- display font
- body font
- mono font
- numeral behavior
- casing rule

Recommended default:

- display: `Geist`
- body: `Inter`
- mono: `JetBrains Mono`
- numbers: tabular numerals for timestamps, counts, durations, metrics
- casing: sentence case by default

Optional restrained accent:

- `Instrument Serif` or `Editorial New` as a non-structural accent only

### 2. Color Token Ladder

Need to lock:

- neutral ladder
- copper ramp
- semantic-state hues
- provider/client identity handling

Recommended direction:

- neutrals: charcoal/graphite, not blue-gray
- primary accent: restrained copper/orange
- warning: amber
- degraded: burnt orange or ochre
- offline: cool desaturated slate
- syncing: muted electric blue
- blocked/error: red family, with blocked more muted and error more acute
- done: subdued green
- active: copper, not green

### 3. Object Color Coverage

Need to lock exactly which objects get distinct families.

Recommended:

Distinct families:

- nudge
- project
- provider/integration
- trust/system state
- event as light secondary family
- run/action as a shared family

Mostly neutral:

- message
- thread
- artifact
- task
- person
- client

Rules:

- message remains neutral
- run and action share one family for MVP
- thread does not inherit project color by default

### 4. Disclosure Maps Per Surface

Need to lock exactly what is inline, drawer, focused, review, or routed.

Recommended default:

#### `Now`

Inline:

- complete / defer / confirm / reject task actions
- nudge acknowledge / dismiss / defer
- quick capture
- maybe one-line task note

Drawer:

- shallow event inspection
- shallow trust explanation
- minimal object preview

Escalate to `Threads`:

- discussion
- reasoning
- evidence
- multi-object context
- anything ambiguous

Escalate to `System`:

- sync/auth/integration problems
- preferences/config
- deeper trust/system detail

Rule:

- active task must not become a giant inline expander

#### `Threads`

Inline expansion:

- messages
- object cards
- nudges
- run/action summaries
- bounded config blocks

Focus mode:

- media
- artifacts
- logs
- runs
- utility blocks
- richer config detail

Inline editing:

- bounded config
- some task/object fields
- message drafting
- lightweight metadata edits

Shared review/detail surface:

- provenance
- logs
- run results
- action traces

Bias:

- provenance should usually open in a shared review panel rather than bloat each block inline

#### `System`

Inline:

- row expansion
- toggles
- compact settings
- summary states

Detail pane / split view:

- integrations
- object browsers
- mappings
- log summaries
- control object detail

Dedicated detail page:

- very large log sets
- complex config structures
- workflow admin
- schema-adjacent editors

Rule:

- `Integrations` should be browse/detail split, not giant expanding spaghetti

### 5. Canonical Row Anatomy

Need to lock the base row.

Recommended anatomy:

`leading icon/status -> title block -> secondary metadata strip -> tags/chips -> trailing primary action + overflow`

Title block:

- title
- one short subordinate line max

Metadata:

- max three visible metadata items by default

Good metadata examples:

- due time
- client
- state
- provider
- count

Rules:

- confidence should not appear as a raw number on standard rows
- provenance should not appear inline on standard rows
- one visible primary action
- at most one visible secondary action
- the rest go in overflow

### 6. Mockup Fidelity

Need to lock the expected design deliverable fidelity.

Recommended:

- mid-fi interactive layouts
- not lo-fi wireframes
- not near-final polished comps

Should include:

- real layout
- real interaction states
- provisional tokens
- enough fidelity to validate shell law and disclosure logic

Coverage:

- desktop first
- mobile for shell plus one key flow per surface
- happy path plus degraded/error states

Recommended scenarios:

- `Now` normal
- `Now` degraded
- `Threads` normal
- `Threads` focused block
- `System` integrations issue
- `System` preferences/config example

### 7. Explicit Banned Patterns

These should be treated as milestone law.

Ban outright:

- inbox-like `Now`
- project-grouped `Now`
- giant generic dashboard panels
- global explanatory side rails
- mixed catch-all surfaces where task/message/run/config blur together
- raw logs dumped inline by default
- always-visible decorative metric strips
- hidden critical actions on hover only
- giant undifferentiated cards with helper-text soup
- surface-specific shell reinventions

Mostly banned except narrow use:

- metric-heavy summary strips
- drawer-for-everything
- smart collapsing action bars
- color-only status communication
- provider brand colors dominating UI

## Not Worth More Milestone Energy

Do not let these consume the milestone unless a strong preference emerges:

- exact max card height in pixels
- exact drawer animation style
- exact nudge animation timing
- breakpoint-specific typeface switching edge cases
- exhaustive focus-order docs per surface
- whether client identity gets color in this milestone
- every edge case of low-severity nudge container treatment

These are implementation or token-pass details, not milestone-defining decisions.

## Final Clarifiers Still Worth Asking

If anything still needs direct user confirmation, keep it to these:

1. Font stack
   Choose between pragmatic `Geist / Inter / JetBrains Mono` and a slightly more stylized variant.

2. Color temperament
   Choose between:
   - warmer industrial / copper instrument
   - darker cleaner technical / graphite with copper accents

3. Provider identity intensity
   Choose between:
   - clearly recognizable but subdued
   - heavily subordinated under Vel’s own language

4. Mockup medium
   Choose between:
   - HTML/CSS interactive prototypes in repo
   - Figma-like clickable mockups
   - both

5. Retirement posture
   Choose between:
   - aggressive removal for shell/surface primitives with gradual migration for low-level pieces
   - broader compatibility wrapping

## Implementation-Ready Next Steps

1. Lock the remaining typography decision.
2. Lock the color token ladder.
3. Lock object color coverage.
4. Convert the disclosure maps into page-level UI specs.
5. Define the canonical row skeleton and variants.
6. Produce mid-fi interactive mockups.
7. Start the follow-on implementation milestone with shared primitive rebuild first.
