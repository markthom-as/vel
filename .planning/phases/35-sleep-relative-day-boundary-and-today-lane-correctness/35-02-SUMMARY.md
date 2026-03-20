# 35-02 Summary

## Outcome

`GET /v1/now` schedule truth now uses the shared current-day seam together with stricter event relevance filtering, instead of treating every persisted calendar row as equally meaningful.

The main slice landed in:

- `crates/veld/src/services/integrations_google.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/app.rs`

## What changed

- Enriched Google Calendar event payloads with backend-usable relevance fields: `all_day`, `transparency`, and self `response_status`.
- Tightened `Now` event filtering so the schedule lane excludes known noise inside the current-day window:
  - all-day entries
  - free/transparent events
  - self-declined events
  - cancelled events
- Corrected `next_event` semantics so it now means the next future relevant event rather than “current or next.”
- Kept active events in `upcoming_events` so the shell can still derive “what is happening now” from the same backend-owned schedule stream.
- Added a focused `Now` regression proving that an active real event remains visible while free/declined/all-day noise is dropped before the client sees it.

## Verification

- `cargo fmt --all`
- `cargo test -p veld now_endpoint_filters_calendar_noise_and_uses_next_future_event -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`

## Notes

- This slice tightened current-day event truth, but true explicit sleep-signal day-boundary promotion is still limited by the absence of a normalized bedtime/wake boundary input in the Rust runtime today.
- Rust test builds still emit the same pre-existing unused/dead-code warnings in `veld`.
