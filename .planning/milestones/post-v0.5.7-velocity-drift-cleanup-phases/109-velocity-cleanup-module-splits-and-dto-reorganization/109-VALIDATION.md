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
- fifth DTO slice: `connect` module owns connect runtime capability, instance, stream event, stdin ack, and attach DTOs, with root re-exports preserving route, CLI, and test imports
- sixth DTO slice: `doctor` module owns diagnostic status, diagnostic check, and doctor response DTOs, with root re-exports preserving doctor route and CLI imports
- seventh DTO slice: `capture` module owns capture, journal, watch signal, and search request/response DTOs, with root re-exports preserving route, CLI, and command payload imports
- eighth DTO slice: `batch_import` module owns batch import item/request/response DTOs, with root re-exports preserving import route and CLI workspace import construction
- ninth DTO slice: `agent_runtime` module owns local agent spec/spawn/runtime-return DTOs, with root re-exports preserving future consumer imports
- tenth DTO slice: `projects` module also owns project create/list route wrapper DTOs, with root re-exports preserving project route, service, and CLI imports
- eleventh DTO slice: `actions` module owns shared action route primitive DTOs, with root re-exports preserving Now, review, chat, and client imports
- twelfth DTO slice: `actions` module also owns `ActionItemData`, with root re-exports preserving cluster bootstrap, Now, inbox, and trust-readiness DTO references
- thirteenth DTO slice: `reviews` module owns `ReviewSnapshotData`, with root re-exports preserving Now, agent review obligations, CLI review, and execution-context fallback references
- fourteenth DTO slice: `writebacks` module owns writeback target, risk, status, kind, and operation DTOs, with root re-exports preserving sync, cluster, Now, and agent-grounding references
- fifteenth DTO slice: `conflicts` module owns conflict case kind/status/record DTOs, with root re-exports preserving sync, cluster, Now, and agent-grounding references
- sixteenth DTO slice: `people` module owns person alias, link ref, record, and alias upsert DTOs, with root re-exports preserving people route, sync, cluster, Now, review, and agent-grounding references
- seventeenth DTO slice: `linking` module owns link status and scope primitives, with root re-exports preserving node CLI, linking routes/services, and larger pairing/trusted-node DTO references
- eighteenth DTO slice: `linking` module also owns `LinkTargetSuggestionData`, with root re-exports preserving `PairingTokenData` and linking service references
- nineteenth DTO slice: `linking` module owns trusted-node endpoint kind/data and reachability DTOs, with root re-exports preserving trust bootstrap and linked-node DTO references
- twentieth DTO slice: `linking` module owns `TrustBootstrapArtifactData`, with root re-exports preserving pairing token and linking prompt references
- twenty-first DTO slice: `linking` module owns `PairingTokenData`, with root re-exports preserving linking routes, node CLI, and pairing datetime tests
- twenty-second DTO slice: `linking` module owns `LinkingPromptData`, with root re-exports preserving worker presence, sync bootstrap, and linking route references
- twenty-third DTO slice: `linking` module owns `LinkedNodeData` and the pairing/linking datetime contract test, with root re-exports preserving cluster, sync, route, and CLI references
- twenty-fourth DTO slice: `sync` module owns branch-sync capability and validation-profile DTOs, with root re-exports preserving cluster and sync route references

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
- fifth DTO slice: `cargo test -p vel-api-types -- --nocapture`, `cargo test -p veld --test connect_runtime -- --nocapture`, `cargo test -p vel-cli connect -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- sixth DTO slice: `cargo test -p vel-api-types -- --nocapture`, `cargo test -p veld app::tests::doctor_endpoint_returns_ok_with_schema_version -- --nocapture`, `cargo test -p vel-cli doctor -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- seventh DTO slice: `cargo test -p vel-api-types -- --nocapture`, targeted app journal/search/command capture tests, `cargo test -p vel-cli capture -- --nocapture`, `cargo test -p vel-cli search -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- eighth DTO slice: `cargo test -p vel-api-types -- --nocapture`, `cargo test -p vel-cli commands::import_::tests -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- ninth DTO slice: `cargo test -p vel-api-types -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- tenth DTO slice: `cargo test -p vel-api-types project_create_request_serializes_project_contract -- --nocapture`, `cargo test -p veld project_service_create_is_local_first -- --nocapture`, `cargo test -p veld project_routes_create_list_and_get_records -- --nocapture`, `cargo test -p vel-cli project_list_command_json_prints_project_records -- --nocapture`, `cargo check -p vel-api-types`
- eleventh DTO slice: `cargo test -p vel-api-types action_item_timestamps_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twelfth DTO slice: `cargo test -p vel-api-types action_item_timestamps_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirteenth DTO slice: `cargo test -p vel-api-types review_snapshot_default_serializes_named_counts -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fourteenth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifteenth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- sixteenth DTO slice: `cargo test -p vel-api-types person_record_last_contacted_at_serializes_as_rfc3339_string -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- seventeenth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- eighteenth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- nineteenth DTO slice: `cargo test -p vel-api-types pairing_and_linking_datetimes_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twentieth DTO slice: `cargo test -p vel-api-types pairing_and_linking_datetimes_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-first DTO slice: `cargo test -p vel-api-types pairing_and_linking_datetimes_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-second DTO slice: `cargo test -p vel-api-types pairing_and_linking_datetimes_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-third DTO slice: `cargo test -p vel-api-types pairing_and_linking_datetimes_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-fourth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`

### Manual Review

- inspect the new module names for obvious ownership clarity
- confirm that no hidden behavior changes were bundled into the file moves

## Failure Conditions

- large-file cleanup causes broad consumer churn without strong justification
- the split replaces one giant file with many poorly named files
- behavior changes are mixed into the reorganization without clear necessity
