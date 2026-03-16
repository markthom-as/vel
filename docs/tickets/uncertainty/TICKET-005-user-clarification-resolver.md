---
title: Create user clarification resolver
status: todo
priority: P1
owner: product-runtime
labels: [uncertainty, resolver, ux]
---

# Goal

Provide a first-class resolver that asks the user concise, discriminative clarification questions.

# Deliverables

- `packages/core/resolvers/user-resolver.ts`
- prompt template / rendering helper for clarification requests
- support for multiple-choice and freeform clarification
- response ingestion that resolves linked uncertainty items and assumptions

# Requirements

- Question payload must include decision point, why it matters, recommended default, and fallback assumption.
- Prefer small choice sets over open-ended prompts.
- Support "answer later / proceed with default" semantics where appropriate.
- Log all user clarifications in the uncertainty ledger.

# Acceptance criteria

- UX copy tests verify concise question formatting.
- Resolver updates task state when a user answers.
- Resolver supports timeout/expiry behavior for stale questions.

# Notes

Good clarification is surgical. Bad clarification is just outsourcing thought with nicer typography.
