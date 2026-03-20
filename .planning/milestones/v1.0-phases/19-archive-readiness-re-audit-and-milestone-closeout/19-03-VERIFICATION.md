# 19-03 Verification

## Goal

Record milestone-level end-to-end operator-flow evidence instead of relying only on plan-local tests and summaries.

## Selected Closeout Flows

### Flow 1: Wake-up to current-day orientation

Expected milestone behavior:

- `Now` opens as a compact current-day control surface
- current status and next event are relevance-filtered and sleep-relative
- the today lane is commitment-first, with tasks demoted unless promoted

Evidence:

- [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx)
- [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx)
- [39-03-SUMMARY.md](/home/jove/code/vel/.planning/phases/39-vel-csv-regression-sweep-and-daily-use-closeout/39-03-SUMMARY.md)

### Flow 2: Assistant and voice continuity

Expected milestone behavior:

- typed and spoken entry route into the same grounded assistant path
- continuity is thread-backed rather than split across separate chat/voice models
- Apple local-first voice remains queued and merges back into canonical continuity cleanly

Evidence:

- [runtime.md](/home/jove/code/vel/docs/api/runtime.md)
- [README.md](/home/jove/code/vel/clients/apple/README.md)
- [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift)
- [21-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/21-cross-surface-voice-assistant-parity-and-desktop-push-to-talk/21-04-SUMMARY.md)
- [38-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/38-local-first-iphone-voice-continuity-and-offline-action-lane/38-04-SUMMARY.md)

### Flow 3: Daily loop and same-day planning follow-through

Expected milestone behavior:

- morning/standup/closeout use one backend-owned daily-loop model
- planning-profile, day-plan, and `reflow` changes are explainable and supervised
- apply lanes preserve proposal vs applied truth

Evidence:

- [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- [planning-profile-application-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-application-contract.md)
- [day-plan-application-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-application-contract.md)
- [32-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/32-approved-planning-profile-edits-and-supervised-routine-application/32-04-SUMMARY.md)
- [33-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/33-approved-day-plan-and-reflow-application-over-commitment-scheduling/33-04-SUMMARY.md)

### Flow 4: Daily-use closeout across web and Apple

Expected milestone behavior:

- the operator can run a day from Vel without fighting the product
- `Vel.csv` pressure is treated as regression evidence rather than product authority
- remaining limits are explicit rather than silent

Evidence:

- [39-01-VELCSV-MATRIX.md](/home/jove/code/vel/.planning/phases/39-vel-csv-regression-sweep-and-daily-use-closeout/39-01-VELCSV-MATRIX.md)
- [39-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/39-vel-csv-regression-sweep-and-daily-use-closeout/39-04-SUMMARY.md)
- [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- [README.md](/home/jove/code/vel/clients/apple/README.md)

## Remaining Truthful Limits

- Full Apple app/Xcode builds were not rerun in this Linux environment.
- The milestone closeout evidence remains command-backed and artifact-backed, but not UAT-backed.
- Historical Phase `2` / `4` baseline limitations remain documented as re-scoped or baseline-only, not silently upgraded to fully satisfied original-scope claims.

## Verification

- `rg -n "good Vel day|wake-up|standup|closeout|Vel.csv|local-first" docs/user/daily-use.md clients/apple/README.md .planning/phases/39-vel-csv-regression-sweep-and-daily-use-closeout/39-01-VELCSV-MATRIX.md`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/ThreadView.test.tsx src/components/SettingsPage.test.tsx src/types.test.ts`
- `make check-apple-swift`

## Verdict

Passed with explicit environment limits. The milestone now has one durable end-to-end flow record instead of relying only on plan-local summaries.
