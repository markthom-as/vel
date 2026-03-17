# Vel Cognitive + Agent Architecture Pack

This repository defines the **cognitive, orchestration, and introspective architecture**
for Vel. It extends the product spec pack by specifying how Vel should think, adapt,
coordinate agents, and operate across devices.

These documents are intended to be **agent-ready** and **implementation-guiding**.

## Scope

This pack covers:

- cognitive architecture
- event bus + state graph
- memory model
- introspection and self-improvement
- agent orchestration
- voice stack
- plugin and integration model
- cross-device architecture

## Relationship to Product Specs

The product spec pack defines **what the user experiences**.

This pack defines **how Vel internally behaves and evolves**.

If implementation is ambiguous, preserve:

1. user trust
2. observability
3. reversibility
4. conceptual integrity

## Reading Order

Start here:

1. [`00-overarching-architecture-and-concept-spec.md`](00-overarching-architecture-and-concept-spec.md)
2. [`01-cross-cutting-system-traits.md`](01-cross-cutting-system-traits.md)
3. [`architecture/README.md`](architecture/README.md)
4. [`architecture/canonical-schemas-and-contracts.md`](architecture/canonical-schemas-and-contracts.md)
5. [`integrations/canonical-data-sources-and-connectors.md`](integrations/canonical-data-sources-and-connectors.md)
6. [`cognition/self-awareness-and-supervised-self-modification.md`](cognition/self-awareness-and-supervised-self-modification.md)
7. [`agents/orchestrator.md`](agents/orchestrator.md)
8. [`agents/tool-access.md`](agents/tool-access.md)
9. [`policies/trust-and-safety.md`](policies/trust-and-safety.md)
10. [`implementation/roadmap.md`](implementation/roadmap.md)

## Sub-Pack Entry Points

- [`architecture/README.md`](architecture/README.md) for eventing, state graph, and cross-subsystem runtime architecture docs
- [`architecture/canonical-schemas-and-contracts.md`](architecture/canonical-schemas-and-contracts.md) for schema ownership, config contracts, manifests, and templates
- [`agents/orchestrator.md`](agents/orchestrator.md) for orchestration and supervision
- [`cognition/context-model.md`](cognition/context-model.md) for cognition-state shape and interpretation
- [`cognition/self-awareness-and-supervised-self-modification.md`](cognition/self-awareness-and-supervised-self-modification.md) for bounded repo visibility, self-modeling, and supervised code changes
- [`integrations/canonical-data-sources-and-connectors.md`](integrations/canonical-data-sources-and-connectors.md) for integration families, source modes, and connector contracts
- [`devices/cross-device-architecture.md`](devices/cross-device-architecture.md) for multi-device sync and client topology

## Durable Principles

- prefer one orchestrator by default, then add bounded specialists only when the split is explicit and reviewable
- mediate capabilities and secrets through narrow boundaries instead of handing raw access to agents
- require execution-backed verification and traces for meaningful agent work
- fail closed on unknown routes, tools, and external-access requests
- treat modularity, accessibility, configurability, logging, replay, and composability as required cross-cutting traits
- treat walkthroughs, fixtures, prompts, and verification recipes as reusable architecture assets
