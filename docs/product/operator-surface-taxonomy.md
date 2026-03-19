# Operator Surface Taxonomy

This document defines the canonical product-surface taxonomy for Vel.

It exists to answer four questions clearly:

1. what belongs in the default daily-use experience
2. what belongs in advanced operator mode
3. what belongs in internal or developer-only surfaces
4. what should not be treated as a first-class product surface at all

The purpose is classification, not UI implementation.

## Principles

- Keep the default operator story daily-use-first.
- Do not let current component placement define product truth.
- Show summary-first trust and onboarding state before raw diagnostics.
- Keep advanced/runtime/developer concerns progressively disclosed.
- Preserve backend-owned policy; shells embody, they do not define.
- Prefer single-screen or near-single-screen operator flows with contextual drill-down instead of sprawling equal-weight dashboards.

## Default Layout Pattern

The default operator surface should be organized around one compact context pane and one primary action entry.

### Top Context Pane

The top pane should provide a brief, lower-visual-hierarchy context strip that is compact by default and expandable on demand.

Default contents:

- brief context summary
- day stats
- location
- current routine block
- current event or task
- compact high-priority nudges and advisories

Preferred interaction pattern:

- icon-driven where helpful
- compact at first glance
- tap/click to expand into richer detail

This is closer to a context drawer or context pane than a separate `Stats` destination.

### Unified Action Entry

The default surface should expose one primary action entry that can route between:

- capture
- voice chat
- text chat
- action or command request
- thread/conversation start

Preferred behavior:

- automatic routing by default
- with an explicit override affordance when the operator wants to force the mode

## Surface Classes

### 1. Default Daily-Use Surfaces

These are the surfaces the operator should encounter first and repeatedly.

They answer:

- what matters now
- what needs triage
- what work is structurally in play
- whether the daily loop is active or blocked

Current members:

- `Now`
- `Inbox`
- daily loop entry and active-session rendering
- unified action entry
- compact context pane

Rules:

- These should remain the primary navigation center of gravity.
- These should not require opening runtime or diagnostics surfaces to understand normal product state.
- Trust and onboarding information shown here should be summary-level only.
- `Inbox` is the primary work surface.
- `Projects` should not compete with `Inbox` for top-level attention in the default model.

### 2. Advanced Operator Surfaces

These are legitimate product surfaces, but they are not the default daily mode.

They answer:

- why a trust or review lane is blocked
- what integration or linking step is missing
- what review queue or capability state needs operator attention
- what passive observability or inspection detail supports a decision

Current members:

- Settings `general`
- Settings `integrations`
- selected trust and review surfaces
- context/stats drill-down from the default context pane
- agent grounding inspection
- execution review inspection
- relaunchable onboarding

Rules:

- These should be one step away from the default path, not the first thing the operator learns.
- Summary-first guidance belongs ahead of raw detail.
- Advanced operator surfaces are still product surfaces; they are not developer junk drawers.
- For now, these may remain visible inside Settings even if the long-term taxonomy will progressively disclose them more cleanly later.

### 3. Internal / Developer Surfaces

These are implementation-aware or environment-aware surfaces that should not define the default operator story.

They answer:

- low-level runtime status
- component control
- debug or inspection needs
- environment-specific operational detail

Current members:

- direct runtime controls
- component restart/control views
- low-level diagnostics and queue detail
- implementation-facing recovery detail beyond the summary path

Rules:

- These should not sit at the same conceptual level as `Now`, `Inbox`, or `Projects`.
- These should be reachable intentionally through advanced entry points.
- These should never become the default explanation for normal product use.

## Current Product Classification

### Primary

- `Now`
  Why: daily orientation, compact context pane, action pressure, daily loop, and primary action entry
- `Inbox`
  Why: explicit triage, queued work, commitments, todos, and the main work surface

### Secondary / Support

- `Threads`
  Why: parallel work, history, searchable/filterable interaction streams, and long-running work context
- `Projects`
  Why: filtering, grouping, and context/configuration for work rather than the main day-to-day surface
- `Suggestions`
  Why: reviewable supporting material, not the main product center

### Advanced Operator

- `Settings`
  Why: onboarding, trust, integrations, and setup belong here, but as guided operator workflows rather than a generic dumping ground
- context and stats drill-down
  Why: passive observability and richer detail should be accessible from the compact context pane rather than treated as a separate primary destination

### Internal / Developer-Leaning

- runtime-specific controls currently grouped under Settings `runtime`
- component-level operational controls
- low-level diagnostics that are useful when things are broken, not as first-contact product surfaces

## Settings Taxonomy

`Settings` currently mixes several categories.

Canonical classification:

- `general`
  Class: advanced operator
  Purpose: trust summaries, onboarding guidance, bounded review state
- `integrations`
  Class: advanced operator
  Purpose: connector setup, linking, path selection, and recovery
