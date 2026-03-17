---
title: Self-Awareness And Supervised Self-Modification
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 021-canonical-schema-and-config-contracts
  - 017-execution-tracing-reviewability
labels:
  - introspection
  - self-awareness
  - repo-visibility
  - safety
  - phase-1
---

# Context & Objectives

Vel already has introspection language and repo-aware coding-agent workflows, but it lacks a single contract for what “self-aware” means and how supervised self-modification should work.

This ticket defines that contract so repository visibility, writable scope, review gates, and verification evidence are explicit instead of implicit.

# Impacted Files & Symbols

- **Docs**: `docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md`
  - **Symbols**: self-model layers, read/write scope, self-modification contract
- **Docs**: `docs/cognitive-agent-architecture/agents/tool-access.md`
  - **Symbols**: repo read/write scope rules
- **Docs**: `docs/cognitive-agent-architecture/policies/trust-and-safety.md`
  - **Symbols**: high-impact change constraints
- **Future Runtime Work**: delegated worker manifests, task envelopes, writable-scope enforcement

# Technical Requirements

- **Self-Model**: Define what Vel may know about runtime state, docs, config, code, and tickets.
- **Scope Split**: Keep repository read scope separate from repository write scope.
- **Supervised Writes**: Code or doc modification must require diff visibility, verification evidence, and review gating.
- **No Ambient Rewrite**: Self-modification should be task-bounded and narrow, not always-on authority.
- **Traceability**: Self-applied or self-proposed changes need stable task/run linkage.

# Cross-Cutting Trait Impact
- **Modularity**: required — self-awareness should operate through explicit self-model and scope contracts.
- **Accessibility**: required — operators need to understand what the system could read and write.
- **Configurability**: affected — writable scopes and review gates may vary by environment or task.
- **Data Logging**: required — self-modification attempts need traceable evidence and summaries.
- **Rewind/Replay**: affected — self-applied changes should preserve diff and verification history.
- **Composability**: required — self-awareness should compose with tickets, docs, tools, and sandbox boundaries.

# Implementation Steps (The "How")

1. **Spec**: Define the self-model layers and scope model.
2. **Tooling Rules**: Align tool-access and trust-and-safety docs with explicit repo read/write semantics.
3. **Queue Sync**: Reference the self-awareness contract from later connect, sandbox, and eval work.
4. **Future Runtime**: Use the contract to drive actual writable-scope enforcement and self-model data structures later.

# Acceptance Criteria

1. [ ] The repo has a clear self-awareness and supervised self-modification contract.
2. [ ] Read scope and write scope are explicitly separated.
3. [ ] Ticket and spec language no longer treats self-improvement as vague ambient behavior.
4. [ ] Later implementation work can reference one canonical contract for repo-aware behavior.

# Verification & Regression

- **Doc Check**: introspection, tool-access, trust-and-safety, and concept docs all reference the same scope model
- **Repo Check**: `node scripts/verify-repo-truth.mjs`
- **Invariants**: no doc should imply that repo visibility automatically grants write authority

# Agent Guardrails

- **No Mysticism-As-Privilege**: “Self-aware” does not mean “allowed to rewrite anything”.
- **Diffs Or It Didn’t Happen**: Require visible diffs and evidence for self-applied changes.
- **Safety First**: Treat auth, secret, and sandbox boundaries as high-impact write surfaces.
