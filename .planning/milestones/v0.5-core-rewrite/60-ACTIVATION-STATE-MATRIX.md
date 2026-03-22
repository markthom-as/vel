# Phase 60 Activation State Matrix

## Purpose

Keep registration and activation vocabulary precise so Phase 60 does not drift into runtime execution semantics.

## State Definitions

- `registered`: canonical registry entity exists with stable identity.
- `valid`: manifest or seeded definition passed structural validation.
- `reconciled`: persisted state has been compared against incoming manifest/seed truth and assigned a reconciliation result.
- `seeded`: entity was materialized by bootstrap from shipped/core or bundled module input.
- `eligible`: policy and feature posture allow activation in the current environment.
- `activated`: entity is enabled and available for later runtime use through lawful downstream paths.
- `invokable`: runtime phase has actually made the entity callable or executable in context. This is downstream of Phase 60.
- `disabled`: entity is intentionally unavailable by config, policy, or feature posture.
- `deprecated`: entity remains present but is marked for replacement or retirement.

## Matrix

| Entity | registered | valid | reconciled | seeded | eligible | activated | invokable | disabled | deprecated |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Module | yes | yes | yes | sometimes | yes | yes | downstream | yes | yes |
| Skill | yes | yes | yes | sometimes | via module/policy | via module/policy | downstream | yes | yes |
| Tool | yes | yes | yes | sometimes | via module/policy | via module/policy | downstream | yes | yes |
| Seeded Workflow | yes | yes | yes | yes | via module/policy | bootstrap-enabled only | downstream | yes | yes |

## Rule

Phase 60 owns:

- registration
- validation
- reconciliation
- seeding
- activation eligibility and enablement

Phase 60 does not own:

- actual workflow execution
- actual skill/tool invocation
- provider sync or outward writes
- runtime ownership/conflict resolution during invocation
