---
title: Apple Embedded Runtime Contract
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-20
updated: 2026-03-20
keywords:
  - apple
  - embedded
  - ffi
  - iphone
  - offline
index_terms:
  - apple embedded runtime
  - iphone ffi contract
  - daemon vs embedded boundary
related_files:
  - docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md
  - docs/cognitive-agent-architecture/apple/apple-watch-edge-client-contract.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - clients/apple/README.md
  - clients/apple/VelAPI/Package.swift
  - config/schemas/apple-embedded-runtime-profile.schema.json
  - config/examples/apple-embedded-runtime-profile.example.json
summary: Canonical contract for the additive iPhone embedded Rust / Apple FFI path, including feature gates and the daemon-versus-embedded flow boundary.
---

# Purpose

Define the first real contract for Apple embedded Rust in Vel.

This document is the authority for:

- what “embedded-capable Apple” means in shipped Phase 37 language
- which flows may move behind an iPhone-local Rust bridge first
- which flows remain daemon-backed even when the embedded seam exists
- how feature and platform gates should fail closed

# Current Truth

Today, Apple remains HTTP-first against `veld`.

That means:

- `VelAPI` is still the active Apple transport boundary
- `veld` remains the daemon/server authority for shared runtime truth
- Swift owns shell concerns such as presentation, permissions, voice UX, local queue UX, notifications, and lifecycle integration

Phase 37 does not replace that truth. It adds an embedded-capable path that can be enabled selectively on iPhone.

# Canonical Stance

The Apple embedded path is:

- additive
- iPhone-first
- feature-gated
- bounded to explicit high-frequency flows
- required to preserve the same Rust-owned domain and policy semantics as the daemon-backed path

It is not:

- a full Apple-local authority runtime
- permission for Swift-owned planner or policy logic
- a claim that watchOS or macOS should embed Rust in this phase

The canonical watchOS stance now lives in [apple-watch-edge-client-contract.md](apple-watch-edge-client-contract.md): watch is an edge client and sensor/haptic surface over `veld`, with iPhone as the local bridge.

# Runtime Modes

Apple should now be described as supporting two valid modes.

## 1. Daemon-Backed Mode

Apple talks to `veld` over HTTP through `VelAPI`.

This remains the default and current-truth path for:

- shared continuity and sync
- heavy recall and recomputation
- connector/integration work
- supervised review/apply lanes
- long-running jobs

## 2. Embedded-Capable Mode

iPhone may link a narrow Rust adapter and use it for explicitly approved local flows.

This is justified only when it materially improves:

- local responsiveness
- bounded offline behavior
- packaging/distribution posture
- handoff into daemon-backed truth later

# Platform Gate

Phase 37 embedded work is limited to:

- iPhone app targets only

Phase 37 excludes:

- watch-first embedded execution
- macOS-first embedded execution
- shell-wide parity claims for every Apple surface

# Feature-Gate Rules

Embedded features must fail closed.

If any of these are missing, the shell should continue using the daemon-backed path:

- embedded bridge available in the build
- platform matches the approved embedded target
- feature gate enabled
- local flow is explicitly listed as embedded-safe

The shell must not silently invent a local replacement for a daemon-backed flow just because the daemon is unreachable.

# Embedded-Safe Phase 37 Flows

Phase 37 may use the embedded seam for narrow, high-frequency flows such as:

- cached `Now` read-model hydration helpers
- bounded local quick-action preparation
- local queue shaping and offline-safe request packaging
- deterministic domain helpers that are already Rust-owned and do not depend on heavy remote authority

The embedded seam should prefer read-model and action-preparation helpers over broad orchestration.

# Daemon-Backed Flows In Phase 37

The following remain daemon-backed even when the embedded bridge exists:

- heavy recall and semantic retrieval
- integration and sync infrastructure
- long-running jobs and recomputation
- shared thread-history sync
- supervised apply/review lanes
- global policy decisions that require authority context broader than the local device slice

# Ownership Rules

Rust remains the owner of:

- domain vocabulary
- invariants
- bounded product logic used by embedded-capable flows
- typed bridge inputs and outputs

Swift remains the owner of:

- UI presentation and navigation
- permissions and local platform APIs
- push-to-talk and speech/TTS shell behavior
- local cache display and user feedback
- deciding when to call an approved embedded seam versus the daemon-backed seam

Swift must not become the owner of:

- planner semantics
- review-gate policy
- durable invariants
- sync/conflict policy

# Bridge Shape

The first bridge should be:

- narrow
- typed
- testable without full app builds
- able to coexist with `VelAPI`

The bridge should expose explicit command/query helpers rather than a generic “run arbitrary Vel core logic” surface.

# Required Checked-In Artifact

The machine-readable contract for this boundary is:

- [apple-embedded-runtime-profile.schema.json](/home/jove/code/vel/config/schemas/apple-embedded-runtime-profile.schema.json)
- [apple-embedded-runtime-profile.example.json](/home/jove/code/vel/config/examples/apple-embedded-runtime-profile.example.json)

Those assets define:

- current truth mode
- embedded target
- feature gates
- embedded-safe flows
- daemon-backed flows

# Acceptance Criteria

1. Apple embedded mode is documented as additive rather than replacing daemon truth.
2. iPhone is the only approved embedded target in Phase 37.
3. Feature-gate and platform-gate rules are explicit and fail closed.
4. Embedded-safe versus daemon-backed flows are clearly separated.
5. The contract is discoverable through checked-in schema/example assets and config manifest references.
