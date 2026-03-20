# Vel Runtime API (`/v1`)

This document describes the currently mounted runtime API exposed by `veld` under `/v1`.

For repo-wide implementation truth, see [`../MASTER_PLAN.md`](../MASTER_PLAN.md). For route-level authority, inspect `crates/veld/src/app.rs`.
For cross-surface command/query/read-model vocabulary, see [`../cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md`](../cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md).
For a shipped proof flow over these boundaries, see [`../cognitive-agent-architecture/architecture/cross-surface-proof-flows.md`](../cognitive-agent-architecture/architecture/cross-surface-proof-flows.md).

## Exposure Classes And Auth Boundary

`build_app_with_state` now mounts routes through explicit exposure classes:

- `local_public`: no auth gate (intentionally public local runtime surfaces only).
- `operator_authenticated`: operator routes gated by centralized auth policy.
- `worker_authenticated`: worker/sync coordination routes gated by centralized auth policy.
- `future_external`: reserved class; deny-by-default.

Current mounted inventory by class:

- `local_public`
  - `GET /v1/health`
  - `GET /api/integrations/google-calendar/oauth/callback`
- `operator_authenticated`
  - all mounted `/v1/*` routes except the explicitly worker-authenticated and future-external reservations below
  - `/api/components*`, `/api/integrations*` (except the OAuth callback above), chat/operator `/api/*` routes, and `/ws`
- `worker_authenticated`
  - `POST /v1/cluster/branch-sync`
  - `POST /v1/cluster/validation`
  - `POST /v1/sync/heartbeat`
  - `GET|POST|PATCH /v1/sync/work-assignments`
  - `GET /v1/sync/work-queue`
  - `POST /v1/sync/work-queue/claim-next`
  - `POST /v1/sync/actions`
  - `POST /v1/sync/branch-sync`
  - `POST /v1/sync/validation`
  - fail-closed auth expectation for `/v1/sync/work-queue*`: `GET /v1/sync/work-queue` and `POST /v1/sync/work-queue/claim-next` return `401` when worker token policy is configured and credentials are missing/invalid
- `future_external` (fail-closed reservation)
  - `/v1/connect`
  - `/v1/connect/worker`
  - `/v1/cluster/clients`
  - `/v1/cluster/clients/*`

Mounted `/api/*` and `/ws` exposure/auth matrix:

| Surface | Exposure class | Auth expectation | Fail-closed expectation |
| --- | --- | --- | --- |
| `GET /api/integrations/google-calendar/oauth/callback` | `local_public` | no operator token challenge | handler-level validation errors (for example missing `code`/`state`) return `400`, not `401` |
| `/api/components*` | `operator_authenticated` | deny `401` without valid operator token when operator token is configured | unknown paths return `404`; unsupported methods return `405` |
| `/api/integrations*` (except OAuth callback) | `operator_authenticated` | deny `401` without valid operator token when operator token is configured | unknown paths return `404`; unsupported methods return `405` |
| `/api/conversations*`, `/api/inbox`, `/api/messages/*`, `/api/settings`, `/api/interventions/*` | `operator_authenticated` | deny `401` without valid operator token when operator token is configured | unknown paths return `404`; unsupported methods return `405` |
| `/ws` | `operator_authenticated` | deny `401` without valid operator token when operator token is configured | `/ws/*` unknown paths return `404`; unsupported methods on `/ws` return `405` |

Auth extraction is centralized at the route-class boundary in `crates/veld/src/app.rs`.
Supported credentials for authenticated classes:

- class header: `x-vel-operator-token` or `x-vel-worker-token`
- bearer fallback: `Authorization: Bearer <token>`

Token configuration knobs:

- `VEL_OPERATOR_API_TOKEN` and `VEL_WORKER_API_TOKEN`: expected secrets for class-gated routes.
- `VEL_STRICT_HTTP_AUTH`: when set to `1/true/yes/on`, class-gated routes deny requests if the class token is unset.

Default local behavior keeps unset-token compatibility unless strict mode is enabled.

Undefined routes are fail-closed with an explicit `404` fallback handler.

## Health, diagnostics, and planning

### `GET /v1/health`
### `GET /v1/doctor`
### `GET /v1/planning-profile`
### `PATCH /v1/planning-profile`

