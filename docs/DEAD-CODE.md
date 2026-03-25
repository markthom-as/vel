# Dead Code and Warning Debt Audit

**Status date:** 2026-03-24  
**Scope:** Rust workspace (`crates/*`) build warnings in `cargo check --workspace --all-targets [--all-features]`.

## Why this doc exists

This file captures warning debt that has historically been present in the active backend (`veld`) and CLI (`vel-cli`) build, what is intentionally deferred, and what appears ready for safe reintegration once routing and policy are completed.

## Current warning surface

* `cargo check --workspace --all-targets --all-features` now reports `0` warnings.
* The zero-warning state is currently achieved through crate-level `dead_code` suppression:
  * `crates/veld/src/lib.rs` line 1
  * `crates/veld/src/main.rs` line 1
  * `crates/vel-cli/src/client.rs` line 1
* A prior snapshot (`/tmp/vel_cargo_check_warnings.txt`) captured the pre-suppression debt.  
  * `228` warning sites
  * `42` Rust source files

## Top warning clusters in the pre-suppression snapshot

`writeback` and integration-adapter surfaces carried the bulk of warnings because they are implemented but not currently active in request pathways:

* `crates/veld/src/services/writeback.rs` — 25
* `crates/veld/src/services/integrations_github.rs` — 22
* `crates/veld/src/services/integrations_email.rs` — 16
* `crates/veld/src/services/workflow_runner.rs` — 15
* `crates/veld/src/routes/integrations.rs` — 13
* `crates/veld/src/adapters/notes.rs` — 10
* `crates/veld/src/services/agent_protocol.rs` — 10
* `crates/veld/src/adapters/reminders.rs` — 8
* `crates/veld/src/services/skill_invocation.rs` — 9
* `crates/veld/src/services/core_module_bootstrap.rs` — 8
* `crates/veld/src/services/object_actions.rs` — 8
* `crates/veld/src/services/registry_loader.rs` — 8
* `crates/veld/src/services/inference/mod.rs` — 7
* `crates/veld/src/services/recurrence_materialization.rs` — 7
* `crates/veld/src/services/module_activation.rs` — 5
* `crates/veld/src/services/module_policy_bridge.rs` — 5
* `crates/veld/src/services/reflow.rs` — 5
* `crates/veld/src/services/availability_projection.rs` — 4
* `crates/veld/src/services/provider_module_registration.rs` — 4
* `crates/veld/src/services/workflow_context_binding.rs` — 3
* `crates/veld/src/services/integrations_todoist.rs` — 3
* `crates/veld/src/services/context_generation.rs` — 3
* `crates/veld/src/services/grant_resolver.rs` — 3
* `crates/veld/src/services/lan_discovery.rs` — 2
* `crates/veld/src/services/action_registry.rs` — 2
* `crates/veld/src/services/calendar_explain.rs` — 2
* `crates/veld/src/services/commitment_scheduling.rs` — 2
* `crates/veld/src/services/commitment_write_bridge.rs` — 2
* `crates/veld/src/services/legacy_compat.rs` — 2
* `crates/veld/src/services/ownership_resolver.rs` — 2
* `crates/veld/src/services/context_runs.rs` — 1
* `crates/veld/src/services/daily_loop_inputs.rs` — 1
* `crates/veld/src/services/now.rs` — 1
* `crates/veld/src/services/check_in.rs` — 1
* `crates/veld/src/services/conflict_classifier.rs` — 1
* `crates/veld/src/services/daily_loop.rs` — 1
* `crates/veld/src/services/inference/registry.rs` — 1
* `crates/veld/src/services/llm_settings.rs` — 1
* `crates/veld/src/services/planning_profile.rs` — 1
* `crates/veld/src/services/sandbox.rs` — 1
* `crates/veld/src/services/todoist_write_bridge.rs` — 1
* `crates/vel-cli/src/client.rs` — 2

## Reintegration findings

### High-value: Integration writeback stack is implemented but intentionally quarantined from active API routes

