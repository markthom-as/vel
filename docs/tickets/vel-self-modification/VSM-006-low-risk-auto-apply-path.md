---
id: VSM-006
title: Low-Risk Auto-Apply Path
status: proposed
priority: P0
owner: platform
labels: [autonomy, apply, rollback, low-risk]
---

## Summary
Enable auto-apply for Class A surfaces such as docs, prompts, tests, and manifests, gated by validation and policy.

## Why
This is the first meaningful slice of self-improvement: enough to be useful, not enough to let the daemon steal the keys.

## Scope
- Auto-apply only on allowed classes/paths.
- Require passing validations.
- Record diff, commit hash, actor, and rationale.
- Support immediate rollback.

## Implementation tasks
1. Add policy decision point for auto-apply eligibility.
2. Implement commit/apply flow in isolated worktree.
3. Add merge/promotion path appropriate to repo architecture.
4. Emit ledger events and metrics.
5. Add revert support keyed by proposal ID.

## Acceptance criteria
- Protected or higher-risk paths are blocked.
- Successful auto-applies produce commit and ledger evidence.
- Failed post-apply checks trigger rollback.
- Operator can trace exactly why a change auto-applied.

## Dependencies
- VSM-001, VSM-004, VSM-005, VSM-009, VSM-017.

