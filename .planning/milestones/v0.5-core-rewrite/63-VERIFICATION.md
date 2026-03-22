# Phase 63 Verification

**Phase:** 63 - Todoist multi-account adapter and canonical task cut-in  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 63 can be considered complete as the task-side proving adapter phase.

## Required Outputs

Phase 63 should leave behind:

- multi-account Todoist linking over canonical `IntegrationAccount`
- backlog import and `SyncLink`-based cut-in
- canonical task/project/tag/comment mapping for Todoist data
- an explicit provider-history/event ingestion posture distinct from current task state
- ownership-aware sync, tombstones, and conservative outward-write bridging
- black-box tests proving Todoist acts constitutionally over the core

## Verification Checks

### A. Multi-account and linkage

- [ ] Multiple Todoist accounts can coexist lawfully.
- [ ] Todoist linkage uses canonical `SyncLink` posture.
- [ ] Backlog import creates canonical objects rather than mirroring payloads.
- [ ] The adapter posture keeps room for incremental sync and provider-history ingestion without collapsing state and history into one layer.

### B. Canonical task mapping

- [ ] Tasks and projects map into canonical Vel objects.
- [ ] `task_type` is first-class and at least one tag-to-`task_semantics` interpretation path is proved without losing raw tags.
- [ ] Sections remain non-first-class.
- [ ] Labels become canonical tags plus provider facets.
- [ ] Comments remain attached records rather than canonical messages by default.

### C. Sync and write posture

- [ ] Source-owned precedence is enforced.
- [ ] Upstream deletes become tombstones with reconciliation state.
- [ ] Outward writes remain mediated by policy and `WriteIntent`.
- [ ] Read-only and denial paths are verifiable.

### D. Adapter proof

- [ ] Todoist black-box tests pass.
- [ ] Refusal/error-surface tests pass.
- [ ] The adapter proves the task-side MVP without reopening core ontology or membrane law.
- [ ] Provider-derived history needed for reschedule/rewrite/churn analysis is explicitly preserved as a distinct concern from current canonical task state.
- [ ] At least one derived analytics proof exists for reschedule or rewrite history.

## Suggested Command-Backed Checks

```bash
rg -n "IntegrationAccount|SyncLink|remote_id|account" crates/vel-adapters-todoist/src crates/veld/tests
rg -n "Task|Project|Tag|AttachedCommentRecord|section|provider facet" crates/vel-adapters-todoist/src crates/veld/tests
rg -n "source-owned|tombstone|WriteIntent|PolicyDenied|ReadOnlyViolation|OwnershipConflict" crates/vel-adapters-todoist/src crates/veld/src/services crates/veld/tests
```

## Exit Standard

Phase 63 is verified when Todoist proves the task-side MVP through canonical mapping, multi-account sync, tombstones, and mediated writes, while remaining a constitutional adapter over the `0.5` core.

---

*Verification target for the Phase 63 planning packet*