* `crates/veld/src/routes/integrations.rs`
  * `todoist_create_task`, `todoist_update_task`, `todoist_complete_task`, `todoist_reopen_task`
  * `notes_create_note`, `notes_append_note`
  * `reminders_create`, `reminders_update`, `reminders_complete`
  * `github_create_issue`, `github_add_comment`, `github_close_issue`, `github_reopen_issue`
  * `email_create_draft_reply`, `email_send_draft`
* All of the above currently return `deprecated_write_path_error` and point at `0.5` migration notes in `legacy_compat`.
* The full service stack behind these paths exists:
  * `crates/veld/src/services/writeback.rs`
  * `crates/veld/src/adapters/notes.rs`
  * `crates/veld/src/adapters/reminders.rs`
  * `crates/veld/src/services/integrations_github.rs`
  * `crates/veld/src/services/integrations_email.rs`
  * `crates/veld/src/services/integrations_todoist.rs`
* Recommendation: re-enable write operations through canonical write-intent surfaces or add an explicit compatibility adapter layer that maps legacy routes into canonical dispatch, instead of returning deprecation-only responses.

### Medium-value: Workflow execution runner is complete enough for activation

`crates/veld/src/services/workflow_runner.rs` provides a concrete run loop with status persistence, manual invocation handling, step execution paths, and refusal/dry-run states, but is never constructed or invoked.

Supporting modules are also present with “unused” warnings, showing an unfinished but coherent subsystem:

* `crates/veld/src/services/skill_invocation.rs`
* `crates/veld/src/services/workflow_context_binding.rs`
* `crates/veld/src/services/object_actions.rs`
* `crates/veld/src/services/registry_loader.rs`
* `crates/veld/src/services/core_module_bootstrap.rs`

Recommendation: restore command routing for manual workflow invocation and add one integration test that executes a small action workflow end-to-end.

### Medium-value: Agent protocol envelope plumbing is partially wired

`crates/veld/src/services/agent_protocol.rs` contains full envelope handlers (`handle_envelope`, `handle_handshake`, `handle_heartbeat`, `handle_capability_request`, `handle_action_batch`) but only its lease constant is consumed through `connect_runtime`.  

Recommendation: connect protocol HTTP/WS ingress to `handle_envelope` so warning debt converts back to real wiring debt and can be resolved structurally.

### Medium-value: Inference, module registration, and scheduling pipeline have implemented but unexercised paths

Modules with warning clusters also contain concrete domain types and helper functions that look like active roadmap work, not accidental mistakes:

* `crates/veld/src/services/inference/mod.rs` (`MessageSummary`, `HealthSummary`, `GitActivitySummary`)
* `crates/veld/src/services/recurrence_materialization.rs`
* `crates/veld/src/services/context_generation.rs`
* `crates/veld/src/services/commitment_scheduling.rs`
* `crates/veld/src/services/llm_settings.rs`
* `crates/veld/src/services/planning_profile.rs`

Recommendation: stage these behind feature flags or smaller module-local allow lists, then activate from the owning routes/services when the owning flows are implemented.

### Low-risk cleanup candidates (non-functional)

* `crates/veld/src/services/legacy_compat.rs`
  * `LegacyCompatDisposition` and `legacy_disposition_note` are currently unreferenced but have explicit compatibility semantics.
* `crates/veld/src/services/todoist_write_bridge.rs`
  * `bridge_todoist_write` is unused while `bridge_todoist_write_with_services` is active in canonical write-intent routing.
  * Recommendation: either remove once unused or keep as a narrow adapter if direct pool-only path is needed in tests/benchmarks.

## Suggested normalization plan

1. Keep current functionality intact, but remove file-wide suppressions once the above reintegration steps are done.
2. Replace crate-level `#[allow(dead_code)]` with:
   * module-scoped suppressions for deferred surfaces.
   * local `#[allow(...)]` on legacy compatibility shims.
3. For each reintegration cluster above, add one ticket and one focused integration test that exercises the real route/service chain.
4. Run `cargo check --workspace --all-targets --all-features` and track warning count deltas by file.

## Known caveat

The current zero-warning status is a noise-reduction state, not an integration-completion state. The dominant debt is now hidden behind explicit allows and should be treated as active work to route and activate.

