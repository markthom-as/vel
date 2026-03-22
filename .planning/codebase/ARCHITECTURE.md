# Architecture

**Analysis Date:** 2026-03-22

## Pattern Overview

**Overall:** Local-first authority runtime with layered Rust backend (`routes -> services -> storage -> SQLite/artifacts`), plus thin client shells in web, Apple, and CLI.

**Key Characteristics:**
- `crates/veld` is the operational center: it starts the daemon, owns HTTP/WebSocket surfaces, launches background workers, and wires shared state in `crates/veld/src/main.rs` and `crates/veld/src/app.rs`.
- Domain semantics stay in `crates/vel-core/src/`; transport DTOs stay in `crates/vel-api-types/src/`; persistence stays in `crates/vel-storage/src/`. This matches the repo rules in `docs/MASTER_PLAN.md` and `AGENTS.md`.
- Client surfaces remain backend-first. `clients/web/src/App.tsx`, `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`, and `crates/vel-cli/src/main.rs` all consume daemon-owned contracts instead of re-owning product policy.
- The backend is both request/response and eventful: services persist runs, artifacts, refs, signals, and broadcasts. Real-time fanout is wired through `crates/veld/src/state.rs` and `/ws` in `crates/veld/src/app.rs`.

## Layers

**Domain Core:**
- Purpose: Hold stable business vocabulary, identifiers, typed records, invariants, and shared enums.
- Location: `crates/vel-core/src/`
- Contains: modules such as `context.rs`, `daily_loop.rs`, `execution.rs`, `operator_queue.rs`, `project.rs`, `run.rs`, `semantic.rs`, `writeback.rs`.
- Depends on: general-purpose Rust libraries only.
- Used by: `crates/vel-storage`, `crates/veld`, `crates/vel-api-types`, `crates/vel-sim`, and other backend crates.

**Configuration And Contract Ownership:**
- Purpose: Load runtime config and publish checked-in contract metadata.
- Location: `crates/vel-config/src/` with checked-in assets under `config/`.
- Contains: `AppConfig` and model/contract loaders in `crates/vel-config/src/lib.rs`, `models.rs`, and `contracts_manifest.rs`.
- Depends on: checked-in files in `config/examples/`, `config/templates/`, and `config/schemas/`.
- Used by: daemon startup in `crates/veld/src/main.rs`, simulation in `crates/vel-sim/src/lib.rs`, and tooling that validates config-bearing surfaces.

**Storage Boundary:**
- Purpose: Isolate SQLite and artifact persistence behind a typed facade.
- Location: `crates/vel-storage/src/`
- Contains: `db.rs` for the `Storage` facade and shared record types, `infra.rs`/`mapping.rs` helpers, and repository modules under `crates/vel-storage/src/repositories/`.
- Depends on: `crates/vel-core/src/` types and SQLx/SQLite.
- Used by: application services in `crates/veld/src/services/`.

**Application Services:**
- Purpose: Own orchestration, policy application, reduction, writeback, integration sync, and flow assembly.
- Location: `crates/veld/src/services/`
- Contains: feature services such as `now.rs`, `daily_loop.rs`, `projects.rs`, `planning_profile.rs`, `integrations.rs`, `operator_queue.rs`, plus nested feature folders like `chat/` and `inference/`.
- Depends on: `vel-core`, `vel-storage`, `vel-config`, adapters in `crates/veld/src/adapters/`, and runtime state in `crates/veld/src/state.rs`.
- Used by: route handlers in `crates/veld/src/routes/`, worker tasks in `crates/veld/src/worker.rs`, and simulation/eval crates.

**Transport Boundary:**
- Purpose: Expose typed HTTP and WebSocket surfaces without embedding business policy.
- Location: `crates/veld/src/routes/`, `crates/veld/src/middleware/`, `crates/vel-api-types/src/`
- Contains: route modules like `now.rs`, `threads.rs`, `integrations.rs`, `execution.rs`; shared response helpers in `crates/veld/src/routes/response.rs`; auth/exposure gates in `crates/veld/src/middleware/mod.rs`.
- Depends on: services and API DTOs.
- Used by: web, Apple, CLI, and internal worker clients.

**Operator Shells:**
- Purpose: Render backend state and invoke backend actions.
- Location: `clients/web/src/`, `clients/apple/`, `crates/vel-cli/src/`
- Contains: React shell/view/core split in `clients/web/src/README.md`; Apple shell and shared packages in `clients/apple/Packages/VelAppleModules` and `clients/apple/VelAPI`; CLI command parsing and API client calls in `crates/vel-cli/src/`.
- Depends on: daemon endpoints and typed transport contracts.
- Used by: human operators, not other backend layers.

**Verification And Tooling:**
- Purpose: Replay scenarios, run evals, and verify flow behavior outside production startup.
- Location: `crates/vel-sim/src/lib.rs`, `crates/veld-evals/src/`, `crates/veld/tests/`
- Contains: deterministic scenario replay, judge-backed eval execution, and focused integration tests around runtime flows.
- Depends on: the same backend crates used by production.
- Used by: `make verify`, targeted test runs, and roadmap closeout work.

## Data Flow

**Daemon Startup:**

1. `crates/veld/src/main.rs` loads `AppConfig` and policy config.
2. It connects `vel_storage::Storage`, runs migrations from `migrations/`, and emits startup events.
3. It builds `AppState` in `crates/veld/src/state.rs`, including config, storage, worker runtime, LLM router, broadcast channel, and public-linking rate limiter.
4. It starts background tasks such as `worker::run_background_workers`, LAN discovery, and integration bootstrap.
5. It binds Axum and serves the router assembled in `crates/veld/src/app.rs`.

**Operator HTTP Request:**

