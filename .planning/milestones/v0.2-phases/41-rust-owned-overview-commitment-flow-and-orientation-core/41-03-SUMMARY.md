## 41-03 Summary

Completed backend-owned orientation and one-nudge overview embodiment for the MVP `Now` surface.

### What changed

- Restricted the backend-selected visible nudge to `ActionSurface::Now` items in [crates/veld/src/services/now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs) so the overview only surfaces one top nudge from the correct lane and excludes the current dominant action.
- Re-grounded freshness intervention items onto the `Now` surface in [crates/veld/src/services/operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) so backend orientation can pick a single visible nudge from persisted queue pressure.
- Reworked [clients/web/src/components/NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) to embody the backend-owned `overview` contract directly:
  - dominant action or suggestion fallback
  - one visible nudge by default
  - explicit `Why + state` details
  - compact timeline
  - backend-provided decision options
- Removed local `Now` ranking affordances that no longer match the MVP contract:
  - local attention indicator pills
  - separate resumable thread panel
- Updated [clients/web/src/components/NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) to cover:
  - consolidated overview rendering
  - suggestion fallback when no dominant action is present
  - compact context-lane handling for thread pressure instead of resurfacing a separate thread panel

### Verification

- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`

### Outcome

`Now` orientation is backend-owned and explainable, with web acting as a thin embodiment of the Rust-owned overview lane rather than re-ranking pressure locally.
