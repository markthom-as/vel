---
id: VSM-008
title: Sandbox Branch Executor
status: proposed
priority: P0
owner: platform
labels: [sandbox, git, isolation, execution]
---

## Summary
Provision isolated worktrees/branches for patch generation and validation.

## Why
Vel should be allowed to experiment in a lab, not directly inside the patient.

## Scope
- Create ephemeral worktrees or branches per proposal.
- Prevent access to protected deploy credentials.
- Clean up environments after completion.
- Support artifact collection.

## Implementation tasks
1. Design branch/worktree naming and lifecycle.
2. Add environment bootstrap and teardown.
3. Restrict credentials/capabilities inside sandbox.
4. Mount repo and test fixtures as needed.
5. Add cleanup on success, failure, and timeout.

## Acceptance criteria
- Each proposal gets isolated execution context.
- Sandbox lacks direct production deploy privileges.
- Validation artifacts are collectible.
- Orphaned worktrees do not accumulate indefinitely.

## Dependencies
- None, but unblocks VSM-005 and VSM-006.

