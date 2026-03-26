# Phase 94 Context

## Goal

Bind the accepted UX to the newly truthful API seams and close the remaining functionality gaps still listed in `TODO.md`.

## Preconditions now satisfied

- `Now` lanes have transport-backed membership and mutation support
- thread rows expose the metadata needed for accepted sidebar behavior
- `System` web settings have a persisted mutation seam
- assistant-entry intent / follow-up / attachment boundary work is no longer speculative

## Main implementation targets

- composer recording / attachment lifecycle behavior
- nudge expansion and standardized action behavior
- `Now` task/title/tag/progress behavior against the widened contract
- `Threads` default selection, filter model, and archive/project metadata behavior
- `System` sidebar depth and truthful field exposure against persisted state

## Risks

- browser-level polish drift should not get mixed into this phase unless needed to make behavior understandable
- TODO items that are still truly API-blocked must be surfaced immediately instead of re-approximated in the client
