---
title: Canonical Schemas And Contracts
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - schema
  - contract
  - manifest
  - config
  - policy
index_terms:
  - object definitions
  - contract catalog
  - config schema
  - policy schema
  - manifest catalog
related_files:
  - docs/MASTER_PLAN.md
  - docs/tickets/phase-1/024-machine-readable-schema-and-manifest-publication.md
  - docs/tickets/phase-1/025-config-and-contract-fixture-parity.md
  - docs/templates/spec-template.md
  - docs/cognitive-agent-architecture/integrations/data-source-catalog.md
  - vel.toml
  - config/agent-specs.yaml
  - config/policies.yaml
summary: Canonical contract catalog for the main Vel objects, config files, policy files, manifests, and schema-governance rules.
---

# Purpose

Define the canonical contract layer for Vel so data shapes, manifests, config files, and policy files are owned explicitly instead of drifting across code, docs, and clients.

# Problem

The repo has real schema-bearing objects and files, but they are scattered across Rust structs, YAML files, TOML files, DTOs, and docs:

- `AppConfig` exists in code and `vel.toml`
- policy config exists in `config/policies.yaml` and `PolicyConfig`
- agent specs exist in `config/agent-specs.yaml` and `AgentSpec`
- integration objects exist in `vel-core`, storage, API DTOs, and UI decoders
- handoffs and manifests exist in docs and DTOs
- `CurrentContext` still straddles typed intent and untyped JSON

Without one canonical catalog, contributors have to rediscover the contract map by grep.

# Goals

- name the owner and boundary for each major schema-bearing object
- make versioning and serialization rules explicit
- provide canonical templates or examples for config-bearing files
- reduce duplication between code, docs, tickets, and surfaced tooling

# Non-Goals

- replacing Rust types with external schema tooling immediately
- introducing speculative codegen before schema ownership is clear

# Current State

Current shipped truth lives in [MASTER_PLAN.md](../../MASTER_PLAN.md).

Today the contract surface is partially explicit but not yet unified.

# Proposed Design

## Contract Inventory

| Contract | Owner | Primary File(s) | Boundary | Versioning Rule | Current State |
| --- | --- | --- | --- | --- | --- |
| App runtime config | `vel-config` | `crates/vel-config/src/lib.rs`, `vel.toml`, `config/templates/vel.toml.template`, `config/examples/app-config.example.toml`, `config/schemas/app-config.schema.json` | runtime config | additive fields, documented defaults | template, schema, and canonical example are present; machine-readable publication now needs consumer rollout |
| Agent spec config | `vel-config` | `crates/vel-config/src/lib.rs`, `config/agent-specs.yaml`, `config/templates/agent-specs.template.yaml`, `config/examples/agent-specs.example.yaml`, `config/schemas/agent-specs.schema.json` | supervised-agent config | validate-on-load, explicit field rules | template, schema, and canonical example are present; broader fixture use is tracked separately |
| Policy config | `veld` | `crates/veld/src/policy_config.rs`, `config/policies.yaml`, `config/templates/policies.template.yaml`, `config/examples/policies.example.yaml`, `config/schemas/policies.schema.json` | runtime policy config | validate-on-load, explicit defaults | template, schema, and canonical example are present; runtime enforcement remains the code owner |
| Model profile config | `vel-config` | `crates/vel-config/src/models.rs`, `configs/models/*.toml`, `configs/models/templates/profile.template.toml`, `config/schemas/model-profile.schema.json` | model backend config | per-file profile contracts | template and schema present; client/tooling consumption is still narrow |
| Routing config | `vel-config` | `crates/vel-config/src/models.rs`, `configs/models/routing.toml`, `configs/models/templates/routing.template.toml`, `config/schemas/model-routing.schema.json` | task-to-profile routing | additive task classes | template and schema present; effective-config inspection is still future work |
| Current context | target `vel-core` | `docs/tickets/phase-1/002-typed-context-transition.md`, `docs/cognitive-agent-architecture/cognition/context-model.md` | cognition state | explicit `CurrentContextVn` structs | still partially JSON-backed in runtime code |
| Integration connection model | `vel-core` | `crates/vel-core/src/integration.rs`, `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md`, `docs/cognitive-agent-architecture/integrations/data-source-catalog.md`, `config/schemas/connector-manifest.schema.json` | domain + API/storage mapping | additive typed records | catalog, example, and schema exist; full runtime alignment is still queued |
| Connect capability manifest | `vel-core` + `vel-api-types` | `crates/vel-core/src/connect.rs`, `crates/vel-api-types/src/lib.rs` | worker/connect capability boundary | explicit manifest shape | route coverage and consumer surfaces are still partial |
| Handoff envelope | architecture pack | `docs/cognitive-agent-architecture/agents/handoffs.md` | agent-to-agent boundary | version by envelope schema, not vibes | still needs machine-readable publication and shared consumption |
| Self-model | architecture pack | `docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md`, `config/examples/self-model-envelope.example.json`, `config/schemas/self-model-envelope.schema.json` | repo/runtime introspection boundary | versioned self-model contract | doc, example, and schema exist; runtime enforcement is still future work |
| Semantic query + record | `vel-core` | `crates/vel-core/src/semantic.rs`, `docs/cognitive-agent-architecture/cognition/semantic-memory-contract.md`, `config/schemas/semantic-query.schema.json`, `config/schemas/semantic-memory-record.schema.json` | semantic retrieval boundary | additive fields with explicit embedding revision | contract published; runtime indexing remains future phase work |
| Sandbox host ABI + policy | `vel-core` | `crates/vel-core/src/sandbox.rs`, `docs/cognitive-agent-architecture/agents/sandbox-host-abi.md`, `config/schemas/sandbox-policy.schema.json`, `config/schemas/sandbox-host-call.schema.json` | sandbox-to-host boundary | explicit ABI version field | contract published; runtime host still unimplemented |
| Swarm protocol envelope | `vel-core` now, `vel-protocol` later | `crates/vel-core/src/protocol.rs`, `docs/cognitive-agent-architecture/architecture/swarm-protocol-contract.md`, `config/schemas/swarm-protocol-envelope.schema.json` | external limb protocol boundary | protocol version in every envelope | contract published now; dedicated protocol crate remains future phase work |

