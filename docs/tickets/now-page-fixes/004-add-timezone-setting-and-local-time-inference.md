---
id: NOW-004
status: proposed
title: Add timezone setting and make inference use user-local day boundaries
owner: backend
priority: P0
---

## Goal

Make “today”, “morning”, and related awareness logic use the user’s timezone instead of UTC.

## Why

UTC-based day boundaries corrupt medication, morning state, and event reasoning for local-first personal software. This is a classic off-by-one-day goblin wearing a respectable hat.

## Files likely touched

- `crates/vel-api-types/src/lib.rs`
- settings route/service implementation
- `clients/web/src/types.ts`
- `clients/web/src/components/SettingsPage.tsx`
- `crates/veld/src/services/inference.rs`
- any settings persistence layer
- tests across backend + frontend

## Requirements

1. Add `timezone: Option<String>` to settings transport + persistence.
2. Expose timezone in settings UI.
3. Use timezone in inference for:
   - start of today
   - meds done today
   - time-of-day/morning classification
   - display formatting in Now page
4. Default behavior:
   - use saved timezone if present
   - else resolve a sane system default if available
   - else use UTC
5. Validate timezone strings against IANA names.

## Implementation notes

- Centralize timezone resolution in one helper/service.
- Do not scatter parsing and fallback logic throughout inference.
- If the existing time crate makes rich timezone handling awkward, add the smallest reasonable dependency rather than writing cursed manual code.

## Tests

- meds completed at 00:30 UTC should count as previous/local day when appropriate
- start-of-day calculations should vary by timezone
- display formatting should render local time consistently

## Acceptance criteria

- “today” semantics are user-local.
- Now page timestamps and inference agree with each other.
