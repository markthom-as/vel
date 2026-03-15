# Vel — Current Status

What is implemented, what is partial, and what is next.

## Implemented

- **Capture storage** — Insert captures, list by ID, lexical search (FTS5).
- **Artifacts API** — Create and fetch artifacts; storage layer with metadata.
- **Run/event schema and inspection** — `runs`, `run_events`, `events` tables; `vel runs`, `vel run inspect <id>` (including linked artifacts); `GET /v1/runs`, `GET /v1/runs/:id`.
- **Doctor diagnostics** — `vel doctor` and `GET /v1/doctor` with structured checks (DiagnosticCheck + DiagnosticStatus).
- **Context endpoints (run-backed)** — `GET /v1/context/today`, `GET /v1/context/morning`, `GET /v1/context/end-of-day` create a run, compute from orientation snapshot, write a managed **context_brief** artifact (write temp → flush → fsync → rename; checksum, size_bytes, metadata_json with `generator`, `context_kind`, `snapshot_window`), link run → artifact and artifact → capture refs, append run events (`run_created`, `run_started`, `context_generated`, `artifact_written`, `refs_created`, `run_succeeded`), then transition to succeeded/failed.
- **Run timing** — API and CLI expose `duration_ms` (derived from started_at/finished_at) on run list and run detail.
- **Runtime invariants** — Documented in runtime-concepts: run_started precedes run_succeeded; artifact_written/refs_created before run_succeeded; run_failed does not produce artifact refs; run_succeeded implies artifact durability.
- **CLI** — `vel health`, `vel capture`, `vel search`, `vel today`, `vel morning`, `vel end-of-day`, `vel doctor`, `vel inspect capture <id>`, `vel runs`, `vel run inspect <id>` (duration, artifact size e.g. 3.4KB, event timestamps in CLI).
- **Crate boundaries** — Domain types (ContextCapture, SearchResult, OrientationSnapshot) live in `vel-core`; storage returns them; API layer maps to DTOs. `vel-storage` does not depend on `vel-api-types`.
- **Run events uniqueness** — `(run_id, seq)` unique on `run_events`.
- **Typed run payloads** — Run and RunEvent use `serde_json::Value` in domain/API; DB remains TEXT; (de)serialization at storage boundary.
- **Run transitions** — Immutable transitions in `vel-core` (start/succeed/fail/cancel return new `Self`).
- **Service layer** — Context generation and doctor logic live in `veld` services; routes are thin.
- **Artifact storage kind** — `storage_kind` (managed | external) in schema, core, and API.
- **Global events (optional)** — `DAEMON_STARTED` emitted on veld startup; `SCHEMA_MIGRATION_COMPLETE` emitted after migrations run (with `schema_version` in payload). Role remains system audit log.
- **Run statuses (future use)** — `RunStatus` includes `RetryScheduled` and `Blocked`; no transition logic yet (for future workflows).
- **Service-level result type** — Context generation returns `ContextRunOutput<T>` (run_id, artifact_id, context_kind, data); routes map `.data` to API response.
- **Canonical doc names** — Renamed `docs/vel-*.md` to `docs/*.md` (e.g. `runtime-concepts.md`, `data-model.md`); README and AGENTS.md updated.
- **Commitments (Phase A)** — Schema (`commitments` table + indexes), domain type in `vel-core` (`Commitment`, `CommitmentId`, `CommitmentStatus`), storage CRUD, API (POST/GET /v1/commitments, GET/PATCH /v1/commitments/:id), CLI (`vel commitments`, `vel commitment add/done/cancel/inspect`). Capture promotion: captures with `capture_type == "todo"` auto-create an open commitment (source_type capture, commitment_kind todo). **Commitment dependency graph:** storage `insert_commitment_dependency`, `list_commitment_dependencies_by_parent/child`; API GET/POST /v1/commitments/:id/dependencies; CLI `vel commitment dependencies <id>`, `vel commitment add-dependency <parent> <child> [--type blocks]`. Spec: [docs/specs/commitments.md](specs/commitments.md).
- **Phase B — Signal ingestion** — `signals` table; storage `insert_signal`, `list_signals`; API POST/GET /v1/signals; adapters: calendar (ICS path/URL → calendar_event signals), Todoist (snapshot JSON → commitments + external_task signals), activity (`vel_invocation` via POST); CLI `vel sync calendar/todoist/activity`; config `calendar_ics_url`, `calendar_ics_path`, `todoist_snapshot_path`.
- **Phase C — Inference** — `inferred_state` table; service reads signals + commitments, computes morning state (inactive/awake_unstarted/underway/engaged/at_risk), meds_pending, prep_window_active; persists one inferred_state record; emits STATE_CHANGED.
- **Phase D — Nudges** — `nudges` table; nudge engine creates meds_not_logged, meeting_prep_window, morning_drift (gentle/warning level); API GET /v1/nudges, GET/PATCH /v1/nudges/:id, POST /v1/nudges/:id/done, POST /v1/nudges/:id/snooze; CLI `vel nudges`, `vel nudge done <id>`, `vel nudge snooze <id> --minutes 10`.
- **Phase E — Notification** — Nudges surfaced via CLI (`vel nudges`) and API; desktop/watch stubbed for later.
- **Phase F — Weekly synthesis** — POST /v1/synthesis/week: run-backed (RunKind::Synthesis), writes **weekly_synthesis** artifact with canonical shape (week_start, week_end, summary, completed/open_commitment_ids, top_threads, drift_patterns, vel_self_review, suggested_adjustments); CLI `vel synthesize week`. Spec: [vel-weekly-synthesis-spec.md](specs/vel-weekly-synthesis-spec.md).
- **Phase G — Observability** — Events emitted: SIGNAL_INGESTED, STATE_CHANGED, NUDGE_GENERATED, NUDGE_RESOLVED; evaluate endpoint POST /v1/evaluate runs inference + nudge engine; CLI `vel evaluate`.
- **Schema (migrations 0012–0022)** — `current_context` (singleton), `context_timeline`, `commitment_dependencies`, `commitment_risk`, `nudge_events`, `suggestions`, `threads`, `thread_links`, `vel_self_metrics`, `assistant_transcripts`; `signals.source_ref` added. See [docs/specs/vel-migrations-and-schema-spec.md](specs/vel-migrations-and-schema-spec.md).
- **Persistent current context** — Inference engine writes canonical context (mode, morning_state, inferred_activity, next_commitment_id, prep/commute windows, meds_status, active_nudge_ids, top_risk_commitment_ids, global_risk_level/score) to `current_context`; material-change detection appends to `context_timeline`; GET /v1/context/current, GET /v1/context/timeline; CLI `vel context` (show), `vel context timeline`; `vel explain context`. Spec: [vel-current-context-spec.md](specs/vel-current-context-spec.md).
- **Nudge events** — Append-only `nudge_events` on nudge create, done, snooze; supports explainability.
- **Explainability** — GET /v1/explain/nudge/:id (nudge + inference/signals snapshot); GET /v1/explain/context; CLI `vel explain nudge <id>`, `vel explain context`.
- **Policy engine** — Resolution policies run first (auto-resolve meds/prep/drift nudges when conditions no longer hold); each nudge creation stores structured explanation in `metadata_json` (policy, decision, level, reasons). Spec: [vel-policy-engine-spec.md](specs/vel-policy-engine-spec.md).
- **Signal adapters** — On capture create, a `capture_created` signal (source `vel`) is inserted in addition to CAPTURE_CREATED event; POST /v1/signals accepts `vel_feedback` for user feedback. Spec: [vel-signal-adapter-spec.md](specs/vel-signal-adapter-spec.md).
- **Policy config** — Loaded from `config/policies.yaml` at startup (or `VEL_POLICIES_PATH`); missing/malformed file fails startup. Injected into nudge engine (default_prep_minutes, commute_leave_time thresholds). Spec: [vel-agent-next-steps-policy-config-commute.md](specs/vel-agent-next-steps-policy-config-commute.md).
- **commute_leave_time nudge** — Fires only when calendar event has `travel_minutes`; leave_by = event_start - travel_minutes; escalation ladder (gentle/warning/danger) from config; resolution on event start or commitment resolve.
- **Context explain** — GET /v1/explain/context returns `signals_used`, `commitments_used`, `risk_used`, `reasons`; inference stores used ids in current context JSON.
- **Thread graph** — Storage: insert_thread, get_thread_by_id, list_threads, update_thread_status, insert_thread_link, list_thread_links. API: GET/POST /v1/threads, GET/PATCH /v1/threads/:id, POST /v1/threads/:id/links. CLI: `vel threads`, `vel thread inspect/close/reopen`. Spec: [vel-thread-graph-spec.md](specs/vel-thread-graph-spec.md).
- **Risk engine compute** — Consequence, proximity, dependency pressure (no uncertainty/progress-penalty in first version). POST /v1/evaluate runs inference → risk::run → nudge engine; commitment_risk table populated; GET /v1/risk (compute and return), GET /v1/risk/:id; CLI `vel risk`, `vel risk <commitment_id>`. Spec: [vel-risk-engine-spec.md](specs/vel-risk-engine-spec.md), [vel-agent-next-implementation-steps.md](specs/vel-agent-next-implementation-steps.md).
- **Attention/drift in current context** — Inference writes `attention_state`, `drift_type`, `drift_severity`, `attention_confidence`, `attention_reasons` into context JSON (morning_drift, prep_drift heuristics). Spec: [vel-attention-and-drift-detection-spec.md](specs/vel-attention-and-drift-detection-spec.md).
- **Suggestions (steering loop)** — Storage: insert_suggestion, list_suggestions, get_suggestion_by_id, update_suggestion_state. API: GET /v1/suggestions, GET/PATCH /v1/suggestions/:id. CLI: `vel suggestions`, `vel suggestion inspect/accept/reject/modify`. Two types (increase_commute_buffer, increase_prep_window) to be triggered by repeated evidence; scaffolding in place. Spec: [vel-agent-next-implementation-steps.md](specs/vel-agent-next-implementation-steps.md).
- **Project synthesis** — POST /v1/synthesis/project/:slug; run_project_synthesis filters commitments by project, writes **project_synthesis** artifact (open_commitments, active_threads, completed_commitment_ids, etc.); CLI `vel synthesize project <name>`. Spec: [vel-agent-next-implementation-steps.md](specs/vel-agent-next-implementation-steps.md).
- **Integration tests** — Commute policy: no commute nudge when calendar event has no travel_minutes; context explain returns signals_used, commitments_used, reasons.
- **Specs** — Current context: [vel-current-context-spec.md](specs/vel-current-context-spec.md). Policy: [vel-policy-engine-spec.md](specs/vel-policy-engine-spec.md). Signal adapters: [vel-signal-adapter-spec.md](specs/vel-signal-adapter-spec.md). Weekly synthesis: [vel-weekly-synthesis-spec.md](specs/vel-weekly-synthesis-spec.md). Thread graph: [vel-thread-graph-spec.md](specs/vel-thread-graph-spec.md). Risk engine: [vel-risk-engine-spec.md](specs/vel-risk-engine-spec.md). Agent next steps: [vel-agent-next-steps-policy-config-commute.md](specs/vel-agent-next-steps-policy-config-commute.md). **Implementation order:** [vel-agent-next-implementation-steps.md](specs/vel-agent-next-implementation-steps.md). **Attention/drift:** [vel-attention-and-drift-detection-spec.md](specs/vel-attention-and-drift-detection-spec.md). **Architecture (planned):** [vel-distributed-and-ambient-architecture-spec.md](specs/vel-distributed-and-ambient-architecture-spec.md) (one brain / many limbs, VELD vs clients, replication, offline). **Clients (planned):** [vel-apple-and-voice-client-spec.md](specs/vel-apple-and-voice-client-spec.md) (Apple Watch, iPhone, desktop voice). Also: [vel-behavioral-constitution.md](specs/vel-behavioral-constitution.md), [vel-explicit-implementation-directive.md](specs/vel-explicit-implementation-directive.md), [vel-architecture-for-agents.md](specs/vel-architecture-for-agents.md), [vel-addendum-self-model-and-assistant-continuity.md](specs/vel-addendum-self-model-and-assistant-continuity.md).

