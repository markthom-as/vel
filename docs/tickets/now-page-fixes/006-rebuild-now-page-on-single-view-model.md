---
id: NOW-006
status: proposed
title: Rebuild Now page against /v1/now view model
owner: web
priority: P1
---

## Goal

Move the Now page to a single contract and reduce duplication.

## Why

The current page is effectively a mashup of explainability data pretending to be a dashboard.

## Files likely touched

- `clients/web/src/components/NowView.tsx`
- `clients/web/src/data/resources.ts`
- `clients/web/src/types.ts`
- `clients/web/src/components/NowView.test.tsx`

## Requirements

1. Add `loadNow()` and `queryKeys.now()`.
2. Replace the four-query composition with a single Now query.
3. Render sections from the new contract:
   - summary band
   - next/upcoming events
   - Todoist backlog
   - operational/attention state
   - freshness
4. Keep a collapsible debug section for raw keys/reasons/signals if present.
5. Remove duplicated explanation lists from default layout.

## UI behavior

- show one canonical reason stack in normal mode
- show freshness badges inline with section headers
- show local timestamps consistently
- render human labels by default

## Acceptance criteria

- The page no longer depends on context explain/drift explain as primary data sources.
- Duplicate context panels are removed or demoted.
