# Architecture

**Analysis Date:** 2026-03-17

## Pattern Overview

**Overall:** Layered backend (HTTP → Services → Storage) with separate frontend layer

**Key Characteristics:**
- Strict dependency layering: routes are thin, services hold logic, storage is substrate-agnostic
- Domain-first design: core types live in `vel-core`, independent of HTTP/storage concerns
- WebSocket broadcast for real-time sync to connected clients
- Event-driven: operations emit events to storage for audit/replay
- Per-entity repositories in storage layer; no ORM—hand-mapped sqlx queries
- Thin client surfaces (web/Apple) that are UI shells only; policy and state rules belong in Rust backend

## Layers

**vel-core (Domain Types):**
- Purpose: Domain semantics and invariants independent of transport or persistence
- Location: `crates/vel-core/src/`
- Contains: Types like `CaptureId`, `ContextCapture`, `Commitment`, `Risk`, `Message`, `Run`, `Loop`, `Intervention`, `Provenance`, `Uncertainty`, `SearchResult`, etc.
- Depends on: Nothing—no imports from other crates
- Used by: All backend layers (storage, services, routes) and type definitions

**vel-config (Configuration Loading):**
- Purpose: Parse and manage TOML/YAML configuration at startup
- Location: `crates/vel-config/src/`
- Contains: `AppConfig` (bind address, database path, artifact root, log level), model routing, contracts manifest, policy loading
- Depends on: serde, toml, serde_yaml; no other crates
- Used by: veld daemon initialization

**vel-storage (SQLite Repositories):**
- Purpose: Isolated SQLite data access via sqlx with compile-time verified queries
- Location: `crates/vel-storage/src/repositories/`
- Contains: One repository file per entity (e.g., `captures_repo.rs`, `commitments_repo.rs`, `threads_repo.rs`, `runs_repo.rs`)
- Depends on: vel-core (types only), sqlx, time
- Used by: Services; must NOT depend on `vel-api-types`
- Key files: `db.rs` (StorageError, schema types), `infra.rs` (database initialization), `lib.rs` (Storage struct facade)

**vel-llm (LLM Provider Abstraction):**
- Purpose: Provider-agnostic interface for language model inference (local/remote)
- Location: `crates/vel-llm/src/`
- Contains: `Router` (dispatch to available models), `Provider` trait, registry of available providers
- Depends on: reqwest, serde, time
- Used by: Services for chat assistant and inference tasks

**vel-api-types (HTTP DTOs):**
- Purpose: Request/response types for HTTP contracts only
- Location: `crates/vel-api-types/src/lib.rs`
- Contains: Serde-derived request/response structs, `ApiResponse<T>` wrapper, decoders
- Depends on: serde, vel-core
- Used by: Routes and web client types
- **Critical rule:** must NOT be imported by storage layer

