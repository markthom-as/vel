# 36-01 Summary

## Outcome

Published the simplified web shell hierarchy by making the sidebar a thin icon rail, keeping thread controls contextual to `Threads`, and documenting that the rail is navigational rather than a second information column.

## Main Changes

- tightened the shell chrome in [clients/web/src/components/AppShell.tsx](/home/jove/code/vel/clients/web/src/components/AppShell.tsx) so the sidebar defaults to a thin rail width
- rewrote [clients/web/src/components/Sidebar.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.tsx) into compact icon-rail navigation with thread history and new-thread controls only on the `Threads` surface
- updated [clients/web/src/components/Sidebar.test.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.test.tsx) to cover the contextual thread-controls rule
- aligned [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) and [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) so the shipped product story matches the thinner, less noisy sidebar role

## Verification

- `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx`
- `rg -n "thin icon rail|second information column|sidebar should" docs/product/operator-mode-policy.md docs/user/daily-use.md`

## Notes

- This slice intentionally stops at shell hierarchy and sidebar behavior. `Settings` restructuring remains the next slice in `36-02`.
