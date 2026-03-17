# Vel Config Contract Pack

This directory contains the canonical configuration assets for Vel:

- live config files used by runtime or operators
- checked-in templates for authoring new config files
- JSON schema contracts for machine-readable validation
- example JSON artifacts for connector and self-model envelopes
- a machine-readable contracts manifest for indexers and tooling

The files in `config/` are part of the architecture contract layer and should stay aligned with:

- `crates/vel-config/src/lib.rs`
- `crates/vel-config/src/models.rs`
- `crates/veld/src/policy_config.rs`
- `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`

## Layout

- `agent-specs.yaml`: live agent spec config used by runtime defaults
- `policies.yaml`: live policy config loaded by `veld`
- `contracts-manifest.json`: canonical index of templates, schemas, and examples
- `templates/vel.toml.template`: template for runtime config (`vel.toml`-compatible keys)
- `templates/agent-specs.template.yaml`: template for agent specs
- `templates/policies.template.yaml`: template for policy config
- `schemas/`: machine-readable JSON schemas for major config and envelope contracts
- `examples/`: example JSON payloads for connector manifest and self-model envelope contracts

## Rules

- Keep templates parseable and close to real runtime shapes.
- Keep schemas additive when possible and avoid breaking required fields without a migration plan.
- Do not treat templates as hidden runtime truth; runtime truth comes from loaded files and code.
- If you update a config shape in code, update the matching template and schema in the same change.
