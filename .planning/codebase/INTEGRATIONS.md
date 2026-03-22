# External Integrations

**Analysis Date:** 2026-03-22

## APIs & External Services

**LLM Backends:**
- Local `llama.cpp` HTTP servers - Primary shipped chat/codegen path through OpenAI-compatible local endpoints
  - SDK/Client: `reqwest` via `crates/veld/src/llm.rs` and `crates/vel-llm/`
  - Config: checked-in profiles in `configs/models/local-qwen3-coder.toml` and `configs/models/local-qwen25-fast.toml`
  - Endpoints used: `/v1/chat/completions` and `/v1/models` through provider adapters described in `crates/vel-llm/README.md`
- Localhost OpenAI-compatible OAuth proxy - Optional `openai_oauth` profile path for chat only
  - SDK/Client: `OpenAiOauthProvider` in `crates/veld/src/llm.rs` and `crates/vel-llm/src/providers/openai_oauth.rs`
  - Auth: mediated by the local proxy; Vel only accepts `base_url` values on `localhost` or `127.0.0.1` per `crates/veld/src/llm.rs` and `configs/models/README.md`
  - Routing: `configs/models/routing.toml` currently prefers `chat = "oauth-openai"` and falls back to `local-qwen3-coder`

**Calendar APIs:**
- Google Calendar - OAuth-backed read sync for schedule ingestion
  - Integration method: REST via `reqwest` in `crates/veld/src/services/integrations_google.rs`
  - Auth: locally stored `client_id`, `client_secret`, `refresh_token`, and access token state owned by `crates/veld/src/services/integrations.rs`
  - Endpoints used: `https://accounts.google.com/o/oauth2/v2/auth`, `https://oauth2.googleapis.com/token`, and `https://www.googleapis.com/calendar/v3`
  - Scope: readonly calendar sync only (`https://www.googleapis.com/auth/calendar.readonly`)

**Task APIs:**
- Todoist - Credential-backed sync plus bounded task writeback
  - Integration method: REST via `reqwest` in `crates/veld/src/services/integrations_todoist.rs`
  - Auth: API token stored in local secret settings records (`integration_todoist_secrets`)
  - Endpoints used: `https://api.todoist.com/api/v1` for sync listing and `https://api.todoist.com/rest/v2` for create/update/complete/reopen task operations
  - Writeback rules: conflict-aware operator routes only, documented in `docs/user/integrations/todoist.md`

**Repo/Issue Writeback:**
- GitHub - Bounded issue/comment writeback lane modeled inside Vel
  - Integration method: local writeback queue/provenance records in `crates/veld/src/services/integrations_github.rs`
  - Auth: no outbound GitHub API client is present in the current repo state
  - Operations implemented: `github_create_issue`, `github_add_comment`, `github_close_issue`, and `github_reopen_issue`
  - Important boundary: current implementation records/applies Vel-side writeback operations and repository scope metadata; it does not ship a live GitHub REST sync client

**Local Host Integrations:**
- Apple/macOS local exporters - Snapshot producers for activity, health, messaging, reminders, and local notes discovery
  - Integration method: file/snapshot ingestion documented in `docs/user/integrations/apple-macos.md` and implemented through config paths in `crates/vel-config/src/lib.rs`
  - Host dependency: local files under `~/Library/Application Support/Vel/` on macOS
- Obsidian - Notes root suggestion/discovery for `notes_path`
  - Integration method: local filesystem path selection only, described in `docs/user/integrations/local-sources.md`
  - Auth: none; uses local files rather than a remote API

## Data Storage

**Databases:**
- SQLite - Primary local durable store
  - Connection: `db_path` in `vel.toml` or `VEL_DB_PATH` override from `crates/vel-config/src/lib.rs`
  - Client: `sqlx` in `crates/vel-storage/Cargo.toml`
  - Migrations: checked-in SQL files under `migrations/`

**File Storage:**
- Local filesystem only
  - Artifacts: `artifact_root` / `VEL_ARTIFACT_ROOT` managed by `crates/vel-config/src/lib.rs`
  - Integration snapshots and local source files: defaults in `config/examples/app-config.example.toml` and `crates/vel-config/src/lib.rs`
  - Containerized data mount: `./var/docker/vel:/data` in `docker-compose.yml`

**Caching:**
- None detected as a separate service
  - Current runtime relies on SQLite plus local files; no Redis/Memcached service is configured in repo manifests