- daemon and storage health
- effective runtime diagnostics
- published contract-manifest parse health (`contracts_manifest` check from `config/contracts-manifest.json`)
- typed inspection and mutation of the durable routine-planning profile used by same-day `day_plan` and `reflow`
- `GET /v1/planning-profile` returns the canonical backend-owned pack of saved routine blocks and bounded planning constraints plus compact planning-profile proposal continuity (`pending`, latest applied, latest failed) for summary surfaces
- `PATCH /v1/planning-profile` applies one typed mutation at a time (`upsert_*` or `remove_*`) with service-level validation and explicit failure for malformed or missing targets
- `POST /v1/planning-profile/proposals/:id/apply` resolves an approved or staged planning-profile proposal thread through the same canonical mutation seam and records `approved`, `applied`, or `failed` continuity back onto that thread
- web `Settings`, CLI, and Apple summary surfaces now inspect that same canonical profile instead of maintaining shell-local planning state
- assistant and voice shells do not bypass this seam: they may stage bounded planning-profile edit proposals, but supervised approval/application still resolves back through the canonical profile mutation model

Exposure:

- `GET /v1/health`: `local_public`
- `GET /v1/doctor`: `operator_authenticated`
- `GET /v1/planning-profile`: `operator_authenticated`
- `PATCH /v1/planning-profile`: `operator_authenticated`

### `GET /v1/backup/status`
### `POST /v1/backup/create`
### `POST /v1/backup/inspect`
### `POST /v1/backup/verify`

- authenticated backup trust surfaces
- `status` returns the backend-owned backup state used by doctor, settings, CLI, and web trust cards
- `create` writes a typed local backup pack with manifest, SQLite snapshot, bounded artifact/config coverage, and explicit omissions
- `inspect` reads a pack non-destructively
- `verify` re-checks manifest path boundaries and checksum state fail-closed

CLI projection:

- `vel backup --create [--output-root <dir>]`
- `vel backup --inspect <backup_root>`
- `vel backup --verify <backup_root>`
- `vel backup --dry-run-restore <backup_root>`

### `POST /v1/command/plan`
### `POST /v1/command/execute`

- command-language planning and execution scaffolding for structured operator intents

## Cluster and coordination

### `GET /v1/cluster/bootstrap`
### `GET /v1/cluster/workers`

- bootstrap metadata and worker-registry inspection

### `POST /v1/cluster/branch-sync`
### `POST /v1/cluster/validation`

- cluster coordination requests for branch-sync and validation flows

Exposure:

- `GET /v1/cluster/bootstrap`: `operator_authenticated`
- `GET /v1/cluster/workers`: `operator_authenticated`
- `POST /v1/cluster/branch-sync`: `worker_authenticated`
- `POST /v1/cluster/validation`: `worker_authenticated`

Reserved for future external/connect architecture:

- `/v1/connect` and `/v1/connect/worker`
- `/v1/cluster/clients` and `/v1/cluster/clients/*`

These are mounted as `future_external` and currently return `403` (deny-by-default) instead of falling through as generic undefined routes.

## Connect runtime lifecycle

### `GET /v1/connect/instances`
### `POST /v1/connect/instances`
### `GET /v1/connect/instances/:id`
### `POST /v1/connect/instances/:id/heartbeat`
### `POST /v1/connect/instances/:id/terminate`

- operator-authenticated connect-runtime lifecycle for supervised local coding runtimes
- `POST /v1/connect/instances` accepts `runtime_kind: "local_command"` and `runtime_kind: "wasm_guest"`
- launches create a persisted backing run plus a persisted connect-run lease record keyed by the same `run_id`
- `runtime_kind: "wasm_guest"` executes through the same brokered sandbox policy boundary and then terminates immediately with an inspectable terminal reason
- `POST /v1/connect/instances/:id/heartbeat` extends the lease for a running instance
- `POST /v1/connect/instances/:id/terminate` marks the connect instance terminated and cancels the backing run
- unsupported runtime kinds fail closed with `400`
- writable roots that escape the declared working directory fail closed with `403`
- guest modules that request undeclared writable roots or any network expansion fail closed with `403`
- unsupported `/v1/connect` and `/v1/connect/worker` paths remain reserved under `future_external`