## Schema Governance Rules

### Ownership

- Domain object definitions live in `vel-core` unless they are purely transport DTOs.
- Config object definitions live next to the loader and validator that enforce them.
- Architecture-only contracts may begin in docs, but implementation should move the stable shape into code once the boundary is active.

### Serialization

- Serialize at the boundary, not deep inside services.
- Prefer typed structs in core and service logic.
- Keep raw `serde_json::Value` for envelope-style extension points, evidence payloads, or staged migrations, not as the default business object.

### Versioning

- Version stateful or replay-sensitive schemas explicitly when compatibility matters.
- Add fields compatibly whenever possible.
- If a schema cannot evolve compatibly, add a new versioned object or migration path instead of mutating semantics in place.

### Templates And Fixtures

The repo keeps checked-in templates and examples for human-authored and shared contract surfaces:

- `config/templates/vel.toml.template`
- `config/templates/agent-specs.template.yaml`
- `config/templates/policies.template.yaml`
- `config/examples/app-config.example.toml`
- `config/examples/agent-specs.example.yaml`
- `config/examples/policies.example.yaml`
- `configs/models/templates/profile.template.toml`
- `configs/models/templates/routing.template.toml`
- `config/examples/model-profile.example.toml`
- `config/examples/model-routing.example.toml`
- `config/examples/connector-manifest.example.json`
- `config/examples/self-model-envelope.example.json`
- `config/examples/semantic-query.example.json`
- `config/examples/semantic-memory-record.example.json`
- `config/examples/sandbox-host-call.example.json`
- `config/examples/swarm-protocol-envelope.example.json`

Templates and fixtures are examples and scaffolds, not hidden sources of truth. They should stay parseable and aligned with ticket `025-config-and-contract-fixture-parity.md`.

### Machine-Readable Publication

Contract surfaces should be publishable in machine-readable form for clients and tooling.

Current published artifacts live under `config/schemas/` and are indexed by `config/contracts-manifest.json`.
Shared consumers now include `vel-cli` docs output and runtime doctor diagnostics through `vel-config`'s published-manifest loader; additional client surface adoption can extend the same registry.
Phase 4 contract publications for semantic memory, sandbox ABI, and swarm protocol should follow the same manifest-backed pattern before runtime implementations widen.

### Scientific Substrate And Symbolic Layer

Vel should separate objective, inspectable substrate from interpretive outputs:

- scientific substrate: configs, templates, schemas, manifests, traces, persisted records, typed state
- symbolic layer: syntheses, hypotheses, nudges, narratives, reflective proposals

The symbolic layer may interpret the scientific substrate, but it must not silently replace or mutate it.

## Minimal Object Definitions

### Runtime Config (`AppConfig`)

The runtime config should answer:

- where the authority runtime binds
- where durable state lives
- where artifacts live
- where local sources live
- which model and agent-spec files are active

### Policy Config (`PolicyConfig`)

The policy config should answer:

- which loops are enabled and on what cadence
- how queued work retries behave
- which suggestion heuristics are active
- which human-impacting policy thresholds are active

### Agent Spec (`AgentSpec`)

Every agent spec must define:

- role identity
- mission
- TTL
- allowed tools
- memory scope
- return contract
- execution budgets when needed

### Connector Manifest

Every connector or integration-facing capability should eventually declare:

- integration family
- provider key
- source mode
- auth mode
- capability scope
- output entities and signals
- freshness and failure semantics
- write permissions

# Boundaries

- `vel-core` owns stable domain contracts.
- `vel-config` owns runtime config, model profiles, routing config, and agent-spec loading contracts.
- `veld` owns policy config loading and runtime-only policy interpretation.
- architecture docs own pre-implementation contract clarification when code does not yet have a stable type.

# Cross-Cutting Traits

- modularity: required — every schema needs a named owner and boundary.
- accessibility: required — canonical templates and docs must be discoverable without code archaeology.
- configurability: required — config-bearing contracts need explicit defaults and effective-config semantics.
- data logging and observability: affected — manifests and envelopes should support stable tracing and evidence fields.
- rewind/replay: required — replay-sensitive state shapes should be versioned and migration-aware.
- composability: required — connector manifests, handoff envelopes, and typed configs should compose across clients and runtimes.

# Operational Considerations

- add parser/loader tests for checked-in templates and examples
- keep templates valid and parseable
- do not let templates become shadow configs with behavior not represented in code
- publish machine-readable schema resources and a contract manifest for discoverability

# Acceptance Criteria

1. The main schema-bearing objects have a canonical owner and boundary.
2. Checked-in config templates exist for the primary human-authored config surfaces.
3. Tickets can refer to this doc instead of restating schema ownership ad hoc.

# Open Questions

- Which schema surfaces should eventually move to generated JSON Schema or similar machine-readable output?
- Should connect manifests and handoff envelopes live in Rust types first or docs first?

# Related Terms

- canonical name: schema and contract catalog
- aliases: object definition catalog, manifest catalog, config schema map
- related packs or subsystems: typed context, integrations, connect, agent specs, policies

# Search Terms

- schema catalog
- config contract
- object definitions
- manifest catalog
