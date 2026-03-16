---
title: Define uncertainty domain model
status: todo
priority: P0
owner: core
labels: [uncertainty, types, core]
---

# Goal

Create the shared type system for uncertainty handling in Vel.

# Deliverables

- `packages/core/uncertainty/types.ts`
- exported types for `AgentAssessment`, `ConfidenceVector`, `UncertaintyItem`, `ResolverCandidate`, `Assumption`, `DecisionRecord`, `UncertaintyLedger`
- Zod or equivalent runtime validation schema if the repo already uses a validation layer

# Requirements

- Keep enums narrow and explicit.
- Distinguish uncertainty category from uncertainty kind.
- Include reversible assumptions as a first-class concept.
- Types must be usable by both orchestrator runtime and UI.

# Acceptance criteria

- Types compile cleanly.
- Runtime validation rejects malformed confidence values and unknown enum members.
- At least 5 representative fixtures exist under test.

# Notes

Do not over-abstract. This is foundational plumbing, not a philosophy seminar.
