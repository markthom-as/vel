---
title: Cross-Surface Core And Adapter Boundaries
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - cross-surface
  - architecture
  - apple
  - web
  - desktop
  - adapter
index_terms:
  - cross-surface core
  - adapter boundaries
  - current-state to target-state mapping
  - embedded daemon server topology
related_files:
  - docs/MASTER_PLAN.md
  - docs/api/runtime.md
  - docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md
  - docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md
  - docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md
  - docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md
  - clients/apple/README.md
  - clients/apple/VelAPI/Sources/VelAPI/VelClient.swift
  - crates/vel-core/src/lib.rs
  - crates/vel-api-types/src/lib.rs
  - crates/vel-storage/src/lib.rs
  - crates/veld/src/app.rs
summary: Canonical cross-surface architecture for keeping one Rust-owned product core across Apple, web, CLI, and future desktop shells without forcing a mass crate rewrite.
---

# Purpose

Define how Vel should preserve one Rust-owned product core across current and future shells.

This document is the authority for:

- supported runtime topologies
- shell adapter boundaries
- current-state to target-state ownership
- the distinction between current implementation truth and future migration targets
- cross-surface ownership of the active `v0.2` MVP loop

It is intentionally aligned to the current codebase rather than a greenfield crate diagram.

For the concrete active-loop behavior, read this together with [mvp-loop-contracts.md](./mvp-loop-contracts.md).

# Problem

Vel already has the right directional instinct:

- `veld` is the authority runtime
- Apple and web are clients
- Rust owns durable state and policy

But the repo does not yet have one explicit architecture doc that answers all of these together:

- how Apple, web, CLI, and future desktop shells should consume the same product core
- which parts of the current crate graph are already the right seams
- which future runtime modes are valid
- how to think about a future Apple FFI path without pretending it is current truth

Without that, future work can drift into shell-shaped product logic or unnecessary refactor theater.

# Goals

- keep one canonical Rust-owned behavior layer across shells
- preserve the current `vel-core` -> `vel-storage` -> `vel-api-types` -> `veld` direction rather than replacing it with a speculative rewrite
- make embedded-capable, local-daemon, and hosted/server runtime modes explicit
- define shell adapter boundaries for Apple, web, CLI, and future desktop
- document the current-state to target-state mapping so later migration phases can be incremental

# Non-Goals

- a big-bang crate rename
- immediate Apple FFI or UniFFI migration
- an immediate Tauri product shell
- turning shell adapters into business-logic owners

# Current Truth

## Runtime Authority

Today, `veld` is the authority runtime.

It owns:

- authenticated HTTP surface mounting
- service orchestration
- policy evaluation
- route-to-service boundaries
- access to durable storage through `vel-storage`

Current runtime/API authority is documented in [docs/api/runtime.md](../../api/runtime.md).

## Apple Boundary

Today, Apple clients are HTTP-first clients of the same daemon.

Current truth:

- [clients/apple/README.md](../../../clients/apple/README.md) states that all Apple apps talk to the same `veld` daemon over HTTP
- [VelClient.swift](../../../clients/apple/VelAPI/Sources/VelAPI/VelClient.swift) is the active Apple transport boundary
- Apple presentation, lifecycle, App Intents, widgets, complications, notifications, and platform permission flows remain Swift-owned shell concerns

Apple does **not** currently embed Rust as the primary runtime path.

The new Phase 37 embedded-capable Apple contract is additive and iPhone-first. It does not change the current-truth statement above.

## Web Boundary

Today, web is also an HTTP-first client.

Current truth:

- browser code consumes typed JSON DTOs via `clients/web/src/types.ts`
- browser loaders live under `clients/web/src/data/`
- product logic should not be re-derived in React when Rust already owns it

## CLI Boundary

`vel-cli` is the operator shell over the same Rust-owned product/runtime truth.

It should stay a client of canonical services and transport contracts, not become a parallel product-definition layer.

# Supported Runtime Topologies

Vel should support three valid runtime modes over time.

## 1. Embedded-Capable Mode

Rust functionality is linked directly into the client process.

Best fit:

- selective Apple-native use cases
- narrow local tools
- future targeted embedded flows where a network round-trip is unnecessary and platform constraints justify it

Current status:

- supported as a future architecture target
- not the current Apple implementation model

## 2. Local-Daemon Mode

Rust runs as a separate local authority process and shells talk to it as clients.

Best fit:

- current local Vel usage
- Apple local-network usage
- future desktop packaging
- multi-surface local continuity

Current status:

- this is the current authority model around `veld`

## 3. Hosted / Server Mode

Rust runs as a hosted backend and shells talk to it remotely.

Best fit:

- browser-first access
- future cloud-hosted operation
- future shared or remote access patterns

Current status:

- architecturally valid and already compatible with the HTTP-first boundary
- not yet the central product deployment story

# Canonical Stance

Vel should not pick one runtime topology forever.

It should preserve one product core that can be hosted through:

- embedded-capable adapters where justified
- local-daemon authority by default
- hosted/server authority where product needs expand

That is the portability target.

# Core Architectural Rule

Treat Rust as the application substrate, not as a helper library attached to one frontend.

That means:

- Apple is a client shell
- web is a client shell
- CLI is a client shell
- future desktop is a client shell
- Rust owns durable product semantics, policy, and orchestration truth

# Current-State To Target-State Mapping

## Responsibility Map

