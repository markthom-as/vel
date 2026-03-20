# 35-03 Summary

## Outcome

`Now` now uses the backend-owned current-day task buckets for commitment-first lane ordering, and the web surface resurfaces at most one clearly resumable thread instead of leaking thread pressure as a mini inbox.

The main slice landed in:

- `crates/veld/src/services/now.rs`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/NowView.test.tsx`

## What changed

- Added a focused backend regression proving that a same-session post-midnight commitment still stays in play while backlog work remains pullable.
- Kept `Now` lane inputs anchored to backend task buckets:
  - `next_commitment` remains the active commitment anchor
  - `other_open` remains additional in-play commitments
  - `todoist` remains the secondary pullable lane
- Added one compact `Resume thread` surface in web `Now`, selecting only the best existing thread route from ranked `Now` action items and otherwise falling back to a thread-backed reflow status when available.
- Avoided widening `Now` into a thread inbox:
  - filtered thread sets do not render as primary thread cards
  - only one resumable thread can appear on the main surface at a time

## Verification

- `cargo fmt --all`
- `cargo test -p veld late_night_current_day_bucket_keeps_commitments_in_play -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`

## Notes

- This slice keeps commitment-first ordering and single-thread resurfacing honest on web without widening the transport contract again.
- Rust test builds still emit the same pre-existing unused/dead-code warnings in `veld`.
