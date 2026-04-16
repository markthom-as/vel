# Phase 5 Ticket Pack: Local Harness & Operator Runtime MVP

This directory defines the v0.1 harness ticket pack for single-operator local-first runtime closure.

## Execution Order

### Wave 1: Foundations

1. `026-core-run-event-schema.md`
2. `027-sqlite-run-store.md`
3. `028-artifact-store.md`
4. `029-policy-config-loader.md`

### Wave 2: Policy and Execution Boundaries

1. `030-capability-resolution-engine.md`
2. `031-tool-runner-abstraction.md`
3. `032-mutation-protocol-discipline.md`
4. `033-llm-provider-interface.md`

### Wave 3: Operator Commands

1. `034-vel-run-command.md`
2. `035-vel-dry-run-command.md`
3. `036-explainability-history-commands.md`

### Wave 4: Release Hardening

1. `037-security-observability-hardening.md`

### Wave 5: Workflow Verticals

1. `038-standup-overdue-workflow-slice.md` `[baseline shipped]`

## Release Gate (v0.1)

1. All ticket acceptance criteria pass.
2. Automated tests pass for touched crates and CLI integration paths.
3. Manual command-backed validation passes:
   - `vel run`
   - `vel dry-run`
   - `vel explain <run-id>`
   - `vel runs`
   - `vel artifacts <run-id>`
4. Evidence includes policy deny-path proof, artifact persistence proof, and redaction/write-scope hardening proof.
