# Codebase Structure

**Analysis Date:** 2026-03-22

## Directory Layout

```text
vel/
├── .planning/                  # GSD planning state, active phases, codebase maps
├── clients/                    # User-facing shells
│   ├── apple/                  # Swift apps, VelAPI package, Apple shared modules
│   └── web/                    # React/Vite operator dashboard
├── config/                     # Canonical config templates, examples, JSON schemas, manifest
├── configs/                    # Model profile and routing TOML files
├── crates/                     # Rust workspace crates
│   ├── vel-core/               # Domain types and invariants
│   ├── vel-config/             # Config loading and contract manifest support
│   ├── vel-storage/            # SQLite facade and repositories
│   ├── vel-api-types/          # HTTP DTOs
│   ├── vel-agent-sdk/          # Shared agent SDK crate
│   ├── vel-llm/                # LLM routing abstractions
│   ├── vel-protocol/           # Rust-side protocol types
│   ├── vel-sim/                # Deterministic replay harness
│   ├── veld/                   # Daemon, routes, services, workers, tests
│   └── veld-evals/             # Eval runner
├── docs/                       # Canonical product, ticket, and architecture docs
├── migrations/                 # SQLx migrations
├── packages/                   # TypeScript/shared rendering and protocol packages
├── scripts/                    # Repo automation scripts
├── var/                        # Local runtime data and artifacts (git-ignored)
├── Cargo.toml                  # Rust workspace manifest
├── package.json                # Root JS workspace scripts
├── Makefile                    # Standard dev/build/verify entrypoints
└── README.md                   # Repo entrypoint
```

## Directory Purposes

**`crates/`**
- Purpose: Hold the production Rust workspace.
- Contains: one crate per backend concern, each with its own `Cargo.toml`.
- Key files: `crates/veld/src/main.rs`, `crates/vel-core/src/lib.rs`, `crates/vel-storage/src/db.rs`, `crates/vel-cli/src/main.rs`.
- Subdirectories: feature crates stay flat under `crates/`; only `crates/veld/` carries deep runtime subtrees such as `routes/`, `services/`, `middleware/`, `adapters/`, and `tests/`.

**`crates/veld/`**
- Purpose: Main authority runtime.
- Contains: startup wiring, Axum app, background workers, service layer, adapters, middleware, and integration tests.
- Key files: `crates/veld/src/app.rs`, `crates/veld/src/state.rs`, `crates/veld/src/worker.rs`, `crates/veld/tests/runtime_loops.rs`.
- Subdirectories: `src/routes/` for transport handlers, `src/services/` for application logic, `src/adapters/` for integration adapters, `tests/` for focused runtime integration tests.

**`crates/vel-storage/`**
- Purpose: Persistence layer.
- Contains: `Storage` facade, repository modules, SQL mapping helpers, shared DB record structs.
- Key files: `crates/vel-storage/src/lib.rs`, `crates/vel-storage/src/db.rs`, `crates/vel-storage/src/repositories/projects_repo.rs`.
- Subdirectories: `src/repositories/` is the place to extend persistence by entity instead of growing `db.rs`.

**`crates/vel-core/`**
- Purpose: Domain vocabulary and typed records shared across the backend.
- Contains: feature-specific domain modules rather than transport or SQL code.
- Key files: `crates/vel-core/src/context.rs`, `crates/vel-core/src/run.rs`, `crates/vel-core/src/project.rs`, `crates/vel-core/src/operator_queue.rs`.
- Subdirectories: none; modules are file-per-domain under `src/`.

**`clients/web/`**
- Purpose: Web operator shell.
- Contains: Vite app, tests, seed script, and source split into API/data/core/shell/views.
- Key files: `clients/web/src/App.tsx`, `clients/web/src/README.md`, `clients/web/vite.config.ts`, `clients/web/package.json`.
- Subdirectories: `src/core/` for reusable UI blocks, `src/shell/` for application chrome, `src/views/` for product surfaces, `src/data/` and `src/api/` for client-side data access.

