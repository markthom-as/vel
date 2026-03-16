---
id: VEL-META-009
title: Cross-source entity linking and metadata propagation
status: proposed
priority: P1
estimate: 4-6 days
dependencies: [VEL-META-003, VEL-META-005]
---

# Goal

Let metadata inferred in one source inform enrichment in another.

# Scope

- Build project/person/entity linking helpers.
- Enable propagation patterns such as:
  - email thread -> task tags/project
  - calendar event -> follow-up task project/location context
  - file/doc -> event/task project classification
- Support confidence-weighted linkage.

# Deliverables

- entity linking service
- propagation rules
- provenance chain support
- tests with cross-source fixtures

# Acceptance criteria

- Candidate reasons can cite linked cross-source evidence.
- Conflicting cross-source signals reduce confidence instead of being flattened.
- Propagation can be disabled per source pair.

# Notes

This is where Vel stops seeing isolated objects and starts seeing a world.
