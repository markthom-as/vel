---
title: Schema Or Manifest Spec
doc_type: spec
status: proposed
owner: team-or-person
created: YYYY-MM-DD
updated: YYYY-MM-DD
keywords:
  - schema
  - manifest
  - contract
index_terms:
  - machine readable contract
  - schema publication
  - envelope definition
related_files:
  - docs/MASTER_PLAN.md
  - docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md
summary: Define a machine-readable schema or manifest surface, its versioning policy, and publication/consumption rules.
---

# Purpose

Describe the contract this schema or manifest defines and why a machine-readable artifact is required.

# Contract Scope

- owner crate or doc authority
- producers
- consumers
- non-goals

# Canonical Shape

Describe required fields, optional fields, invariants, and validation behavior.

Include an example object:

```json
{
  "schema_version": "v1",
  "example_field": "value"
}
```

# Versioning And Migration

- compatibility rules
- breaking change policy
- migration expectations

# Publication

- artifact path
- registry or manifest listing path
- update workflow when fields change

# Verification

- parser or loader tests
- schema validation checks
- integration smoke checks

# Cross-Cutting Traits

- modularity
- accessibility
- configurability
- data logging and observability
- rewind/replay
- composability