CLI inspection surface:

- `vel connect instances`
- `vel connect inspect <run_id>`

Repo-local supervised workflow:

- persist/export context with `vel exec save|preview|export`
- review persisted handoffs with `vel exec review`, `vel exec launch-preview`, `vel exec approve`, and `vel exec reject`
- launch the approved runtime through authenticated `POST /v1/connect/instances`
- `vel exec preview|export` now includes `agent-grounding.md` and `agent-inspect.json` under the bounded `.planning/vel` output directory so repo-local agents receive the same backend-owned grounding contract exposed by `GET /v1/agent/inspect`

## Execution handoff review

### `GET /v1/agent/inspect`
### `GET /v1/execution/handoffs`
### `POST /v1/execution/handoffs`
### `GET /v1/execution/handoffs/:id/launch-preview`
### `POST /v1/execution/handoffs/:id/approve`
### `POST /v1/execution/handoffs/:id/reject`

- operator-authenticated backend-owned inspect surface for grounded agents and operator tooling
- returns one typed grounding bundle over current `Now`, current context references, projects, people, open commitments, review pressure, and pending execution handoffs
- capability groups are summarized server-side and fail closed with explicit blockers when SAFE MODE keeps writeback disabled, handoff review is still pending, or no approved repo-local write grant exists
- this route does not widen `/api/chat` behavior or create a second persisted agent-state blob
- assistant recall and grounding should build on this same backend-owned substrate: bounded recall context is assembled from persisted Vel records plus inspect/`Now` state, not from shell-local memory heuristics
- typed human-to-agent and agent-to-agent handoff persistence for coding work
- routing decisions keep task kind, agent profile, token budget, review gate, scopes, and explicit reasons first-class
- launch preview reports whether review state still blocks execution
- approvals and rejections update explicit review state; launch readiness does not rely on implicit side effects
- pending handoffs are surfaced on operator `Now` and the General settings review queue
- assistant proposal continuity can point into this same lane: repo-local proposal threads may move from `staged` to `approved` when handoff review succeeds, and their thread metadata preserves the launch-preview follow-through without creating a second execution state model

## Capture and journal

### `GET /v1/captures`
### `POST /v1/captures`
### `GET /v1/captures/:id`

- capture intake and listing

### `POST /v1/journal/mood`
### `POST /v1/journal/pain`

- typed journal event creation

## Commitments, risk, and suggestions

### `GET /v1/commitments`
### `POST /v1/commitments`
### `GET /v1/commitments/:id`
### `PATCH /v1/commitments/:id`
### `GET /v1/commitments/:id/dependencies`
### `POST /v1/commitments/:id/dependencies`

- commitment CRUD plus dependency management
- persisted commitment records now carry canonical `scheduler_rules` derived from compatibility labels, text tokens, and due metadata
- these rules are the backend-owned scheduling semantics used by same-day `reflow`, assistant context, and grounding paths
- raw upstream labels remain compatibility/search metadata rather than the durable runtime truth for scheduling behavior
- durable routine blocks and bounded planning constraints are now persisted separately from generic settings and are consumed by same-day `day_plan` / `reflow` as backend-owned planning inputs
- those durable planning inputs are now operator-manageable through `/v1/planning-profile`; shells should submit typed profile mutations there instead of hiding routine edits inside generic settings JSON

### `GET /v1/projects`
### `GET /v1/projects/:id`
### `POST /v1/projects`
### `GET /v1/projects/families`

- typed project workspace list/detail/create surfaces
- project creation is local-first and stores pending upstream confirmation only
- no repo, notes root, or external upstream record is created by this phase

### `POST /v1/linking/tokens`
### `POST /v1/linking/redeem`
### `GET /v1/linking/status`
### `POST /v1/linking/revoke/:node_id`

- guided linking backend for short-lived scoped pairing tokens and durable linked-node trust state
- operators can inspect current linked-node status plus granted scopes before trusting a link
- redemption fails closed for malformed, expired, already-redeemed, or out-of-scope tokens

CLI fallback when the web shell is unavailable:

- `vel node link issue --read-context --write-safe-actions --expires-seconds 900`
- `vel node link redeem <token_code> --node-id <node_id> --node-display-name "<display name>"`
- `vel node status`

`vel node status` exposes granted scopes so the operator can inspect read/write/execute access before trusting a link.

### `GET /v1/risk`
### `GET /v1/risk/:id`

- read-only risk views backed by persisted evaluation state

### `GET /v1/suggestions`
### `GET /v1/suggestions/:id`
### `PATCH /v1/suggestions/:id`
### `GET /v1/suggestions/:id/evidence`
### `POST /v1/suggestions/:id/accept`
### `POST /v1/suggestions/:id/reject`

- suggestion review, evidence inspection, and operator decisions

## Artifacts, runs, and context

### `GET /v1/artifacts`
### `POST /v1/artifacts`
### `GET /v1/artifacts/latest`
### `GET /v1/artifacts/:id`

- persisted artifacts produced by runs or explicit creation

### `GET /v1/runs`
### `GET /v1/runs/:id`
### `PATCH /v1/runs/:id`

- run inspection and terminal-state updates
- run summaries/details now expose `trace_id` for every run and `parent_run_id` when the run is part of a delegated chain
- compatibility rule: when older persisted runs do not carry an explicit trace identifier, the runtime falls back to the stable `run_id` as the trace ID
- operator surfaces should treat `trace_id` as the workflow inspection key and `parent_run_id` as lineage context, not as a substitute for the concrete `run_id`

### `GET /v1/context/today`
### `GET /v1/context/morning`
### `GET /v1/context/end-of-day`

- run-backed context generation endpoints
- `GET /v1/context/morning` remains available as the legacy context brief; it is not the bounded Phase 10 morning/standup session authority

### `POST /v1/daily-loop/sessions`
### `GET /v1/daily-loop/sessions/active`
### `POST /v1/daily-loop/sessions/:id/turn`

- backend-owned morning overview and standup session-turn authority
- shared by CLI (`vel morning`, `vel standup`), web Now, and Apple clients
- `POST /v1/daily-loop/sessions` starts a typed session for `phase=morning_overview` or `phase=standup`
- `GET /v1/daily-loop/sessions/active` resumes the active session for a `session_date` and `phase`
- `POST /v1/daily-loop/sessions/:id/turn` advances the current prompt with bounded submit/skip actions and returns the updated typed session
- when a daily-loop `check_in` needs longer follow-through, the backend preserves a deterministic thread-backed escalation target and updates that thread with typed deferred/resolved status as the session advances

### `GET /v1/context/current`
### `GET /v1/context/timeline`
### `GET /v1/now`

- persisted current-context and operator-facing "what matters now" projections
- `GET /v1/now` is the typed place to orient in Now: it returns ranked `action_items` plus the `review_snapshot` counts (`open_action_count`, `triage_count`, `projects_needing_review`)
- current shipped web `Now` treats this route as an execution-first current-day surface: compact context bar, current status, next event, unified today lane, and compressed attention indicators all derive from `GET /v1/now`
- that current-day model is sleep-relative rather than midnight-bound: the backend keeps the same operator day active until the rollover boundary is crossed, so late-night unfinished work, remaining commitments, and relevant events can still belong to the same day
- `next_event` is the next future relevant calendar event, not the current event and not routine/noise placeholders; all-day, free/transparent, declined, and cancelled calendar rows are filtered before the shell sees them
- the unified today lane is commitment-first on the backend seam: `next_commitment` and `other_open` represent work already in play for the current sleep-relative day, while `todoist` remains the secondary pullable task lane
- Apple surfaces should treat `GET /v1/now` as the schedule and quick-loop authority instead of synthesizing schedule answers locally
- cross-surface continuity should stay equally bounded: `Now` may resurface one clearly ranked resumable thread, but shells should not widen that into a live thread inbox on the main surface
- the `day_plan` portion of `GET /v1/now` now carries a backend-owned bounded same-day planning proposal with explicit `scheduled`, `deferred`, `did_not_fit`, and `needs_judgment` outcomes plus the routine blocks used to shape the proposal
- those routine blocks now come from the durable routine-planning profile when configured, with inferred fallback only when no durable blocks exist
- bounded planning constraints can now influence default time-window preference, calendar buffer windows, and overflow judgment inside the same backend-owned planning substrate
- the `reflow` portion of `GET /v1/now` now carries a backend-owned same-day recovery proposal with explicit `moved`, `unscheduled`, `needs_judgment`, and normalized scheduler `rule_facets`
- `GET /v1/now` now also carries compact same-day scheduling proposal continuity (`pending_count`, latest pending, latest applied, latest failed) over the same backend-owned commitment-scheduling seam used by supervised apply
- shells should render `day_plan` and `reflow` directly from those typed proposals; they should not compute their own placements, diffs, or routine semantics locally
- shells may render the compact continuity summary, but they must not derive scheduling state locally from raw thread metadata or invent planner-side apply rules
- staged assistant proposals feed this same operator lane: trust/readiness follow-through and ranked action items may surface assistant-originated review work, but the runtime still uses the canonical operator queue and review state instead of a chat-only side channel
- current recall limit: the shipped semantic layer is still a bounded local hybrid retrieval baseline over persisted Vel data. It returns explainable scores and provenance, but it is not yet a broad graph-memory or hosted RAG system.
- current closeout limit: `GET /v1/now` is sufficient for compact current-day orientation and next-event truth, but the runtime still does not publish a separate contextual-help payload or a dedicated forward-browse/pagination contract for schedule exploration on the main `Now` surface