## Authentication & Identity

**Auth Provider:**
- Custom token-gated local runtime auth
  - Implementation: route-class auth in `crates/veld/src/app.rs` and `crates/veld/src/middleware/mod.rs`
  - Tokens: `VEL_OPERATOR_API_TOKEN` and `VEL_WORKER_API_TOKEN` described in `docs/api/runtime.md`
  - Default mode: local compatibility when tokens are unset unless strict mode is enabled via `VEL_STRICT_HTTP_AUTH`

**OAuth Integrations:**
- Google OAuth - Browser callback path for Google Calendar connect
  - Credentials: Google client ID and client secret entered through Settings and stored locally
  - Callback: `GET /api/integrations/google-calendar/oauth/callback` in `docs/api/runtime.md`
  - State management: pending OAuth state persisted through `crates/veld/src/services/integrations_google.rs`

## Monitoring & Observability

**Error Tracking:**
- None detected as an external SaaS
  - Runtime uses local error/status fields in integration settings and standard Rust error handling

**Logs:**
- Local tracing/log output
  - Framework: `tracing` and `tracing-subscriber` from root `Cargo.toml`
  - Runtime log level/config: `log_level` / `VEL_LOG_LEVEL` in `crates/vel-config/src/lib.rs`
  - Data location: local logs under `var/logs/` per `README.md`; container path uses `/data` in `Dockerfile`

## CI/CD & Deployment

**Hosting:**
- Local daemon or local container deployment
  - Native runtime: `veld` started by `cargo run -p veld` or `make dev-api` from `Makefile`
  - Container runtime: `Dockerfile` plus `docker-compose.yml`
  - Web UI: local Vite dev server in `clients/web/` or static build produced by `npm run build`

**CI Pipeline:**
- GitHub Actions
  - Workflow: `.github/workflows/ci.yml`
  - Checks: `make ci`, `make smoke`, and `cargo run -p veld-evals -- run ...`

## Environment Configuration

**Required env vars:**
- Core runtime: `VEL_BIND_ADDR`, `VEL_BASE_URL`, `VEL_DB_PATH`, `VEL_ARTIFACT_ROOT`, `VEL_LOG_LEVEL`, `VEL_AGENT_SPEC_PATH`
- Auth: `VEL_OPERATOR_API_TOKEN`, `VEL_WORKER_API_TOKEN`, optional `VEL_STRICT_HTTP_AUTH`
- LLM routing/runtime: `VEL_MODELS_DIR`, `VEL_LLM_MODEL`, `VEL_LLM_FAST_MODEL`
- Local-source overrides: `VEL_CALENDAR_ICS_URL`, `VEL_CALENDAR_ICS_PATH`, `VEL_TODOIST_SNAPSHOT_PATH`, `VEL_ACTIVITY_SNAPSHOT_PATH`, `VEL_HEALTH_SNAPSHOT_PATH`, `VEL_GIT_SNAPSHOT_PATH`, `VEL_MESSAGING_SNAPSHOT_PATH`, `VEL_REMINDERS_SNAPSHOT_PATH`, `VEL_NOTES_PATH`, `VEL_TRANSCRIPT_SNAPSHOT_PATH`
- Web client: `VITE_API_URL` in `clients/web/src/api/client.ts`

**Secrets location:**
- Checked-in templates/examples under `config/examples/` and `.env.example` are non-secret scaffolding
- Runtime secrets are stored locally in Vel settings records for integrations such as Google and Todoist, per `docs/user/integrations/google-calendar.md`, `docs/user/integrations/todoist.md`, and `crates/veld/src/services/integrations.rs`
- Container deployments inject runtime settings through `docker-compose.yml`

## Webhooks & Callbacks

**Incoming:**
- Google OAuth callback - `GET /api/integrations/google-calendar/oauth/callback`
  - Validation: handler-level `code` and `state` checks described in `docs/api/runtime.md` and implemented in `crates/veld/src/services/integrations_google.rs`

**Outgoing:**
- Google OAuth/token/calendar requests from `crates/veld/src/services/integrations_google.rs`
- Todoist sync and writeback requests from `crates/veld/src/services/integrations_todoist.rs`
- LLM provider HTTP calls from `crates/veld/src/llm.rs` and `crates/vel-llm/`
- No general webhook sender or inbound third-party webhook receiver is detected outside the Google OAuth callback path

---

*Integration audit: 2026-03-22*
