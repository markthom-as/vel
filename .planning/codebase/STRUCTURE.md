# Codebase Structure

**Analysis Date:** 2026-03-17

## Directory Layout

```
vel/
├── crates/                      # Rust workspace with 7 crates
│   ├── vel-core/               # Domain types, invariants (zero dependencies)
│   ├── vel-config/             # Configuration loading (TOML/YAML)
│   ├── vel-storage/            # SQLite repositories and storage facade
│   ├── vel-api-types/          # HTTP request/response DTOs
│   ├── vel-llm/                # Provider-agnostic LLM abstraction
│   ├── veld/                   # Main daemon (HTTP + WebSocket + worker)
│   └── vel-cli/                # CLI binary
├── clients/
│   ├── web/                    # React + TypeScript dashboard (Vite)
│   └── apple/                  # iOS/watchOS/macOS Swift clients
├── migrations/                 # Numbered SQLx SQL migrations
├── config/
│   ├── examples/               # Example config files
│   ├── schemas/                # JSON Schema definitions
│   ├── templates/              # Config templates
│   └── README.md               # Config documentation
├── configs/
│   └── models/                 # ML model definitions and weights
├── docs/                       # Project documentation
│   ├── MASTER_PLAN.md          # Canonical authority on architecture & phases
│   ├── tickets/                # Phase implementation tickets
│   └── cognitive-agent-architecture/  # Conceptual architecture
├── var/                        # Local development data (Git-ignored)
│   ├── data/                   # SQLite database (vel.sqlite)
│   ├── artifacts/              # Stored files, media
│   └── logs/                   # Application logs
├── Cargo.toml                  # Rust workspace definition
├── Makefile                    # Development commands
├── vel.toml                    # Local daemon configuration
├── CLAUDE.md                   # Claude-specific guidelines
└── README.md                   # Project overview
```

## Directory Purposes

**crates/vel-core/src/:**
- Purpose: Domain types and semantics independent of HTTP or persistence
- Contains: Core types with invariants (no external dependencies except serde, time, uuid)
- Key files:
  - `types.rs` — Core type definitions
  - `message.rs` — Message and conversation types
  - `commitment.rs` — Commitment and obligation types
  - `risk.rs` — Risk and uncertainty assessment types
  - `run.rs` — Execution run and job types
  - `command/` — Command planning and resolution types
  - `context.rs` — Context and orientation types
  - `loops.rs` — Feedback loop and process types
  - `integration.rs` — Integration configuration types
  - `provenance.rs` — Data source tracking
  - `uncertainty.rs` — Uncertainty and confidence types
  - `intervention.rs` — System intervention types
  - `vocabulary.rs` — Semantic vocabulary and ontology

**crates/vel-config/src/:**
- Purpose: Configuration loading and schema
- Contains: AppConfig struct parsing from TOML, model routing, contracts manifest
- Key files:
  - `models.rs` — Configuration schema
  - `contracts_manifest.rs` — External contracts/webhooks
  - `lib.rs` — Main config loading entry point

**crates/vel-storage/src/:**
- Purpose: SQLite data access via sqlx with per-entity repositories
- Contains: Storage facade, db types, entity repositories
- Key structure:
  - `lib.rs` — Public `Storage` struct (main facade)
  - `db.rs` — StorageError, schema types (CaptureInsert, etc.), database initialization
  - `infra.rs` — Infrastructure setup, connection pooling
  - `mapping.rs` — Type conversions (domain ↔ storage)
  - `repositories/` — One file per entity type:
    - `captures_repo.rs` — Capture CRUD and search
    - `chat_repo.rs` — Chat messages and threads
    - `commitments_repo.rs` — Commitment CRUD
    - `threads_repo.rs` — Conversation threads
    - `runs_repo.rs` — Execution runs
    - `signals_repo.rs` — Signal events
    - `suggestions_repo.rs` — Suggestions and feedback
    - `nudges_repo.rs` — Nudge events
    - `uncertainty_records_repo.rs` — Uncertainty tracking
    - `commitment_risk_repo.rs` — Risk assessments
    - `integration_connections_repo.rs` — External service tokens
    - `processing_jobs_repo.rs` — Async job queue
    - `context_timeline_repo.rs` — Context computation history
    - `current_context_repo.rs` — Latest context snapshot
    - `work_assignments_repo.rs` — Worker task assignments
    - `cluster_workers_repo.rs` — Cluster node heartbeats
    - (20+ total repository files)

**crates/vel-api-types/src/:**
- Purpose: HTTP request/response contract types
- Contains: Request/response structs, ApiResponse wrapper, decoders
- Pattern: Serde-derived, mapped from vel-core types in route handlers
- Never imported by storage layer