## Apple quick loops

### `POST /v1/apple/voice/turn`
### `GET /v1/apple/behavior-summary`

- operator-authenticated Apple shell routes for iPhone/watch quick loops
- `POST /v1/apple/voice/turn` persists transcript provenance first, then returns a typed backend-owned reply for supported Apple intents; `MorningBriefing` delegates into the shared `/v1/daily-loop/*` authority after transcript capture instead of using a separate Apple-only morning policy path
- for supported backend-handled voice turns, the route also preserves shared thread continuity and may return a `thread_id` hint so Apple can acknowledge follow-up without inventing local thread policy
- bounded planning-profile edit requests now use that same confirmation-first voice lane: Apple can stage a typed routine-block or planning-constraint proposal, but the edit remains staged and thread-backed rather than silently applying profile mutations
- when a planning-profile edit is recognized, the route returns typed proposal continuity metadata rather than mutating saved routines directly; Apple should treat that as an explicit follow-through handoff, not as an already-applied planner change
- `GET /v1/apple/behavior-summary` returns the bounded daily behavior rollup used by Apple quick-loop surfaces
- Apple clients should send the same operator auth headers as the rest of `/v1/*` (`x-vel-operator-token` or `Authorization: Bearer <token>`) when token policy is configured
- safe offline Apple mutations should continue to reuse `POST /v1/sync/actions`; clients should not invent a parallel Apple-only write lane
- current limit: Apple still depends on the dedicated `/v1/apple/voice/turn` compatibility route for typed quick-loop replies, while browser/desktop voice goes through `/api/assistant/entry` after local speech-to-text
- Phase 37 adds the first additive iPhone embedded-capable seam for bounded local helper flows such as cached-`Now` hydration and quick-action preparation, but daemon-backed HTTP remains the current authority path and the route surface above is still the source of truth
- Phase 38 now defines the local-first iPhone voice/offline contract over that seam: cached `Now`, queued voice capture, local quick actions, and local thread drafts are allowed as one bounded recovery lane, but canonical thread merge and backend-only answers remain daemon-backed
- Apple shells may surface that lane as compact continuity in `Now` and `Threads` only: local draft ready, recovery pending, merged, or saved-to-Threads. They must not invent a second offline-only thread model or imply that cached answers were freshly reasoned locally

### `POST /api/assistant/entry`

- browser/desktop assistant entry remains the shared text and local-STT route for backend-owned conversation, daily-loop starts, end-of-day closeout, staged assistant proposals, and now staged planning-profile edits
- when a planning-profile request is recognized, the backend stages a typed `PlanningProfileEditProposal` over the canonical mutation seam, returns explicit `thread_id` / `thread_type` continuity metadata, and does not apply the edit silently
- these staged edits intentionally reuse the same backend-owned planning-profile vocabulary that `GET/PATCH /v1/planning-profile` exposes; assistant entry is a proposal surface, not a parallel planner API
- current limit: this route stages bounded routine-block and planning-constraint edits only; later application still requires the supervised proposal-apply lane and should remain explicit in `Threads` continuity rather than becoming an inline planner write

