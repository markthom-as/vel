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
- twenty-fifth DTO slice: `sync` module also owns branch-sync and validation request DTOs, with root re-exports preserving sync route and CLI request references
- twenty-sixth DTO slice: `sync` module owns queued-work routing kind/data DTOs, with root re-exports preserving sync route, cluster route, app test, and CLI client references
- twenty-seventh DTO slice: `sync` module owns `PlacementRecommendationData`, with root re-exports preserving heartbeat response and sync route placement-hint parsing
- twenty-eighth DTO slice: `sync` module owns `SyncHeartbeatResponseData`, with root re-exports preserving heartbeat route response references
- twenty-ninth DTO slice: `sync` module owns work-assignment status and receipt DTOs, with root re-exports preserving sync route lifecycle/list response references
- thirtieth DTO slice: `sync` module owns standalone work-assignment claim request DTOs, with root re-exports preserving claim and claim-next route request references
- thirty-first DTO slice: `sync` module owns `WorkAssignmentUpdateRequest`, with root re-exports preserving work-assignment update route request references
- thirty-second DTO slice: `sync` module owns `QueuedWorkItemData`, with root re-exports preserving worker queue list response references
- thirty-third DTO slice: `sync` module owns `WorkAssignmentClaimedWorkData`, with root re-exports preserving work claim route response references
- thirty-fourth DTO slice: `sync` module owns `WorkAssignmentClaimNextResponseData`, with root re-exports preserving claim-next route response references
- thirty-fifth DTO slice: `sync` module owns `SyncHeartbeatRequestData`, with root re-exports preserving heartbeat route request references
- thirty-sixth DTO slice: `client_sync` module owns client-action kind, action, batch request, action result, and batch result DTOs, with root re-exports preserving sync actions route references
- thirty-seventh DTO slice: `sync` module owns `ClusterNodeStateData`, with root re-exports preserving cluster state route and CLI references
- thirty-eighth DTO slice: `sync` module owns `SwarmClientActiveWorkData`, with root re-exports preserving worker and client active-work references
- thirty-ninth DTO slice: `sync` module owns `ClusterWorkerStateData`, with root re-exports preserving cluster state route worker references
- fortieth DTO slice: `sync` module owns `SwarmClientData`, with root re-exports preserving cluster state and swarm clients response references
- forty-first DTO slice: `sync` module owns `SyncClusterStateData`, with root re-exports preserving cluster state route and CLI references
- forty-second DTO slice: `sync` module owns `WorkerCapacityData`, with root re-exports preserving worker presence capacity references
- forty-third DTO slice: `sync` module owns `SyncResultData`, with root re-exports preserving sync route and CLI result references
- forty-fourth DTO slice: `sync` module owns `WorkerPresenceData`, with root re-exports preserving worker presence and cluster worker-list references
- forty-fifth DTO slice: `sync` module owns `ClusterWorkersData`, with root re-exports preserving cluster workers response references
- forty-sixth DTO slice: `sync` module owns `SwarmClientsData`, with root re-exports preserving swarm clients response references
- forty-seventh DTO slice: `capture` module owns `ContextCapture`, with root re-exports preserving capture, context, command-language, CLI, and test references
- forty-eighth DTO slice: `sync` module owns `ClusterBootstrapData`, with root re-exports preserving cluster bootstrap, sync bootstrap, CLI, and client-sync references
- forty-ninth DTO slice: `sync` module owns `SyncBootstrapData`, with root re-exports preserving sync bootstrap route, CLI, web, and Apple client references
- fiftieth DTO slice: `agent_grounding` module owns agent capability and blocker DTOs, with root re-exports preserving agent inspect service, CLI, web, and contract-test references
- fifty-first DTO slice: `agent_grounding` module owns the remaining agent grounding and inspect DTOs, with root re-exports preserving `/v1/agent/inspect`, CLI, web, execution-context, and contract-test references
- fifty-second DTO slice: `integrations` module owns integration status and auth-start DTOs, with root re-exports preserving integrations route, CLI, web, and Apple client references
- fifty-third DTO slice: `integrations` module owns canonical integration write-intent DTOs, with root re-exports preserving integrations route and writeback service references
- fifty-fourth DTO slice: `websocket` module owns websocket event/envelope DTOs, with root re-exports preserving broadcast, route, and test references
- fifty-fifth DTO slice: `artifacts` module owns artifact CRUD DTOs, with root re-exports preserving artifacts route, command payload, CLI, and test references
- fifty-sixth DTO slice: `runs` module owns run summary/detail/update DTOs and their datetime contract test, with root re-exports preserving runs route, CLI, web, websocket cache, and test references
- fifty-seventh DTO slice: `chat` module owns conversation and message DTOs, with root re-exports preserving chat route, CLI client, web, and command payload references
- fifty-eighth DTO slice: `signals` module owns signal create/read DTOs, with root re-exports preserving signal route, CLI client, sync bootstrap, and test references
- fifty-ninth DTO slice: `nudges` module owns nudge DTOs, with root re-exports preserving nudge route, sync bootstrap, CLI client, web, and test references

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
- twenty-fifth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-sixth DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::sync_branch_sync_endpoint_queues_structured_work_request -- --nocapture`, `cargo test -p veld app::tests::cluster_validation_endpoint_queues_structured_work_request -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-seventh DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-eighth DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::sync_heartbeat_endpoint_persists_remote_worker -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- twenty-ninth DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::work_assignment_lifecycle_claims_updates_and_lists_receipts -- --nocapture`, `cargo test -p veld app::tests::worker_queue_lists_pending_item_and_hides_completed_receipt -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirtieth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-first DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::work_assignment_lifecycle_claims_updates_and_lists_receipts -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-second DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::worker_queue_lists_pending_item_and_hides_completed_receipt -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-third DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::work_assignment_lifecycle_claims_updates_and_lists_receipts -- --nocapture`, `cargo test -p veld app::tests::claim_next_work_picks_oldest_unclaimed_item -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-fourth DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::claim_next_work_picks_oldest_unclaimed_item -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-fifth DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::sync_heartbeat_endpoint_persists_remote_worker -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-sixth DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::sync_actions_endpoint_applies_nudge_snooze -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-seventh DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::sync_cluster_endpoint_returns_nodes_and_workers -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-eighth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- thirty-ninth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fortieth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-first DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-second DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-third DTO slice: `cargo check -p vel-api-types`, `cargo test -p veld app::tests::sync_calendar_ingests_tzid_events -- --nocapture`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-fourth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-fifth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-sixth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-seventh DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-eighth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- forty-ninth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fiftieth DTO slice: `cargo test -p vel-api-types agent_grounding_capability_entries_preserve_explicit_blockers -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-first DTO slice: `cargo test -p vel-api-types agent_grounding_round_trips_typed_sections -- --nocapture`, `cargo test -p vel-api-types agent_grounding_contract_assets_parse_and_register -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-second DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-third DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-fourth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-fifth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-sixth DTO slice: `cargo test -p vel-api-types run_summary_datetimes_serialize_as_rfc3339_strings -- --nocapture`, `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-seventh DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-eighth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`
- fifty-ninth DTO slice: `cargo check -p vel-api-types`, `cargo check -p veld --all-targets`, `cargo check -p vel-cli --all-targets`

### Manual Review

- inspect the new module names for obvious ownership clarity
- confirm that no hidden behavior changes were bundled into the file moves

## Failure Conditions

- large-file cleanup causes broad consumer churn without strong justification
- the split replaces one giant file with many poorly named files
- behavior changes are mixed into the reorganization without clear necessity
