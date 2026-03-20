# 38-04 Summary

## Outcome

Closed Phase 38 by aligning the shipped Apple/offline docs to the real local-first voice lane: compact continuity in Apple `Now` and `Threads`, one bounded recovery story, and explicit daemon-backed limits.

## Shipped

- updated [runtime.md](/home/jove/code/vel/docs/api/runtime.md) so Apple local-first voice recovery is documented as compact continuity only, not a second offline-only thread model
- updated [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) so operators are told where draft/pending/merged Apple voice continuity should surface
- updated [clients/apple/README.md](/home/jove/code/vel/clients/apple/README.md) so the Apple surface docs describe draft/pending/merged posture and canonical `thread_id` precedence
- updated [operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) so Apple summary-first policy explicitly allows compact local voice recovery cues while keeping backend authority primary

## Verification

- `make check-apple-swift`
- `nix-shell --run 'cd /home/jove/code/vel/clients/apple/Packages/VelAppleModules && swift build'`
- `rg -n "compact continuity|draft/pending/merged|local-first iPhone voice|canonical thread" docs/api/runtime.md docs/user/daily-use.md clients/apple/README.md docs/product/operator-mode-policy.md`

## Limits

- remaining daemon-backed limits are unchanged: backend-only reasoning answers, canonical thread persistence authority, broader shared-history sync, integrations, and heavier recall stay off-phone in this phase
