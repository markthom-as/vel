---
title: Apple Watch Edge Client Contract
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-26
updated: 2026-03-26
keywords:
  - apple watch
  - watchos
  - edge client
  - veld
  - iphone bridge
  - haptics
index_terms:
  - watch edge client
  - veld watch contract
  - iphone bridge
  - watch offline queue
related_files:
  - clients/apple/README.md
  - clients/apple/Docs/apple-architecture.md
  - clients/apple/Docs/feature-capability-matrix.md
  - clients/apple/Apps/VelWatch/ContentView.swift
  - clients/apple/Apps/VelWatch/VelWatchApp.swift
  - docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md
  - docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - docs/tickets/phase-5/039-webui-ios-ipados-watch-plan.md
summary: Canonical contract for treating Apple Watch as a thin edge client of `veld`, with iPhone as the local bridge and Rust authority remaining outside watchOS.
---

# Purpose

Define the canonical architectural model for `VelWatch`.

This document is the authority for:

- what role Apple Watch plays in the Vel system
- where Rust authority lives when watchOS is part of the operator loop
- how Apple Watch, iPhone, and `veld` should relate in online and offline paths
- which watch behaviors are high-value and in-bounds
- which watch behaviors are explicitly out of bounds

# Canonical Stance

`VelWatch` is an edge client of `veld`.

It is not:

- a second authority runtime
- a place to embed the full Rust core
- a watch-local planner, recall engine, or policy engine
- a compact clone of iPhone, web, or desktop thread management

The watch should be treated as:

- interface
- sensor node
- haptic interrupt surface
- bounded write-ahead action producer

# System Topology

The canonical topology is:

`Vel Core (Rust)` -> `VELD node` -> `iPhone bridge` -> `Apple Watch`

Interpretation:

- `Vel Core (Rust)` owns domain semantics, synthesis inputs, scoring, and durable invariants
- `veld` remains the runtime authority for shared truth, event ingestion, policy, continuity, and artifact generation
- iPhone is the local Apple bridge, cache, transport selector, and reconciliation proxy
- Apple Watch is the body-facing edge surface for capture, nudges, haptics, and compact state display

# Ownership Boundaries

## Rust / `veld` Owns

- event ingestion and normalization
- nudge decision logic
- risk scoring
- thread continuity
- artifact generation
- durable review/apply semantics
- explainability and provenance rules

## iPhone Bridge Owns

- Apple-local auth/session handling
- local cache and offline presentation
- transport selection such as local route, Tailscale, or HTTPS
- reconciliation and queued-action drain
- optional narrow FFI helpers when explicitly approved

## Apple Watch Owns

- glanceable rendering
- quick capture entry
- bounded nudge actions
- local haptic presentation
- bounded local queueing
- sensor/event emission through approved lanes

## Apple Watch Must Not Own

- synthesis
- LLM calls
- heavy policy evaluation
- durable conflict resolution
- broad thread or project management

# Watch Product Role

The watch is not a general-purpose Vel shell.

It is the operator's:

- nervous-system interrupt surface
- quickest capture device
- fastest path to acknowledge or snooze a bounded nudge
- smallest trustworthy window into current state

The watch should feel magical because of timing, haptics, and restraint rather than because it exposes more screens.

# In-Bounds Watch Capabilities

## Input Surface

High-value watch input includes:

- quick capture such as note, task, check-in, or feeling
- voice-to-text capture sent upstream through the same continuity model
- one-tap nudge actions such as `done`, `snooze`, or bounded escalate/handoff
- short append into the active thread when a canonical active thread already exists

## Output Surface

High-value watch output includes:

- haptic nudges and escalation ladders
- glanceable next commitment or next action
- bounded risk or drift state
- compact "what matters now" posture
- explicit handoff cues to iPhone or Mac when deeper work is required

## Sensor Surface

Approved watch-originated signals may include:

- heart rate
- motion/activity
- sleep-related inputs exposed through Apple Health on the phone bridge
- wake or alarm-dismissal proxies when the platform boundary allows them

Those are event-log inputs, not watch-local policy triggers.

# Event-First Communication Model

Prefer an event-driven protocol over watch-specific RPC sprawl.

Watch-to-upstream examples:

- `capture.quick_note`
- `capture.quick_task`
- `capture.feeling`
- `nudge_ack`
- `nudge_snooze`
- `signal.heart_rate`
- `signal.motion`
- `signal.wake`

Upstream-to-watch examples:

- `state.snapshot`
- `nudge.emit`
- `risk.update`
- `next_commitment.update`
- `handoff.prompt`

The watch should consume compact typed snapshots and emit bounded typed events. It should not depend on a wide ad hoc menu of bespoke watch-only command routes.

# Offline And Reconciliation Model

Offline behavior is required and should remain narrow.

## On Watch

Allowed watch-local state:

- last known compact snapshot
- last known commitments or nudge posture needed for glanceability
- local queue of safe operator actions

## On iPhone

The phone is the real offline bridge and reconciliation layer.

It should own:

- write-ahead style action persistence
- queue drain and replay
- merge into canonical `veld` truth once reachable
- transport fallback and route preference

The watch should prefer the phone bridge over talking directly to a remote node when both are available.

# Rust On Watch

The default rule is: do not put Rust core on watchOS.

If Rust ever appears on watchOS, it must stay tiny and deterministic, for example:

- encoding or decoding helpers
- packet validation
- small scoring or formatting helpers

It must not become:

- a second async authority runtime
- a shell-local synthesis engine
- a substitute for `veld`

Treat watch-local Rust as a math or codec coprocessor, not as the brain.

# UX Rules

Watch UX should optimize for:

- one-handed, one-glance use
- haptic-first escalation
- one-tap resolution paths
- immediate capture with minimal friction
- explicit handoff when the task stops being compact

Watch UX should avoid:

- UI clutter
- long lists
- deep thread browsing
- settings-heavy flows
- speculative parity with phone or desktop layouts

If a flow requires reading, comparing, editing, or deciding across multiple items, it should hand off to iPhone or Mac.

# Current Implementation Direction

Current watch work should be described as a reduced edge surface over existing Apple and `veld` contracts:

- compact `Now` snapshot
- active nudge resolution
- quick capture
- active-thread append when a canonical thread is already available
- explicit handoff for deeper continuity work

That is the correct direction. Future work should make it more edge-native and less like a tiny general app shell.

# Acceptance Criteria

1. Apple Watch is documented as an edge client of `veld`, not as an embedded Rust authority runtime.
2. iPhone is documented as the local Apple bridge, cache, and reconciliation proxy.
3. Watch responsibilities are limited to interface, sensors, haptics, compact state, and bounded action emission.
4. Watch-local synthesis, heavy policy logic, and full thread management are explicitly ruled out.
5. Offline behavior is documented as watch-local minimal cache plus phone-owned reconciliation.
