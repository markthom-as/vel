---
phase: 61-workflow-and-skill-primitives-over-canonical-objects
plan: 02
work_id: 0.5.61.2
title: Workflow grant envelopes and mediated skill invocation
status: completed
completed_at: 2026-03-22
---

# 61-02 Summary

## What Landed

- Added narrowing workflow grant-envelope contracts in `crates/vel-core/src/workflow_grants.rs`
- Added mediated skill runtime contracts in `crates/vel-core/src/skill_runtime.rs`
- Added membrane-governed skill invocation in `crates/veld/src/services/skill_invocation.rs`
- Added focused mediation proof in `crates/veld/tests/phase61_skill_mediation.rs`

## Contract Shape

- Workflow authority is now explicit through `GrantEnvelope` rather than ambient runtime inheritance
- Effective skill authority narrows across caller grant, workflow capabilities, and module capabilities
- Skills remain mediated runtime affordances and cannot call raw tools directly
- Skill invocation now composes module activation, grant narrowing, action-membrane policy, and audit emission instead of inventing a separate execution path
- Early denials at activation or grant-narrowing time still emit runtime audit evidence

## Verification

- `cargo test -p vel-core workflow_grants --lib`
- `cargo test -p vel-core skill_runtime --lib`
- `cargo test -p veld --test phase61_skill_mediation`
- `cargo check -p vel-core && cargo check -p veld`
- `rg -n "GrantEnvelope|narrow|caller|workflow|module|mediated|raw tool|action membrane|confirmation|audit|run record|SkillInvocation|PolicyDenied" crates/vel-core/src/workflow_grants.rs crates/vel-core/src/skill_runtime.rs crates/veld/src/services/skill_invocation.rs crates/veld/tests/phase61_skill_mediation.rs`

## Outcome

Phase 61 now has one lawful skill-invocation path: workflow authority narrows through an explicit grant envelope, module activation and membrane policy remain in force, and allowed, denied, approval-required, and read-only outcomes all stay auditable.