**`clients/apple/`**
- Purpose: Apple platform shells and shared Swift packages.
- Contains: app targets, `VelAPI`, `VelAppleModules`, docs, and Xcode project files.
- Key files: `clients/apple/README.md`, `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`, `clients/apple/Packages/VelAppleModules/Sources/VelApplication/Services.swift`.
- Subdirectories: `Apps/` for app targets, `VelAPI/` for the HTTP client package, `Packages/VelAppleModules/` for shared boundary modules, `Docs/` for Apple-specific architecture notes.

**`packages/`**
- Purpose: TypeScript packages for shared protocol/rendering experiments outside the main web app.
- Contains: `vel-protocol`, `vel-render-web`, `vel-render-watch`, `vel-affect-core`, `vel-visual-morphology`.
- Key files: `packages/vel-protocol/README.md`, `packages/vel-render-web/README.md`.
- Subdirectories: each package has its own `src/`.

**`config/`**
- Purpose: Canonical checked-in config artifacts and schemas.
- Contains: `examples/`, `templates/`, `schemas/`, and `contracts-manifest.json`.
- Key files: `config/README.md`, `config/schemas/app-config.schema.json`, `config/templates/vel.toml.template`.
- Subdirectories: keep examples, templates, and schemas separate; do not bury live contracts under feature directories.

**`docs/`**
- Purpose: Durable written authority.
- Contains: roadmap docs, tickets, product contracts, architecture references, and user docs.
- Key files: `docs/MASTER_PLAN.md`, `docs/tickets/README.md`, `docs/product/`, `docs/cognitive-agent-architecture/`.
- Subdirectories: `docs/tickets/phase-*` for implementation authority, `docs/user/` for shipped user guidance, `docs/templates/` for agent process guidance.

**`.planning/`**
- Purpose: Current planning state for the GSD workflow.
- Contains: `STATE.md`, active phase folders, milestones, todos, and this codebase map.
- Key files: `.planning/STATE.md`, `.planning/codebase/ARCHITECTURE.md`, `.planning/phases/54-final-ui-cleanup-and-polish-pass/`.
- Subdirectories: `codebase/`, `phases/`, `milestones/`, `todos/`, `research/`.

## Key File Locations

**Entry Points:**
- `crates/veld/src/main.rs`: daemon startup and runtime wiring.
- `crates/vel-cli/src/main.rs`: CLI entrypoint.
- `clients/web/src/App.tsx`: web UI root.
- `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift`: Apple transport entrypoint used by Apple shells.
- `crates/veld-evals/src/main.rs`: eval CLI entrypoint.

**Configuration:**
- `Cargo.toml`: Rust workspace membership.
- `clients/web/package.json`: web scripts and frontend dependencies.
- `config/README.md`: config asset map and ownership rules.
- `config/schemas/`: machine-readable schemas for config and durable contracts.
- `configs/models/`: model profiles and routing TOML files.

**Core Logic:**
- `crates/veld/src/services/`: application services by feature.
- `crates/veld/src/routes/`: HTTP boundary modules by feature.
- `crates/vel-storage/src/repositories/`: persistence modules by entity.
- `crates/vel-core/src/`: domain modules.
- `clients/web/src/views/`: feature-specific UI surfaces.

**Testing:**
- `crates/veld/tests/`: backend integration and flow tests.
- `clients/web/src/test/`: shared web test setup.
- `clients/web/src/types.test.ts`: frontend type/decoder tests.
- `clients/apple/VelAPI/Tests/VelAPITests/`: Swift package tests.

**Documentation:**
- `README.md`: repo entrypoint.
- `docs/MASTER_PLAN.md`: implementation truth.
- `clients/web/src/README.md`: web UI layout rules.
- `clients/apple/README.md`: Apple surface boundaries and setup.
- `config/README.md`: config and schema authority.

## Naming Conventions