### `POST /v1/commitment-scheduling/proposals/:id/apply`

- supervised backend apply lane for staged same-day `day_plan` / `reflow` scheduling proposals
- reconstructs the typed proposal from thread metadata, applies canonical commitment mutations, and records `approved`, `applied`, or `failed` continuity back into the proposal thread
- summary surfaces such as web `Now`, CLI review output, and Apple quick-loop shells should reflect that compact continuity from backend DTOs rather than inventing second planner state

## Explainability and search

### `GET /v1/explain/context`
### `GET /v1/explain/nudge/:id`
### `GET /v1/explain/commitment/:id`
### `GET /v1/explain/drift`

- explainability views for context, nudges, commitments, and drift

### `GET /v1/search`

- local search across supported runtime entities

## Threads, signals, nudges, uncertainty, and loops

Phase 12 shell/help contract note:

- there is no separate contextual-help runtime payload yet
- operator shell and settings help should route through the existing typed surfaces in this document plus the user guides under `docs/user/`
- if a later Phase 12 slice needs new shell/help metadata, add it as a typed contract first instead of hiding it in client-only state

### `GET /v1/threads`
### `POST /v1/threads`
### `GET /v1/threads/:id`
### `PATCH /v1/threads/:id`
### `POST /v1/threads/:id/links`

- thread graph inspection and mutation
- thread detail may include typed metadata describing follow-through state for backend-owned `check_in`, `reflow`, intervention, and commitment resolution work
- for reflow specifically, Threads is the longer-form continuity lane after `Now` has already surfaced the compact recovery summary; that metadata should be treated as the authority for what was edited, deferred, or left unresolved
- assistant-mediated staged actions use the same thread continuity seam: dedicated `assistant_proposal` threads can carry typed confirmation, execution-handoff review, or gated follow-through metadata
- once the operator resolves or dismisses the matching intervention, those same proposal threads can also carry typed `applied`, `failed`, or `reversed` lifecycle state
- current limit: reversal metadata only records what Vel can honestly say about the assistant proposal lane. It does not imply that every upstream provider has a durable undo path.
- shells should treat that metadata as the continuity truth for resolution history rather than deriving meaning only from thread titles or message text

### `GET /v1/signals`
### `POST /v1/signals`

- signal listing and manual signal creation

### `GET /v1/nudges`
### `GET /v1/nudges/:id`
### `POST /v1/nudges/:id/done`
### `POST /v1/nudges/:id/snooze`
### `POST /v1/nudges/:id/dismiss`

- operator actions on generated nudges

### `GET /v1/uncertainty`
### `GET /v1/uncertainty/:id`
### `POST /v1/uncertainty/:id/resolve`

- uncertainty review and resolution

### `GET /v1/loops`
### `GET /v1/loops/:kind`
### `PATCH /v1/loops/:kind`

- loop configuration and inspection

## Sync, evaluation, and synthesis

### `POST /v1/sync/calendar`
### `POST /v1/sync/todoist`
### `POST /v1/sync/activity`
### `POST /v1/sync/health`
### `POST /v1/sync/git`
### `POST /v1/sync/messaging`
### `POST /v1/sync/reminders`
### `POST /v1/sync/notes`
### `POST /v1/sync/transcripts`
### `GET /v1/sync/bootstrap`
### `GET /v1/sync/cluster`
### `POST /v1/sync/heartbeat`
### `GET|POST|PATCH /v1/sync/work-assignments`
### `GET /v1/sync/work-queue`
### `POST /v1/sync/work-queue/claim-next`
### `POST /v1/sync/actions`
### `POST /v1/sync/branch-sync`
### `POST /v1/sync/validation`

- local-source sync, distributed coordination, and worker assignment surfaces

Exposure:

