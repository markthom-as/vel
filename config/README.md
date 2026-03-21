# Vel Config Asset Map

This directory is the canonical entrypoint for checked-in configuration assets that support Vel's runtime contracts.

Use it to answer four questions quickly:

1. which files are live configuration
2. which files are templates or examples
3. which files are machine-readable schemas
4. where the canonical contract manifest lives

## Asset Classes

- Live configs:
  - `../vel.toml`
  - `agent-specs.yaml`
  - `policies.yaml`
  - `../configs/models/*.toml`
  - `../configs/models/routing.toml`
- Templates:
  - `templates/vel.toml.template`
  - `templates/agent-specs.template.yaml`
  - `templates/policies.template.yaml`
  - `../configs/models/templates/profile.template.toml`
  - `../configs/models/templates/routing.template.toml`
- Machine-readable schemas:
  - `schemas/app-config.schema.json`
  - `schemas/agent-specs.schema.json`
  - `schemas/policies.schema.json`
  - `schemas/model-profile.schema.json`
  - `schemas/model-routing.schema.json`
  - `schemas/connector-manifest.schema.json`
  - `schemas/self-model-envelope.schema.json`
- `schemas/semantic-query.schema.json`
- `schemas/semantic-memory-record.schema.json`
- `schemas/apple-voice-turn.schema.json`
- `schemas/apple-behavior-summary.schema.json`
- `schemas/project-workspace.schema.json`
- `schemas/project-execution-context.schema.json`
- `schemas/execution-handoff.schema.json`
- `schemas/local-agent-manifest.schema.json`
- `schemas/operator-action-item.schema.json`
- `schemas/canonical-scheduler-rules.schema.json`
- `schemas/day-plan-proposal.schema.json`
- `schemas/commitment-scheduling-proposal.schema.json`
- `schemas/routine-planning-profile.schema.json`
- `schemas/planning-profile-mutation.schema.json`
- `schemas/planning-profile-edit-proposal.schema.json`
- `schemas/apple-embedded-runtime-profile.schema.json`
- `schemas/apple-local-voice-continuity.schema.json`
- `schemas/writeback-operation.schema.json`
- `schemas/person-record.schema.json`
- `schemas/sandbox-policy.schema.json`
- `schemas/sandbox-host-call.schema.json`
- `schemas/swarm-protocol-envelope.schema.json`
- `schemas/backup-manifest.schema.json`
- Examples and manifests:
  - `examples/app-config.example.toml`
  - `examples/agent-specs.example.yaml`
  - `examples/policies.example.yaml`
  - `examples/model-profile.example.toml`
  - `examples/model-routing.example.toml`
  - `examples/connector-manifest.example.json`
  - `examples/self-model-envelope.example.json`
  - `examples/semantic-query.example.json`
  - `examples/semantic-memory-record.example.json`
  - `examples/apple-voice-turn.example.json`
  - `examples/apple-behavior-summary.example.json`
  - `examples/project-workspace.example.json`
  - `examples/project-execution-context.example.json`
  - `examples/execution-handoff.example.json`
  - `examples/local-agent-manifest.example.json`
  - `examples/operator-action-item.example.json`
  - `examples/canonical-scheduler-rules.example.json`
  - `examples/day-plan-proposal.example.json`
  - `examples/commitment-scheduling-proposal.example.json`
  - `examples/routine-planning-profile.example.json`
  - `examples/planning-profile-mutation.example.json`
  - `examples/planning-profile-edit-proposal.example.json`
  - `examples/apple-embedded-runtime-profile.example.json`
  - `examples/apple-local-voice-continuity.example.json`
  - `examples/writeback-operation.example.json`
  - `examples/person-record.example.json`
  - `examples/sandbox-host-call.example.json`
  - `examples/swarm-protocol-envelope.example.json`
  - `examples/backup-manifest.example.json`
  - `contracts-manifest.json`

## Ownership Rules

- `crates/vel-config` owns runtime config, agent specs, model profiles, and routing config.
- `crates/vel-config::AppConfig.now` owns governed `Now` presentation policy such as title mode, count-display mode, and reduced-watch parity flags.
- `crates/veld/src/policy_config.rs` owns policy config loading and runtime interpretation.
- `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md` owns the contract map and schema-governance rules.
- `docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md` owns the cross-surface command/query/read-model versus transport vocabulary used by Apple, web, CLI, and future desktop shells.
- `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md` owns connector vocabulary and manifest semantics.
- `docs/cognitive-agent-architecture/integrations/writeback-and-conflict-contracts.md` owns the Phase 06 write-back, conflict, and people contract vocabulary.
- `docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md` owns self-model and writable-scope semantics.
- `docs/cognitive-agent-architecture/architecture/project-action-linking-contracts.md` owns the Phase 05 typed project, operator-action, and linking vocabulary.
- `docs/cognitive-agent-architecture/apple/apple-action-loop-contracts.md` owns the Phase 07 Apple voice, schedule, and bounded behavior-summary vocabulary.
- `docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md` owns the Phase 37 embedded-capable Apple runtime and daemon-vs-embedded boundary vocabulary.
- `docs/cognitive-agent-architecture/apple/apple-local-first-voice-continuity-contract.md` owns the Phase 38 iPhone local-first voice continuity, cached `Now`, and offline quick-action vocabulary.
- `docs/cognitive-agent-architecture/agents/coding-execution-contracts.md` owns the Phase 08 repo-aware execution-context, local-manifest, and supervised handoff vocabulary.
- `docs/cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md` owns the Phase 09 backup manifest, inspection, and manual-first restore vocabulary.
- `docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md` owns the durable planning-profile input vocabulary.
- `docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md` owns the typed planning-profile management mutation vocabulary.
- `docs/cognitive-agent-architecture/architecture/planning-profile-parity-contract.md` owns the cross-surface parity and assistant-managed planning-profile edit vocabulary.
- `docs/cognitive-agent-architecture/architecture/planning-profile-application-contract.md` owns the supervised planning-profile proposal lifecycle and approved-application vocabulary.
- `docs/cognitive-agent-architecture/architecture/day-plan-application-contract.md` owns the supervised same-day `day_plan` / `reflow` application vocabulary over commitment scheduling.

## Scientific Substrate vs Symbolic Layer

Vel should keep its measurable substrate separate from its interpretive outputs.

- Scientific substrate:
  - configs
  - templates
  - schemas
  - manifests
  - traces
  - persisted records
- Symbolic layer:
  - syntheses
  - hypotheses
  - suggestions
  - narratives
  - reflective proposals

Symbolic outputs may reference the scientific substrate, but they must not silently override it.

## Maintenance Rule

When a durable config or contract changes, update these together in one slice when applicable:

1. the loader or typed owner in code
2. the checked-in live config or example
3. the checked-in template
4. the machine-readable schema or manifest
5. the governing architecture doc or ticket
6. the verification that parses or checks the asset

For `vel.toml`-backed `Now` policy, keep the `[now]` and `[now.watch]` sections aligned across `AppConfig`, `config/examples/app-config.example.toml`, `config/templates/vel.toml.template`, and `config/schemas/app-config.schema.json`.
