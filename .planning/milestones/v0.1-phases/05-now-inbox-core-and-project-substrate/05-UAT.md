---
status: complete
phase: 05-now-inbox-core-and-project-substrate
source:
  - 05-01-SUMMARY.md
  - 05-02-SUMMARY.md
  - 05-03-SUMMARY.md
  - 05-04-SUMMARY.md
  - 05-05-SUMMARY.md
  - 05-06-SUMMARY.md
  - 05-07-SUMMARY.md
  - 05-08-SUMMARY.md
  - 05-09-SUMMARY.md
started: 2026-03-19T03:18:53Z
updated: 2026-03-19T03:25:34Z
---

## Current Test

[testing complete]

## Tests

### 1. Cold Start Smoke Test
expected: Kill any running Vel server/service, start the application from scratch, and let migrations/bootstrap run on a clean runtime. The daemon should boot without startup errors, the new project/linking tables should migrate cleanly, and a primary query such as `GET /v1/health`, loading the web shell, or a basic CLI call should return live data instead of warm-cache-only behavior.
result: pass

### 2. Create And Inspect A Project Workspace
expected: Creating a project through the shipped Phase 05 path should persist a typed project with `Personal`, `Creative`, or `Work` family, primary repo and notes roots, and local-first pending provision intent only. After creation, the project should appear in the Projects surface and be retrievable through the typed runtime/API path without any hidden upstream side effects.
result: pass

### 3. Guided Linking From The CLI
expected: `vel node link issue` should issue a short-lived token with explicit scopes, `vel node link redeem` should accept the token for a target node, and `vel node status` should show durable linked-node trust state plus the granted `read_context`, `write_safe_actions`, and `execute_repo_tasks` scopes so trust can be inspected without the web shell.
result: skipped
reason: skipped for now

### 4. Now Surface Shows Ranked Action State
expected: The primary Now surface should render ranked backend-owned action items first, with review counts visible and supporting freshness or schedule panels still present below. The result should feel like one coherent action stack rather than separate client-local urgency logic.
result: skipped
reason: skipped for now

### 5. Inbox Triage Uses Explicit Server-Backed Actions
expected: Inbox rows should expose dense triage details, evidence or project context, and explicit actions such as Acknowledge, Snooze, Dismiss, and Open thread. Acting on a row should update or remove it in line with server-backed state rather than leaving stale client-only triage.
result: skipped
reason: skipped for now

### 6. Web Projects And Linking Surfaces Reflect Shared Truth
expected: The web shell should show Projects grouped by family with a real create flow, and Settings should show linked-node status plus pairing-token issuance with CLI fallback copy. These views should reflect the shared backend state rather than browser-owned project or trust policy.
result: skipped
reason: skipped for now

### 7. Review Commands Emit Typed Phase 05 JSON
expected: `vel review today --json` should include `open_action_count`, `triage_count`, and `top_action_titles`, while `vel review week --json` should include typed `project_review_candidates` with project ID, slug, family, and open commitment counts. The output should describe Now, Inbox, and Projects rather than only capture volume.
result: skipped
reason: skipped for now

### 8. Project Synthesis Resolves Typed Projects First
expected: Running project synthesis for a typed project slug should succeed and produce a project synthesis artifact tied to the canonical typed project slug. Legacy alias fallback should only matter when no typed project exists for the requested workspace.
result: skipped
reason: skipped for now

### 9. Apple Continuity Surfaces Stay Read-Only And In Sync
expected: After sync/bootstrap hydration, the Apple clients should show cached project, action, and linked-node continuity summaries that match the shared backend truth. They should remain thin read-only summaries without introducing Apple-specific ranking or triage behavior.
result: skipped
reason: skipped for now

## Summary

total: 9
passed: 2
issues: 0
pending: 0
skipped: 7

## Gaps

none yet
