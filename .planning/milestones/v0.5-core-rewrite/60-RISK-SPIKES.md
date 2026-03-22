# Phase 60 Registry And Bootstrap Risk Spikes

## Spike A: Governed Registration And Seeded Reconciliation

Prove:

- load one core module manifest through `ManifestSource`
- materialize canonical registry objects through `RegistryLoader`
- persist reconciliation state through the registry store seam
- seed one built-in workflow through the same governed path
- rerun bootstrap idempotently without duplicate registry entities
- detect drift on editable, forked, and local variants
- register `module.integration.todoist` and `module.integration.google-calendar` through the same path
- block activation when policy denies requested capability posture

## Rule

This spike exists to prove the registry/loader/bootstrap seam is real before later phases build on it.

It is not a substitute for workflow runtime or adapter implementation phases.
