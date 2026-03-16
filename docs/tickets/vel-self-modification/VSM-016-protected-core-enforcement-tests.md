---
id: VSM-016
title: Protected Core Enforcement Tests
status: proposed
priority: P0
owner: platform
labels: [tests, safety, protected-core]
---

## Summary
Add negative tests proving Vel cannot self-apply changes to Ring 0 / Class E surfaces.

## Why
The whole point of a constitution is that the executive cannot rewrite it because it had a productive afternoon.

## Scope
- Test blocked proposals targeting protected paths.
- Test mixed diffs containing protected and unprotected files.
- Test policy-bypass attempts.

## Implementation tasks
1. Create protected-path fixtures.
2. Add integration tests for proposal creation/apply flow.
3. Ensure blocked decisions are written to ledger.
4. Add regression tests for registry precedence bugs.
5. Test operator-only workflow separation if available.

## Acceptance criteria
- Protected-path self-apply attempts fail deterministically.
- Mixed diffs are either blocked or split according to explicit policy.
- Ledger records blocked attempts with reasons.
- Tests break if someone weakens enforcement later.

## Dependencies
- VSM-001, VSM-004, VSM-006.

