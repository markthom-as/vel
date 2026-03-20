# 37-01 Summary

## Outcome

Published the first real Phase 37 contract for the additive iPhone embedded Rust / Apple FFI path.

## Shipped

- added the owner doc [apple-embedded-runtime-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md)
- added the checked-in machine-readable assets:
  - [apple-embedded-runtime-profile.schema.json](/home/jove/code/vel/config/schemas/apple-embedded-runtime-profile.schema.json)
  - [apple-embedded-runtime-profile.example.json](/home/jove/code/vel/config/examples/apple-embedded-runtime-profile.example.json)
- updated [contracts-manifest.json](/home/jove/code/vel/config/contracts-manifest.json) and [config/README.md](/home/jove/code/vel/config/README.md) so the new boundary is discoverable
- aligned [cross-surface-core-and-adapters.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md), [apple-rust-integration-path.md](/home/jove/code/vel/docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md), and [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md) to the same current-truth-versus-embedded-target language

## Verification

- `node -e "JSON.parse(require('fs').readFileSync('config/examples/apple-embedded-runtime-profile.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/apple-embedded-runtime-profile.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`
- `rg -n "embedded-capable|iphone-first|daemon-backed|feature gate|VelAPI|current truth" docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md clients/apple/README.md`

## Notes

- this slice is contract-first only; it does not add the bridge or local execution path yet
- daemon-backed Apple remains the current truth after this slice
