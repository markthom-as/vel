# Phase 57 Verification

**Phase:** 57 - Architecture freeze, canonical contracts, and milestone lock  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 57 can be considered complete as a planning/contract phase.

This is not implementation verification for Todoist, Google Calendar, or the `0.5` backend itself. It is verification that the contract packet is strong enough to support those implementation phases honestly.

## Required Outputs

Phase 57 is expected to leave behind:

- a canonical object/linkage contract
- a membrane/ownership/write-intent contract
- a registry/workflow-runtime contract
- a provider-boundary/migration/proving-flow contract
- a Rust-backend constraints contract
- a backend trait/capability map
- a dependency and invariants sheet
- a test/proving ladder
- an architecture risk spike list

## Verification Checks

### A. Contract coverage

- [ ] `57-01` defines canonical objects, registry entities, read models, runtime records, IDs, envelope, relations, `IntegrationAccount`, and `SyncLink`.
- [ ] `57-02` defines generic action semantics, ownership overlays, conflict rules, explainability, and `WriteIntent`.
- [ ] `57-03` defines registry identities, module/skill/tool/workflow lifecycle classes, seeded workflow rules, and workflow runtime primitives.
- [ ] `57-04` defines Todoist and Google Calendar boundaries, migration artifacts, compatibility DTO exit criteria, and proving flows.
- [ ] `57-05` defines crate/module boundary guidance, storage/runtime/query/secret/scheduler abstractions, feature gating, serialization, concurrency, bootstrap, and testability posture.

### B. Cross-cutting consistency

- [ ] `57-05` is visibly binding across `57-01` through `57-04`.
- [ ] No earlier chunk contradicts the backend constraints doc.
- [ ] No adapter contract silently overrides a core semantic decision from earlier chunks.
- [ ] The dependency/invariants sheet remains aligned with the plans.

### C. Backend executability

- [ ] The packet names the minimum backend trait families needed to implement the architecture.
- [ ] The packet names the minimum target capability assumptions.
- [ ] The packet names the minimum error/concurrency/bootstrap expectations.
- [ ] The packet keeps the domain storage-agnostic and platform-neutral in contract language.

### D. Testability

- [ ] Every major seam maps to at least one rung in the proving ladder.
- [ ] The architecture risk spikes cover the highest-risk Rust seams:
  - typed IDs + serde
  - object/relation/SyncLink persistence
  - `WriteIntent` lifecycle
  - registry seeding
  - one fake adapter path
- [ ] The packet is concrete enough that those spikes can be implemented without reopening Phase 57 ontology questions.

## Suggested Command-Backed Checks

Use these as lightweight verification aids once the packet is updated:

```bash
rg -n "content|registry|read_model|runtime" .planning/milestones/v0.5-core-rewrite/57-*.md
rg -n "SyncLink|source_summary|WriteIntent|policy.explain|object.explain" .planning/milestones/v0.5-core-rewrite/57-*.md
rg -n "module.integration.todoist|module.integration.google-calendar|fork-before-modify|AttachedCommentRecord" .planning/milestones/v0.5-core-rewrite/57-*.md
rg -n "vel_core_types|ObjectStore|RegistryLoader|CredentialProvider|JobScheduler|optimistic concurrency|deterministic bootstrap" .planning/milestones/v0.5-core-rewrite/57-*.md docs/cognitive-agent-architecture/architecture/0.5-rust-backend-implementation-constraints.md docs/cognitive-agent-architecture/architecture/0.5-required-backend-traits-and-capability-matrix.md
```

## Exit Standard

Phase 57 can be treated as verified when:

1. The full packet exists on disk.
2. Each plan chunk has a clear objective, acceptance criteria, and verification posture.
3. Cross-cutting invariants and backend constraints are explicitly referenced by earlier chunks.
4. The packet could be handed to an implementation team and used to create spikes, schemas, and code without reopening the same architectural arguments.

## Not Verified By Phase 57 Alone

Phase 57 does **not** by itself prove:

- the storage traits are correctly implemented
- the adapters actually sync correctly
- the runtime/scheduler works end to end
- the migration engine succeeds on live data

Those belong to later implementation and milestone verification phases.

---

*Verification target for the Phase 57 planning packet*