| Current Surface / Crate | Current Role | Target Role | Notes |
| --- | --- | --- | --- |
| `crates/vel-core` | domain vocabulary, invariants, core types | canonical product-core domain layer | Already the closest existing equivalent to the desired universal core. Evolve incrementally rather than replacing it for naming purity. |
| `crates/vel-storage` | persistence and repository boundary | durable storage boundary | Keep transport concerns out. Extend focused repositories rather than widening `db.rs` patterns. |
| `crates/vel-api-types` | HTTP transport DTOs | canonical transport DTO seam for networked shells | Continue using it for runtime/API contracts. Future adapter crates may translate from the same core semantics, not replace them. |
| `crates/veld` | authority runtime, routes, services, auth, policy | canonical daemon/server host | Remains the current authority process for web and Apple HTTP flows. |
| `crates/vel-cli` | operator shell | thin shell over canonical runtime/core contracts | Should consume canonical commands, queries, and read models rather than inventing new product truth. |
| `clients/apple/VelAPI` | Swift HTTP client | current Apple transport adapter | Current truth. Future embedded/FFI options should be additive, not retroactively claimed as current. |
| `clients/web/src/types.ts` + `src/data/*` | browser decoder/loader layer | web transport adapter | Keep policy and durable summarization in Rust. |
| future Apple adapter crate | not present | embedded-capable Apple adapter | Only justified when a specific Apple use case benefits materially from embedded Rust over the current HTTP path. |
| future desktop/Tauri adapter | not present | desktop shell adapter | Should consume the same command/query/read-model seams as other shells. |

## Current Shell Alignment

The current repo already demonstrates the intended model in several places:

- daily loop is backend-owned and consumed across CLI/web/Apple
- agent inspect is backend-owned and consumed across CLI/web
- current runtime auth and route classes are centralized in `veld`

The architectural task is therefore refinement and explicitness, not reinvention.

# Adapter Boundary Rules

## What Shells Should Consume

Shells should consume:

- commands that cause product actions or transitions
- queries that return operator-facing read models
- typed transport DTOs at the boundary
- explicit capability summaries and review state where needed
- the active `v0.2` contracts for overview, commitments, reflow, threads, and review

## What Shells Should Not Own

Shells should not own:

- business logic for daily prioritization
- overview or commitment-selection policy
- policy evaluation
- durable review gating
- capability decisions
- integration conflict rules
- ad hoc parallel versions of the same read model semantics

# Apple, Web, CLI, And Future Desktop

## Apple

Current mode:

- HTTP-first via `VelAPI`

Future valid modes:

- continue using HTTP-first against local-daemon or hosted authority
- selectively embed Rust for justified product slices

Shell-owned concerns:

- native presentation
- App Intents
- widgets / Live Activities
- complications
- notifications
- lifecycle and permission glue

Rust-owned concerns:

- product logic
- daily loops
- overview, commitments, reflow, thread escalation, and review semantics
- grounding logic
- policy and review gates
- durable state rules

## Web

Current mode:

- HTTP/JSON client to `veld`

Likely long-term mode:

- same, with local-daemon and hosted authority both valid

Shell-owned concerns:

- browser routing
- dashboard composition
- interaction design

Rust-owned concerns:

- application logic
- read-model generation
- overview, commitments, reflow, thread escalation, and review semantics
- policy and review logic

## CLI

Current mode:

- operator shell against the same runtime-owned truth

CLI should remain:

- a thin operational surface
- useful for inspection, fallback, and scripting

CLI should not become:

- a second architecture
- a separate product-definition path

## Future Desktop / Tauri

Current status:

- not yet implemented

Allowed future modes:

- in-process host over canonical Rust services
- shell over a local daemon

Architecture rule:

- future desktop is a shell choice, not a product-core owner

# Migration Discipline

## Preferred Sequence

1. Define the architecture and ownership rules.
2. Define the product shape and operator modes.
3. Migrate only the seams needed for the next real logic slice.
4. Implement canonical Rust-owned logic.
5. Expand shell embodiment on top of proven seams.

## Refactor Rule

Do not reorganize crates or rename modules unless the move clearly improves one of:

- shell-vs-core ownership
- transport discipline
- portability across runtime modes
- reviewability of future logic slices

If a rename or split is mostly aesthetic, defer it.

# Proof-Oriented Seams

The current repo already contains proof-oriented seams that later phases should study and extend:

- shared daily loop authority
- shared agent inspect authority
- explicit execution-handoff review and launch readiness

These should be treated as evidence that the cross-surface model is workable in the live codebase.

# Relationship To Active Phases

- Phase 40 locks the MVP boundary, contracts, and architecture references.
- Phase 41 implements Rust-owned overview, commitment flow, and orientation behavior on these seams.
- Phase 42 implements explainable same-day reflow on the same seams.
- Phase 43 uses the same authority model for bounded thread continuation.
- Phase 44 rebuilds web and Apple as thin shells over those contracts.
- Phase 45 closes the loop with review and milestone verification.

This order is intentional. UI should not be allowed to define product truth by accident.

# Cross-Cutting Traits

- modularity: required
- configurability: required
- observability: required
- replay/reviewability: required
- composability: required

Cross-surface portability is only valuable if these traits remain intact across runtime modes.

# Acceptance Criteria

1. The repo has one explicit cross-surface architecture authority aligned to the current codebase.
2. Embedded-capable, local-daemon, and hosted/server modes are explicitly named and distinguished.
3. The current-state to target-state mapping is clear enough that later phases do not need to rediscover the same decisions.

# Related Terms

- cross-surface core
- adapter boundary
- runtime topology
- shell boundary
- current-state to target-state mapping
