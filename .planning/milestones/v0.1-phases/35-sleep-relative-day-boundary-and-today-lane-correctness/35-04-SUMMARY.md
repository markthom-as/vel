# 35-04 Summary

## Outcome

Phase 35 is now closed. The shipped runtime, user, product, and Apple shell docs now describe one consistent current-day model: `Now` is sleep-relative rather than midnight-bound, `next_event` is future-facing and relevance-filtered, the today lane is commitment-first, and thread resurfacing stays bounded.

The main closeout slice landed in:

- `docs/api/runtime.md`
- `docs/user/daily-use.md`
- `docs/product/operator-mode-policy.md`
- `clients/apple/README.md`

## What changed

- Aligned runtime docs to the shipped backend truth:
  - the current day extends past midnight until the rollover boundary is crossed
  - `next_event` excludes routine/noise rows and means the next future relevant calendar event
  - `next_commitment` / `other_open` remain the in-play commitment lane, while `todoist` remains pullable backlog
- Aligned daily-use docs to the operator acceptance model:
  - `Now` is execution-first and low-noise
  - post-midnight unfinished work may still belong to the same day-between-sleeps session
  - thread resurfacing stays zero-or-one rather than widening into thread clutter
- Aligned product policy and Apple guidance so future Apple work inherits the same current-day and bounded-thread rules instead of reintroducing midnight or chat-first local behavior.
- Recorded the closeout against the explicit operator direction and the relevant `Vel.csv` simplification pressure:
  - sidebar/context should stay secondary
  - sync/freshness noise should not dominate primary surfaces
  - continuity should resurface contextually rather than through thread-heavy clutter

## Verification

- `rg -n "sleep-relative|midnight-bound|next event means|one clearly relevant resumable thread|commitment-first|rollover boundary|day-between-sleeps" docs/api/runtime.md docs/user/daily-use.md docs/product/operator-mode-policy.md clients/apple/README.md`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`
- `cargo test -p veld late_night_current_day_bucket_keeps_commitments_in_play -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`

## Notes

- Rust test builds still emit the same pre-existing unused/dead-code warnings in `veld`.
- The repeated unified-exec warnings were environment session-count noise, not a blocker in this phase.
