# 20-04 Summary

## Outcome

Closed the remaining daily-use friction across `Inbox`, `Threads`, and `Settings`, which completes Phase 20.

The default web loop is now coherent:

- enter through `Now`
- triage unresolved decisions in `Inbox`
- continue/search in `Threads`
- use `Settings` for summary-first readiness and setup guidance instead of default runtime spelunking

## Shipped changes

- Tightened Inbox triage copy and demotion rules in [clients/web/src/components/InboxView.tsx](/home/jove/code/vel/clients/web/src/components/InboxView.tsx).
- Added regression coverage for that posture in [clients/web/src/components/InboxView.test.tsx](/home/jove/code/vel/clients/web/src/components/InboxView.test.tsx).
- Added lightweight recent-thread filtering and continuity switching in [clients/web/src/components/ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx), with wiring through [clients/web/src/components/MainPanel.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.tsx).
- Added focused thread continuity/filter coverage in [clients/web/src/components/ThreadView.test.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx) and kept [clients/web/src/components/MainPanel.test.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.test.tsx) aligned.
- Added a summary-first assistant readiness card in [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) and corresponding coverage in [clients/web/src/components/SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx).
- Updated [docs/api/chat.md](/home/jove/code/vel/docs/api/chat.md) and [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) so the shipped loop is documented accurately.

## Verification

- `npm --prefix clients/web test -- --run src/components/InboxView.test.tsx src/components/ThreadView.test.tsx src/components/SettingsPage.test.tsx src/components/MainPanel.test.tsx`
- `rg -n "optional|assistant|Now|Inbox|Threads|setup|runtime" docs/api/chat.md docs/user/daily-use.md clients/web/src/components/SettingsPage.tsx`

## Notes

- This slice intentionally kept `Inbox` out of archive/history behavior.
- Thread filtering is scoped to already-loaded persisted conversations; it is not a new retrieval engine.
- Assistant/model setup remains optional and summary-first here. Apple voice parity and deeper cross-surface voice work are still Phase 21.
