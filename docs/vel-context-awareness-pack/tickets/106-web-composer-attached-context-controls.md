---
id: 106
title: web-composer-attached-context-controls
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# web-composer-attached-context-controls

## Summary

Add attached-context mode controls and next-message attachment chips to the web composer.

## Why this exists

Server-side awareness is necessary but not sufficient. Operators need a way to steer what gets attached for the next turn.

## Scope

- Add Auto/Manual/None controls to the composer.
- Add chip row for pending attachments.
- Include fields in the outgoing message POST body.

## Deliverables

- Composer mode toggle.
- Attachment chip strip.
- Minimal local state for next-message attachments.
- POST payload updated to include new fields.

## Implementation notes

- Update `MessageComposer.tsx`.
- Persist mode and chips in thread-level or app-level UI state, not inside random DOM islands.
- Default to `auto`.
- Manual chips should be removable before send.
- For this ticket, manual attachment creation can begin with simple entry points from existing UI components rather than a full search browser.

## Acceptance criteria

- User can send in all three modes.
- Manual chips appear in the payload.
- Existing send/error behavior remains intact.
- Frontend tests cover mode changes and payload construction.

## Files likely touched

- `clients/web/src/components/MessageComposer.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/MessageComposer.test.tsx`
- `clients/web/src/types.ts`

## Risks / gotchas

- Do not build a massive attachment picker in this ticket. Get the control plane working first.
