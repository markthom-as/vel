---
id: NOW-010
status: proposed
title: Make upcoming events actually upcoming and locally formatted
owner: backend+web
priority: P1
---

## Goal

Ensure the schedule section reflects future/current events only and uses local semantics.

## Why

A page that says “Updated now” while showing obviously wrong event timing is how software earns distrust.

## Files likely touched

- `crates/veld/src/services/now.rs`
- calendar integration logic if needed
- `clients/web/src/components/NowView.tsx`
- tests

## Requirements

1. Build `upcoming_events` from event signals with explicit filtering:
   - future events only, plus optionally currently active event
2. Sort ascending by start time.
3. Include meaningful event fields:
   - title
   - start/end
   - location
   - prep/travel/leave-by when applicable
4. Format times in the user timezone.
5. If no calendars selected, show that exact empty state.
6. If calendar sync is stale or disconnected, show that exact degraded state.

## Acceptance criteria

- The schedule section never shows random old events as “upcoming.”
- Local formatting is consistent with the page’s notion of “today.”
