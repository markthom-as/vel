# Phase 61 Runtime State Machine

## Purpose

Keep workflow runtime lifecycle explicit and refusal-aware.

## Run States

| State | Meaning | Terminal? |
| --- | --- | --- |
| `created` | invocation record exists, execution has not begun | no |
| `ready` | context, grants, and step contract validation passed | no |
| `running` | workflow is actively executing one or more steps | no |
| `awaiting_approval` | execution is paused on an approval requirement | no |
| `dry_run_complete` | dry-run finished without irreversible execution | yes |
| `completed` | workflow finished successfully | yes |
| `failed` | workflow ended in non-refusal failure | yes |
| `refused` | execution was denied or blocked by policy/mediation | yes |
| `cancelled` | execution was intentionally stopped | yes |

## Step-Level Expectations

- step refusal propagates into explicit run outcome or pause state
- approval-required steps do not silently fall through
- dry-run can evaluate and record, but not perform irreversible external mutation
- denied or blocked skill access must not degrade into raw-tool fallback

## Distinct Record Types

- `RunRecord`: workflow execution lifecycle and step progression
- `AuditEntry`: governed actions and policy-significant events
- `ApprovalRecord`: approval decision lifecycle
- `WriteIntent`: outward mutation proposal and dispatch lifecycle

These remain distinct even when one runtime event causes all four to be updated.
