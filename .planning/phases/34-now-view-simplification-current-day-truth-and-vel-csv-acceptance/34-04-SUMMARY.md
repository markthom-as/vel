# 34-04 Summary

## Outcome

Closed Phase 34 with a materially calmer `Now`, secondary controls for freshness/debug/planning posture, and docs aligned to the shipped current-day model.

## What Changed

- removed duplicated default panels and demoted freshness, sync, trust, planning summaries, and debug fields behind a single secondary `More context and controls` seam in [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx)
- restored bounded day-plan, reflow, planning-profile, same-day scheduling, and raw debug visibility as secondary surfaces instead of deleting them
- updated focused `NowView` tests to validate the new low-noise contract rather than the previous dashboard layout
- aligned the user/runtime/product docs in [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md), [runtime.md](/home/jove/code/vel/docs/api/runtime.md), and [operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md)

## Verification

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/types.test.ts`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `rg -n "current-day orientation|execution-first|secondary controls|compact context bar|unified today lane" docs/user/daily-use.md docs/api/runtime.md docs/product/operator-mode-policy.md`

## Notes

- this closeout is web-first; it does not yet implement the sleep-relative day-boundary model from Phase 35
- Rust test builds still emit the existing unused/dead-code warnings in `veld`
