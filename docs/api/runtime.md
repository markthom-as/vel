# Vel Runtime API (`/v1`)

This document describes the currently mounted runtime API exposed by `veld` under `/v1`.

For repo-wide implementation truth, see [`../MASTER_PLAN.md`](../MASTER_PLAN.md). For route-level authority, inspect `crates/veld/src/app.rs`.

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
  - `/v1/connect/*`
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

- daemon and storage health
- effective runtime diagnostics
- published contract-manifest parse health (`contracts_manifest` check from `config/contracts-manifest.json`)

Exposure:

- `GET /v1/health`: `local_public`
- `GET /v1/doctor`: `operator_authenticated`

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

- `/v1/connect` and `/v1/connect/*`
- `/v1/cluster/clients` and `/v1/cluster/clients/*`

These are mounted as `future_external` and currently return `403` (deny-by-default) instead of falling through as generic undefined routes.

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

### `GET /v1/context/current`
### `GET /v1/context/timeline`
### `GET /v1/now`

- persisted current-context and operator-facing "what matters now" projections
- `GET /v1/now` is the typed place to orient in Now: it returns ranked `action_items` plus the `review_snapshot` counts (`open_action_count`, `triage_count`, `projects_needing_review`)

## Explainability and search

### `GET /v1/explain/context`
### `GET /v1/explain/nudge/:id`
### `GET /v1/explain/commitment/:id`
### `GET /v1/explain/drift`

- explainability views for context, nudges, commitments, and drift

### `GET /v1/search`

- local search across supported runtime entities

## Threads, signals, nudges, uncertainty, and loops

### `GET /v1/threads`
### `POST /v1/threads`
### `GET /v1/threads/:id`
### `PATCH /v1/threads/:id`
### `POST /v1/threads/:id/links`

- thread graph inspection and mutation

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

### `POST /v1/evaluate`

- orchestrated recompute-and-persist path for context, risk, and downstream outputs

### `POST /v1/synthesis/week`
### `POST /v1/synthesis/project/:slug`

- run-backed synthesis generation
- `POST /v1/synthesis/project/:slug` resolves typed projects by `projects.slug` first and falls back to a legacy commitment project alias only when no typed project exists
