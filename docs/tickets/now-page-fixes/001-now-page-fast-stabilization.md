---
id: NOW-001
status: proposed
title: Fast stabilization for stale Now page data and broken invalidation
owner: web
priority: P0
---

## Goal

Stop the worst lies immediately without waiting for the full `/v1/now` redesign.

## Why

Right now the page can look updated while explain/drift/commitments remain stale. That is rotten operator ergonomics.

## Scope

Frontend-only stabilization:

- invalidate all Now dependencies after settings sync actions
- add manual refresh affordance
- add focus-based refresh
- add interval refresh while page is visible

## Files likely touched

- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/data/query.ts`
- `clients/web/src/data/resources.ts`
- `clients/web/src/components/NowView.test.tsx`
- `clients/web/src/data/query.test.tsx`

## Requirements

1. Update `refreshIntegrationViews()` so it invalidates/refetches:
   - `currentContext`
   - `contextExplain`
   - `driftExplain`
   - `commitments(COMMITMENT_LIMIT)`
2. Add a small “Refresh” control to the Now page header.
3. On page focus / visibility restore, refetch all Now queries.
4. Add a 30s refetch interval while the Now page is mounted and document-cleanup on unmount.
5. While refetching, do not blank the page; use the existing cached snapshot and render a subtle refreshing state.

## Implementation notes

- Prefer adding generic query options if it stays small and clean:
  - `refetchIntervalMs?: number`
  - `refetchOnWindowFocus?: boolean`
  - `refetchOnVisibilityChange?: boolean`
- If generic query plumbing starts to sprawl, keep it local to `NowView` for this ticket.
- Do not accidentally create fetch storms via repeated effect registration.

## Acceptance criteria

- Syncing Todoist from Settings causes the backlog on the Now page to update without a hard reload.
- Switching away from the tab and back triggers a refresh.
- The page auto-refreshes every 30 seconds while open.
- Tests cover invalidation + focus/interval refresh behavior.
