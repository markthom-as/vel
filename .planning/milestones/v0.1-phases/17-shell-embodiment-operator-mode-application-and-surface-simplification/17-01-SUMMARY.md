# 17-01 Summary

Completed the shared web shell-classification slice by moving top-level surface taxonomy into one metadata source and rebuilding the default navigation posture around it.

## What changed

- Added [operatorSurfaces.ts](/home/jove/code/vel/clients/web/src/data/operatorSurfaces.ts) as the single source of truth for web surface classification, labels, icons, disclosure level, and navigation visibility.
- Updated [Sidebar.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.tsx) to consume that metadata instead of hard-coding the old peer-surface posture. The sidebar now foregrounds daily-use surfaces first, keeps `Threads` and `Projects` as support surfaces, and moves `Settings` into an explicit advanced section.
- Updated [App.tsx](/home/jove/code/vel/clients/web/src/App.tsx) and [MainPanel.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.tsx) so the shared `MainView` type comes from the new metadata seam rather than from sidebar-local assumptions.
- Updated [MainPanel.test.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.test.tsx) and [Sidebar.test.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.test.tsx) to verify the approved taxonomy: only `Now`, `Inbox`, `Threads`, `Projects`, and `Settings` are nav-visible first-contact shell categories, while `Suggestions` and `Stats` remain detail surfaces.
- Updated [surfaces.md](/home/jove/code/vel/docs/user/surfaces.md) so the shipped operator docs match the new shell posture and no longer teach `Suggestions` or `Stats` as top-level peer destinations.

## Verification

- `npm --prefix clients/web test -- --run src/components/MainPanel.test.tsx`
- `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx`
- `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx`

## Why this matters

Phase 17 starts by locking the shell taxonomy in one place. That removes duplicated navigation assumptions from the web shell and leaves the rest of the phase free to sharpen `Now`, `Inbox`, `Threads`, `Projects`, and `Settings` behavior without relitigating which surfaces are supposed to be primary.
