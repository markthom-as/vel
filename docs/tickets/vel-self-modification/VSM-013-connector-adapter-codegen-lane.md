---
id: VSM-013
title: Connector Adapter Codegen Lane
status: proposed
priority: P2
owner: platform
labels: [connectors, codegen, integrations]
---

## Summary
Create a bounded generation lane for connector/adapter code so Vel can repair or regenerate glue code without arbitrary edits across the repo.

## Why
Adapters are exactly the kind of brittle surface that benefits from regeneration. Also exactly the kind of place where hand-rolled entropy breeds.

## Scope
- Define generated directory conventions.
- Add source markers and stable interfaces.
- Add reproducible generation command/path.

## Implementation tasks
1. Identify integration layers suitable for codegen.
2. Create templates and output directories.
3. Add generation metadata/markers.
4. Add tests for generated interface compatibility.
5. Wire into validation pipeline.

## Acceptance criteria
- Generated code is isolated and identifiable.
- Generation is reproducible.
- Manual edits to generated regions are detectable or blocked.
- Vel can propose generated diffs without touching protected core.

## Dependencies
- VSM-001, VSM-005.

