# External Integrations

**Analysis Date:** 2026-03-17

## APIs & External Services

**Calendar Integration:**
- Google Calendar (OAuth) - Calendar event sync via Google Calendar API v3
  - SDK/Client: reqwest 0.12 (HTTP client)
  - Auth: OAuth 2.0 (client_id, client_secret stored in settings)
  - Base URLs: https://accounts.google.com/o/oauth2/v2/auth (auth), https://oauth2.googleapis.com/token (token), https://www.googleapis.com/calendar/v3 (API)
  - File: `crates/veld/src/services/integrations_google.rs`
  - Scope: `https://www.googleapis.com/auth/calendar.readonly` (read-only)
  - Lookback: 60 days, Lookahead: 180 days
  - Callback endpoint: `/api/integrations/google-calendar/oauth/callback` (public)

- Local ICS (Calendar) - Pull-based .ics file or URL ingestion
  - File: `crates/veld/src/adapters/calendar.rs`
  - Config: `calendar_ics_path` (local file), `calendar_ics_url` (remote URL)
  - Default path: `var/integrations/calendar/local.ics`
  - Client: reqwest for URL fetching

**Task Management:**
- Todoist - Snapshot-based task sync
  - File: `crates/veld/src/adapters/todoist.rs`, `crates/veld/src/services/integrations_todoist.rs`
  - Config: `todoist_snapshot_path` (snapshot JSON)
  - Default path: `var/integrations/todoist/snapshot.json`
  - Integration type: File snapshot (not live API polling)
  - Settings: Stored in `settings` table with key `integration_todoist`, secrets key `integration_todoist_secrets`

**Activity & Metrics:**
- Computer Activity - Workstation activity snapshot ingestion
  - File: `crates/veld/src/adapters/activity.rs`
  - Config: `activity_snapshot_path`
  - Default path: `var/integrations/activity/snapshot.json`
  - Integration type: File snapshot or local process introspection

- Health Metrics - Local health/activity snapshot (Apple HealthKit export, Oura, Whoop, etc.)
  - File: `crates/veld/src/adapters/health.rs`
  - Config: `health_snapshot_path`
  - Default path: `var/integrations/health/snapshot.json`
  - Integration type: File snapshot
  - Emits: `health_metric` signals

- Git Activity - Repository activity snapshot
  - File: `crates/veld/src/adapters/git.rs`
  - Config: `git_snapshot_path`
  - Default path: `var/integrations/git/snapshot.json`
  - Integration type: File snapshot

- Messaging - Chat/messaging activity snapshot
  - File: `crates/veld/src/adapters/messaging.rs`
  - Config: `messaging_snapshot_path`
  - Default path: `var/integrations/messaging/snapshot.json`
  - Integration type: File snapshot

- Reminders - System reminders snapshot
  - File: `crates/veld/src/adapters/reminders.rs`
  - Config: `reminders_snapshot_path`
  - Default path: `var/integrations/reminders/snapshot.json`
  - Integration type: File snapshot

- Transcripts - Chat/assistant transcript snapshot
  - File: `crates/veld/src/adapters/transcripts.rs`
  - Config: `transcript_snapshot_path`
  - Default path: `var/integrations/transcripts/snapshot.json`
  - Integration type: File snapshot

**Notes & Documents:**
- Local Notes - Markdown/plaintext note directory sync
  - File: `crates/veld/src/adapters/notes.rs`
  - Config: `notes_path`
  - Default path: `var/integrations/notes`
  - Integration type: File directory ingestion

## Data Storage

**Databases:**
- SQLite 3 (local file)
  - Connection: `VEL_DB_PATH` environment variable (default: var/data/vel.sqlite)
  - Client: sqlx 0.8 with compile-time query verification
  - Migrations: Numbered SQL files in `migrations/` directory (sqlx managed)
  - Tables: captures, artifacts, signals, commitments, chat_messages, settings, runs, events, threads, suggestions, nudges, commitments_risk, thread_links, context_timeline, inferred_state, current_context, vel_self_metrics, assistant_transcripts, runtime_loops, and more

