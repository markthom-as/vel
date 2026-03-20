# 38-01 Summary

## Outcome

Published the Phase 38 contract for local-first iPhone voice continuity and offline-safe action behavior.

## Shipped

- added the owner doc [apple-local-first-voice-continuity-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/apple/apple-local-first-voice-continuity-contract.md)
- added the machine-readable assets:
  - [apple-local-voice-continuity.schema.json](/home/jove/code/vel/config/schemas/apple-local-voice-continuity.schema.json)
  - [apple-local-voice-continuity.example.json](/home/jove/code/vel/config/examples/apple-local-voice-continuity.example.json)
- updated [contracts-manifest.json](/home/jove/code/vel/config/contracts-manifest.json) and [config/README.md](/home/jove/code/vel/config/README.md) so the new contract is discoverable
- aligned [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md), [runtime.md](/home/jove/code/vel/docs/api/runtime.md), and [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) to the same local-first voice/offline baseline

## Verification

- `node -e "JSON.parse(require('fs').readFileSync('config/examples/apple-local-voice-continuity.example.json','utf8')); JSON.parse(require('fs').readFileSync('config/schemas/apple-local-voice-continuity.schema.json','utf8')); JSON.parse(require('fs').readFileSync('config/contracts-manifest.json','utf8')); console.log('ok')"`
- `rg -n "local-first iPhone voice|queued voice capture|local thread drafts|daemon-backed|embedded-capable" docs/cognitive-agent-architecture/apple/apple-local-first-voice-continuity-contract.md docs/api/runtime.md docs/user/daily-use.md clients/apple/README.md`

## Notes

- this slice is contract-first only; queue/draft implementation changes remain Phase `38-02` and `38-03`
