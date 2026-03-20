# Phase 37 Context

## Title

iPhone embedded Rust core and Apple FFI foundation

## Why this phase exists

Phase 13 deliberately documented the future Apple embedded/FFI path without claiming it was already shipped. Since then, more and more product behavior has moved into Rust-owned seams, but Apple remains an HTTP-first shell over `veld`.

The operator still wants the real embedded-core path for:

- universal backend/API architecture with platform-gated local use
- better offline behavior
- cleaner architecture
- better future distribution posture
- stronger job handoff potential

## Product problem

The product brain is now far more unified in Rust, but the Apple process boundary is still remote-first:

- `VelAPI` is still the active Apple transport boundary
- daemon-backed truth remains the only real authority path
- local-first Apple flows remain constrained by that boundary

That makes Apple architecture cleaner than before, but not yet embedded-capable.

## Phase goal

Add the real iPhone embedded Rust path behind explicit feature and platform gates while preserving one canonical Rust-owned behavior model and keeping heavier authority lanes daemon-backed for now.

## Must stay true

- iPhone first
- no premature watch/mac split
- embedded path is additive, not a rewrite of current truth
- one product model across daemon-backed and embedded-capable paths
- heavy recall, jobs, integrations, sync, and supervised review/apply remain daemon-backed in this phase

## Architectural intent

- follow the Phase 13 topology guidance instead of inventing a new Apple-specific architecture
- use the embedded seam only where it materially improves local responsiveness and offline resilience
- avoid a second Apple-local policy brain

## Likely touch points

- `clients/apple/VelAPI`
- Apple app targets
- Rust FFI bridge / adapter work
- `vel-core`
- selected `veld` seams that need shared extraction or adapter exposure
- Apple architecture docs and runtime docs

## Expected next step

Phase 37 planning should break this into:

1. embedded-core / FFI contract publication
2. first iPhone-safe bridge and feature-gating seam
3. selective local execution for high-value flows
4. docs/examples/verification for the new Apple topology