**veld (HTTP Daemon):**
- Purpose: Axum-based HTTP server, WebSocket, background worker
- Location: `crates/veld/src/`
- Layers within:
  - **routes/** — HTTP handlers; thin layer that parses → calls service → maps DTO → error handling
  - **services/** — Application logic, business rules, multi-step operations
  - **adapters/** — Integration drivers for external systems (calendar, todoist, git, health, messaging, notes, reminders, transcripts, activity)
  - **state.rs** — `AppState` (immutable request-scoped injection: storage, config, policy config, llm router, broadcast channel, worker runtime)
  - **app.rs** — Router construction, auth middleware, exposure gates (LocalPublic, OperatorAuthenticated, WorkerAuthenticated)
  - **errors.rs** — `AppError` with HTTP status mapping
  - **broadcast.rs** — WebSocket envelope types for real-time sync
  - **worker.rs** — Background task executor
  - **policy_config.rs** — Policy loading and application
  - **llm.rs** — LLM initialization
- Depends on: All other crates, axum, tokio, tower-http, tracing

**vel-cli (CLI Binary):**
- Purpose: Command-line operator interface
- Location: `crates/vel-cli/src/`
- Contains: `commands/` (subcommands), `client.rs` (HTTP client to veld)
- Depends on: vel-core, vel-config, reqwest, clap

## Data Flow

**Capture Creation (Web/CLI → API → Storage → Worker):**

1. Client sends POST `/v1/captures` with text content
2. Route handler (`routes/captures.rs::create_capture`) validates non-empty text
3. Handler calls `state.storage.insert_capture(CaptureInsert {..})`
4. Storage transaction begins: insert row in `captures` table, insert processing job with status `Pending` in `processing_jobs` table
5. Transaction commits, returns `CaptureId`
6. Handler emits event `CAPTURE_CREATED` to storage (for audit trail)
7. Handler returns `ApiResponse::success(CaptureCreateResponse {id})`
8. Background worker (`worker.rs`) polls `processing_jobs` for `Pending` jobs, executes job (ingest logic), updates job status
9. Worker may broadcast sync event to WebSocket clients via `broadcast_tx`

**Chat Conversation & Message Flow:**

1. Client creates conversation POST `/api/conversations` or opens existing
2. Service (`services/chat/mod.rs`) loads conversation and threads
3. Client sends message POST `/api/conversations/{id}/messages`
4. Route handler calls service to append message to thread
5. Service calls `services/chat/assistant.rs` to generate response (calls LLM via `state.llm_router`)
6. Service stores assistant message, updates message mapping (provenance, sensitivity)
7. Handler broadcasts to WebSocket clients via `broadcast_tx`
8. Client's WebSocket listener receives update and refreshes conversation view

**Context Run Execution (Morning/Now/End-of-Day):**

1. Client requests GET `/v1/context/now` or `/v1/context/today`
2. Service (`services/context_generation.rs` or `services/context_runs.rs`) loads relevant data:
   - Recent captures (via `storage.list_captures_recent`)
   - Active commitments (via `storage.list_commitments`)
   - Current signals, inferred state
   - Risk assessment (via `services/risk.rs`)
3. Service composes context prompt, calls LLM for synthesis
4. Service stores result in `current_context` table
5. Service stores metadata in `context_timeline`
6. Handler returns synthesized context to client
7. Client renders as ContextPanel

**State Management:**
- Backend: Single source of truth in SQLite. Storage facade (`vel-storage::Storage`) marshals all reads/writes.
- Frontend (web): Query cache in `data/query.ts` with automatic invalidation. WebSocket sync layer (`data/ws-sync.ts`) receives real-time updates and invalidates queries.
- No in-memory state durability expectations; everything persists to disk.

## Key Abstractions

**Storage Facade (`vel-storage::Storage`):**
- Purpose: Single entry point for all database operations
- Examples: `insert_capture`, `list_captures_recent`, `get_conversation`, `update_commitment`
- Pattern: Per-entity repository modules + public async methods on Storage struct
- Located at: `crates/vel-storage/src/lib.rs`

**Service Pattern:**
- Purpose: Encapsulate multi-step logic, call storage, call external services (LLM, integrations)
- Examples: `services/chat/assistant.rs`, `services/context_generation.rs`, `services/evaluate.rs`
- Pattern: Async functions receiving storage, config, state dependencies; return domain types or errors
- Located at: `crates/veld/src/services/`

**Route Handler Pattern:**
- Purpose: HTTP contract + auth + validation → service call → DTO mapping → response
- Pattern:
  ```rust
  pub async fn create_capture(
    State(state): State<AppState>,
    Json(payload): Json<CaptureCreateRequest>,
  ) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
    // 1. Validate input
    if payload.content_text.trim().is_empty() {
      return Err(AppError::bad_request("..."));
    }
    // 2. Call service/storage
    let capture_id = state.storage.insert_capture(...).await?;
    // 3. Emit event for audit
    state.storage.emit_event("CAPTURE_CREATED", ...).await?;
    // 4. Map to response DTO
    Ok(Json(ApiResponse::success(CaptureCreateResponse { id: capture_id })))
  }
  ```
- Located at: `crates/veld/src/routes/`

**AppState (Request-Scoped Injection):**
- Purpose: Immutable shared context for all request handlers
- Fields:
  - `storage: Storage` — database facade
  - `config: AppConfig` — application configuration
  - `policy_config: PolicyConfig` — policies and rules
  - `worker_runtime: Arc<WorkerRuntimeState>` — background job concurrency tracking
  - `broadcast_tx: broadcast::Sender<WsEnvelope>` — WebSocket broadcast channel
  - `llm_router: Option<Arc<Router>>` — LLM inference router (if configured)
  - `chat_profile_id: Option<String>` — active profile for chat
  - `chat_fallback_profile_id: Option<String>` — fallback LLM profile for overflow
- Located at: `crates/veld/src/state.rs`

**API Response Wrapper:**
- Purpose: Standardized JSON envelope for all responses
- Fields: `ok: bool`, `data?: T`, `error?: {code, message}`, `request_id: string`
- Located at: `crates/vel-api-types/src/lib.rs`

**Processing Job Queue:**
- Purpose: Async work scheduling (capture ingest, context synthesis, etc.)
- Table: `processing_jobs` (id, job_type, status, payload_json, created_at, started_at, completed_at)
- Worker polls and executes jobs, updates status to `Running` → `Completed` or `Failed`
- Located at: `crates/veld/src/worker.rs`

**WebSocket Broadcast Envelope:**
- Purpose: Real-time sync messages to connected UI clients
- Types: Update events (message created, commitment changed, context updated)
- Mechanism: Services/routes call `broadcast_tx.send(WsEnvelope {...})`; `/ws` endpoint listens on receiver
- Located at: `crates/veld/src/broadcast.rs`

## Entry Points

**HTTP Server (`crates/veld/src/main.rs`):**
- Location: `crates/veld/src/main.rs`
- Triggers: `make dev-api` or daemon startup
- Responsibilities:
  1. Initialize tracing
  2. Load `vel.toml` config
  3. Load `config/policies.yaml` policy config
  4. Connect to SQLite, run migrations
  5. Initialize LLM router if models configured
  6. Build Axum router with routes (public + authenticated)
  7. Spawn background worker task
  8. Listen on `config.bind_addr`

**Web Dev Server (`clients/web/vite.config.ts`):**
- Triggers: `make dev-web`
- Proxies `/api`, `/v1`, `/ws` to `http://localhost:4130` (veld daemon)

**Web App Entry (`clients/web/src/App.tsx`):**
- Triggers: Browser navigation to http://localhost:5173
- Responsibilities:
  1. Initialize state for active conversation, main view (now/threads/inbox/settings)
  2. Render AppShell (sidebar + main panel + context panel)
  3. Load chat conversations list
  4. Load context/signals/suggestions on demand
  5. Establish WebSocket connection for real-time sync

**CLI Entry (`crates/vel-cli/src/main.rs`):**
- Triggers: `vel <command>` invocation
- Responsibilities:
  1. Parse command-line arguments (clap)
  2. Create HTTP client pointing to daemon
  3. Execute subcommand (capture, health, runs, config show, today, etc.)
  4. Pretty-print results to stdout

## Error Handling

**Strategy:** Hierarchical error types with HTTP status mapping

**Patterns:**
- **Storage Layer:** `StorageError` (enum wrapping sqlx errors) → caught by route handlers
- **Service Layer:** Return `Result<T, StorageError>` or anyhow::Result for complex operations
- **Route Layer:** Map errors to `AppError` with `StatusCode` and error code string
- **HTTP Response:** `AppError::IntoResponse` wraps error in `ApiResponse::error {code, message, request_id}`

**Error Codes (HTTP layer):**
- `validation_error` — 400 Bad Request
- `not_found` — 404 Not Found
- `internal_error` — 500 Internal Server Error
- `unauthorized` — 401 Unauthorized (if auth check fails)

**Example:**
```rust
// Storage layer
Err(sqlx::Error::RowNotFound) → StorageError::NotFound

// Service layer
state.storage.get_capture_by_id(id).await? // Propagates StorageError

// Route layer
.ok_or_else(|| AppError::not_found("capture not found"))? // Converts to AppError

// HTTP response
AppError::into_response() // Converts to (StatusCode::NOT_FOUND, ApiResponse::error {...})
```

## Cross-Cutting Concerns

**Logging:**
- Framework: `tracing` + `tracing-subscriber`
- Pattern: Route handlers log request start; services log major decisions; storage logs transaction boundaries
- Configuration: `RUST_LOG` environment variable (e.g., `RUST_LOG=veld=debug`)

**Validation:**
- Input validation happens in route handlers (required fields, format, business rules)
- Domain invariants enforced in types (private fields, constructors with checks in vel-core)

**Authentication:**
- Headers: `x-vel-operator-token`, `x-vel-worker-token`
- Implementation: Middleware in `app.rs` checks env vars `VEL_OPERATOR_API_TOKEN`, `VEL_WORKER_API_TOKEN`
- Routes marked with exposure class (LocalPublic, OperatorAuthenticated, WorkerAuthenticated)
- Mode: Strict auth via `VEL_STRICT_HTTP_AUTH` env var

**Authorization:**
- Token-based; no fine-grained RBAC yet
- Operator token: Full API access
- Worker token: Background job execution only

**Transactions:**
- Storage layer uses sqlx transactions for multi-table consistency
- Example: `insert_capture` transaction ensures capture row + processing job row both succeed or both fail

---

*Architecture analysis: 2026-03-17*
