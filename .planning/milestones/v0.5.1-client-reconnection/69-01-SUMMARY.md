# 69-01 Summary

Completed the `v0.5.1` `Now` rebinding slice and removed the remaining client-side operational drift that Phase 69 was meant to kill.

## Delivered

- updated [NowView.tsx](/home/jove/code/vel/clients/web/src/views/now/NowView.tsx)
- added [NowScheduleSection.tsx](/home/jove/code/vel/clients/web/src/views/now/components/NowScheduleSection.tsx)
- updated [NowTasksSection.tsx](/home/jove/code/vel/clients/web/src/views/now/components/NowTasksSection.tsx)
- updated [ActionRow.tsx](/home/jove/code/vel/clients/web/src/views/now/components/ActionRow.tsx)
- updated [CompactTaskLaneRow.tsx](/home/jove/code/vel/clients/web/src/views/now/components/CompactTaskLaneRow.tsx)
- updated [NowView.test.tsx](/home/jove/code/vel/clients/web/src/views/now/NowView.test.tsx)
- updated [MainPanel.tsx](/home/jove/code/vel/clients/web/src/shell/MainPanel/MainPanel.tsx)
- added [commitment_write_bridge.rs](/home/jove/code/vel/crates/veld/src/services/commitment_write_bridge.rs)
- updated [commitments.rs](/home/jove/code/vel/crates/veld/src/routes/commitments.rs)
- added [phase69_now_commitment_write_intent.rs](/home/jove/code/vel/crates/veld/tests/phase69_now_commitment_write_intent.rs)

## What Changed

- `Now` no longer routes users into `Inbox` from risk rows or nudge actions
- `Now` no longer applies client-side nudge ranking or synthetic backup-nudge injection
- `Now` now renders tasks and calendar commitments as adjacent canonical sections instead of overfitting everything into a task-only queue
- task queue rendering now depends on canonical `task_lane` truth rather than provider-shaped Todoist pull lists for the visible operating queue
- the fake client-side reschedule affordance is removed until a lawful canonical due/date action exists
- commitment completion still uses the existing `/v1/commitments/:id` contract, but that route now records and dispatches an internal canonical write intent before applying storage mutation

## Verification

- `npm test -- src/views/now/NowView.test.tsx src/shell/MainPanel/MainPanel.test.tsx`
- `npm run build`
- `cargo test -p veld commitment_write_bridge --lib`
- `cargo test -p veld --test phase69_now_commitment_write_intent`
- `cargo check -p veld`

## Outcome

Phase 69 leaves `Now` as a thinner truthful surface: canonical task and calendar truth are adjacent instead of merged, inbox escape hatches are gone, client-side prioritization is removed, and the surviving direct completion action no longer bypasses the write-intent seam.
