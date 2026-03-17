---
title: Vel Modular Cross-Platform Architecture Convergence Spec
status: proposed
owner: architecture / runtime / clients
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

This spec defines the architecture convergence rules Vel should use for near-term work.

Its purpose is not to describe a distant ideal system. Its purpose is to make the current codebase more modular, more cross-platform coherent, less repetitive, and easier to evolve in parallel.

# Why This Spec Exists

Vel now has enough real code in:

- `vel-core`
- `vel-storage`
- `vel-api-types`
- `veld`
- `vel-cli`
- `clients/web`
- `clients/apple`

that architecture quality is no longer mostly about new feature ideas.

It is now about:

- keeping shared logic shared
- keeping platform clients thin
- avoiding duplicate shaping of the same state
- splitting oversized files before they become permanent choke points
- making cross-platform UX consistent without pretending Swift and React should share literal widget code

# Core Principle

Vel should converge on:

> shared semantics, shared contracts, shared read-model vocabulary, and platform-specific presentation

not:

> duplicated platform logic or fake universal UI code reuse

# Architecture Goals

## 1. Modularity

Each major code root should own one kind of concern:

- `vel-core`: domain semantics
- `vel-storage`: persistence
- `vel-api-types`: transport DTOs
- `veld`: runtime orchestration and read-model assembly
- `vel-cli`: operator shell
- `clients/web`: web presentation and web-local state
- `clients/apple`: Apple presentation and Apple-local state
- `docs/`: truth, contracts, and planning

## 2. Cross-platform sharing

Cross-platform sharing should happen at these layers:

- domain types and rules
- transport contracts
- sync/bootstrap semantics
- freshness/degraded-state vocabulary
- UX semantics and operator terminology

Cross-platform sharing should not require:

- shared React/Swift widget code
- duplicate local policy engines
- separate client-owned truth models

## 3. DRYness

The system should avoid duplicating:

- context/risk/policy logic across clients
- transport decoding semantics across clients
- read-model naming and freshness rules
- route-level shaping that belongs in services
- service-level persistence logic that belongs in storage

## 4. Performance

Performance work should begin with structural performance:

- smaller modules
- less repeated shaping of the same data
- fewer giant files that force broad rebuild and review surfaces
- clearer read-model ownership
- consistent caching/bootstrap contracts for clients

Micro-optimization without structural cleanup should not be the default.

# Layer Model

## Layer 1: domain semantics

Owned by `vel-core`.

Responsibilities:

- stable domain types
- invariant-preserving transitions
- shared vocabulary
- semantics that must not drift between daemon, CLI, web, and Apple

Rule:

- if a concept changes meaning across platforms, that is an architecture bug unless explicitly justified.

## Layer 2: persistence

Owned by `vel-storage`.

Responsibilities:

- schema-backed persistence
- query/update helpers
- mapping between durable storage and domain types

Rule:

- persistence families should be modularized by concern, but still present one stable `Storage` facade outward.

## Layer 3: transport contracts

Owned by `vel-api-types`.

Responsibilities:

- DTO families
- API request/response shapes
- shared bootstrap/sync/read-model contracts

Rule:

- Apple and web should consume the same contract semantics even when their local decoding code differs.

## Layer 4: runtime orchestration and read-model assembly

Owned by `veld`.

Responsibilities:

- service orchestration
- read-model assembly
- bootstrap/sync surfaces
- worker/runtime control
- evaluate/inference/suggestion/read flows

Rules:

- route handlers stay thin
- service modules own shaping/orchestration
- read models should be assembled once in the daemon when they are shared across clients

## Layer 5: platform shared adapters

Owned by platform-local shared packages such as:

- `clients/apple/VelAPI`
- web transport/data modules

Responsibilities:

- consume daemon contracts
- manage local cache/queue/client state
- present one platform-consistent adapter layer for app surfaces

Rules:

- this layer should not fork domain semantics
- this layer should stay thinner than the daemon service layer

## Layer 6: platform-specific surfaces

Owned by:

