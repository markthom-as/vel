# Phase 93 Verification

## Automated checks

- `npm --prefix clients/web test -- --run src/views/system/SystemView.test.tsx src/views/now/NowView.test.tsx src/types.test.ts`
- `cargo test -p veld chat_settings_patch_persists_web_settings -- --nocapture`
- `cargo test -p veld now_task_lane_patch_persists_lane_membership_and_completion_state -- --nocapture`

## Verified behaviors

- widened `Now` lane DTOs decode correctly at the web boundary
- `NowView` uses the lane mutation seam instead of legacy commitment-status patching
- `SystemView` test harness now reflects persisted `web_settings`
- `PATCH /api/settings` persists and re-reads typed `web_settings`
- `PATCH /v1/now/task-lane` persists lane membership and completion state truthfully

## Notes

- frontend tests are treated as regression checks only, not acceptance evidence
- Rust verification for this phase is the stronger proof because the phase goal was API/persistence truth
