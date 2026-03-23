# 73-01 Summary

## Outcome

Phase 73 is closed.

The shared shell now matches the embodied `v0.5.2` doctrine closely enough for surface-specific work to build on top of it without dragging old shell noise forward. The three-surface frame is cleaner, the old global info rail is gone, and reusable disclosure behavior now lives in a shared primitive instead of ad hoc thread-specific chrome.

## Landed

- removed the shell-global info rail and its toggle from the active web frame
- kept the shell limited to `Now`, `Threads`, and `System` with icon-plus-label navigation
- introduced a shared [SurfaceDrawer](/home/jove/code/vel/clients/web/src/core/SurfaceDrawer/SurfaceDrawer.tsx) primitive for explicit per-surface disclosure
- moved thread provenance disclosure onto the shared drawer primitive
- added focused shell tests for the shared frame and updated navbar expectations to match the embodied doctrine
- preserved a recoverable pre-Phase-73 UI snapshot in [73-UI-BASELINE.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/73-UI-BASELINE.md) and the `v0.5.2-ui-baseline-pre-phase73` tag
- added browser proof evidence under [73-evidence](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/73-evidence)

## Verification

- `cd clients/web && npm test -- src/core/SurfaceDrawer/SurfaceDrawer.test.tsx src/views/threads/ProvenanceDrawer/ProvenanceDrawer.test.tsx src/shell/AppShell/AppShell.test.tsx src/shell/MainPanel/MainPanel.test.tsx src/shell/Navbar/Navbar.test.tsx`
- `cd clients/web && npm run build`
- `cd clients/web && npm run proof:phase73:shell-frame`

## Next

Phase 74 is now the active slice: `Now` ideal-state embodiment and operator-speed repair.
