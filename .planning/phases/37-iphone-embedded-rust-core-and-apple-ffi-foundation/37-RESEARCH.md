# 37 Research

## Goal

Introduce the real iPhone embedded Rust / Apple FFI path as an additive topology without pretending the current HTTP-first daemon model has already been replaced.

## Inputs

- Phase arc and operator interview decisions captured in [37-CONTEXT.md](/home/jove/code/vel/.planning/phases/37-iphone-embedded-rust-core-and-apple-ffi-foundation/37-CONTEXT.md)
- current Apple topology docs in [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md)
- cross-surface topology authority in [cross-surface-core-and-adapters.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md)
- Apple-specific migration rules in [apple-rust-integration-path.md](/home/jove/code/vel/docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md)
- current Swift transport seam in [VelClient.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/VelClient.swift)

## Key Findings

- the repo already has the right architectural language for an embedded-capable Apple path, but it is still only prose; there is no typed contract or feature-gating artifact yet
- iPhone should be the only first-class embedded target in this phase; watch and Mac can continue consuming daemon-backed/cached read models for now
- the highest-value early embedded seam is not full authority migration; it is narrow local execution for bounded high-frequency flows that benefit from offline resilience and lower latency
- the Apple shell should keep Swift-owned presentation, permissions, push-to-talk, and local queue UX while Rust remains the owner of domain and policy semantics
- heavy recall, integrations, long-running jobs, shared sync, and review/apply lanes must remain daemon-backed in Phase 37 or the scope will explode

## Risks

- if Phase 37 starts by moving product logic into Swift, the architecture goal is missed before the bridge exists
- if the embedded boundary is undocumented, later FFI slices will fork around ad hoc assumptions about which flows are local-safe
- if Phase 37 claims full offline parity too early, Phase 38 loses its proving-flow focus and trust will regress

## Recommended Shape

1. publish a typed embedded-runtime contract, feature-gating rules, and daemon-vs-embedded boundary map
2. stand up the first bridge seam and Apple package layout for iPhone-only embedded use
3. route a small set of bounded local flows through that seam without duplicating domain logic in Swift
4. close docs and verification with explicit honesty about what remains daemon-backed
