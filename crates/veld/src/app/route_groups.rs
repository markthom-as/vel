use axum::{
    http::StatusCode,
    middleware as axum_middleware,
    response::{IntoResponse, Response},
    routing::{any, get, patch, post},
    Router,
};

use crate::{routes, state::AppState};

pub(super) fn public_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/v1/health", get(routes::health::health))
        .route(
            "/v1/discovery/bootstrap",
            get(routes::cluster::discovery_bootstrap),
        )
        .merge(
            Router::new()
                .route(
                    "/v1/linking/prompts",
                    post(routes::linking::receive_linking_prompt),
                )
                .route(
                    "/v1/linking/public-revoke",
                    post(routes::linking::receive_remote_revoke),
                )
                .route(
                    "/v1/linking/public-redeem",
                    post(routes::linking::public_redeem_pairing_token),
                )
                .layer(axum_middleware::from_fn_with_state(
                    state,
                    crate::middleware::enforce_public_linking_abuse_guard,
                )),
        )
        .route(
            "/api/integrations/google-calendar/oauth/callback",
            get(routes::integrations::google_calendar_oauth_callback),
        )
}

pub(super) fn operator_authenticated_routes() -> Router<AppState> {
    Router::new()
        .merge(routes::connect::connect_routes())
        .route(
            "/v1/command/complete",
            post(routes::command_lang::complete_command),
        )
        .route("/v1/command/plan", post(routes::command_lang::plan_command))
        .route(
            "/v1/command/execute",
            post(routes::command_lang::execute_command),
        )
        .route("/v1/cluster/bootstrap", get(routes::cluster::bootstrap))
        .route("/v1/cluster/workers", get(routes::cluster::workers))
        .route("/v1/doctor", get(routes::doctor::doctor))
        .route("/v1/backup/status", get(routes::backup::backup_status))
        .route("/v1/backup/create", post(routes::backup::create_backup))
        .route(
            "/v1/backup/export/status",
            get(routes::backup::backup_export_status),
        )
        .route("/v1/backup/export", post(routes::backup::export_backup))
        .route("/v1/backup/inspect", post(routes::backup::inspect_backup))
        .route("/v1/backup/verify", post(routes::backup::verify_backup))
        .route("/v1/import/batch", post(routes::import::import_batch))
        .route(
            "/v1/apple/voice/turn",
            post(routes::apple::apple_voice_turn),
        )
        .route(
            "/v1/apple/behavior-summary",
            get(routes::apple::apple_behavior_summary),
        )
        .route(
            "/v1/captures",
            get(routes::captures::list_captures).post(routes::captures::create_capture),
        )
        .route("/v1/captures/:id", get(routes::captures::get_capture))
        .route(
            "/v1/journal/mood",
            post(routes::journal::create_mood_journal),
        )
        .route(
            "/v1/journal/pain",
            post(routes::journal::create_pain_journal),
        )
        .route(
            "/v1/journal/watch-signal",
            post(routes::journal::create_watch_signal_journal),
        )
        .route(
            "/v1/commitments",
            get(routes::commitments::list_commitments).post(routes::commitments::create_commitment),
        )
        .route(
            "/v1/commitments/:id",
            get(routes::commitments::get_commitment).patch(routes::commitments::update_commitment),
        )
        .route(
            "/v1/commitments/:id/dependencies",
            get(routes::commitments::list_commitment_dependencies)
                .post(routes::commitments::add_commitment_dependency),
        )
        .route(
            "/v1/projects/families",
            get(routes::projects::list_project_families),
        )
        .route(
            "/v1/projects",
            get(routes::projects::list_projects).post(routes::projects::create_project),
        )
        .route("/v1/projects/:id", get(routes::projects::get_project))
        .route(
            "/v1/execution/projects/:id/context",
            get(routes::execution::get_execution_context)
                .post(routes::execution::save_execution_context),
        )
        .route(
            "/v1/execution/projects/:id/preview",
            post(routes::execution::preview_execution_artifacts),
        )
        .route(
            "/v1/execution/projects/:id/export",
            post(routes::execution::export_execution_artifacts),
        )
        .route(
            "/v1/execution/handoffs",
            get(routes::execution::list_execution_handoffs)
                .post(routes::execution::create_execution_handoff),
        )
        .route(
            "/v1/execution/handoffs/:id/launch-preview",
            get(routes::execution::preview_execution_handoff_launch),
        )
        .route(
            "/v1/execution/handoffs/:id/launch",
            post(routes::execution::launch_execution_handoff),
        )
        .route(
            "/v1/execution/handoffs/:id/approve",
            post(routes::execution::approve_execution_handoff),
        )
        .route(
            "/v1/execution/handoffs/:id/reject",
            post(routes::execution::reject_execution_handoff),
        )
        .route("/v1/people", get(routes::people::list_people))
        .route("/v1/people/:id", get(routes::people::get_person))
        .route(
            "/v1/people/:id/aliases",
            post(routes::people::upsert_person_alias),
        )
        .route(
            "/v1/agent/inspect",
            get(routes::agent_grounding::get_agent_inspect),
        )
        .route("/v1/risk", get(routes::risk::list_risk))
        .route("/v1/risk/:id", get(routes::risk::get_commitment_risk))
        .route("/v1/suggestions", get(routes::suggestions::list))
        .route(
            "/v1/suggestions/:id/evidence",
            get(routes::suggestions::evidence),
        )
        .route(
            "/v1/suggestions/:id",
            get(routes::suggestions::get).patch(routes::suggestions::update),
        )
        .route(
            "/v1/suggestions/:id/accept",
            post(routes::suggestions::accept),
        )
        .route(
            "/v1/suggestions/:id/reject",
            post(routes::suggestions::reject),
        )
        .route(
            "/v1/artifacts",
            get(routes::artifacts::list_artifacts).post(routes::artifacts::create_artifact),
        )
        .route(
            "/v1/artifacts/latest",
            get(routes::artifacts::get_artifact_latest),
        )
        .route("/v1/artifacts/:id", get(routes::artifacts::get_artifact))
        .route("/v1/runs", get(routes::runs::list_runs))
        .route(
            "/v1/runs/:id",
            get(routes::runs::get_run).patch(routes::runs::update_run),
        )
        .route("/v1/context/today", get(routes::context::today))
        .route("/v1/context/morning", get(routes::context::morning))
        .route("/v1/context/end-of-day", get(routes::context::end_of_day))
        .route(
            "/v1/daily-loop/sessions",
            post(routes::daily_loop::start_session),
        )
        .route(
            "/v1/daily-loop/sessions/active",
            get(routes::daily_loop::active_session),
        )
        .route(
            "/v1/daily-loop/sessions/:id/check-ins",
            get(routes::daily_loop::list_session_check_in_events)
                .post(routes::daily_loop::submit_check_in),
        )
        .route(
            "/v1/daily-loop/check-ins/:check_in_event_id/skip",
            post(routes::daily_loop::skip_check_in),
        )
        .route(
            "/v1/daily-loop/sessions/:id/turn",
            post(routes::daily_loop::submit_turn),
        )
        .route(
            "/v1/daily-loop/sessions/:id/overdue/menu",
            post(routes::daily_loop::overdue_menu),
        )
        .route(
            "/v1/daily-loop/sessions/:id/overdue/confirm",
            post(routes::daily_loop::overdue_confirm),
        )
        .route(
            "/v1/daily-loop/sessions/:id/overdue/apply",
            post(routes::daily_loop::overdue_apply),
        )
        .route(
            "/v1/daily-loop/sessions/:id/overdue/undo",
            post(routes::daily_loop::overdue_undo),
        )
        .route("/v1/context/current", get(routes::context::current))
        .route("/v1/context/timeline", get(routes::context::timeline))
        .route(
            "/v1/linking/tokens",
            post(routes::linking::issue_pairing_token),
        )
        .route(
            "/v1/linking/redeem",
            post(routes::linking::redeem_pairing_token),
        )
        .route("/v1/linking/status", get(routes::linking::linking_status))
        .route(
            "/v1/linking/revoke/:node_id",
            post(routes::linking::revoke_link),
        )
        .route("/v1/now", get(routes::now::get_now))
        .route(
            "/v1/now/task-lane",
            patch(routes::now::update_now_task_lane),
        )
        .route(
            "/v1/now/tasks/reschedule-today",
            post(routes::now::reschedule_now_tasks_to_today),
        )
        .route(
            "/v1/now/calendar-events/reschedule",
            post(routes::now::reschedule_now_calendar_event),
        )
        .route(
            "/v1/commitment-scheduling/proposals/:id/apply",
            post(routes::commitment_scheduling::apply_commitment_scheduling_proposal),
        )
        .route(
            "/v1/planning-profile",
            get(routes::planning_profile::get_planning_profile)
                .patch(routes::planning_profile::patch_planning_profile),
        )
        .route(
            "/v1/planning-profile/proposals/:id/apply",
            post(routes::planning_profile::apply_planning_profile_proposal),
        )
        .route("/v1/explain/nudge/:id", get(routes::explain::explain_nudge))
        .route("/v1/explain/context", get(routes::explain::explain_context))
        .route(
            "/v1/explain/commitment/:id",
            get(routes::explain::explain_commitment),
        )
        .route("/v1/explain/drift", get(routes::explain::explain_drift))
        .route(
            "/v1/threads",
            get(routes::threads::list_threads).post(routes::threads::create_thread),
        )
        .route(
            "/v1/threads/:id",
            get(routes::threads::get_thread).patch(routes::threads::update_thread),
        )
        .route(
            "/v1/threads/:id/links",
            post(routes::threads::add_thread_link),
        )
        .route("/v1/search", get(routes::search::search))
        .route(
            "/v1/signals",
            get(routes::signals::list_signals).post(routes::signals::create_signal),
        )
        .route("/v1/nudges", get(routes::nudges::list_nudges))
        .route("/v1/nudges/:id", get(routes::nudges::get_nudge))
        .route("/v1/nudges/:id/done", post(routes::nudges::nudge_done))
        .route("/v1/nudges/:id/snooze", post(routes::nudges::nudge_snooze))
        .route(
            "/v1/nudges/:id/dismiss",
            post(routes::nudges::nudge_dismiss),
        )
        .route(
            "/v1/uncertainty",
            get(routes::uncertainty::list_uncertainty),
        )
        .route(
            "/v1/uncertainty/:id",
            get(routes::uncertainty::get_uncertainty),
        )
        .route(
            "/v1/uncertainty/:id/resolve",
            post(routes::uncertainty::resolve_uncertainty),
        )
        .route("/v1/loops", get(routes::loops::list_loops))
        .route(
            "/v1/loops/:kind",
            get(routes::loops::get_loop).patch(routes::loops::update_loop),
        )
        .route("/v1/sync/calendar", post(routes::sync::sync_calendar))
        .route("/v1/sync/todoist", post(routes::sync::sync_todoist))
        .route("/v1/sync/activity", post(routes::sync::sync_activity))
        .route("/v1/sync/health", post(routes::sync::sync_health))
        .route("/v1/sync/git", post(routes::sync::sync_git))
        .route("/v1/sync/messaging", post(routes::sync::sync_messaging))
        .route("/v1/sync/reminders", post(routes::sync::sync_reminders))
        .route("/v1/sync/notes", post(routes::sync::sync_notes))
        .route("/v1/sync/transcripts", post(routes::sync::sync_transcripts))
        .route("/v1/sync/bootstrap", get(routes::sync::sync_bootstrap))
        .route("/v1/sync/cluster", get(routes::sync::sync_cluster))
        .route("/v1/evaluate", post(routes::evaluate::run_evaluate))
        .route(
            "/api/diagnostics",
            get(routes::diagnostics::get_diagnostics),
        )
        .route("/api/components", get(routes::components::list_components))
        .route(
            "/api/components/:id/logs",
            get(routes::components::list_component_logs),
        )
        .route(
            "/api/components/:id/restart",
            post(routes::components::restart_component),
        )
        .route(
            "/api/integrations",
            get(routes::integrations::get_integrations),
        )
        .route(
            "/api/integrations/connections",
            get(routes::integrations::list_integration_connections),
        )
        .route(
            "/api/integrations/connections/:id",
            get(routes::integrations::get_integration_connection),
        )
        .route(
            "/api/integrations/connections/:id/events",
            get(routes::integrations::list_integration_connection_events),
        )
        .route(
            "/api/integrations/:id/logs",
            get(routes::integrations::list_integration_logs),
        )
        .route(
            "/api/integrations/:id/source",
            axum::routing::patch(routes::integrations::patch_local_integration_source),
        )
        .route(
            "/api/integrations/:id/path-dialog",
            post(routes::integrations::choose_local_integration_source_path),
        )
        .route(
            "/api/integrations/google-calendar",
            axum::routing::patch(routes::integrations::patch_google_calendar),
        )
        .route(
            "/api/integrations/google-calendar/disconnect",
            post(routes::integrations::disconnect_google_calendar),
        )
        .route(
            "/api/integrations/google-calendar/auth/start",
            post(routes::integrations::start_google_calendar_auth),
        )
        .route(
            "/api/integrations/google-calendar/write-intent",
            post(routes::integrations::google_calendar_write_intent),
        )
        .route(
            "/api/integrations/todoist",
            axum::routing::patch(routes::integrations::patch_todoist),
        )
        .route(
            "/api/integrations/todoist/disconnect",
            post(routes::integrations::disconnect_todoist),
        )
        .route(
            "/api/integrations/todoist/write-intent",
            post(routes::integrations::todoist_write_intent),
        )
        .route(
            "/v1/synthesis/week",
            post(routes::synthesis::synthesis_week),
        )
        .route(
            "/v1/synthesis/project/:slug",
            post(routes::synthesis::synthesis_project),
        )
        .route("/ws", get(routes::ws::ws_handler))
        .merge(routes::chat::chat_routes())
}

