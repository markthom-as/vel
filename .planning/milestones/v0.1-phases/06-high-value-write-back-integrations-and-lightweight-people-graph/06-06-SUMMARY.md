---
phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
plan: 06
subsystem: github-and-email-writeback-lanes
tags: [phase-6, writeback, github, email, people, provenance, docs]
requires:
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: typed write-back/conflict/people contracts from 06-01
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: durable write-back/conflict/upstream-ref persistence from 06-02
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: practical people registry and provenance-bearing graph expansion from 06-05
provides:
  - bounded GitHub issue/comment/state writebacks with typed project and people linkage
  - draft-first email reply writebacks with confirm-required send semantics
  - canonical connector inventory and runtime docs for the new provider lanes
affects: [phase-06, writeback, github, email, people, provenance, docs]
tech-stack:
  added: []
  patterns: [bounded provider writebacks, alias-resolved people linkage, upstream-ref-backed provenance, confirm-required outbound actions]
key-files:
  created:
    - crates/veld/src/services/integrations_github.rs
    - crates/veld/src/services/integrations_email.rs
    - .planning/phases/06-high-value-write-back-integrations-and-lightweight-people-graph/06-06-SUMMARY.md
  modified:
    - crates/veld/src/services/mod.rs
    - crates/veld/src/services/integrations.rs
    - crates/veld/src/services/writeback.rs
    - crates/veld/src/routes/integrations.rs
    - crates/veld/src/app.rs
    - crates/veld/src/services/integrations_host.rs
    - docs/cognitive-agent-architecture/integrations/data-source-catalog.md
    - docs/user/integrations/README.md
    - docs/api/runtime.md
    - .planning/ROADMAP.md
    - .planning/STATE.md
key-decisions:
  - "GitHub stays bounded to four explicit operations: create issue, add comment, close issue, and reopen issue."
  - "Email stays draft-first; send remains confirm-required and persists denied writeback history until explicitly confirmed."
  - "Typed project linkage comes from existing project IDs or project upstream mappings, while typed people linkage resolves through PersonAlias-compatible handles."
  - "Provider artifacts attach provenance through IntegrationSourceRef and stored upstream refs instead of ad hoc provider-only payloads."
patterns-established:
  - "New provider lanes can ship through the shared writeback boundary without widening durable policy beyond the explicitly named operations."
  - "Env-mutating host-source tests need the shared test lock whenever they touch HOME-based auto-discovery logic."
requirements-completed: [GH-01, EMAIL-01, WB-01, WB-02, PROV-01, PEOPLE-02]
duration: 15m
completed: 2026-03-19
---

# Phase 06-06 Summary

**Vel now has bounded GitHub and email writeback lanes with typed project/people linkage and durable provenance**

## Performance

- **Duration:** 15 min
- **Started:** 2026-03-19T05:05:00Z
- **Completed:** 2026-03-19T05:20:23Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added `services::integrations_github` with the exact allowed GitHub write surface: `github_create_issue`, `github_add_comment`, `github_close_issue`, and `github_reopen_issue`.
- Added `services::integrations_email` with `email_create_draft_reply` and `email_send_draft`, keeping email draft-first and confirm-required for send.
- Routed both providers through the shared writeback boundary and mounted `/api/integrations/github/*` plus `/api/integrations/email/*` operator-authenticated routes.
- Resolved typed `project_id` linkage from the project substrate and typed people linkage from `PersonAlias`-compatible `github` and `email` aliases.
- Persisted provider-scoped provenance through `IntegrationSourceRef` and stored upstream refs for repository, issue/comment, and draft/send artifacts.
- Updated the canonical data-source catalog, user integrations guide, and runtime API docs to document the new bounded write lanes.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 06 worktree and left uncommitted for review.

## Files Created/Modified

- `crates/veld/src/services/integrations_github.rs` - Implements the bounded GitHub write lane, typed project/people resolution, and upstream-ref provenance.
- `crates/veld/src/services/integrations_email.rs` - Implements the draft-first email lane with confirm-required send behavior and alias-driven people linkage.
- `crates/veld/src/services/writeback.rs` - Extends the shared writeback dispatch with GitHub and email entry points.
- `crates/veld/src/routes/integrations.rs` - Adds request DTOs and `/api/integrations/github/*` plus `/api/integrations/email/*` write routes.
- `crates/veld/src/app.rs` - Mounts the new GitHub and email integration routes.
- `crates/veld/src/services/integrations.rs` - Seeds the canonical `github` and `email` foundation connectors in connection inventory tests and runtime listing.
- `docs/cognitive-agent-architecture/integrations/data-source-catalog.md` - Adds canonical `github` and `email` provider entries as bounded brokered-tool write lanes.
- `docs/user/integrations/README.md` - Documents the shipped GitHub and email write surfaces and the email draft-first/confirm-required rule.
- `docs/api/runtime.md` - Documents the exact mounted GitHub and email write endpoints and their operator/auth/writeback semantics.
- `crates/veld/src/services/integrations_host.rs` - Repairs the HOME-mutation test harness so the broad `integrations` verification target is stable in both lib and bin test runs.
- `.planning/ROADMAP.md` - Advances Phase 06 to `6/7` with `06-06` complete.
- `.planning/STATE.md` - Advances the active tracker to `06-07` next.

## Decisions Made

- Provider identity is now canonicalized as `github` and `email` in the foundation connection inventory instead of leaving GitHub represented only as the earlier partial `gh` seed.
- GitHub repository scope is remembered as a connection setting ref and project linkage can resolve through `projects.upstream_ids["github"]` when no explicit project ID is provided.
- Email send is not silently queued or auto-applied without confirmation; it records a denied `confirm_required` writeback until the operator explicitly confirms the send.

## Deviations from Plan

- `crates/veld/src/services/integrations_host.rs` changed even though it was not in the original plan file because the broad required verification target (`cargo test -p veld integrations`) exposed a cross-test HOME-environment race that had to be fixed for the slice to verify cleanly.
- The shipped provider lanes are intentionally bounded and synthetic at this phase boundary: they persist durable writeback/upstream-ref history and typed linkage without widening into arbitrary provider API mutation or raw secret handling.

## Issues Encountered

- The first compile pass in `integrations_email.rs` consumed `request.sender` twice; that ownership bug was fixed inline before the verification rerun.
- The first broad `integrations` pass still hit the pre-existing HOME-based auto-discovery flake in the duplicated `bin` test target; locking the second env-mutating host-source test fixed the race and brought the verification target back to green.

## User Setup Required

- No new mandatory operator setup is required to verify the bounded GitHub/email contract itself.
- Typed people linkage is richer when the people registry already contains `github` or `email` aliases for the relevant handles or addresses.

## Next Phase Readiness

- Phase 06 now has typed writeback/conflict foundations, Todoist plus local notes/reminders lanes, a practical people registry, and bounded GitHub/email provider lanes with provenance.
- The next dependent slice is `06-07`, surfacing writeback, conflicts, provenance, and people status through operator views, CLI, and docs.

---
*Phase: 06-high-value-write-back-integrations-and-lightweight-people-graph*
*Completed: 2026-03-19*
