## 41-04 Summary

Closed Phase 41 with transport-parity checks and truthful daily-use documentation for the shipped backend-owned overview and commitment flow.

### What changed

- Tightened backend `/v1/now` verification in [crates/veld/src/app.rs](/home/jove/code/vel/crates/veld/src/app.rs) so the app-level test now checks the shipped overview contract shape instead of stale item-order copy.
- Strengthened web transport assertions in [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) to verify `continuity_summary` and bounded `allowed_actions` for both morning-overview and standup sessions.
- Updated Apple transport regression coverage in [DailyLoopTests.swift](/home/jove/code/vel/clients/apple/VelAPI/Tests/VelAPITests/DailyLoopTests.swift) so the mocked daily-loop payloads now include `continuity_summary` and `allowed_actions`, with explicit assertions for the bounded action vocabulary.
- Rewrote the stale `Now` guidance in [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) to match the shipped action-first truth:
  - dominant action or bounded suggestion fallback
  - one visible nudge by default
  - `Why + state` disclosure
  - inline commitment actions stay bounded
  - `Threads` remains the continuation path only when work becomes multi-step

### Verification

- `cargo test -p veld app::tests::now_endpoint_returns_consolidated_snapshot -- --nocapture`
- `cargo test -p veld routes::now::tests::now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts src/components/NowView.test.tsx`

### Verification gaps

- `swift test` in [clients/apple/VelAPI](/home/jove/code/vel/clients/apple/VelAPI) is not runnable in this shell because the local wrapper fails with `swift-test: not found`. Apple transport coverage was updated in code but could not be executed here.

### Outcome

Phase 41 now closes with backend, web, and Apple transport parity evidence captured in tests and user docs, with the only remaining gap being local Apple test-runner availability in this environment.