**Files:**
- Rust modules use `snake_case.rs`: `crates/veld/src/services/daily_loop.rs`.
- React components use `PascalCase.tsx` inside feature folders: `clients/web/src/views/now/NowView.tsx`, `clients/web/src/shell/Navbar/Navbar.tsx`.
- Directory exports use `index.ts` in the web app: documented in `clients/web/src/README.md`.
- Test files are descriptive and feature-specific: `crates/veld/tests/planning_profile_api.rs`, `clients/web/src/types.test.ts`.

**Directories:**
- Rust crate and repo directories use kebab-case: `crates/vel-storage`, `packages/vel-render-watch`.
- Web UI buckets are semantic, not technical layers: `core/`, `shell/`, `views/`.
- `views/` subdirectories are singular product surfaces: `clients/web/src/views/now/`, `clients/web/src/views/threads/`.

**Special Patterns:**
- Add new backend route modules under `crates/veld/src/routes/` with a matching service module under `crates/veld/src/services/`.
- Extend persistence by adding a repository file under `crates/vel-storage/src/repositories/` instead of expanding unrelated logic in `crates/vel-storage/src/db.rs`.
- Keep transport DTO additions in `crates/vel-api-types/src/lib.rs` and update client boundaries in the same slice.

## Where to Add New Code

**New Backend Feature:**
- Primary code: `crates/veld/src/services/` for orchestration and `crates/veld/src/routes/` for HTTP exposure.
- Domain types: `crates/vel-core/src/` when the feature adds shared vocabulary or durable records.
- Persistence: `crates/vel-storage/src/repositories/` plus migrations in `migrations/`.
- Tests: `crates/veld/tests/` for integration behavior; unit tests can live beside focused modules when appropriate.

**New Web Surface Or Component:**
- Feature screen: `clients/web/src/views/<surface>/`.
- Reusable UI building block: `clients/web/src/core/<ComponentName>/`.
- App chrome/navigation: `clients/web/src/shell/`.
- Client data helpers: `clients/web/src/data/` or `clients/web/src/api/`.
- Tests: colocate light tests under `clients/web/src/` or use shared setup from `clients/web/src/test/`.

**New Apple Capability:**
- HTTP/API contract call: `clients/apple/VelAPI/Sources/VelAPI/`.
- Shared Apple boundary logic: `clients/apple/Packages/VelAppleModules/Sources/VelApplication/` or the nearest Apple module seam.
- App-specific UI: `clients/apple/Apps/<Target>/`.

**New Config Or Contract Surface:**
- Live/example/template/schema assets: `config/examples/`, `config/templates/`, `config/schemas/`.
- Loader/typed owner: `crates/vel-config/src/` or the owning backend service, depending on the contract.
- Governing docs: nearest authority under `docs/cognitive-agent-architecture/` or `docs/product/`.

**Utilities:**
- Backend shared helper: keep it inside the owning crate near the feature, not in a generic top-level utils crate.
- Web shared helper: `clients/web/src/data/`, `clients/web/src/hooks/`, or `clients/web/src/core/` depending on whether it is state, behavior, or presentation.

## Special Directories

**`migrations/`**
- Purpose: SQLx migrations applied by `Storage::migrate()` in `crates/veld/src/main.rs`.
- Source: authored in-repo.
- Committed: Yes.

**`var/`**
- Purpose: local runtime state such as SQLite DB, artifacts, integrations, and logs.
- Source: generated by local runs.
- Committed: No.

**`clients/web/dist/`**
- Purpose: built web assets.
- Source: generated by Vite build.
- Committed: No.

**`clients/web/node_modules/` and `target/`**
- Purpose: dependency/build output.
- Source: package manager and Cargo.
- Committed: No.

**`.planning/codebase/`**
- Purpose: generated codebase reference docs consumed by later planning and execution commands.
- Source: maintained by mapping agents.
- Committed: Yes.

---

*Structure analysis: 2026-03-22*
*Update when directory structure changes*
