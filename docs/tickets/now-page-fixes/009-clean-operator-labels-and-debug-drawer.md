---
id: NOW-009
status: proposed
title: Separate operator-facing labels from raw inference/debug state
owner: web+backend
priority: P1
---

## Goal

Make the default Now page readable while keeping debug detail available.

## Why

`awake_unstarted` is useful to us. “Not started” is useful to the human actually living the day.

## Files likely touched

- `/v1/now` DTO/service
- `clients/web/src/components/NowView.tsx`
- tests

## Requirements

1. Add label mapping for mode/phase/attention/drift/severity.
2. Default UI renders labels, not raw enum keys.
3. Add a debug disclosure that can reveal:
   - raw context JSON
   - raw keys
   - signal ids
   - commitments used
   - risk ids
4. Remove CLI-centric text from primary reasons list.
5. If help text is still needed, put it in a subtle footer/help section or debug drawer.

## Acceptance criteria

- Default UI contains no raw snake_case state names.
- Debug information remains accessible without cluttering primary UX.