- `clients/web/src/components`
- Apple app targets under `clients/apple/Apps`
- `vel-cli` command families

Responsibilities:

- presentation
- interaction flow
- surface-local navigation
- operator-facing rendering

Rules:

- these surfaces may differ in layout and interaction style
- they should not differ in the underlying meaning of state

## Layer 7: docs and truth surfaces

Owned by `docs/`.

Responsibilities:

- current truth
- execution entrypoints
- architecture constraints
- maintenance protocol

# Cross-Platform Sharing Rules

## Rule 1: share semantics before code

When web and Apple need the same behavior, first ask:

- is this domain semantics?
- is this transport contract?
- is this sync/bootstrap contract?
- is this UX state vocabulary?

If yes, share it there before building platform-local wrappers.

## Rule 2: share UX through state classes and terminology

Cross-platform UX consistency should come from shared meanings such as:

- `fresh`
- `aging`
- `stale`
- `error`
- `disconnected`
- `pending`
- `replay_failed`

not from trying to make SwiftUI and React render the same components.

## Rule 3: platforms may differ in embodiment, not truth

Examples:

- Apple Watch may be brief
- web may be richer
- CLI may be dense

But all must refer to the same underlying state and action semantics.

# Modularity Rules

## Rule 1: decompose by ownership, not by vibes

Split files and modules by:

- route family
- service family
- persistence family
- DTO family
- surface family

Do not split code into many tiny files without a clear ownership boundary.

## Rule 2: one oversized file is usually a signal, not an accident

If a file becomes the place where unrelated work collides, that is a modularity failure even if the code still “works.”

## Rule 3: shared code should have one obvious home

If the same logic is wanted in multiple clients, it should usually live in one of:

- `vel-core`
- `vel-api-types`
- `veld` read-model services
- platform shared adapter layer

# DRYness Rules

## Rule 1: no duplicate policy engines in clients

Clients may cache and queue, but they must not reimplement:

- context computation
- risk scoring
- nudge creation/escalation
- suggestion policy

## Rule 2: no duplicate read-model naming

The same concept should not have different names in:

- daemon DTOs
- web decoders
- Apple shared models
- docs

unless there is a strong compatibility reason.

## Rule 3: docs should consolidate, not multiply

New planning docs should either:

- become a canonical execution/doc entrypoint
- or clearly declare themselves source-only

# Performance Rules

## Rule 1: move repeated shaping upstream

If multiple clients need the same shaped state, prefer shaping it once in daemon services and sharing the contract.

## Rule 2: optimize module boundaries before micro-optimizing code paths

Common wins:

- split giant files
- reduce repeated serialization/decode logic
- centralize bootstrap/read-model contracts
- give each client one stable data adapter layer

## Rule 3: performance must not destroy inspectability

Vel is an operator-oriented system. Faster code that makes it harder to explain or inspect is not automatically better.

# Near-Term Convergence Targets

The most important near-term convergence targets are:

1. split `vel-storage` by persistence family
2. split `vel-api-types` by DTO family
3. split `veld` router and major service hotspots by family
4. normalize one shared bootstrap/sync contract used by web and Apple
5. split web transport/state/surface hotspots
6. split `VelAPI` and Apple shared client code by family
7. keep docs/status/indexes aligned with those seams

# Relationship To Flat Execution Packs

This spec is the architecture lens for:

- `runtime-core-storage/`
- `daemon-api-runtime/`
- `cli-operator-shell/`
- `web-operator-runtime/`
- `apple-client-bootstrap/`
- `docs-truth-and-planning/`

Those packs remain the execution shape.

This spec defines how they should converge.

# Acceptance Criteria

This spec is meaningfully satisfied when:

1. shared domain and transport contracts are the default way cross-platform behavior is aligned
2. clients remain thin over shared daemon semantics
3. major file hotspots are reduced through ownership-based decomposition
4. shared UX state vocabulary is explicit across docs and surfaces
5. parallel work can proceed through disjoint write scopes without recreating duplicate logic
