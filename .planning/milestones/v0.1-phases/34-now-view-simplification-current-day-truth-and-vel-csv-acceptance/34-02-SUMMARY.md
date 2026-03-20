# 34-02 Summary

## Outcome

Repaired the shipped `Now` truth model so current status and next-event rendering are compact and current-day oriented instead of dashboard-like.

## What Changed

- rewrote [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) around explicit current-status derivation, current/next event separation, active routine-block visibility, and compact event summaries
- kept `next event` calendar-authoritative and routine-noise-free on the primary surface
- reused the existing backend `GET /v1/now` schedule substrate instead of inventing shell-local scheduling heuristics
- added targeted render coverage in [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) for current status, next event, and degraded freshness interaction paths

## Verification

- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/types.test.ts`

## Notes

- no backend schedule DTO widening was required in this slice; the existing `GET /v1/now` contract was sufficient once the shell stopped duplicating status blocks
