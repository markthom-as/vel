---
title: Self-Awareness And Supervised Self-Modification
doc_type: spec
status: proposed
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - self-awareness
  - introspection
  - repo visibility
  - self-modification
index_terms:
  - self model
  - code visibility
  - supervised self modification
  - repo-aware agent
related_files:
  - docs/MASTER_PLAN.md
  - docs/cognitive-agent-architecture/cognition/introspection.md
  - docs/cognitive-agent-architecture/agents/tool-access.md
  - docs/cognitive-agent-architecture/policies/trust-and-safety.md
summary: Contract for how Vel may inspect its own runtime, docs, config, repository, and bounded writable surfaces without turning self-modification into ambient authority.
---

# Purpose

Define what it means for Vel to be self-aware, introspective, repository-visible, and capable of supervised self-modification.

# Problem

The repo talks about introspection and self-improvement, but the current guidance is mostly about policy tuning. It does not yet clearly define:

- what Vel may know about its own code and docs
- what surfaces it may inspect automatically
- what surfaces it may modify
- what review and verification gates are mandatory before any self-applied change

Without that clarity, “self-aware” risks meaning either too little or something dangerously vague.

# Goals

- define a bounded self-model for runtime, docs, config, queue, and repository awareness
- separate read visibility from write authority
- define a supervised path for repo-aware code or doc changes
- make self-modification a reviewable, evidence-backed capability rather than a magical ambient power

# Non-Goals

- unrestricted autonomous code rewriting
- silent mutation of product personality, policy, or security posture
- bypassing tickets, tests, traces, or human review

# Current State

Current shipped truth lives in [MASTER_PLAN.md](../../MASTER_PLAN.md).

Today, Vel has introspection concepts and repo-aware coding-agent workflows, but there is no single contract for self-awareness or supervised self-modification.

# Proposed Design

## Self-Model Layers

Vel's self-model should be able to represent:

### Runtime Awareness

- running components
- recent runs and artifacts
- current config and policy paths
- connector health and freshness
- known failure or degraded states

### Contract Awareness

- active schema docs
- config and policy templates
- current authority chain
- relevant tickets and queue position

### Repository Awareness

- file and module layout
- code ownership boundaries
- writable scopes granted for the active task
- diff under construction
- tests and checks required before closing the task

## Read And Write Scopes

Read scope and write scope are distinct.

### Read Scope

Vel may inspect:

- source code
- docs
- tickets
- config files
- templates
- tests

when the active task or operator request requires it.

### Write Scope

Vel may modify only:

- files explicitly within the active task scope
- docs, templates, config examples, or code approved by the task boundary
- files inside allowed writable roots

Write scope must be narrower than read scope and should be recorded explicitly.

## Self-Modification Contract

A supervised self-modification path should require:

1. explicit task or ticket objective
2. explicit writable scope
3. diff visibility
4. automated checks or direct execution evidence
5. traceable summary of what changed and why
6. human review or explicit operator authorization when required

Write classes that should always require explicit operator authorization:

- auth, policy, and secret-boundary changes
- sandbox or capability-boundary changes
- wide-scope repo writes outside ticket-bounded modules

## Scientific Substrate And Symbolic Proposals

Vel's self-awareness should distinguish between:

- scientific substrate: repo state, docs, config, manifests, traces, diffs, and verification evidence
- symbolic proposals: diagnoses, improvement ideas, architectural suggestions, and patch plans

Symbolic proposals may summarize or interpret the substrate, but they must not silently rewrite it without the supervised write path above.

## Self-Model Envelope

Minimal conceptual shape:

```json
{
  "active_task": "ticket_or_request_id",
  "authority_docs": ["docs/MASTER_PLAN.md"],
  "relevant_tickets": ["021-canonical-schema-and-config-contracts.md"],
  "read_scopes": ["docs/**", "crates/**"],
  "write_scopes": ["docs/**", "config/*.template.yaml"],
  "required_checks": [
    "cargo test -p vel-config",
    "node scripts/verify-repo-truth.mjs"
  ],
  "review_gate": "operator_or_ticket_policy"
}
```

## Hard Rules

- self-awareness should improve correctness, not inflate autonomy
- repo visibility does not imply permission to edit
- self-modification may propose broad change, but should only apply narrow, scoped change
- safety, auth, secret, and sandbox boundaries are high-impact surfaces and should require stronger review
- every self-applied change must be explainable from task scope, diff, and verification evidence

# Boundaries

- introspection and self-model logic belong to the authority runtime and its architecture contracts
- writable-scope enforcement belongs to tool and sandbox boundaries
- tickets and authority docs constrain what self-modification may attempt
- runtime type ownership should move to `vel-core` once self-model contracts become active code-level envelopes

# Cross-Cutting Traits

- modularity: required — self-awareness should operate through explicit self-model and scope contracts.
- accessibility: required — the system should be able to explain what it knew and what it was allowed to touch.
- configurability: affected — writable scopes, review gates, and enabled self-modification paths should be explicit.
- data logging and observability: required — repo-aware actions need traces, run IDs, or equivalent evidence.
- rewind/replay: affected — self-applied changes should preserve diff history and verification evidence.
- composability: required — self-awareness should compose with tickets, docs, contracts, and tool scopes instead of bypassing them.

# Operational Considerations

- treat self-modification like a high-impact capability
- prefer doc, schema, and template changes before broad code mutation
- do not let repo-aware behavior outrun the contract and safety layer

# Acceptance Criteria

1. The repo has an explicit self-awareness and supervised-self-modification contract.
2. Read scope and write scope are clearly separated in the spec.
3. Ticket coverage exists for implementing the self-model and guarded code/doc modification paths.

# Open Questions

- Which self-model fields should eventually become runtime types rather than architecture-doc concepts?
- Which run/event schema should carry self-modification review decisions in the runtime?

# Related Terms

- canonical name: self-awareness and supervised self-modification
- aliases: self model, repo-aware introspection, bounded self-modification
- related packs or subsystems: introspection, tool access, trust and safety, tickets

# Search Terms

- self-awareness
- self modification
- repo visibility
- self model
