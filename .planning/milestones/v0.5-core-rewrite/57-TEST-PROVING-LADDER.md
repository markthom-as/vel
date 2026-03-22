# Phase 57 Test And Proving Ladder

## Purpose

Turn the Phase 57 contract packet into something that can fail tests instead of only collecting nods.

## Test Tiers

### 1. Unit tests

Use for:

- typed IDs and envelope invariants
- enum and relation naming semantics
- serde/version roundtrips
- ownership-rule helpers

### 2. Contract tests

Use for:

- storage trait behavior
- registry loader/reconciler behavior
- query abstraction behavior
- policy evaluator semantics
- workflow runtime step-shape semantics

### 3. In-memory backend tests

Use for:

- object + relation + SyncLink persistence invariants
- optimistic concurrency behavior
- `WriteIntent` lifecycle
- projection rebuilds
- bootstrap idempotence

### 4. Projection/read-model tests

Use for:

- `source_summary` derivation
- `Availability` derivation
- tombstone visibility flags
- rebuildability/non-authoritative projection guarantees

### 5. Fake adapter integration tests

Use for:

- Todoist adapter contract against fake provider
- Google Calendar adapter contract against fake provider
- ownership mapping
- dry-run, approval, and outward execution stubs

### 6. Real adapter smoke tests

Use for:

- minimal real-provider auth + sync proof
- bounded Google Calendar sync window
- Todoist full-backlog path
- read-only enforcement with real provider posture

### 7. Migration and replay tests

Use for:

- representative old-data import
- migration artifact replay
- idempotence checks
- bootstrap + migration interaction

### 8. Golden explanation tests

Use for:

- `policy.explain`
- `object.explain`
- ownership/conflict explanations
- audit summary rendering

## Proving Ladder Mapping

| Proving Flow | Minimum Test Tier |
| --- | --- |
| typed IDs + envelope serde | unit |
| object/relation/SyncLink persistence | in-memory backend |
| WriteIntent lifecycle | in-memory backend |
| registry seeding | contract + in-memory backend |
| workflow bootstrap/materialization | contract + in-memory backend |
| Todoist fake adapter path | fake adapter integration |
| Google Calendar fake adapter path | fake adapter integration |
| availability explainability | projection/read-model |
| tombstone behavior | projection/read-model + fake adapter integration |
| read-only enforcement | contract + fake adapter integration |
| real provider smoke path | real adapter smoke |
| migration artifact import | migration and replay |

## Requirement

Every major Phase 57 contract seam should map to at least one test tier before later implementation phases claim it is settled.
