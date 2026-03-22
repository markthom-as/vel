# Phase 61 Verification

**Phase:** 61 - Workflow and skill primitives over canonical objects  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 61 can be considered complete as the minimal workflow-runtime phase.

## Required Outputs

Phase 61 should leave behind:

- typed workflow context-binding and minimal step contracts
- explicit workflow grant envelopes and mediated skill invocation
- run-record and approval runtime types
- manual workflow runner behavior over the membrane
- explicit runtime state machine and dry-run/refusal semantics
- black-box tests for manual invocation, refusal, approval, and mediated runtime behavior

## Verification Checks

### A. Context and step law

- [ ] Workflow context binding is typed and canonical-object-based.
- [ ] Step taxonomy remains minimal and explicit.

### B. Authority and mediation

- [ ] Workflow grants are explicit and narrowing.
- [ ] Skills remain mediated and do not bypass the membrane.
- [ ] Module activation and runtime invocation remain distinct.

### C. Runtime evidence

- [ ] Run records are emitted as runtime/control evidence.
- [ ] Approval pause and dry-run behavior are representable and tested.
- [ ] Manual invocation works without trigger infrastructure.
- [ ] Dry-run does not mutate canonical content or perform irreversible external mutation.
- [ ] Run records, audit entries, approval records, and `WriteIntent` records remain distinct.

### D. Black-box proof

- [ ] Manual workflow invocation tests pass.
- [ ] Denied/hostile-path workflow tests pass.
- [ ] Workflow runtime error behavior remains explicit and stable.
- [ ] Refusal and approval-needed states propagate into run state and audit evidence coherently.

## Suggested Command-Backed Checks

```bash
rg -n "WorkflowContext|action|skill|approval|sync|condition" crates/vel-core/src crates/veld/src/services
rg -n "GrantEnvelope|mediated|SkillInvocation|WorkflowRunner|ApprovalRequired" crates/vel-core/src crates/veld/src/services
rg -n "manual_invocation|dry_run|PolicyDenied|UnsupportedCapability|PendingReconciliation" crates/veld/tests
```

## Exit Standard

Phase 61 is verified when a minimal manual workflow runtime exists over canonical objects and the lawful membrane, with mediated skill/action execution, approval-aware pauses, runtime evidence, and refusal-path proof.

---

*Verification target for the Phase 61 planning packet*
