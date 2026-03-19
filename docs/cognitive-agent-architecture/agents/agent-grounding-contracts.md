---
title: Agent Grounding Contracts
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - agents
  - grounding
  - trust
  - phase-11
summary: Canonical Phase 11 contract vocabulary for grounded supervised-agent state, persisted review obligations, and bounded operator-facing capability summaries.
---

# Purpose

Publish the stable Phase 11 grounding boundary before runtime/API, CLI, or web consumers widen.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Grounding and inspect transport DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |
| Grounding pack schema/example publication | config assets | `config/schemas/agent-grounding-pack.schema.json`, `config/examples/agent-grounding-pack.example.json` |
| Inspect schema/example publication | config assets | `config/schemas/agent-inspect.schema.json`, `config/examples/agent-inspect.example.json` |

# Stable Vocabulary

## Top-level transport DTOs

- `AgentGroundingPackData`
- `AgentInspectData`
- `AgentReviewObligationsData`
- `AgentCapabilitySummaryData`
- `AgentCapabilityGroupData`
- `AgentCapabilityEntryData`
- `AgentBlockerData`
- `AgentContextRefData`

## Capability groups

- `read_context`
- `review_actions`
- `mutation_actions`

## Persisted seams reused by this contract

- `NowData` for the typed operator-facing state bundle
- `ProjectRecordData`
- `PersonRecordData`
- `CommitmentData`
- `WritebackOperationData`
- `ConflictCaseData`
- `ExecutionHandoffRecordData` for persisted handoff review metadata

# Contract Rules

- `AgentInspectData` is the single inspect payload for Phase 11 grounding. Runtime/API, CLI, web, and execution-export consumers should share this shape instead of introducing parallel inspect models.
- `AgentGroundingPackData` reuses shipped typed records wherever possible. The grounding boundary is a typed summary pack, not a second raw storage dump.
- Current-context and explainability links stay explicit through `AgentContextRefData`; raw context JSON is supporting evidence, not the primary contract.
- Review obligations stay operator-visible and persisted. Pending writebacks, conflicts, and execution handoffs remain first-class instead of being compressed into generic warning strings.
- Capability summaries must stay grouped in operator terms and fail closed. If a capability is unavailable, `blocked_reason` and optional escalation guidance explain why.
- Mutation affordances do not bypass SAFE MODE, writeback gates, or existing handoff review gates. Phase 11 makes those constraints inspectable; it does not weaken them.

# Published Artifacts

- `config/schemas/agent-grounding-pack.schema.json`
- `config/examples/agent-grounding-pack.example.json`
- `config/schemas/agent-inspect.schema.json`
- `config/examples/agent-inspect.example.json`

# Downstream Usage

- Backend services should assemble `AgentGroundingPackData` from persisted `Now`, projects, people, commitments, review queues, and execution handoff records instead of inventing a second agent-state model.
- Runtime/API surfaces should expose `AgentInspectData` directly and keep policy decisions in Rust.
- CLI and web surfaces should render the same grouped capabilities and blocker semantics without re-deriving availability client-side.
- Execution-export or handoff-preparation flows may embed or reference the grounding pack, but they should preserve the same typed inspect vocabulary and review-gate semantics.
