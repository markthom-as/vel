# 13-03 Summary

## Outcome

Completed the future-path documentation slice for Phase 13:

- documented the future Apple embedded-Rust / FFI path without confusing it with current shipped behavior
- documented the future desktop/Tauri runtime-host and adapter path as a shell over shared contracts
- preserved the discovery finding that a post-16 shell embodiment phase should be evaluated instead of letting Phase 16 absorb UI simplification implicitly

This captures the migration decisions from the planning thread as durable repo docs instead of leaving them as remembered context.

## Implementation

### New migration-path docs

- added [apple-rust-integration-path.md](../../../../docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md)
  - states the current Apple truth is HTTP-first through `VelAPI`
  - defines when selective embedded Rust / FFI becomes justified
  - keeps shell-vs-core ownership explicit even in a future embedded mode
- added [desktop-runtime-and-adapter-path.md](../../../../docs/cognitive-agent-architecture/architecture/desktop-runtime-and-adapter-path.md)
  - defines future desktop/Tauri as a shell/adaptor choice
  - compares in-process host versus local-daemon desktop modes
  - prevents desktop packaging from becoming a product-core owner

### Roadmap preservation

- updated [ROADMAP.md](../../../../.planning/ROADMAP.md) so Phase 14 now explicitly notes the discovery follow-on question of a dedicated post-16 shell embodiment and surface-simplification phase

## Verification

Automated:

- `rg -n "Current truth|HTTP|FFI|embedded|daemon|server|Tauri|desktop" docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md docs/cognitive-agent-architecture/architecture/desktop-runtime-and-adapter-path.md .planning/ROADMAP.md`

Manual:

- read both migration-path docs against the current Apple HTTP-first boundary and the current daemon/web/CLI reality to confirm they describe future options without falsely claiming those migrations already happened

## Notes

- the Phase 14 research stream completed in parallel during this slice and recommends keeping Phase 14 doc-first/contract-first and likely adding a dedicated shell embodiment phase after Phase 16
