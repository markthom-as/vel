# Phase 61 Grant Envelope Matrix

## Purpose

Make workflow authority explicit so grant envelopes are implemented as a real narrowing contract rather than as a slogan.

## Inputs To Effective Workflow Authority

| Constraint Source | What It Constrains | Can Widen Authority? | Can Narrow Authority? | Notes |
| --- | --- | --- | --- | --- |
| Caller grant | initial invocation authority | no | yes | caller never grants more than it already has |
| Workflow declaration | requested workflow capabilities and allowed step surfaces | no | yes | declarative ceiling inside workflow definition |
| Module policy | module/skill/tool enablement and capability posture | no | yes | may disable or constrain runtime use |
| Workspace policy | environment-level governance and read-only posture | no | yes | broadest local governance layer |
| Account policy | provider/account write and read posture | no | yes | especially relevant for external mutation paths |
| Confirmation mode | whether execution may proceed automatically | no | yes | may require ask/deny before side effects |
| Effective grant envelope | actual runtime authority for the invocation | no | n/a | computed intersection/narrowing result |

## Rule

Workflow authority is computed by narrowing across all applicable sources.

It is never widened by:

- workflow definition
- module provenance
- skill implementation details
- internal runtime shortcuts

## Operational Consequences

- a workflow may be registered and activated yet still lack effective authority to run a given step
- a skill may be present in registry state yet remain unavailable under the effective grant envelope
- approval requirements and read-only posture may narrow execution further even after invocation starts
