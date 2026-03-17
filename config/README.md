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
- Examples and manifests:
  - `examples/connector-manifest.example.json`
  - `examples/self-model-envelope.example.json`
  - `contracts-manifest.json`

## Ownership Rules

- `crates/vel-config` owns runtime config, agent specs, model profiles, and routing config.
- `crates/veld/src/policy_config.rs` owns policy config loading and runtime interpretation.
- `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md` owns the contract map and schema-governance rules.
- `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md` owns connector vocabulary and manifest semantics.
- `docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md` owns self-model and writable-scope semantics.

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
