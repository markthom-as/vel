# Phase 109 Validation

## Required Checks

### Structure

- `app.rs` is visibly smaller and delegates grouped responsibilities to submodules
- `now.rs` public API remains stable while internal responsibilities are split
- `vel-api-types/src/lib.rs` becomes a re-export index rather than the DTO blob itself
- first DTO slice: `common`, `responses`, and `commands` modules exist, and the remaining root file re-exports those modules while preserving existing consumer imports
- second DTO slice: `backup` and `health` modules own backup/export DTOs and `HealthData`, with root re-exports preserving existing consumer imports
- third DTO slice: `apple` module owns Apple voice, schedule, behavior-summary, and response DTOs, with root re-exports preserving existing consumer imports
- fourth DTO slice: `projects` module owns project family/status/root/provision/record DTOs and `integrations` owns integration connection DTOs, with root re-exports preserving existing project, sync, integration, and agent-grounding consumers
- route-registration slice: `app/route_groups.rs` owns public/operator/worker/future route groups and the undefined-route fallback; `app.rs` retains builder entrypoints and tests
- app test-ownership slice: `app/tests.rs` owns the moved app test module body, and `app.rs` retains only the test module declaration
- first Now slice: `services/now/output.rs` owns public `Now*Output` and `TrustReadiness*Output` structs, with `services::now::*` re-exports preserving existing route and Apple voice consumers
- fifth DTO slice: `integrations` module owns `IntegrationFamilyData` and `IntegrationSourceRefData`, with root re-exports preserving existing consumer imports
- sixth DTO slice: `integrations` module also owns integration connection, setting reference, and event DTOs, with root re-exports preserving route, CLI, and client imports

### Automated

- targeted `veld` route tests
- targeted `Now` service tests
- compile/test checks that prove `vel-api-types` re-exports did not break existing consumers
- first DTO slice: `cargo test -p vel-api-types -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`, `cargo test -p vel-cli command_lang`, `cargo test -p veld command_lang -- --nocapture`
- second DTO slice: `cargo test -p vel-api-types -- --nocapture`, `cargo test -p vel-cli backup -- --nocapture`, `cargo test -p veld app::tests::health_endpoint_returns_ok`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- third DTO slice: focused Apple API-type tests, `cargo test -p vel-api-types -- --nocapture`, `cargo test -p veld --test apple_voice_loop -- --nocapture`, `cargo test -p veld --test apple_behavior_summary -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fourth DTO slice: focused project and integration API-type tests, `cargo test -p vel-api-types -- --nocapture`, `cargo test -p vel-cli projects -- --nocapture`, integration connection route/CLI tests, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- route-registration slice: targeted app exposure tests, `cargo test -p veld connect_runtime -- --nocapture`, `cargo test -p veld execution_context -- --nocapture`, `cargo test -p veld agent_grounding -- --nocapture`, `cargo test -p veld command_lang -- --nocapture`, `cargo check -p veld --all-targets`
- app test-ownership slice: targeted app tests including health, auth, route fallback, now snapshot, exposure matrix; `cargo test -p veld app::tests::`; `cargo check -p veld --all-targets`
- first Now slice: `cargo test -p veld now::tests:: -- --nocapture`; `cargo test -p veld app::tests::now_endpoint_returns_consolidated_snapshot`; `cargo test -p veld app::tests::now_task_lane_patch_persists_lane_membership_and_completion_state`; `cargo test -p veld --test apple_voice_loop -- --nocapture`; `cargo check -p veld --all-targets`
- fifth DTO slice: `cargo test -p vel-api-types -- --nocapture`, targeted integration connection route test, `cargo check -p vel-cli --all-targets`
- sixth DTO slice: `cargo test -p vel-api-types -- --nocapture`, targeted integration connection route and CLI tests, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`

### Manual Review

- inspect the new module names for obvious ownership clarity
- confirm that no hidden behavior changes were bundled into the file moves

## Failure Conditions

- large-file cleanup causes broad consumer churn without strong justification
- the split replaces one giant file with many poorly named files
- behavior changes are mixed into the reorganization without clear necessity