pub(super) fn worker_authenticated_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v1/cluster/branch-sync",
            post(routes::cluster::branch_sync_request),
        )
        .route(
            "/v1/cluster/validation",
            post(routes::cluster::validation_request),
        )
        .route("/v1/sync/heartbeat", post(routes::sync::sync_heartbeat))
        .route(
            "/v1/sync/work-assignments",
            get(routes::sync::list_work_assignments)
                .post(routes::sync::claim_work_assignment)
                .patch(routes::sync::update_work_assignment),
        )
        .route("/v1/sync/work-queue", get(routes::sync::list_worker_queue))
        .route(
            "/v1/sync/work-queue/claim-next",
            post(routes::sync::claim_next_worker_queue_item),
        )
        .route("/v1/sync/actions", post(routes::sync::sync_actions))
        .route(
            "/v1/sync/branch-sync",
            post(routes::sync::sync_branch_sync_request),
        )
        .route(
            "/v1/sync/validation",
            post(routes::sync::sync_validation_request),
        )
}

pub(super) fn future_external_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/connect", any(deny_undefined_route))
        .route("/v1/connect/worker", any(deny_undefined_route))
        .route("/v1/cluster/clients", any(deny_undefined_route))
        .route("/v1/cluster/clients/*path", any(deny_undefined_route))
}

pub(super) async fn deny_undefined_route() -> Response {
    (
        StatusCode::NOT_FOUND,
        "route is not defined for this authority runtime",
    )
        .into_response()
}
