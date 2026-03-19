# Backlog: Vel

**Purpose:** Capture future work that is worth remembering but is not yet committed to an active roadmap phase.

## Authority

- `ROADMAP.md` is the authoritative priority order for committed phase work.
- `PROJECT.md` is the authoritative product-direction summary and accepted decision record.
- `BACKLOG.md` is the authoritative list of non-phase future work.
- `.planning/todos/pending/*.md` remains the execution-ready micro-task queue for GSD workflows.

## Operating Rules

- Do not duplicate work here if it already has a committed home in `ROADMAP.md`.
- Add backlog items here when they are real candidate work but not yet assigned to a phase.
- Promote an item out of this file when one of these becomes true:
  - it is scheduled into a roadmap phase
  - it is broken into an execution-ready todo in `.planning/todos/pending/`
  - it is rejected or explicitly moved out of scope
- Every item should name its likely phase fit, source, and next promotion path.

## Priority Bands

- `A`: likely next-milestone candidate once current phase commitments allow it
- `B`: meaningful future work, but not expected to land next
- `C`: parking-lot idea worth preserving

## Status Values

- `proposed`: captured but not yet discussed deeply
- `shaping`: direction seems useful, but scope or constraints still need definition
- `blocked`: cannot sensibly schedule until another phase or decision lands
- `scheduled`: moved into `ROADMAP.md` or a concrete phase plan
- `rejected`: explicitly not moving forward

## Backlog Items

| ID | Priority | Status | Area | Likely Phase Fit | Title | Source | Promotion Path | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `BL-001` | `C` | `proposed` | `integrations` | `Phase 6` | Home Assistant integration exploration | user discussion on 2026-03-18 | decide whether this belongs as a source connector, write-back integration, or both; then either add a Phase 6 plan slice or capture a focused todo | Not present in current roadmap or provider catalog. Fits the canonical connector model, but needs a boundary decision first: local snapshot, credential API, brokered tool, or delegated connector. |

## Intake Template

Add new entries with this shape:

| ID | Priority | Status | Area | Likely Phase Fit | Title | Source | Promotion Path | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `BL-XXX` | `A|B|C` | `proposed|shaping|blocked|scheduled|rejected` | `planning area` | `Phase N` or `TBD` | concise title | doc, ticket, or conversation source | how this should get promoted | why it matters, key constraints, and any boundary decisions |
