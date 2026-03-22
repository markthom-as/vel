# Phase 60 Verification

**Phase:** 60 - Module loader, registry, and core-module bootstrap  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 60 can be considered complete as the governed registry and loader phase.

## Required Outputs

Phase 60 should leave behind:

- canonical registry ID and registry object implementation
- explicit `ManifestSource`, `RegistryLoader`, and `RegistryReconciler` seams in code
- deterministic core bootstrap and seeded workflow reconciliation
- policy-mediated module activation and capability request handling
- narrow provider-module registration for Todoist and Google Calendar through the same registry path
- explicit lifecycle vocabulary for registered, reconciled, eligible, activated, and downstream invokable states
- focused tests that prove idempotence, policy mediation, and shared registration behavior

## Verification Checks

### A. Registry substrate

- [ ] Registry IDs are stable and semantic.
- [ ] Registry persistence is dedicated and not folded into generic blob storage.
- [ ] Loader/reconciler contracts are explicit.

### B. Bootstrap and reconciliation

- [ ] Core bootstrap is deterministic and idempotent.
- [ ] Seeded workflows reconcile without violating fork-before-modify rules.
- [ ] Local forks and editable seeded drift are preserved lawfully.

### C. Policy and capabilities

- [ ] Module capability requests are explicit.
- [ ] Module activation reuses Phase 59 policy/grant behavior.
- [ ] Feature-disabled provider modules fail clearly without destabilizing the registry.
- [ ] Activation remains governed enablement rather than runtime invocation.

### D. Shared registration path

- [ ] Core modules and provider modules use the same loader/registry path.
- [ ] Todoist and Google Calendar register with canonical module IDs.
- [ ] Provider registration does not imply adapter sync behavior is already implemented.
- [ ] Integration modules do not bypass reconciliation or policy mediation.

## Explicit Non-Goals

Phase 60 is not verified by:

- workflow execution
- skill/tool runtime invocation
- provider sync execution
- outward provider writes
- invocation-time ownership conflict handling
- bespoke integration-only bootstrap paths

## Suggested Command-Backed Checks

```bash
rg -n "module.integration.todoist|module.integration.google-calendar|RegistryLoader|ManifestSource|RegistryReconciler" crates/vel-core/src crates/veld/src/services
rg -n "deterministic|idempotent|seed_version|forked_from_workflow_id|reconciliation_state" crates/vel-core/src crates/veld/src/services crates/veld/tests
rg -n "requested_capabilities|ModuleActivation|PolicyEvaluator|UnsupportedCapability|ReadOnlyViolation" crates/vel-core/src crates/veld/src/services crates/veld/tests
```

## Exit Standard

Phase 60 is verified when one canonical, policy-mediated, deterministic registry/loader path exists for both core and provider modules, with bootstrap and reconciliation behavior proved through focused tests.

---

*Verification target for the Phase 60 planning packet*