**File Storage:**
- Local filesystem only - No cloud storage integration
  - Artifacts: `VEL_ARTIFACT_ROOT` (default: var/artifacts)
  - Snapshots: Integration snapshots stored as JSON files (var/integrations/*)
  - Logs: var/logs/

**Caching:**
- In-memory broadcast channel - Tokio broadcast for real-time sync
  - Channel size: 64 (defined in `crates/veld/src/main.rs`)
  - Used for: WebSocket subscriptions and real-time event distribution

## Authentication & Identity

**Auth Provider:**
- Custom token-based (local)
  - Operator token: `VEL_OPERATOR_API_TOKEN` (optional env var)
  - Worker token: `VEL_WORKER_API_TOKEN` (optional env var)
  - Header: `x-vel-operator-token`, `x-vel-worker-token`
  - File: `crates/veld/src/app.rs`

- Google OAuth (optional, for calendar)
  - Settings table stores: `client_id`, `client_secret`, `pending_oauth_state`, `access_token`, `refresh_token`
  - Flow: Authorization Code flow with `offline` access for refresh tokens
  - Callback: `/api/integrations/google-calendar/oauth/callback` (GET, public)

**Route Classes:**
- LocalPublic - No auth required (health checks, OAuth callbacks)
- OperatorAuthenticated - Requires operator API token (main operator routes)
- WorkerAuthenticated - Requires worker API token (cluster worker routes)
- FutureExternal - Placeholder for external access control

## Monitoring & Observability

**Error Tracking:**
- None detected - No external error tracking service (Sentry, etc.)

**Logs:**
- Local structured logging via tracing/tracing-subscriber
  - Format: Compact text with env-filter control
  - Env var: `VEL_LOG_LEVEL` (default: info)
  - Targets: Application events, database queries, HTTP requests

**Metrics:**
- None detected as external service
- Vel self-metrics table (`vel_self_metrics`) stores internal performance/health data

## CI/CD & Deployment

**Hosting:**
- Docker (containerized deployment)
  - Base image: `debian:bookworm-slim`
  - Dockerfile: `Dockerfile` in root
  - Container port: 4130 (exposed)
  - Health check: `vel health` command

**CI Pipeline:**
- None detected (GitHub Actions not configured, local make-based CI)
- Makefile targets for local verification: `make verify`, `make ci`, `make test-api`

## Environment Configuration

**Required env vars:**
- `VEL_BIND_ADDR` - Server bind address
- `VEL_DB_PATH` - SQLite database file path
- `VEL_ARTIFACT_ROOT` - Artifact storage directory
- `VEL_LOG_LEVEL` - Log level
- `VEL_BASE_URL` - External URL for OAuth callbacks and references

**Optional env vars:**
- `VEL_OPERATOR_API_TOKEN` - Bearer token for operator routes
- `VEL_WORKER_API_TOKEN` - Bearer token for worker routes
- `VEL_STRICT_HTTP_AUTH` - Enable strict auth enforcement (flag)

**Secrets location:**
- Environment variables only (.env not committed)
- Settings table stores encrypted/unhashed integration secrets (client secrets, OAuth tokens)
- Google Calendar: `client_secret` stored in settings (plaintext in DB, should consider encryption)
- File: `crates/veld/src/app.rs` loads `VEL_OPERATOR_API_TOKEN`, `VEL_WORKER_API_TOKEN` from env

## Webhooks & Callbacks

**Incoming:**
- Google Calendar OAuth callback: `/api/integrations/google-calendar/oauth/callback` (GET, public)
  - Parameters: `code` (authorization code), `state` (CSRF token)
  - File: `crates/veld/src/routes/integrations.rs`

**Outgoing:**
- None detected
- All integrations are pull-based (snapshots ingested via adapters)
- No webhook subscriptions to external services

## Integration Data Flow

**Snapshot Ingestion Pattern:**
All integrations follow a common pattern:
1. Check config path for snapshot file existence
2. Read JSON or ICS file from disk
3. Parse into typed snapshot struct
4. Transform to signals or commitments
5. Insert into database
6. Return count of entities ingested

**Files:**
- `crates/veld/src/adapters/*.rs` - Individual adapter implementations
- `crates/veld/src/services/integrations.rs` - Orchestration and bootstrap
- `crates/veld/src/services/integrations_google.rs` - Google Calendar OAuth flow
- `crates/veld/src/services/integrations_todoist.rs` - Todoist settings/secrets management

**Timing:**
- Bootstrap: On daemon startup via `services::integrations::bootstrap_local_context_sources()`
- Periodic: Background workers (file `crates/veld/src/worker.rs` runs integration loops)

---

*Integration audit: 2026-03-17*
