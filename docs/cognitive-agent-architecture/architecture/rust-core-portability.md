---
title: Rust Core Portability
doc_type: guide
status: complete
owner: staff-eng
created: 2026-03-25
updated: 2026-03-25
keywords:
  - rust
  - portability
  - adapters
  - apple
  - web
  - wasm
summary: Concise helper guide for why Vel keeps a portable Rust-owned core across daemon, Apple, web, and future embedded or WASM adapters.
---

# Purpose

Explain one of Vel's core architectural choices in a short form:

why keep the product core portable and Rust-owned instead of letting each shell own its own logic.

# Canonical Choice

Vel treats Rust as the product substrate and shells as adapters.

This is true across:

- local daemon mode
- Apple HTTP-first clients today
- future selective embedded Apple paths
- web and browser-facing clients
- future WASM or portable packet-core reuse

# Why This Choice Exists

## One Product Truth

Portable Rust reduces logic forks.

It keeps:

- policy
- durable semantics
- review gates
- run lifecycle rules
- connector conflict rules

owned in one place.

## Cross-Surface Consistency

If the same behavior matters on Apple, web, CLI, or future desktop surfaces, it should usually be defined once and adapted outward.

## Incremental Portability

Portability does not require one deployment mode forever.

The same Rust-owned core can support:

- daemon-hosted execution
- narrow embedded slices
- separate browser or WASM adapters

without changing who owns the semantics.

# What Portability Does Not Mean

It does not mean:

- all code must move into one crate
- every shell should embed Rust immediately
- Apple-native concerns should move out of Swift
- browser code should be forced through WASM when HTTP is simpler

Portability is about preserving one core ownership model, not forcing one technical packaging choice everywhere.

# Shell-Owned Versus Rust-Owned

## Shell-Owned

- presentation
- lifecycle and OS hooks
- permissions UI
- push, notification, widget, or App Intent embodiment
- browser-specific interaction and rendering concerns

## Rust-Owned

- domain rules
- durable state meaning
- policy and approval semantics
- orchestration and run logic
- canonical connectors and conflict posture

# Apple And WASM Implications

## Apple

Apple stays HTTP-first by default until a narrow embedded slice is clearly worth it.

Even when embedded Rust is justified:

- Swift still owns platform embodiment
- Rust still owns shared semantics

## WASM

WASM is a runtime packaging or sandbox boundary, not a license to duplicate product logic in JS.

If browser or guest reuse is needed:

- extract deterministic Rust helpers
- add a bounded adapter
- preserve the same ownership rules

# Anti-Patterns

- reimplementing daily-loop or policy logic in Swift for convenience
- putting backend truth into React because the API shape is awkward
- using WASM as a reason to bypass sandbox or review boundaries
- treating “portable” as an excuse for premature abstraction or crate churn

# Agent Heuristic

Before moving logic into a shell, ask:

1. is this actually platform embodiment rather than product semantics
2. does another surface need the same behavior
3. would moving this into the shell create a logic fork
4. can the Rust-owned seam be reused instead
