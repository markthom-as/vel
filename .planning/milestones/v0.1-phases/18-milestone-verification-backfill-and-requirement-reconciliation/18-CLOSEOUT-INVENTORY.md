# Phase 18 Closeout Inventory

## Purpose

This document is the Phase 18 truth source for milestone-closeout reconciliation. It maps milestone phases to on-disk summary coverage, missing verification coverage, and requirement-family posture so later Phase 18 slices can backfill evidence and reconcile the ledger without rediscovering repo truth.

## Command-Backed Snapshot

Commands used:

```bash
find .planning/phases -maxdepth 2 \( -name '*-SUMMARY.md' -o -name '*-VERIFICATION.md' \) | sort
find .planning/phases -maxdepth 2 -name '*-SUMMARY.md' | sed 's#^.planning/phases/##' | cut -d/ -f1 | sort | uniq -c
find .planning/phases -maxdepth 2 -name '*-VERIFICATION.md' | sed 's#^.planning/phases/##' | cut -d/ -f1 | sort | uniq -c
rg -n '^### Phase|^\*\*Requirements\*\*:' .planning/ROADMAP.md
node /home/jove/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze
```

Observed facts:

- `76` `*-SUMMARY.md` artifacts exist under `.planning/phases/`
- `1` `*-VERIFICATION.md` artifact exists under `.planning/phases/`
- The only existing verification artifact is [1.1-VERIFICATION.md](/home/jove/code/vel/.planning/phases/1.1-preflight-pre-phase-2-hardening/1.1-VERIFICATION.md)
- [v0.1-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.1-MILESTONE-AUDIT.md) reports the same gap shape: summary-heavy, verification-light, requirements ledger stale
- `roadmap analyze` still reports `missing_phase_details: ["1"]` and `roadmap_complete: false` for Phases `4` and `13-17`, which matches the milestone-audit claim that archive inputs still drift

## Verification Baseline

| Phase | Summaries | Verification | Requirement families | Truth posture | Phase 18 action |
|---|---:|---:|---|---|---|
| `1.1` | 1 | 1 | pre-Phase-2 hardening only | control/reference artifact; not a milestone-closeout gap | reference format only |
| `2` | 1 | 0 | `SIG-*`, `SYNC-*`, `CONN-*`, `CAP-*`, `OPS-*` | historical baseline with explicit re-scope/deferred work | `18-02` + `18-04` |
| `3` | 5 | 0 | `VERIFY-*`, `EVAL-*`, `TRACE-*`, `DOCS-*` | shipped milestone phase; verification missing | `18-02` + `18-04` |
| `4` | 5 | 0 | `MEM-*`, `SAND-*`, `SDK-*` | historical baseline with explicit re-scope/deferred work | `18-02` + `18-04` |
| `5` | 9 | 0 | `NOW-*`, `INBOX-*`, `ACTION-*`, `REVIEW-*`, `CONTINUITY-*`, `PROJ-*`, `FAMILY-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md` | `18-03` + `18-04` |
| `6` | 7 | 0 | `WB-*`, `CONFLICT-*`, `PROV-*`, `RECON-*`, `TODO-*`, `NOTES-*`, `REMIND-*`, `GH-*`, `EMAIL-*`, `PEOPLE-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md` | `18-03` + `18-04` |
| `7` | 4 | 0 | `IOS-*`, `HEALTH-*`, `APPLE-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md` | `18-03` + `18-04` |
| `8` | 6 | 0 | `EXEC-*`, `GSD-*`, `HANDOFF-*`, `LOCAL-*`, `POLICY-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md` | `18-03` + `18-04` |
| `9` | 4 | 0 | `BACKUP-*`, `CTRL-*` | shipped milestone phase; ledger-tracked in `REQUIREMENTS.md` | `18-03` + `18-04` |
| `10` | 5 | 0 | `MORNING-*`, `STANDUP-*`, `SESSION-*`, `VOICE-*` | shipped milestone phase; ledger-tracked in `REQUIREMENTS.md` | `18-03` + `18-04` |
| `11` | 3 | 0 | `AGENT-CTX-*`, `AGENT-TOOLS-*`, `AGENT-REVIEW-*`, `AGENT-TRUST-*` | shipped milestone phase; ledger-tracked in `REQUIREMENTS.md` | `18-03` + `18-04` |
| `12` | 4 | 0 | `SHELL-*`, `DOCS-*`, `ONBOARD-*`, `INTEGR-UX-*`, `PROJ-UX-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md` | `18-03` + `18-04` |
| `13` | 4 | 0 | `ARCH-XS-*`, `ADAPT-*`, `APPLE-ARCH-*`, `API-ARCH-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md`; roadmap metadata still inconsistent | `18-03` + `18-04` |
| `14` | 4 | 0 | `PROD-*`, `MODE-*`, `UX-CORE-*`, `TRUST-UX-*`, `ONBOARD-*`, `ROADMAP-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md`; roadmap metadata still inconsistent | `18-03` + `18-04` |
| `15` | 5 | 0 | `MIGRATE-*`, `SERVICE-*`, `DTO-*`, `READMODEL-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md`; roadmap metadata still inconsistent | `18-03` + `18-04` |
| `16` | 5 | 0 | `LOGIC-*`, `FLOW-*`, `MODE-*`, `READMODEL-*`, `SHELL-ARCH-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md`; roadmap metadata still inconsistent | `18-03` + `18-04` |
| `17` | 4 | 0 | `SHELL-MODE-*`, `TRUST-SUMMARY-*`, `APPLE-SHELL-*` | shipped roadmap phase; requirement families live in `ROADMAP.md`, not `REQUIREMENTS.md`; roadmap metadata still inconsistent | `18-03` + `18-04` |

