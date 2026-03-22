# Technology Stack

**Analysis Date:** 2026-03-22

## Languages

**Primary:**
- Rust 2021 edition with toolchain `1.86.0` - Core runtime, storage, CLI, eval harness, protocol, config, and SDK crates in `Cargo.toml`, `crates/veld/`, `crates/vel-cli/`, `crates/vel-storage/`, `crates/vel-core/`, `crates/vel-config/`, `crates/vel-llm/`, `crates/vel-protocol/`, `crates/vel-sim/`, and `crates/veld-evals/`
- TypeScript 5.9 - Web operator UI in `clients/web/package.json` and `clients/web/src/`

**Secondary:**
- JavaScript/ESM on Node 20 - Web build/config scripts in `clients/web/vite.config.ts`, `clients/web/postcss.config.js`, `clients/web/tailwind.config.js`, `scripts/*.mjs`, and root `package.json`
- Bash - Dev, container, Apple, and LLM launcher scripts in `scripts/`
- Swift - Apple client surfaces and simulator/device workflows under `clients/apple/` and `Makefile`
- SQL - SQLite migration set in `migrations/`

## Runtime

**Environment:**
- Rust `1.86.0` with `clippy` and `rustfmt` from `rust-toolchain.toml`
- Node.js 20.x in CI (`.github/workflows/ci.yml`) and via Nix shell tooling in `shell.nix`
- Browser runtime for the Vite-built React app in `clients/web/`
- Optional local container runtime via Docker or Podman from `docker-compose.yml` and `Makefile`

**Package Manager:**
- Cargo - Rust workspace manager for the root `Cargo.toml`
- npm - Frontend package manager for `clients/web/package.json` and top-level pass-through scripts in `package.json`
- Lockfiles: `Cargo.lock`, `package-lock.json`, and `clients/web/package-lock.json` present

## Frameworks

**Core:**
- Axum `0.7` - HTTP runtime and routing in `crates/veld/Cargo.toml`
- Tokio `1.44` - Async runtime across backend crates from workspace dependencies in `Cargo.toml`
- SQLx `0.8` with SQLite - Persistence and migrations in `crates/vel-storage/Cargo.toml` and `migrations/`
- React `19.2.4` - Web UI in `clients/web/package.json`

**Testing:**
- Rust built-in test harness - Workspace crate tests run through `cargo test --workspace --all-features` from `Makefile`
- Vitest `2.1.8` with Testing Library - Web tests in `clients/web/package.json` and `clients/web/vitest.config.ts`
- GitHub Actions CI - Full repo verification in `.github/workflows/ci.yml`

**Build/Dev:**
- Vite `8.0.0` - Web dev server and production bundling in `clients/web/package.json` and `clients/web/vite.config.ts`
- TypeScript compiler `5.9.3` - Frontend type-check/build step in `clients/web/package.json`
- ESLint `9.39.4` - Web linting in `clients/web/eslint.config.js`
- Tailwind CSS `4.2.1` plus PostCSS - Web styling pipeline in `clients/web/package.json`, `clients/web/tailwind.config.js`, and `clients/web/postcss.config.js`
- Nix shell - Reproducible local toolchain in `shell.nix`

## Key Dependencies

**Critical:**
- `axum` `0.7` - Backend API surface for `veld` in `crates/veld/Cargo.toml`
- `sqlx` `0.8` - SQLite access and migration support in `crates/vel-storage/Cargo.toml`
- `reqwest` `0.12` - External HTTP access for LLM and credential-backed integrations from `Cargo.toml`, `crates/veld/src/services/integrations_google.rs`, and `crates/veld/src/services/integrations_todoist.rs`
- `vel-llm` local crate - Provider-agnostic LLM abstraction in `crates/vel-llm/README.md` and `crates/veld/src/llm.rs`
- `react` / `react-dom` `19.2.4` - Operator UI foundation in `clients/web/package.json`

**Infrastructure:**
- `tower-http` `0.6` - Trace and CORS middleware in root `Cargo.toml`
- `clap` `4.5` - CLI command surface in `crates/vel-cli/Cargo.toml` and `crates/veld-evals/Cargo.toml`
- `serde`, `serde_json`, `serde_yaml`, `toml` - Typed config and transport serialization across the workspace
- `uuid`, `time`, `chrono`, `chrono-tz`, `iana-time-zone` - Stable IDs and time handling in backend crates
- `lucide-react` `0.577.0` and `react-icons` `5.6.0` - Web icon system in `clients/web/package.json`

## Configuration

**Environment:**
- Runtime config is owned by `crates/vel-config/src/lib.rs` and the checked-in assets under `config/`
- Main daemon config lives in `vel.toml` with checked-in examples/templates documented by `config/README.md` and exemplified in `config/examples/app-config.example.toml`
- Environment overrides are read in `crates/vel-config/src/lib.rs`; high-value keys include `VEL_BIND_ADDR`, `VEL_BASE_URL`, `VEL_DB_PATH`, `VEL_ARTIFACT_ROOT`, `VEL_LLM_MODEL`, `VEL_LLM_FAST_MODEL`, `VEL_CALENDAR_ICS_URL`, `VEL_CALENDAR_ICS_PATH`, `VEL_TODOIST_SNAPSHOT_PATH`, `VEL_ACTIVITY_SNAPSHOT_PATH`, `VEL_HEALTH_SNAPSHOT_PATH`, `VEL_GIT_SNAPSHOT_PATH`, `VEL_MESSAGING_SNAPSHOT_PATH`, `VEL_REMINDERS_SNAPSHOT_PATH`, `VEL_NOTES_PATH`, `VEL_TRANSCRIPT_SNAPSHOT_PATH`, `VEL_OPERATOR_API_TOKEN`, and `VEL_WORKER_API_TOKEN`
- `.env.example` is present at repo root; treat it as a template only

**Build:**
- Rust workspace and dependency graph: `Cargo.toml`
- Toolchain pin: `rust-toolchain.toml`
- Nix shell definition: `shell.nix`
- Frontend build config: `clients/web/vite.config.ts`, `clients/web/tsconfig.json`, `clients/web/tsconfig.app.json`, `clients/web/tsconfig.node.json`
- Styling/lint/test config: `clients/web/tailwind.config.js`, `clients/web/postcss.config.js`, `clients/web/eslint.config.js`, `clients/web/vitest.config.ts`
- Dev/verification orchestration: `Makefile`
- Container packaging: `Dockerfile`, `docker-compose.yml`

## Platform Requirements

**Development:**
- Nix-friendly macOS/Linux environment is the documented default in `README.md` and `shell.nix`
- Rust, Cargo, Node, npm, OpenSSL headers, and pkg-config are required by `shell.nix`
- Optional extras:
  - `llama-server` on PATH for local LLM-backed chat, with model weights under `configs/models/weights/`
  - Docker or Podman for containerized runtime via `docker-compose.yml` and `Makefile`
  - Swift/Xcode tooling for Apple clients and simulator/device flows under `clients/apple/`

**Production:**
- Primary shipped runtime is the local authority daemon `veld` plus local SQLite/artifact storage from `README.md`, `Dockerfile`, and `docker-compose.yml`
- Container target is a Debian Bookworm image exposing port `4130` in `Dockerfile`
- Persistent data expectations are local filesystem paths under `var/` in dev or `/data` inside the container, with SQLite at `var/data/vel.sqlite` or `/data/db/vel.sqlite`

---

*Stack analysis: 2026-03-22*
