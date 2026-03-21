## 42-03 Summary

Moved ambiguous and review-gated reflow out of the inline apply lane and into explicit thread continuity, then tightened the web shell so it renders that status compactly instead of behaving like a planner.

### What changed

- Updated [reflow.rs](/home/jove/code/vel/crates/veld/src/services/reflow.rs) so confirm-required or judgment-bearing reflow now escalates through `reflow_edit` thread continuity instead of applying inline.
- Added backend coverage in [reflow.rs](/home/jove/code/vel/crates/veld/src/services/reflow.rs) for the confirm-required escalation path, preserving staged proposal metadata and leaving commitments untouched until thread follow-through.
- Added `/v1/now` verification in [app.rs](/home/jove/code/vel/crates/veld/src/app.rs) proving that after escalation the backend suppresses the live reflow card and returns typed `reflow_status` continuity instead.
- Updated [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) to render thread-backed reflow status more explicitly with preview lines and a compact `Continue in Threads` cue.
- Added focused web coverage in [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) for compact status-only rendering and thread continuity.

### Verification

- `cargo test -p veld reflow -- --nocapture`
- `cargo test -p veld app::tests::now_endpoint_surfaces_thread_backed_reflow_status_after_escalation -- --nocapture`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`

### Outcome

Phase 42 now keeps bounded reflow inline only when it is truly bounded. Ambiguous or review-gated cases move into typed `Threads` continuity with explicit backend status, and the web shell renders that returned state without inventing planner behavior.
