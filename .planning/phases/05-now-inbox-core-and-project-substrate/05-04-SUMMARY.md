---
phase: 05-now-inbox-core-and-project-substrate
plan: 04
subsystem: api
tags: [phase-5, cli, linking, fallback, docs, operator-shell]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: backend linking routes, pairing token persistence, and durable trust state from 05-03
provides:
  - `vel node link issue|redeem` CLI fallback commands
  - `vel node status` CLI trust-state inspection with granted scopes
  - truthful runtime docs for the shipped linking endpoints and CLI fallback
affects: [phase-05, continuity, cli, web, apple, docs]
tech-stack:
  added: []
  patterns: [cli fallback over typed runtime routes, config-backed node identity fallback, scope-disclosing status output]
key-files:
  created:
    - crates/vel-cli/src/commands/node.rs
  modified:
    - crates/vel-cli/src/client.rs
    - crates/vel-cli/src/commands/mod.rs
    - crates/vel-cli/src/main.rs
    - docs/api/runtime.md
key-decisions:
  - "CLI token issuance accepts an explicit `--issued-by-node-id` override but falls back to configured `node_id` so the default operator path stays short."
  - "Status output surfaces durable link state and granted scopes directly in the CLI so trust review does not require the web shell."
  - "Runtime docs now treat the linking endpoints as shipped behavior and document the CLI fallback alongside them."
patterns-established:
  - "New operator fallbacks land as typed client methods plus a focused command module plus route-truth docs in the same slice."
  - "CLI trust-sensitive flows should print granted scopes explicitly before asking the operator to rely on a link."
requirements-completed: [CONTINUITY-02]
duration: 6m
completed: 2026-03-19
---

# Phase 05-04 Summary

**CLI-guided linking commands and truthful runtime docs for the shipped non-web fallback path**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-19T02:02:44Z
- **Completed:** 2026-03-19T02:08:51Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `vel node link issue`, `vel node link redeem`, and `vel node status` on top of typed linking client methods.
- Made CLI status and issue output show granted scopes so operators can inspect trust state without using the web shell.
- Updated runtime docs to list the shipped `/v1/linking/*` routes and the CLI fallback commands as implemented behavior.

## Task Commits

No task commits were created. This slice was executed inline in an already-dirty Phase 05 worktree and left uncommitted for review.

## Files Created/Modified

- `crates/vel-cli/src/commands/node.rs` - Adds linking issue/redeem/status handlers, scope formatting, and focused node tests.
- `crates/vel-cli/src/client.rs` - Adds typed client methods for `/v1/linking/tokens`, `/v1/linking/redeem`, and `/v1/linking/status`.
- `crates/vel-cli/src/commands/mod.rs` - Exports the new node command module.
- `crates/vel-cli/src/main.rs` - Registers the `node` top-level command and parser tests for issue, redeem, and status flows.
- `docs/api/runtime.md` - Documents the shipped linking routes and the CLI fallback path with granted-scope disclosure.

## Decisions Made

- Token issuance uses configured `node_id` by default but still allows an explicit CLI override for mixed-node or debugging cases.
- Redeem stays focused on the required token and node identity inputs; scope narrowing can remain a later enhancement because the trust state is inspectable immediately after linking.
- Human-readable CLI output keeps the concrete scope names (`read_context`, `write_safe_actions`, `execute_repo_tasks`) instead of inventing alternate labels.

## Deviations from Plan

None - plan executed within the intended scope.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Guided linking now has a truthful non-web fallback path and operator-visible trust disclosure in the CLI.
- The next dependent slice is `05-05`, which can build the backend action/intervention projection plus Inbox triage and sync-state work on top of the current continuity substrate.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*