- `runtime`
  Class: internal / developer-leaning operator surface
  Purpose: active control, deep inspection, and implementation-aware recovery

This means Settings should not be treated as one undifferentiated product surface.

Near-term priority note:

- keep the internal classification explicit
- do not spend product energy on hiding or mode-gating everything immediately
- first rework Settings so it stops being structurally overloaded

## Trust Placement

Trust belongs in two layers:

- default layer: summary-first trust state
  Examples: backup freshness summary, daily-loop readiness, blocked review count, key onboarding blocker
- advanced layer: full inspection
  Examples: agent grounding detail, execution review state, runtime control, connector-specific troubleshooting

Base interaction pattern:

- short summary
- one primary action affordance
- one smaller affordance to inspect raw details

This pattern is a strong default for:

- nudges
- advisories
- trust issues
- onboarding blockers
- runtime warnings

It may vary contextually, but it should be the starting rule.

## Apple And CLI Implications

Apple:

- should remain summary-first
- should not expose broad runtime or developer detail by default
- should prioritize compact context, unified action entry, daily loop, nudges/advisories, and commitments/todos
- should include Apple-specific onboarding during main onboarding when on an Apple platform
- may surface bounded trust/setup cues, but deep control remains web/CLI-first unless a specific mobile use case justifies it
- should eventually gain text chat and thread access, but those remain secondary to the summary-first default mobile flow

CLI:

- remains a legitimate operator shell
- may expose deeper inspection than default web mode
- should still follow the same taxonomy rather than inventing a separate product model

## Onboarding Implications

Onboarding should launch from a first-time advisory and remain relaunchable from Settings.

The happy-path onboarding should gather:

- user name
- goals
- calendar connector
- todo connector
- notes connector
- wake and bed times when not already available from Apple signals
- nudging preferences, with sensible defaults and a skip path

If on Apple:

- ask during main onboarding whether to enable Apple-specific capabilities such as health, reminders, messages, activity, and location
- use that decision to determine which functionality modules are on or off

## Navigation Implications

The current preferred top-level navigation is:

- `Now`
- `Inbox`
- `Threads`
- `Settings`

Current surfaces that should not remain top-level by default:

- `Projects`
- `Stats`
- `Suggestions`

## Boundary Note: `Now`, `Inbox`, and `Threads`

These three surfaces should stay distinct even when they share data.

- `Now`
  Purpose: orientation, current-state summary, compact context, daily-loop pressure, nudges/advisories, and the primary action entry
- `Inbox`
  Purpose: explicit triage and actionable queue management for captures, todos, commitments, and reviewable work
- `Threads`
  Purpose: parallel interactive work, conversational history, searchable/filterable streams, and running work context

Working rule:

- `Now` answers "what matters right now?"
- `Inbox` answers "what needs explicit triage or action?"
- `Threads` answers "what conversations, running jobs, or parallel workstreams exist?"

Cross-surface overlap is acceptable, but ownership should stay clear:

- `Now` may summarize inbox pressure, but it should not become the full inbox
- `Now` may show direct actionable UI when something needs immediate attention, but otherwise it should prefer summary counts and launch points
- handled `Now` items may drop into a muted recent-history section at the bottom of the surface instead of staying pinned in the active area
- `Now` may expose the primary entry into thread-capable interaction, but it should not become the long-form history surface
- `Inbox` may launch a thread or show thread-linked work, but it should not become the main conversation archive
- `Inbox` should keep unresolved/actionable items primary and may demote recently handled items into a muted recent-history section instead of keeping one flat endless list
- `Threads` should lean archive/search-first by default, and `Now` or `Inbox` may deep-link into specific thread messages or filtered thread views when attention is needed
- `Threads` may surface captures or work artifacts, but it should not replace inbox triage or daily orientation

See [now-inbox-threads-boundaries.md](now-inbox-threads-boundaries.md) for the fuller boundary draft used by Phase 14 discovery.

## Action Taxonomy Note

Surface filters such as `Needs triage`, `Needs reply`, `Needs review`, or `Commitments` should be treated as views over a canonical action model, not as the core model itself.

That canonical model should capture:

- action kind
- actor or initiator
- permission mode
- surface affinity
- urgency
- state
- source reference
- target reference
- short explainability summary plus inspectable detail

See [operator-action-taxonomy.md](operator-action-taxonomy.md) for the current Phase 14 discovery draft.

## Non-Goals

This taxonomy does not decide:

- final UI layout
- final navigation chrome
- final component decomposition

Those belong to later shell embodiment work.

## Acceptance Criteria

1. Vel has one canonical classification for default, advanced operator, and internal/developer surfaces.
2. `Now`, `Inbox`, the daily loop, the compact context pane, and unified action entry define the default product story.
3. `Projects` is treated as a secondary filtering/context surface rather than a co-equal primary destination.
4. Settings/runtime sprawl is named explicitly as a classification problem rather than silently normalized.
