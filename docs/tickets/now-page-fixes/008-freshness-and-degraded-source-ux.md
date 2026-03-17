---
id: NOW-008
status: proposed
title: Surface freshness and degraded-source warnings in Now UI
owner: web+backend
priority: P1
---

## Goal

Tell the user when the page is fresh, aging, stale, broken, or disconnected.

## Why

Trustworthy dashboards expose their uncertainty. Otherwise you get a beautiful hallucination wearing Tailwind.

## Files likely touched

- backend freshness fields in `/v1/now`
- `clients/web/src/components/NowView.tsx`
- related styles/tests

## Requirements

1. Add overall freshness summary.
2. Add per-source freshness for at least:
   - context
   - calendar
   - todoist
   - activity
3. Define thresholds, for example:
   - fresh: <= 2 min
   - aging: <= 10 min
   - stale: > 10 min
   - error/disconnected from integration status
4. Show inline warnings when a section is degraded.
5. Keep stale data visible, but clearly marked.

## Acceptance criteria

- A user can see at a glance whether Now is trustworthy.
- Stale integrations do not masquerade as live state.
