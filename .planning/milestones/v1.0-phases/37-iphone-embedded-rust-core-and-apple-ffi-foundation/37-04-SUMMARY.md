# 37-04 Summary

## Outcome

Closed Phase 37 by aligning the shipped Apple topology story across runtime, daily-use, product-policy, and Apple architecture docs.

## Shipped

- updated [runtime.md](/home/jove/code/vel/docs/api/runtime.md) to describe the Phase 37 embedded-capable iPhone seam as additive and bounded
- updated [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) so operators do not mistake embedded helper flows for a second authority runtime
- updated [operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) to keep embedded Apple use explicitly subordinate to one backend-owned policy model

## Verification

- `rg -n "embedded-capable|second authority|daemon-backed truth|cached-\\\`Now\\\`|quick-action preparation" docs/api/runtime.md docs/user/daily-use.md docs/product/operator-mode-policy.md clients/apple/README.md docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md`
- `nix-shell --run 'cd /home/jove/code/vel/clients/apple/Packages/VelAppleModules && swift build'`
- `make check-apple-swift`

## Notes

- Phase 37 intentionally stops at the embedded-capable foundation plus bounded local helpers
- the proving local-first voice/offline loop remains the Phase 38 lane
