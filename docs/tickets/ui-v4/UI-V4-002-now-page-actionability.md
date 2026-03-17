# UI-V4-002 — Clean up the Now page around actionable work only

Status: todo
Priority: P0
Lane: A

## Why

The `Now` screenshots show useful information, but too many categories compete for the page's primary attention.

Evidence:

- `~/Downloads/localhost_5173_.png`
- `~/Downloads/localhost_5173_ (1).png`

Observed overload:

- schedule
- operational state
- backlog
- freshness and sync actions
- recent source activity
- extra commitments
- debug
- full context rail

## Goal

Make `Now` answer one question clearly: what should happen next?

## Ownership / likely write scope

- now-page layout and cards
- supporting now-page view model
- docs for `Now` responsibility boundaries

## Deliverables

- stronger priority order for active commitments, at-risk commitments, and suggested actions
- relocation of observability/debug-heavy content to better surfaces
- regression tests for primary card order and degraded-state behavior

## Acceptance criteria

- the page leads with action
- debug/system-health detail is no longer a primary `Now` concern
- an operator can scan the page and decide what to do next without parsing diagnostic clutter