**crates/vel-llm/src/:**
- Purpose: Provider-agnostic LLM abstraction
- Contains: Router (dispatch), Provider trait, registry
- Key files:
  - `lib.rs` — Main exports
  - `provider.rs` — Provider trait and implementations
  - `registry.rs` — Available models registry
  - `types.rs` — Request/response types for inference

**crates/veld/src/:**
- Purpose: HTTP daemon, background worker, business logic
- Key subdirectories:
  - **routes/** — HTTP handlers (thin layer)
    - `health.rs` — Health check endpoint
    - `captures.rs` — Capture CRUD endpoints
    - `chat.rs` — Chat/conversation/message endpoints
    - `commitments.rs` — Commitment endpoints
    - `context.rs`, `now.rs` — Context generation endpoints
    - `journal.rs` — Journal entry endpoints (mood, pain)
    - `threads.rs` — Thread endpoints
    - `suggestions.rs` — Suggestion endpoints
    - `signals.rs` — Signal endpoints
    - `risk.rs` — Risk assessment endpoints
    - `runs.rs` — Execution run endpoints
    - `evaluate.rs`, `explain.rs` — Analysis endpoints
    - `synthesis.rs` — Synthesis endpoints
    - `command_lang.rs` — Command planning/execution
    - `uncertainty.rs` — Uncertainty endpoints
    - `nudges.rs` — Nudge endpoints
    - `loops.rs` — Feedback loop endpoints
    - `integrations.rs` — Integration management endpoints
    - `cluster.rs` — Cluster/worker endpoints
    - `ws.rs` — WebSocket endpoint
    - `search.rs` — Full-text search endpoint
    - `doctor.rs` — System diagnostics endpoint
    - `sync.rs` — Client sync endpoints
    - `artifacts.rs` — Artifact storage endpoints
    - `response.rs` — Response formatting utilities
    - `components.rs` — Component endpoints
    - `mod.rs` — Route registration
  - **services/** — Application logic
    - `chat/` subdirectory:
      - `assistant.rs` — LLM assistant interaction
      - `conversations.rs` — Conversation management
      - `messages.rs` — Message handling
      - `threads.rs` — Thread operations (public in mod.rs)
      - `events.rs` — Chat event handling
      - `interventions.rs` — System interventions
      - `reads.rs` — Message read tracking
      - `provenance.rs` — Message source tracking
      - `mapping.rs` — Type conversions
      - `settings.rs` — Chat settings
      - `mod.rs` — Chat service exports
    - `context_generation.rs` — Generate context for now/today
    - `context_runs.rs` — Manage context execution runs
    - `evaluate.rs` — Evaluation/reasoning service
    - `inference.rs` — LLM inference wrapper
    - `suggestions.rs` — Suggestion engine
    - `risk.rs` — Risk assessment service
    - `uncertainty.rs` — Uncertainty computation
    - `synthesis.rs` — Synthesis engine
    - `nudge_engine.rs` — Nudge generation
    - `explain.rs` — Explanation service
    - `command_lang.rs` — Command parsing and execution
    - `components.rs` — Component management
    - `integrations.rs` — Integration orchestration
    - `integration_runtime.rs` — Integration executor
    - `integrations_google.rs`, `integrations_todoist.rs`, `integrations_host.rs` — Provider-specific
    - `journal.rs` — Journal entry handling
    - `now.rs` — Now/present moment service
    - `timezone.rs` — Timezone utilities
    - `doctor.rs` — System diagnostics
    - `adaptive_policies.rs` — Policy engine
    - `client_sync.rs` — Client synchronization
    - `operator_settings.rs` — Operator configuration
    - `mod.rs` — Service layer exports (with comment: "Read vs evaluate boundary")
  - **adapters/** — Integration drivers
    - `calendar.rs` — Calendar integration
    - `todoist.rs` — Todoist integration
    - `git.rs` — Git repository integration
    - `health.rs` — Health data integration
    - `messaging.rs` — Chat/messaging integration
    - `notes.rs` — Notes integration
    - `reminders.rs` — Reminders integration
    - `transcripts.rs` — Transcript integration
    - `activity.rs` — Activity tracking integration
    - `mod.rs` — Adapter exports
  - `main.rs` — Daemon entry point
  - `app.rs` — Axum router construction, auth middleware
  - `state.rs` — AppState definition, worker runtime tracking
  - `errors.rs` — AppError type, HTTP status mapping
  - `broadcast.rs` — WebSocket envelope types
  - `worker.rs` — Background job executor
  - `policy_config.rs` — Policy loading
  - `llm.rs` — LLM initialization
  - `lib.rs` — Library exports
- `tests/` — Integration tests

**crates/vel-cli/src/:**
- Purpose: Command-line operator interface
- Contains: Clap command definitions, HTTP client
- Key files:
  - `main.rs` — CLI entry point
  - `client.rs` — HTTP client to daemon
  - `commands/` — Subcommand implementations
  - `command_lang/` — Command language parsing

**clients/web/src/:**
- Purpose: React dashboard UI
- Key subdirectories:
  - **api/** — HTTP and WebSocket client
    - `client.ts` — Fetch wrapper (apiGet, apiPost, apiPatch, etc.)
  - **data/** — State management and queries
    - `query.ts` — Cache layer with `useQuery` hook (query key + fetcher)
    - `chat.ts` — Chat-related queries
    - `context.ts` — Context queries
    - `operator.ts` — Operator settings queries
    - `resources.ts` — Resource loading utilities
    - `chat-state.ts` — Conversation state
    - `ws-sync.ts` — WebSocket sync listener and query invalidation
  - **components/** — React components
    - `AppShell.tsx` — Main layout (sidebar + main + context panel)
    - `Sidebar.tsx` — Navigation and conversation list
    - `MainPanel.tsx` — Content area dispatcher
    - `ThreadView.tsx` — Conversation display
    - `MessageComposer.tsx` — Input/message sending
    - `ContextPanel.tsx` — Right panel (signals, suggestions, risk)
    - `NowView.tsx` — Now/present moment view
    - `InboxView.tsx` — Inbox/pending items
    - `SuggestionsView.tsx` — Suggestions list
    - `StatsView.tsx` — Statistics dashboard
    - `SettingsPage.tsx` — Settings/integrations
    - `ProvenanceDrawer.tsx` — Data source details
    - `MarkdownMessage.tsx` — Message rendering
    - `MessageRenderer.tsx` — Message component
    - `SurfaceState.tsx` — Surface state debugging
    - `cards/` — Reusable card components
      - `SummaryCard.tsx` — Summary display
      - `SuggestionCard.tsx` — Suggestion item
      - `RiskCard.tsx` — Risk display
      - `ReminderCard.tsx` — Reminder display
      - `CardLayout.tsx` — Card wrapper
  - **hooks/** — React hooks (custom)
  - **realtime/** — WebSocket management
  - **test/** — Test utilities
  - `App.tsx` — App entry point
  - `main.tsx` — React DOM render
  - `types.ts` — Comprehensive TypeScript types (API responses, domain types)

**clients/web/public/:**
- Static assets (favicon, etc.)

**migrations/:**
- SQL migration files, numbered 0001–0030+ for SQLx
- Examples:
  - `0001_bootstrap.sql` — Initial schema (tables, indexes)
  - `0004_runs_and_events.sql` — Run and event tables
  - `0008_commitments.sql` — Commitment tables
  - `0023_chat.sql` — Chat/message/thread schema
  - `0030_integration_foundation.sql` — Integration tokens table

**config/:**
- `examples/` — Example configurations
- `schemas/` — JSON Schema definitions for validation
- `templates/` — Configuration templates
- `README.md` — Configuration documentation

**docs/:**
- `MASTER_PLAN.md` — Canonical authority
- `tickets/phase-1/`, `phase-2/`, etc. — Phase implementation tickets
- `cognitive-agent-architecture/` — Conceptual architecture
- `user/` — User-facing documentation

**var/ (Git-ignored):**
- `data/vel.sqlite` — SQLite database
- `artifacts/` — Stored media files
- `logs/` — Application logs

## Key File Locations

**Entry Points:**
- `crates/veld/src/main.rs` — HTTP daemon startup
- `crates/vel-cli/src/main.rs` — CLI startup
- `clients/web/src/App.tsx` — Web dashboard startup
- `clients/web/src/main.tsx` — React DOM render

**Configuration:**
- `Cargo.toml` — Workspace definition
- `vel.toml` — Daemon configuration (bind address, database path, log level)
- `clients/web/vite.config.ts` — Vite configuration
- `crates/vel-config/src/models.rs` — Configuration schema

**Core Logic:**
- `crates/vel-core/src/types.rs` — Core domain types
- `crates/veld/src/services/mod.rs` — Service layer comment on read vs. evaluate boundary
- `crates/veld/src/app.rs` — Router construction with exposure gates

**Storage:**
- `crates/vel-storage/src/lib.rs` — Storage facade (public API)
- `crates/vel-storage/src/db.rs` — StorageError and schema types
- `migrations/` — SQLx migrations

**Testing:**
- `clients/web/src/data/query.test.tsx` — Query cache tests
- `clients/web/src/api/client.test.ts` — API client tests
- `crates/veld/tests/` — Integration tests
- Test files co-located with source: `*.test.ts`, `*.test.tsx`

## Naming Conventions

**Files:**
- Rust crate source: `snake_case.rs` (e.g., `captures_repo.rs`, `message.rs`)
- React components: `PascalCase.tsx` (e.g., `AppShell.tsx`, `ContextPanel.tsx`)
- TypeScript utilities: `camelCase.ts` (e.g., `client.ts`, `resources.ts`)
- Test files: `{name}.test.{ts|tsx}` (co-located with source)
- SQL migrations: `NNNN_snake_case_description.sql` (e.g., `0001_bootstrap.sql`)

**Directories:**
- Rust: `snake_case` (e.g., `vel-core`, `vel-storage`, `command_lang`)
- TypeScript/Web: `camelCase` (e.g., `src/api`, `src/components`, `src/data`)
- Config: `lowercase` (e.g., `config/`, `configs/`, `migrations/`)

**Types & Functions:**
- Rust types: `PascalCase` (e.g., `AppState`, `StorageError`, `CaptureInsert`)
- Rust functions: `snake_case` (e.g., `list_captures`, `insert_capture`)
- TypeScript types: `PascalCase` (e.g., `ApiResponse<T>`, `ConversationData`)
- TypeScript functions: `camelCase` (e.g., `apiGet`, `createWsUrl`)

**Entity Types:**
- Domain types in `vel-core`: no suffix (e.g., `Capture`, `Commitment`, `Message`)
- Storage insert types: `{Entity}Insert` (e.g., `CaptureInsert`, `CommitmentInsert`)
- API request types: `{Entity}CreateRequest` or `{Entity}UpdateRequest` (e.g., `CaptureCreateRequest`)
- API response types: `{Entity}CreateResponse` or `{Entity}` direct (e.g., `CaptureCreateResponse`)

## Where to Add New Code

**New Feature (e.g., Habit Tracking):**
1. **Domain types:** Add to `crates/vel-core/src/habit.rs` (define Habit, HabitLog, etc.)
2. **Storage:** Create `crates/vel-storage/src/repositories/habits_repo.rs` with CRUD functions
3. **Migration:** Add `migrations/NNNN_habits.sql` with schema
4. **Services:** Create `crates/veld/src/services/habits.rs` with business logic (list, create, evaluate)
5. **Routes:** Create `crates/veld/src/routes/habits.rs` with HTTP handlers
6. **API types:** Add request/response structs to `crates/vel-api-types/src/lib.rs`
7. **Register routes:** Add routes to `crates/veld/src/app.rs` router
8. **Web:** Create `clients/web/src/components/HabitsView.tsx` component, add queries to `clients/web/src/data/resources.ts`
9. **Tests:** Add `crates/veld/tests/habit_*.rs` integration tests

**New Service (internal logic):**
- File: `crates/veld/src/services/{service_name}.rs`
- Pattern: Receive storage, config, state; return domain types or AppError
- Tests: Co-located with service or in `crates/veld/tests/`

**New Adapter (external integration):**
- File: `crates/veld/src/adapters/{provider_name}.rs`
- Pattern: Async functions that fetch data from external service, map to domain types
- Storage: Credentials stored in `integration_connections` table
- Service orchestration: Called from `services/integration_runtime.rs`

**New Route/Endpoint:**
- File: `crates/veld/src/routes/{domain}.rs` (or add to existing file)
- Pattern: Thin handler that validates → calls service → maps to DTO → handles errors
- Register: Add to router in `crates/veld/src/app.rs`
- Auth: Decide exposure class (LocalPublic, OperatorAuthenticated, WorkerAuthenticated)

**New React Component:**
- File: `clients/web/src/components/{ComponentName}.tsx`
- Pattern: Functional component, use `useQuery` for data, handle loading/error states
- Tests: `clients/web/src/components/{ComponentName}.test.tsx`
- Styling: Tailwind CSS classes

**New Query/State:**
- File: `clients/web/src/data/{domain}.ts` (or add to existing file)
- Pattern: Export `use{Resource}Query()` hook using `useQuery` cache
- Example: `clients/web/src/data/chat.ts` exports `useChatQuery()`

**Utilities:**
- Shared helpers: `clients/web/src/` (no subdirectory, or create `utils/` folder)
- Rust helpers: Add to relevant crate in `src/` directory

## Special Directories

**migrations/:**
- Purpose: SQLx compile-time verified SQL migrations
- Generated: No (manually authored)
- Committed: Yes
- Run command: Automatic on daemon startup via `storage.migrate().await`

**var/ (Git-ignored, development only):**
- Purpose: Local database, artifacts, logs
- Generated: Yes (at first daemon startup)
- Committed: No

**config/:**
- Purpose: Schema definitions, example configs, templates
- Generated: No
- Committed: Yes

**docs/:**
- Purpose: Project documentation, tickets, architecture specs
- Generated: No (manually authored)
- Committed: Yes

**.planning/codebase/ (this directory):**
- Purpose: Claude Code analysis documents for reference
- Generated: Yes (by gsd:map-codebase)
- Committed: No (or kept separate from tracked codebase)

---

*Structure analysis: 2026-03-17*
