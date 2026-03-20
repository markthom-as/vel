# 37 Validation

## What must be proven

- iPhone gains an explicit embedded-capable Rust path behind documented feature/platform gates
- daemon-backed Apple behavior remains the current truth while the embedded seam is additive
- one Rust-owned domain/policy model is preserved across embedded and daemon-backed modes
- the first embedded-capable boundary is typed, documented, and testable without overclaiming local parity

## Evidence expectations

- owner documentation for the embedded runtime contract and Apple boundary map
- checked-in machine-readable schema and example for the embedded-capable runtime profile
- contract-manifest/config map updates so the new boundary is discoverable
- verification that the checked-in JSON assets still parse
- focused documentation parity checks across Apple/runtime/architecture entrypoints

## Out of scope for this phase

- watch-first or Mac-first embedded rollout
- full local heavy recall or long-running reasoning
- local replacement of connector sync, shared thread sync, or supervised apply/review
- shipping a second Apple-specific policy engine
