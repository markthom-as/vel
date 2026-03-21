## 49-02 Summary

Tightened the compact web `Now` interaction lane in [`NowView.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.tsx) so header buckets, nudge bars, and task-lane items route through existing thread or settings handlers instead of growing local planner behavior. Bucket clicks now respect backend-provided thread targets, nudge actions can hand off to `Threads`, `Inbox`, or `Settings`, and compact task rows expose thread continuity when the backend provides a primary thread.

Extended [`NowView.test.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) with interaction checks for compact routing so the web embodiment verifies real handoff behavior, not just structure.

Verification:

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`
