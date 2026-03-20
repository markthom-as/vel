# 36-04 Summary

## Outcome

Closed Phase 36 by verifying the simplified shell against the relevant `Vel.csv` friction points and aligning the shipped docs to one coherent daily-use story.

## `Vel.csv` Evidence

Relevant pressure checked against the shipped shell:

- sidebar should be thinner / icon-driven:
  covered by the thin icon rail in `36-01`
- `Threads` should auto-open the latest thread and stay continuity-first:
  covered by the latest-thread fallback plus continuity-first framing in `36-03`
- `Settings` should be less cluttered and docs should not feel buried:
  covered by the summary-first regrouping in `36-02`
- shell copy should reduce sync/recovery slop:
  covered across `Now`, `Threads`, sidebar, and `Settings` docs alignment

Still explicitly out of scope for Phase 36:

- contextual markdown docs rendered on every route
- template editing inside Settings
- broader project-surface redesign
- fully collapsible secondary context-panel behavior

## Main Changes

- verified the shipped web shell against the relevant `Vel.csv` rows and recorded which requests are satisfied now versus still out of scope
- aligned [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) and [operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) to the simplified hierarchy so the docs match the shell
- closed Phase 36 in roadmap/state after the focused `Now`, `Threads`, sidebar, and `Settings` verification passes

## Verification

- `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx`
- `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx`
- `npm --prefix clients/web test -- --run src/components/ThreadView.test.tsx src/components/NowView.test.tsx`
- `rg -n "sidebar|threads view should auto open the latest thread|documentation|settings ui|icon driven" ~/Downloads/Vel.csv`
- `rg -n "thin icon rail|general Settings tab|continuity surface|chat inbox|summary-first" docs/user/daily-use.md docs/product/operator-mode-policy.md`

## Notes

- Phase 36 intentionally closed shell hierarchy and clarity debt only. Larger Apple embedded-core and broader docs/rendering work stay in later phases.
