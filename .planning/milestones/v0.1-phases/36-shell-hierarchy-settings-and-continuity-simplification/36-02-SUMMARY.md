# 36-02 Summary

## Outcome

Restructured the web `Settings` general tab into clearer summary-first categories so it reads like a management surface instead of one long mixed-purpose document.

## Main Changes

- simplified the general-tab hierarchy in [SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) with a compact category overview, clearer section labels, and lower-clutter grouping around daily-use defaults, planning/recovery, devices, and support
- kept the existing backend-owned planning-profile, sync, linking, backup, and documentation seams intact while reducing top-level prose and misleading emphasis
- updated [SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx) so the focused general-tab expectations match the new section labels and summary-first copy
- aligned [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) and [operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) to the new Settings grouping rule

## Verification

- `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx`
- `rg -n "general Settings tab|daily-use defaults|planning/recovery|support/docs|summary-first" docs/user/daily-use.md docs/product/operator-mode-policy.md`

## Notes

- This slice intentionally reshapes the general Settings hierarchy only. `Threads`, affordance cleanup, and broader button-vs-link tightening remain the next Phase 36 slice.
