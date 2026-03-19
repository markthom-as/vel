---
title: Cross-Surface Proof Flows
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - proof flow
  - daily loop
  - cross-surface
  - architecture
index_terms:
  - daily loop proof flow
  - backend-owned flow
  - shell consumption
related_files:
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md
  - docs/api/runtime.md
  - docs/user/daily-use.md
  - clients/apple/README.md
  - .planning/phases/10-daily-loop-morning-overview-and-standup-commitment-engine/10-05-SUMMARY.md
summary: Concrete shipped proof flows showing that Vel's cross-surface architecture is executable in the live codebase.
---

# Purpose

Anchor the cross-surface architecture to real shipped flows.

This document exists to answer:

- what a good cross-surface flow looks like in this repo
- which parts already conform to the target architecture
- which remaining gaps belong to later phases

# Proof Flow 1: Daily Loop

The daily loop is the current best proof of the intended architecture.

It is a real shipped flow that already spans:

- Rust-owned product logic
- typed runtime transport
- CLI consumption
- web consumption
- Apple consumption
- operator-facing documentation

## Why Daily Loop Is The Right Proof

The daily loop is a good proof seam because it already demonstrates all of these properties:

- one backend-owned authority for morning and standup behavior
- one typed transport surface under `/v1/daily-loop/*`
- no Apple-only or web-only planning logic
- multiple shells consuming the same product semantics differently

## Flow Chain

### 1. Rust-Owned Logic

The daily loop begins as backend-owned product logic in Rust.

Architecture rule demonstrated:

- planning behavior, commitment limits, prompt sequencing, and resume rules are not owned by any shell

### 2. Typed Runtime Boundary

The runtime authority is the shared daily-loop API:

- `POST /v1/daily-loop/sessions`
- `GET /v1/daily-loop/sessions/active`
- `POST /v1/daily-loop/sessions/:id/turn`

This is the canonical command/query surface for the workflow.

Architecture rule demonstrated:

- the transport boundary is explicit and typed
- shells do not invent parallel transport contracts

### 3. CLI Consumption

The CLI consumes the same flow through:

- `vel morning`
- `vel standup`

Architecture rule demonstrated:

- CLI is a shell over the same workflow, not a second product definition

### 4. Web Consumption

The web `Now` surface consumes the same flow as the daily-use shell entry.

Architecture rule demonstrated:

- web embodies the workflow in its own interface
- web does not own the workflow semantics

### 5. Apple Consumption

Apple voice uses:

- `POST /v1/apple/voice/turn`

But `MorningBriefing` delegates into the same shared backend daily-loop authority.

Architecture rule demonstrated:

- Apple remains a transcript capture and rendering shell
- Apple does not synthesize a separate local morning/standup planner
- offline Apple behavior is cache-only for active session state and queued safe actions

### 6. Operator Documentation

The same flow is reflected in operator docs:

- [docs/api/runtime.md](../../api/runtime.md)
- [docs/user/daily-use.md](../../user/daily-use.md)
- [clients/apple/README.md](../../../clients/apple/README.md)

Architecture rule demonstrated:

- operator docs point to one authority path instead of describing different product truths per shell

# What Already Conforms

The daily loop already conforms to the target architecture in these ways:

- product logic is Rust-owned
- transport is typed and explicit
- CLI, web, and Apple all consume the same workflow
- Apple is thin-shell and transcript-first
- docs identify the same authority path

# What Still Belongs To Later Phases

The daily loop proof does **not** mean the architecture work is finished.

Later phases still need to address:

- more canonical command/query/read-model vocabulary across other product areas
- migration of remaining shell-leaning seams into the same pattern
- product discovery on what belongs in default vs advanced surfaces
- logic-first closure of additional workflows beyond daily loop
- later shell embodiment and simplification work

# Migration Guardrails

When implementing a new shell surface or migrating an existing one, the daily-loop proof implies these rules:

1. Start from a backend-owned command/query workflow.
2. Publish one typed transport/read-model boundary.
3. Let each shell embody the same workflow differently without changing semantics.
4. Keep offline shell behavior bounded; do not let a shell become a local policy fork.
5. Update operator docs so they point to the same authority path.

# Anti-Patterns The Proof Rejects

The daily-loop proof is evidence against:

- Apple-only morning policy
- web-only planner logic
- separate transport shapes per shell
- shell-owned commitment cap logic
- doc drift where CLI, runtime, and Apple each describe a different authority

# Related Future Proofs

The next-best proof seam is `agent inspect`, which already shows:

- backend-owned grounding policy
- typed runtime contract
- CLI and web trust-surface consumption

That flow should be used as a secondary reference in later migration phases.

# Acceptance Criteria

1. At least one live flow is documented end-to-end as evidence for the cross-surface architecture.
2. The proof flow identifies both what already conforms and what remains for later phases.
3. Future migration and shell work can use these guardrails instead of rediscovering the same architecture rules from scratch.
