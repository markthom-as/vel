# 44-01 Summary

## What changed

- Tightened [operatorSurfaces.ts](/home/jove/code/vel/clients/web/src/data/operatorSurfaces.ts) so the web MVP shell now treats `Now`, `Inbox`, and `Threads` as daily-use primary surfaces, `Settings` as the only support surface, and `Projects`/`Suggestions`/`Stats` as hidden detail routes instead of top-level navigation peers.
- Simplified [Sidebar.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.tsx) so the visible web shell now teaches only `Daily Use` plus `Support`, removing the older `Advanced` bucket and dropping `Projects` from first-class navigation.
- Simplified [MainPanel.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.tsx) so hidden detail surfaces no longer get first-class route handling; `Projects` remains available as a contextual drill-down route, while `Suggestions` and `Stats` now fall back to a clear non-MVP placeholder.
- Updated [MainPanel.test.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.test.tsx) and [Sidebar.test.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.test.tsx) to verify the reduced MVP hierarchy instead of the older broader shell taxonomy.

## Verification

- `npm --prefix clients/web test -- --run src/components/MainPanel.test.tsx src/components/Sidebar.test.tsx`

## Outcome

The shipped web shell now teaches one minimal MVP model: `Now`, `Inbox`, and `Threads` are the primary loop surfaces, `Settings` is support, and broader detail surfaces no longer present themselves as daily-use peers.