## Reconciliation Rules

### 1. Evidence hierarchy

When closeout truth conflicts, trust evidence in this order:

1. `VERIFICATION.md`
2. explicit verification section in `*-SUMMARY.md`
3. `requirements-completed` summary frontmatter
4. phase requirements listed in [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md)

Lower layers can suggest work to reconcile, but cannot justify a completed requirement by themselves.

### 2. Requirement-complete rule

A requirement may be marked complete in [REQUIREMENTS.md](/home/jove/code/vel/.planning/REQUIREMENTS.md) only when all of the following are true:

- milestone scope still claims the requirement as shipped rather than deferred
- a phase verification artifact cites explicit passing evidence
- the evidence points to the exact phase or follow-on phase that closed the requirement
- no re-scope note in [PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md), [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md), or the milestone audit contradicts that closure

### 3. Partial claims stay partial

Any summary claim marked `(partial)` or otherwise described as a baseline-only slice does not become a checked requirement automatically. Phase 18 may record partial evidence in traceability notes, but must leave the requirement unresolved unless later shipped work explicitly closes the original requirement semantics.

### 4. Historical baseline rule for Phases 2 and 4

Phases `2` and `4` are historical baselines with explicit re-scoped work. Phase 18 must preserve that history:

- do not rewrite original unmet requirement scope into false completion
- do record what actually shipped as baseline evidence
- do point unresolved parts to their forward phases where the roadmap already says they moved

This rule protects the milestone ledger from flattening “baseline shipped + follow-on deferred” into “fully done.”

### 5. One verification artifact per milestone phase

Phase `18-02` and `18-03` must leave one durable `VERIFICATION.md` artifact for every milestone phase `2` through `17`. Phase `1.1` remains the reference format, not a gap.

### 6. Summary frontmatter is advisory only

`requirements-completed` frontmatter is useful input, but it is not authoritative closeout evidence. If a summary claims completion without explicit verification evidence, Phase 18 must backfill the evidence or downgrade the ledger claim.

### 7. Missing ledger rows must be made explicit

Phases `5-8` and `12-17` have requirement families in [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md) that are not yet represented in [REQUIREMENTS.md](/home/jove/code/vel/.planning/REQUIREMENTS.md). Before archival, Phase `18-04` must make those families explicit in the milestone ledger instead of leaving them implied only by roadmap text.

### 8. No new product behavior

Phase 18 may create or repair closeout artifacts only:

- verification reports
- requirements-ledger rows and statuses
- milestone-closeout documentation

It must not introduce new product/runtime behavior.

### 9. Audit-consistency rule

Any new inventory or verification artifact created in Phase 18 must remain consistent with the current audit facts unless the artifact itself is the repair. In particular:

- `76` summaries / `1` verification is the starting point
- metadata drift reported by `roadmap analyze` is still real until Phase 19 repairs it
- Phase 18 should repair evidence and ledger truth first, then let Phase 19 clean the remaining archive inputs

## Slice Routing

- [18-02-PLAN.md](/home/jove/code/vel/.planning/phases/18-milestone-verification-backfill-and-requirement-reconciliation/18-02-PLAN.md): historical baseline verification for Phases `2-4`
- [18-03-PLAN.md](/home/jove/code/vel/.planning/phases/18-milestone-verification-backfill-and-requirement-reconciliation/18-03-PLAN.md): shipped-phase verification for Phases `5-17`
- [18-04-PLAN.md](/home/jove/code/vel/.planning/phases/18-milestone-verification-backfill-and-requirement-reconciliation/18-04-PLAN.md): milestone ledger reconciliation against the new verification truth
