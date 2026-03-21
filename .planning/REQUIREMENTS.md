# Requirements: Vel

**Defined:** 2026-03-21
**Milestone:** 0.4.x
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust.

## 0.4.x Requirements

Requirements for the `Now/UI MVP Conformance Closure` release line. This line closes the remaining gap between the canonical `Now` contract, the operator's explicit correction memo, and the shipped web/client surfaces so the product feels compact, trustworthy, and usable in repeated daily use.

## Milestone Acceptance Checklist

The `0.4.x` line is only complete if all of these are true:

- [x] the web `Now` surface matches the operator-corrected contract as the reference implementation
- [x] the top area of `Now` is containerless, compact, and visually subordinate to the task container
- [x] tasks are grouped by real current-day truth and are the only dominant visual container on `Now`
- [x] nudges render as compact styled info boxes with icon, severity color, and project tag context
- [x] the floating `Now` input is always visible, bottom-center, and inline-first with text and voice only
- [x] shell navigation uses a compact top nav plus collapsible right sidebar instead of the current left-nav helper-heavy shell
- [x] `Inbox` contains the same underlying actionable objects surfaced in `Now`
- [x] `Threads` and `Settings` match the compact IA expected for MVP use instead of debug-leaning or prose-heavy layouts
- [x] iOS/client parity work is defined against the same reference behavior rather than reinterpreting the product contract

## Non-Goals

- widening `Now` into a dashboard again
- adding new planner/product surfaces not needed for MVP closure
- broad provider/platform expansion unrelated to the conformance issues in this memo
- reopening Rust-owned product semantics already settled in `0.3.0` unless a data-truth bug requires it

### Now Surface Conformance

- [x] **NOWUI-01**: `Now` top area is containerless and composed of compact stacked micro-rows for title, minimal timing/context, active description, and nudges
- [x] **NOWUI-02**: the top area excludes elapsed-time and verbose metadata, truncates active description to a single line, and hides empty-state controls entirely
- [x] **NOWUI-03**: nudges render as compact styled info boxes with icon, severity color, and project tag context; they live in their own row under timing/description
- [x] **NOWUI-04**: tasks are the only dominant visual container and render grouped sections in strict order: `NOW`, `TODAY`, `AT RISK`, `NEXT`
- [x] **NOWUI-05**: project reviews are removed from `Now` unless explicitly due today, and zero-task state keeps the task container with a terse neutral line
- [x] **NOWUI-06**: `Now` uses a floating bottom-center input with text and voice only, no helper text, and inline-first escalation to threads
- [x] **NOWUI-07**: `Now` removes helper prose, `More Context and Controls`, and other shell noise from the main surface

### Navigation And Shell

- [x] **SHELL-01**: the app shell uses a compact top nav that includes sync/context indicators, `Now`, `Inbox`, `Threads`, `Settings`, and `Documentation`
- [x] **SHELL-02**: the right sidebar is collapsible context/documentation, collapsed by default with an open affordance on desktop and promoted to a top-level info affordance on mobile
- [x] **SHELL-03**: `Daily Use` and similar explanatory shell labels are removed from the primary navigation
- [x] **SHELL-04**: iconography is more compact and semantically clearer across the shell

### Inbox, Threads, And Settings

- [x] **INBOX-01**: `Inbox` remains the superset queue for the same underlying actionable objects shown in `Now`
- [x] **INBOX-02**: empty `Inbox` while `Now` shows actionable items is prevented by the underlying data/query model, not papered over in UI
- [x] **THREADS-01**: `Threads` uses global top nav plus a left thread list and main content panel
- [x] **THREADS-02**: thread rows show title, truncated last message, unread indicator, unread count when applicable, and optional lower-priority tags
- [x] **THREADS-03**: unread status has highest visual priority in constrained thread-list layouts
- [x] **SETTINGS-01**: `Settings` is compact and organized by a left tab rail, not top tabs
- [x] **SETTINGS-02**: `Settings` groups only `Profile / Onboarding`, `Device / Sync`, `Agent Grounding / State`, and `Backups`
- [x] **SETTINGS-03**: documentation access is removed from `Settings` and promoted to top-level navigation

### Verification And Parity

- [ ] **VERIFY-01**: manual conformance checklist exists and is the first gate for milestone acceptance
- [ ] **VERIFY-02**: contract/DTO tests cover the corrected `Now` grouping, hidden-empty-state behavior, and inbox/now shared-object invariants
- [ ] **VERIFY-03**: UI tests cover the reference web embodiment after the manual checklist passes
- [x] **PARITY-01**: web is treated as the reference implementation for MVP closure
- [x] **PARITY-02**: iOS/client parity work explicitly tracks the same behavior after the web reference is corrected

## Future Requirements

- [ ] broader visual polish beyond the compact MVP closure needed for repeated daily use
- [ ] additional Apple-specific refinements that are not required to preserve reference parity
- [ ] further client-mesh expansion beyond the shell/nav/support corrections in this milestone

## Traceability

| Requirement | Phase |
|-------------|-------|
| NOWUI-01 | Phase 52 |
| NOWUI-02 | Phase 52 |
| NOWUI-03 | Phase 52 |
| NOWUI-04 | Phase 52 |
| NOWUI-05 | Phase 52 |
| NOWUI-06 | Phase 52 |
| NOWUI-07 | Phase 52 |
| SHELL-01 | Phase 52 |
| SHELL-02 | Phase 52 |
| SHELL-03 | Phase 52 |
| SHELL-04 | Phase 52 |
| INBOX-01 | Phase 52 |
| INBOX-02 | Phase 52 |
| THREADS-01 | Phase 52 |
| THREADS-02 | Phase 52 |
| THREADS-03 | Phase 52 |
| SETTINGS-01 | Phase 52 |
| SETTINGS-02 | Phase 52 |
| SETTINGS-03 | Phase 52 |
| VERIFY-01 | Phase 56 |
| VERIFY-02 | Phase 56 |
| VERIFY-03 | Phase 56 |
| PARITY-01 | Phase 52 |
| PARITY-02 | Phase 52 |

---
*Last updated: 2026-03-21 for the active `0.4.x` release line*