1. A client calls a route in `crates/veld/src/app.rs`, usually under `/v1/*` or `/api/*`.
2. `crates/veld/src/middleware/mod.rs` enforces route exposure class and optional operator or worker auth tokens.
3. A thin route handler in `crates/veld/src/routes/*.rs` extracts state/input and calls a service.
4. The service in `crates/veld/src/services/*.rs` reads/writes via `vel_storage::Storage`, may call adapters or integration helpers, and returns backend-owned output structs.
5. The route maps service output into `vel-api-types` DTOs and returns a standard `ApiResponse<T>`.

**Now Surface Assembly:**

1. `/v1/now` enters `crates/veld/src/routes/now.rs`.
2. The route calls `services::now::get_now_with_state` in `crates/veld/src/services/now.rs`.
3. The service joins current context, planning, queue, writeback, conflict, people, and freshness signals from storage and related services.
4. The route maps `NowOutput` to `NowData` in `crates/vel-api-types/src/lib.rs`.
5. Web and Apple shells render that same contract through `clients/web/src/views/now/` and `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`.

**Client Interaction Loop:**

1. Web starts from `clients/web/src/App.tsx`; Apple uses `VelClient`; CLI starts in `crates/vel-cli/src/main.rs`.
2. Each shell selects a surface and calls backend endpoints.
3. State changes remain daemon-owned; clients invalidate/refetch local view state rather than recomputing business rules.
4. For web, route selection happens in `clients/web/src/shell/MainPanel/MainPanel.tsx`; shared chrome lives in `clients/web/src/shell/`.

**State Management:**
- Durable state is local-first: SQLite data plus artifact files under `var/data/` and `var/artifacts/` in development.
- Process-local runtime state lives in `AppState`, mainly for config, broadcast, worker load, and request-time dependencies.
- The repo does not use a separate service tier between daemon and database; application services call `Storage` directly.

## Key Abstractions

**Storage Facade:**
- Purpose: Present one typed entrypoint to SQLite and repository modules.
- Examples: `crates/vel-storage/src/db.rs`, `crates/vel-storage/src/repositories/projects_repo.rs`, `crates/vel-storage/src/repositories/threads_repo.rs`
- Pattern: facade over per-entity repository modules.

**AppState:**
- Purpose: Carry long-lived runtime dependencies through routes and services.
- Examples: `crates/veld/src/state.rs`, injected into routes via Axum state in `crates/veld/src/app.rs`
- Pattern: shared runtime context object.

**Route Module:**
- Purpose: Map HTTP requests to service calls and map service outputs to transport DTOs.
- Examples: `crates/veld/src/routes/now.rs`, `crates/veld/src/routes/threads.rs`, `crates/veld/src/routes/integrations.rs`
- Pattern: thin boundary adapter.

**Service Module:**
- Purpose: Encapsulate feature logic that can be reused across routes, workers, and tests.
- Examples: `crates/veld/src/services/now.rs`, `crates/veld/src/services/daily_loop.rs`, `crates/veld/src/services/writeback.rs`
- Pattern: feature-oriented orchestration modules.

**Shell/View/Core split for web:**
- Purpose: Keep reusable UI pieces separate from product-surface composition.
- Examples: `clients/web/src/core/`, `clients/web/src/shell/`, `clients/web/src/views/`
- Pattern: shared component library plus feature views, documented in `clients/web/src/README.md`.

## Entry Points

**Daemon:**
- Location: `crates/veld/src/main.rs`
- Triggers: `cargo run -p veld`, `make dev-api`, `make dev`
- Responsibilities: load config, connect storage, migrate, initialize runtime state, spawn workers, serve HTTP/WebSocket.

**CLI:**
- Location: `crates/vel-cli/src/main.rs`
- Triggers: `vel ...` commands
- Responsibilities: parse operator commands and proxy to backend surfaces.

**Web Client:**
- Location: `clients/web/src/App.tsx`
- Triggers: Vite app bootstrap from `clients/web`
- Responsibilities: mount shell chrome, choose the active surface, and wire info panel plus composer.

**Apple Client Transport:**
- Location: `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`
- Triggers: Apple shell actions from `clients/apple/Apps/*`
- Responsibilities: call daemon endpoints and decode backend envelopes.

**Simulation And Evals:**
- Location: `crates/vel-sim/src/lib.rs`, `crates/veld-evals/src/main.rs`
- Triggers: test and eval workflows
- Responsibilities: replay fixtures and run deterministic plus judge-backed verification.

## Error Handling

**Strategy:** Let domain/storage/service errors propagate to transport boundaries, then map them into consistent API responses or CLI failures.

**Patterns:**
- Daemon startup uses `anyhow::Context` in `crates/veld/src/main.rs` to fail fast on config, DB, migration, bind, and serve errors.
- HTTP routes return `Result<..., AppError>` and central response helpers from `crates/veld/src/routes/response.rs`.
- Storage exposes a typed `StorageError` in `crates/vel-storage/src/db.rs` rather than returning raw SQLx errors directly.
- Undefined external transport is denied explicitly through `future_external_routes()` in `crates/veld/src/app.rs`.

## Cross-Cutting Concerns

**Logging:** `tracing` is initialized in `crates/veld/src/main.rs`; startup and service-adjacent warnings are logged there and across service modules.

**Validation:** Boundary validation happens at route parsing, DTO decoding, config/schema ownership in `config/`, and typed domain/storage conversions rather than ad hoc JSON blobs.

**Authentication:** HTTP exposure classes and token checks are enforced in `crates/veld/src/middleware/mod.rs`; operator and worker routes are authenticated by default when tokens or strict auth are enabled.

---

*Architecture analysis: 2026-03-22*
*Update when major patterns change*