- `POST /v1/sync/heartbeat`: `worker_authenticated`
- `GET|POST|PATCH /v1/sync/work-assignments`: `worker_authenticated`
- `GET /v1/sync/work-queue`: `worker_authenticated`
- `POST /v1/sync/work-queue/claim-next`: `worker_authenticated`
- `POST /v1/sync/actions`: `worker_authenticated`
- `POST /v1/sync/branch-sync`: `worker_authenticated`
- `POST /v1/sync/validation`: `worker_authenticated`
- remaining `/v1/sync/*` routes in this section: `operator_authenticated`

Todoist read/write boundary:

- `POST /v1/sync/todoist` remains the read/sync path.
- The allowed Todoist write surface is bounded to `todoist_create_task`, `todoist_update_task`, `todoist_complete_task`, and `todoist_reopen_task` through `/api/integrations/todoist/create-task`, `/api/integrations/todoist/update-task`, `/api/integrations/todoist/complete-task`, and `/api/integrations/todoist/reopen-task`.
- Those write routes are `operator_authenticated`, execute only after checking the latest upstream task state, and are denied while runtime `writeback_enabled` is false.
- If upstream state drifted since the last synced snapshot, the runtime opens a conflict review item with `stale_write` or `upstream_vs_local` instead of silently overwriting.
- Todoist labels remain compatibility-only metadata at the adapter boundary; Vel's durable typed contract is `project_id`, `scheduled_for`, `priority`, `waiting_on`, and `review_state`.

Notes and reminders write boundary:

- `POST /v1/sync/notes`, `POST /v1/sync/reminders`, and `POST /v1/sync/transcripts` remain the read/sync paths.
- The allowed notes write surface is bounded to `notes_create_note` and `notes_append_note` through `/api/integrations/notes/create-note` and `/api/integrations/notes/append-note`.
- Notes and reminders writes are denied while runtime `writeback_enabled` is false so SAFE MODE remains the default operator posture.
- Notes writes are scoped to the configured `notes_path` or a typed project's project notes roots. Out-of-scope writes are persisted as `blocked` instead of escaping the configured filesystem boundary.
- Transcript ingestion is read-only and is tagged as a notes source subtype so transcript context folds under the same local-first notes lane without becoming a separate write surface.
- Reminder writes are intent-based through `/api/integrations/reminders/create`, `/api/integrations/reminders/update`, and `/api/integrations/reminders/complete`.
- Reminder intents persist explicit lifecycle state through durable writeback and conflict records: `queued`, `applied`, `executor_unavailable`, and `conflicted`. Safe-mode denials stay outside that queue and return a direct forbidden response until the operator opts in.
- If no approved local reminder executor is available, the runtime opens an `executor_unavailable` conflict instead of pretending the write succeeded.

GitHub and email write boundary:

- The allowed GitHub write surface is bounded to `github_create_issue`, `github_add_comment`, `github_close_issue`, and `github_reopen_issue` through `/api/integrations/github/create-issue`, `/api/integrations/github/add-comment`, `/api/integrations/github/close-issue`, and `/api/integrations/github/reopen-issue`.
- GitHub writeback records persist typed `project_id` linkage, provider-scoped provenance, and `PersonAlias`-compatible assignee or participant handles where Vel can resolve them from the people registry.
- The allowed email write surface is bounded to `email_create_draft_reply` and `email_send_draft` through `/api/integrations/email/create-draft-reply` and `/api/integrations/email/send-draft`.
- Email remains draft-first: `email_create_draft_reply` is the safe default, while `email_send_draft` is confirm-required and is denied with durable writeback history until the operator explicitly confirms send.
- Both GitHub and email routes are `operator_authenticated`, denied while runtime `writeback_enabled` is false, and use the same writeback/conflict vocabulary as the other Phase 06 write lanes.
- Operator-facing status surfaces expose `pending_writebacks`, `open_conflicts`, and people-linked review items so the queue stays inspectable before and after writeback is enabled.

### `POST /v1/evaluate`

- orchestrated recompute-and-persist path for context, risk, and downstream outputs

### `POST /v1/synthesis/week`
### `POST /v1/synthesis/project/:slug`

- run-backed synthesis generation
- `POST /v1/synthesis/project/:slug` resolves typed projects by `projects.slug` first and falls back to a legacy commitment project alias only when no typed project exists
