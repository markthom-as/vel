# Phase 34 Validation

## Goal

Verify that `Now` becomes a compact current-day control surface instead of a stacked dashboard, while preserving honest backend-owned truth for current status, next event, and today-lane data.

## Must prove

1. `Now` renders the intended high-level order:
   - compact context bar
   - current status
   - ask/capture/talk
   - next event
   - unified today lane
   - compressed attention indicators
2. duplicated low-value sections are removed from default view
3. calendar and task rendering align better to current-day expectations and no longer foreground obvious noise
4. the dominant ask/capture/talk affordance remains inline-first and thread-backed
5. `Vel.csv` concerns directly relevant to `Now` are used as regression pressure, not ignored

## Verification approach

- targeted web tests for `NowView`
- targeted Rust tests for `/v1/now` contract compatibility
- targeted type-decoder tests if the `Now` transport shape changes
- `rg` truth checks across docs and planning artifacts if wording/authority shifts

## Human review focus

- does `Now` feel like a control surface instead of a dashboard?
- is the primary information actually actionable?
- is sync/freshness posture no longer visually dominant?
- does the today lane feel commitment-first?
