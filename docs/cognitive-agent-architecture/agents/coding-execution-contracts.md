---
title: Coding Execution Contracts
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - execution
  - connect
  - coding
  - phase-8
summary: Canonical Phase 08 contract vocabulary for repo-aware coding execution context, local-agent manifests, routing inputs, and supervised handoffs.
---

# Purpose

Publish the stable Phase 08 contract boundary before connect transport, routing, or runtime execution widen further.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Execution domain vocabulary | `vel-core` | `crates/vel-core/src/execution.rs` |
| Handoff envelope extension | `vel-core` | `crates/vel-core/src/run.rs` |
| Transport DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |
| Protocol manifest reference | `vel-protocol` | `crates/vel-protocol/src/lib.rs` |
| Schema/example publication | config assets | `config/schemas/project-execution-context.schema.json`, `config/schemas/execution-handoff.schema.json`, `config/schemas/local-agent-manifest.schema.json` |

# Stable Vocabulary

## Execution task kinds

- `planning`
- `implementation`
- `debugging`
- `review`
- `research`
- `documentation`

## Agent profiles

- `budget`
- `balanced`
- `quality`
- `inherit`

## Token budget classes

- `small`
- `medium`
- `large`
- `xlarge`

## Review gates

- `none`
- `operator_approval`
- `operator_preview`
- `post_run_review`

## Local runtime kinds

- `local_cli`
- `wasm_guest`

# Contract Rules

- Execution context is rooted in the project's declared primary repo and notes roots, not ad hoc runtime metadata.
- Write scope must be explicit in both durable execution context and per-handoff envelopes. Read visibility may be broader; write authority may not.
- Local-agent manifests are capability allowlists, not ambient permission grants.
- Repo-local GSD artifacts are sidecar inputs to supervised execution. They do not authorize writes outside the declared project roots.
- Routing policy inputs must keep task kind, agent profile, token budget, review gate, and network need first-class rather than inferred.
- Coding handoffs must remain trace-linked and project-linked so later review can explain why a runtime was launched and what it was allowed to touch.

# Published Artifacts

- `config/schemas/project-execution-context.schema.json`
- `config/examples/project-execution-context.example.json`
- `config/schemas/execution-handoff.schema.json`
- `config/examples/execution-handoff.example.json`
- `config/schemas/local-agent-manifest.schema.json`
- `config/examples/local-agent-manifest.example.json`

# Downstream Usage

- Storage and service layers should persist and serve execution context using these typed fields rather than a second project-metadata blob.
- Connect transport should carry manifest references and enforce runtime kind plus writable-root checks against the published contract.
- CLI and operator surfaces may preview or export repo-local sidecars from this context, but policy ownership stays in Rust backend layers.
