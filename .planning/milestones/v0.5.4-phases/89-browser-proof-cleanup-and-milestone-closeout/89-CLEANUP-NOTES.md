# Phase 89 Cleanup Notes

## Retained Temporary Wrappers

- `clients/web/src/core/PanelChrome/PanelChrome.tsx`
  - still acts as a compatibility facade over `ObjectCard` and `ObjectRow`
  - removal path: migrate remaining call sites to canonical primitives directly, then delete compatibility exports

- `clients/web/src/core/NowItemRow/NowItemRow.tsx`
  - still wraps the canonical row skeleton for legacy call sites
  - removal path: migrate surviving imports to `ObjectRow*` primitives, then remove `NowItemRow`

## Aggressively Replaced Shell / Surface Primitives

- `Now`
- `Threads`
- `System`
- shell top-band layout
- shell action bar
- shell nudge zone

These were rebuilt directly rather than compatibility-wrapped, matching the milestone rule.
