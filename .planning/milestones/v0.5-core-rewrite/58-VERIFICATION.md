# Phase 58 Verification

**Phase:** 58 - Canonical object kernel and system-of-record storage rewrite  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 58 can be considered complete as a substrate implementation phase.

## Required Outputs

Phase 58 should leave behind:

- typed IDs and durable envelope implementation
- storage backend/store traits in code
- canonical object, registry, relation, SyncLink, integration-account, runtime-record, and projection persistence seams
- deterministic bootstrap and migration-artifact scaffolding
- storage-neutral query and projection rebuild seams
- narrow proving tests for the highest-risk substrate behavior

## Verification Checks

### A. Storage substrate

- [ ] Canonical objects persist through dedicated seams.
- [ ] Registry entities persist through dedicated seams.
- [ ] Relations persist directionally and typed.
- [ ] SyncLinks remain external linkage truth.
- [ ] Runtime records remain outside canonical content storage.

### B. Backend safety

- [ ] Core domain remains storage-agnostic.
- [ ] Typed ID newtypes are used in the backend model.
- [ ] Deterministic bootstrap and migration replay are implemented.
- [ ] Query and projection behavior stay storage-neutral and rebuildable.

### C. Test/proving coverage

- [ ] There is test coverage for typed ID/envelope roundtrip.
- [ ] There is test coverage for object/relation/SyncLink persistence.
- [ ] There is test coverage for WriteIntent-ready runtime-record persistence scaffolding.
- [ ] There is test coverage for bootstrap idempotence and migration replay.
- [ ] There is test coverage for tombstone/query/projection behavior.

## Suggested Command-Backed Checks

```bash
rg -n "TaskId|WorkflowId|IntegrationAccountId|SyncLinkId" crates/vel-core/src
rg -n "ObjectStore|RegistryStore|RelationStore|SyncLinkStore|RuntimeStore|ProjectionStore" crates/vel-storage/src
rg -n "source_summary|relation_type|write_intent|deleted_upstream|rebuild|include_deleted" crates/vel-storage/src crates/vel-storage/tests
```

## Exit Standard

Phase 58 is verified when the new canonical storage substrate exists in code, respects the Phase 57 contract packet, and is demonstrated through narrow substrate-level tests rather than vague future intent.

---

*Verification target for the Phase 58 planning packet*
