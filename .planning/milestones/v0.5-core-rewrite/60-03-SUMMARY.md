# 60-03 Summary

Completed the module capability request, activation, and policy-mediated enablement slice for Phase 60.

## Delivered

- added [module_capabilities.rs](/home/jove/code/vel/crates/vel-core/src/module_capabilities.rs)
- added [module_activation.rs](/home/jove/code/vel/crates/veld/src/services/module_activation.rs)
- added [module_policy_bridge.rs](/home/jove/code/vel/crates/veld/src/services/module_policy_bridge.rs)
- added [phase60_module_policy.rs](/home/jove/code/vel/crates/veld/tests/phase60_module_policy.rs)
- updated [lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs)
- updated [mod.rs](/home/jove/code/vel/crates/veld/src/services/mod.rs)

## Locked Truths

- module capability requests are now typed runtime-facing contracts instead of implicit manifest trivia
- activation reports `registered`, `reconciled`, `eligible`, `activated`, and `invokable` explicitly instead of collapsing them into one generic enabled flag
- module activation reuses the Phase 59 `PolicyEvaluator` through a dedicated bridge instead of creating a loader-side trust bypass
- feature-gated provider modules fail with `UnsupportedCapability`, while read-only or denied module paths surface the lawful refusal vocabulary from the membrane

## Verification

- `rg -n "requested_capabilities|enablement_state|eligible|activated|invokable|feature gate|read_only|PolicyEvaluator|Grant|ReadOnlyViolation|UnsupportedCapability|PolicyDenied" crates/vel-core/src/module_capabilities.rs crates/veld/src/services/module_activation.rs crates/veld/src/services/module_policy_bridge.rs crates/veld/tests/phase60_module_policy.rs`
- `cargo test -p vel-core module_capabilities --lib`
- `cargo test -p veld --test phase60_module_policy`
- `cargo check -p vel-core && cargo check -p veld`

## Outcome

Phase 60 now governs module enablement through explicit capability requests, feature gates, grants, and policy mediation. Core modules and provider modules share the same lawful activation path, and activation remains distinct from later runtime invocation.