## Partial

- **Global events** — Full set (e.g. `config_updated`) not implemented; role is narrow and documented.

## Planned next

- Run lifecycle: transition logic for RetryScheduled/Blocked once additional run-backed workflows exist.

## Next to make Vel usable daily (dogfooding roadmap)

- **Recent/review flows** — `vel recent`, `vel review today`, `vel review week`, `vel artifact latest` (implemented).
- **Capture ergonomics** — `vel capture` with `--type`, `--source`, `--stdin` / `-` (implemented).
- **Inspection** — `vel inspect artifact <id>` (implemented).
- **Synthesis** — `vel synthesize week` (run-backed; implemented). `vel synthesize project <slug>` (run-backed; implemented).
- **Import** — `vel import file`, `vel import lines`, `vel capture-url` (planned).
- **Trust** — `vel export` (dump captures/runs as JSON); `vel backup` (prints **backup instructions** only; not automated backup).

See [docs/using-vel-daily.md](using-vel-daily.md) for a simple daily workflow. **Canonical schema:** [vel-migrations-and-schema-spec.md](specs/vel-migrations-and-schema-spec.md). **Behavioral guidance:** [vel-behavioral-constitution.md](specs/vel-behavioral-constitution.md), [vel-explicit-implementation-directive.md](specs/vel-explicit-implementation-directive.md). **Self-model / transcripts (planned):** [vel-addendum-self-model-and-assistant-continuity.md](specs/vel-addendum-self-model-and-assistant-continuity.md).

## Intentionally deferred

- Distributed sync, mobile clients, execution automation, synthesis jobs, agent flows.
