# Phase 85 Summary

## Outcome

Phase 85 introduced the canonical shared object-presentation primitives and routed the older row/card helpers through them.

Implemented:

- `ObjectRow` as the new base row skeleton
- `ObjectCard` as the bounded card taxonomy
- compatibility wrapping for `NowItemRow*`
- compatibility wrapping for key `PanelChrome` section/inset/dense-row helpers
- generic message card layout now routes through the bounded card primitive
- metric strips reduced in prominence

## Main Code Changes

- `clients/web/src/core/ObjectRow/`
  - new canonical row frame, layout, and title/meta band
- `clients/web/src/core/ObjectCard/`
  - new bounded card primitive with role-based variants
- `clients/web/src/core/NowItemRow/*`
  - now acts as a thin compatibility layer over `ObjectRow`
- `clients/web/src/core/PanelChrome/PanelChrome.tsx`
  - page sections, dense rows, inset cards, and related wrappers now delegate to `ObjectCard` / `ObjectRow`
- `clients/web/src/core/Cards/CardLayout.tsx`
  - generic rendered cards now use the bounded card taxonomy
- `clients/web/src/core/PanelMetricStrip/PanelMetricStrip.tsx`
  - reduced visual emphasis

## Migration Posture

- `NowItemRow` and selected `Panel*` helpers remain temporarily to avoid forcing full page rewrites in this phase.
- Those wrappers now have a clear removal target: direct `ObjectRow` / `ObjectCard` adoption in Phases 86–88.
