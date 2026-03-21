## 49-01 Summary

Rebuilt the web `Now` surface around the canonical compact frame in [`NowView.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.tsx). The top-level structure now leads with the shared Rust-owned header, filter buckets, status row, context line, compact mesh trust posture, stacked nudge bars, compact task lane, and a bottom-docked input shell instead of the older card-heavy dashboard.

Updated [`NowView.test.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) to assert the compact transport-driven structure and to stop depending on stale explanatory copy or single-instance text assumptions where the compact lane and disclosed detail view both render the same task.

Verification:

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx`
