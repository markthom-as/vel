---
title: Embedded Packet Core And Browser WASM Path
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-25
updated: 2026-03-25
keywords:
  - embedded
  - wasm
  - browser
  - ffi
  - packets
index_terms:
  - browser wasm bridge path
  - embedded packet core
  - apple ffi reuse
related_files:
  - crates/vel-embedded-bridge/src/lib.rs
  - crates/vel-embedded-bridge/src/portable_core.rs
  - crates/vel-embedded-bridge/src/browser_wasm.rs
  - clients/web/src/data/embeddedBridgePackets.ts
  - docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md
  - docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md
summary: Defines the future extraction path from the current Apple-native embedded bridge into a cross-surface portable packet core with a browser/WASM adapter.
---

# Purpose

Document the future path for reusing embedded Rust packet-shaping logic in browser contexts.

Current truth:

- Apple embedded mode is a native iPhone-first FFI path.
- Browser surfaces remain daemon-backed and browser-local for shell concerns such as STT/TTS.
- `vel-embedded-bridge` now contains a growing set of deterministic packet and normalization helpers that are good candidates for cross-surface reuse.

# Current Separation

Today `vel-embedded-bridge` mixes two concerns:

1. portable deterministic transform logic
2. native FFI entrypoints for Apple embedding

That is acceptable for Phase 37 iPhone work, but it is the wrong long-term shape for browser/WASM reuse.

# Canonical Future Shape

`vel-embedded-bridge` should evolve into three explicit layers:

1. `portable_core`
- pure Rust transform and packet-shaping helpers
- no raw pointer ownership
- no Darwin assumptions
- no app-process loading assumptions

2. `native_ffi`
- C-ABI exports for Apple/native embedding
- owns `CString` allocation and free-buffer discipline
- adapts native inputs/outputs to the portable core

3. `browser_wasm`
- JS/WASM-callable exports
- no raw pointer ABI
- adapts browser inputs/outputs to the same portable core

# Portable Candidates

The following flow families are good browser/WASM candidates because they are deterministic local shaping rather than authority-runtime decisions:

- cached `Now` hydration
- offline request packaging
- deterministic domain helpers
- thread draft packaging
- voice capture payload packaging
- voice quick-action packaging
- voice continuity packet shaping
- queued action packaging
- linking settings normalization
- assistant-entry fallback packaging
- linking request packaging
- capture metadata packaging
- voice continuity summary packaging
- voice offline response packaging
- voice cached query packaging
- linking feedback packaging

# Not Browser-WASM Reusable As-Is

The following are not directly reusable in browser form:

- Swift `VelEmbeddedBridge` runtime loading
- Darwin `dlopen` / symbol resolution
- Apple platform gating and packaging assumptions
- any flow that depends on daemon authority rather than deterministic local shaping

# Ownership Rules

Browser/WASM reuse does not change ownership:

- Rust still owns packet vocabulary and deterministic transform logic.
- browser code still owns shell concerns such as gesture handling, local STT/TTS, and rendering.
- `veld` still owns authority-runtime decisions, shared continuity, review/apply lanes, and heavy recall.

# Checked-In Scaffold

The current repository scaffold for this future path is:

- [portable_core.rs](/home/jove/code/vel/crates/vel-embedded-bridge/src/portable_core.rs)
- [browser_wasm.rs](/home/jove/code/vel/crates/vel-embedded-bridge/src/browser_wasm.rs)
- [embeddedBridgePackets.ts](/home/jove/code/vel/clients/web/src/data/embeddedBridgePackets.ts)

Current truth of that scaffold:

- `portable_core.rs` is the extraction point for pure reusable helpers.
- `browser_wasm.rs` now exposes scaffold-level browser packet helpers for a first low-risk set:
  - pairing-token normalization
  - domain-hint normalization
  - queued-action packet shaping
  - voice quick-action packet shaping
  - assistant-entry fallback packet shaping
  - capture-metadata packet shaping
- `embeddedBridgePackets.ts` mirrors that packet vocabulary on the web side as a non-live adapter scaffold inside the browser data layer.
- those helpers are still scaffold-only and are not yet wired into the shipped web client as a live runtime path.

# Next Implementation Sequence

1. continue moving deterministic helpers from `lib.rs` into `portable_core.rs`
2. add a browser-oriented export layer that returns JS-friendly typed packets
3. keep Apple native FFI as a thin adapter over the same portable core
4. add web-side adapter code only for flows that clearly benefit from local reuse
5. avoid widening browser-local logic into authority-runtime policy

# Acceptance Criteria

1. The repository has an explicit markdown authority for the browser/WASM extraction path.
2. `vel-embedded-bridge` contains a checked-in portable core scaffold rather than only Apple-native FFI code.
3. The future browser path is documented as adapter reuse, not as browser ownership of product logic.
