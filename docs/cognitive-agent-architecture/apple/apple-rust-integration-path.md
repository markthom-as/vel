---
title: Apple Rust Integration Path
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - apple
  - rust
  - ffi
  - uniffi
  - embedding
index_terms:
  - apple rust integration
  - apple ffi path
  - current truth
  - future embedded mode
related_files:
  - clients/apple/README.md
  - clients/apple/VelAPI/Sources/VelAPI/VelClient.swift
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md
  - docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md
summary: Current Apple integration truth and the future migration path from today's HTTP-first model to optional embedded Rust / FFI usage.
---

# Purpose

Document how Apple clients should relate to the Rust core over time.

This document distinguishes:

- what Apple does today
- what future embedded Rust paths are valid
- what must remain shell-owned even if Apple later embeds Rust

# Current Truth

Today, Apple clients are HTTP-first clients of the same `veld` daemon.

Current truth is defined by:

- [clients/apple/README.md](../../../clients/apple/README.md)
- [VelClient.swift](../../../clients/apple/VelAPI/Sources/VelAPI/VelClient.swift)

Today Apple uses:

- SwiftUI and Apple-native frameworks for presentation and lifecycle
- `VelAPI` as the shared HTTP transport adapter
- backend-owned runtime contracts for daily loop, `Now`, Apple voice, behavior summary, and related flows

Today Apple does **not** use:

- embedded Rust as the primary runtime path
- FFI/UniFFI as the main product boundary
- Swift-owned business logic for daily planning or review policy

# Why Current HTTP-First Is Correct Today

The current Apple boundary is correct because it:

- keeps one authority runtime in `veld`
- matches the web and CLI contract model
- avoids mobile build and runtime complexity while product semantics are still evolving
- keeps Apple-native work focused on embodiment, not product-core ownership

# Future Valid Apple Modes

Apple may eventually support two valid modes.

## Mode A: Continue HTTP-First

Use SwiftUI plus `VelAPI` against local-daemon or hosted `veld`.

This remains valid when:

- the feature needs shared authority
- the feature benefits from one backend-owned policy engine
- the same contract already serves web or CLI well

## Mode B: Selective Embedded Rust / FFI

Use a narrow Rust adapter embedded into the Apple app process.

This becomes justified only when a concrete feature benefits materially from:

- lower latency than the current daemon path can provide
- reliable offline local computation that should not depend on reconnecting to `veld`
- platform-specific packaging where a network boundary is the main source of friction

Even in this mode, Apple should embed only narrow, well-chosen product-core seams rather than attempting to move the whole authority runtime into Swift app processes.

# FFI / Embedded Migration Rules

## What May Move Behind FFI

Future embedded Rust may own:

- narrowly scoped domain or application services
- local read-model generation for explicitly approved offline-first slices
- deterministic business logic that benefits from being shared without HTTP overhead

## What Must Not Move Into Apple Shell Ownership

Even if Apple adopts FFI, Swift should not become the owner of:

- policy rules
- review-gate semantics
- durable state invariants
- shell-independent product logic
- connector conflict rules

FFI is an integration boundary choice, not permission for shell-owned product semantics.

# Suggested Migration Sequence

1. Keep HTTP-first as the default Apple mode while Phase 13-16 architecture, discovery, migration, and logic work stabilize.
2. Identify one concrete Apple feature that suffers materially from the HTTP boundary.
3. Introduce one narrow Apple adapter seam for that feature only.
4. Prove that the embedded path still preserves Rust-owned semantics and does not fork product logic.
5. Expand only if the benefits are clear and repeatable.

# Anti-Patterns

## Premature Full Migration

Bad:

- replacing `VelAPI` broadly before product contracts and read models stabilize

## Swift-Owned Logic Fork

Bad:

- reimplementing daily loop, trust logic, or review gates in Swift for convenience

## FFI As A Metaphysical Goal

Bad:

- embedding Rust because it feels more “pure” rather than because a concrete slice needs it

# Acceptance Criteria

1. Current Apple HTTP-first truth is clearly documented.
2. Future embedded/FFI use is allowed but bounded and conditional.
3. Shell-vs-core ownership stays explicit even in the embedded future path.
4. Phase 37 contract publication is discoverable through the embedded runtime contract plus checked-in schema/example assets.
