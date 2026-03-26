---
title: Language-Specific Agentic Coding Guidance
doc_type: guide
status: complete
owner: staff-eng
created: 2026-03-25
updated: 2026-03-25
keywords:
  - agents
  - coding
  - rust
  - typescript
  - javascript
  - wasm
summary: Durable best practices for agentic coding work in Vel across Rust, JS/TS, and WASM-adjacent runtime boundaries.
---

# Purpose

Define durable best practices for coding agents working in Vel across the main implementation languages and runtime seams:

- Rust backend and storage layers
- JavaScript and TypeScript client surfaces
- WASM-adjacent execution and sandbox boundaries

This document is implementation guidance, not shipped-behavior authority.

# Shared Rules Across Languages

- preserve the documented ownership split instead of copying nearby drift
- keep contracts explicit and typed before widening implementation
- prefer extending existing seams over inventing parallel abstractions
- keep read scope broader than write scope for any repo-aware or agent-aware behavior
- fail closed on unsupported actions, routes, capabilities, or external requests
- add or extend focused tests for changed behavior
- verify by execution, not by reasoning alone

# Rust Guidance

Rust is the authority layer for policy, durable state rules, typed contracts, and most application logic.

## What To Prefer

- put domain semantics in `vel-core`
- keep transport DTOs in `vel-api-types`
- keep repository and persistence logic in `vel-storage`
- keep route handlers thin in `veld`
- keep application logic in services, not route handlers
- use typed structs and enums instead of growing JSON blobs
- prefer `serde_json::Value` over raw JSON strings when structured payloads are needed
- let errors propagate to the correct boundary rather than swallowing them mid-stack
- extend focused repository modules before adding more behavior back into `db.rs`

## What To Avoid

- do not return HTTP DTOs from services
- do not push API-layer concerns into storage or core
- do not normalize current route or storage drift into new work
- do not add new large scenario tests to `crates/veld/src/app.rs`
- do not add speculative abstractions with one implementation and no clear boundary payoff

## Testing And Verification

- run the narrowest affected crate or module tests first
- prefer focused integration tests under `crates/veld/tests/` for route and service seams
- add migration or repository tests when changing storage contracts
- manually exercise CLI or HTTP paths when changing operator-facing behavior

## Rust-Specific Review Questions

- is the boundary between core, storage, services, and DTOs still clean
- did any new JSON blob or stringly-typed contract sneak in
- are run events, logs, or terminal states still persisted when required
- did the change add a route-only truth path that should live in Rust services instead

# JavaScript And TypeScript Guidance

JS/TS in Vel is primarily for shell embodiment, view composition, and operator ergonomics. It should not become the hidden authority for policy or durable state rules.

## What To Prefer

- keep the web client as a thin shell over Rust-owned truth
- consume typed backend contracts rather than inventing local shadow schemas
- preserve the existing surface model and subtree layout under `core/`, `shell/`, and `views/`
- centralize repeated presentation logic in shared view models or primitives
- use existing semantic and display helpers when they exist instead of duplicating feature-local rules
- keep state transitions explicit and inspectable

## What To Avoid

- do not add UI-only truth for settings, `Now`, `Threads`, or `System` behavior when the backend should own it
- do not spread semantic display decisions across many components when a shared presentation seam is available
- do not add ad hoc fetch shapes that drift from backend DTOs
- do not use the frontend to silently widen permissions or action scope

## React And Frontend Discipline

- preserve the established visual language when modifying existing surfaces
- prefer modern React patterns already accepted by the repo
- do not add memoization boilerplate by default; follow the repo's existing compiler and style guidance
- keep mobile and desktop behavior intentional, not accidental
- verify browser behavior directly when changing interaction-heavy surfaces

## JS/TS Review Questions

- is this view rendering backend truth or inventing local truth
- should this repeated logic move into a shared primitive or view-model seam
- did the change preserve the canonical surface boundaries
- was the behavior actually exercised in the browser or only reasoned about

# WASM And Sandbox Guidance

WASM in Vel is a supervised runtime boundary, not a privileged escape hatch.

## What To Prefer

- keep WASM guests behind explicit manifests, allowlists, and host-call mediation
- declare writable roots up front and keep them narrow
- keep host ABI calls explicit, typed, and reviewable
- reuse the existing sandbox and connect boundaries instead of inventing side channels
- preserve trace linkage, denial behavior, and terminal state recording

## What To Avoid

- do not allow guests to widen filesystem scope after launch
- do not allow ambient network access
- do not treat guest execution as equivalent to trusted native backend code
- do not bypass broker or policy checks through compatibility shortcuts

## WASM Review Questions

- are writable roots explicit and bounded
- can the guest widen permissions after launch
- are denials visible and trace-linked
- is the guest path reusing the same governed execution seam as other runtimes

# Agentic Coding Checklist

Before closing a language-specific change, check:

1. the code lives in the correct layer for that language and boundary
2. contracts stayed typed and explicit
3. no new hidden authority path was introduced
4. verification was execution-backed
5. the resulting behavior remains explainable from persisted inputs, rules, and run evidence
