# Technology Stack

**Analysis Date:** 2026-03-17

## Languages

**Primary:**
- Rust 1.86.0 - Backend daemon (veld), CLI (vel-cli), and core libraries
- TypeScript ~5.9.3 - Web frontend type layer (clients/web)
- JavaScript (ES modules) - Web frontend runtime

**Secondary:**
- SQL - SQLite schema and raw SQL migrations
- YAML - Configuration (agent specs, policies, routing)
- TOML - Rust Cargo manifests, Vel daemon configuration

## Runtime

**Environment:**
- Tokio 1.44 (async runtime) - Multi-threaded async executor for veld daemon
- Node.js (via Vite) - Web development and build server
- SQLite 3 - Local file-based SQL database via sqlx 0.8

**Package Manager:**
- Cargo (Rust) - Workspace resolver="2", version 0.1.0
- npm (Node.js) - Monorepo package management
- Lockfiles: `Cargo.lock` (Rust), not present for Node (npm ci fetches from lock)

## Frameworks

**Core Backend:**
- Axum 0.7 - HTTP server framework with WebSocket support
- SQLx 0.8 - Compile-time verified SQL queries, SQLite runtime via `tokio-rustls`
- Tower 0.5, Tower-HTTP 0.6 - HTTP middleware (CORS, tracing)

**Web Frontend:**
- React 19.2.4 - Component-based UI framework
- Vite 8.0.0 - Build tool and dev server
- TypeScript with React plugins - Type safety for JSX

**Testing:**
- Vitest 2.1.8 - Frontend test runner and assertion library
- @testing-library/react 16.1.0 - Component testing utilities
- jsdom 25.0.1 - DOM simulator for tests
- Cargo test (built-in) - Backend unit and integration tests

**Build/Dev:**
- Cargo (Rust) - Compilation and test runner
- Tailwind CSS 4.2.1 - Utility-first CSS framework (clients/web)
- PostCSS 8.5.8 - CSS transformation pipeline
- ESLint 9.39.4 - Frontend linting
- TypeScript compiler (tsc) - Type checking for web frontend

## Key Dependencies

**Critical (Workspace):**
- `tokio` 1.44 - Async runtime with multi-threading, signals, process, sync
- `serde` 1.0 + `serde_json` 1.0 - Serialization/deserialization (JSON, YAML)
- `sqlx` 0.8 - SQLite database with compile-time query verification
- `uuid` 1.8 - Unique identifiers for entities
- `axum` 0.7 - Web framework with JSON and WebSocket support
- `reqwest` 0.12 - HTTP client (rustls-tls, not OpenSSL)
- `time` 0.3 - Date/time with formatting and parsing; `chrono` 0.4 for legacy support
- `chrono-tz` 0.10, `iana-time-zone` 0.1 - Timezone handling

**Infrastructure:**
- `tracing` 0.1 + `tracing-subscriber` 0.3 - Structured logging and observability
- `thiserror` 2.0 - Error type derivation
- `anyhow` 1.0 - Flexible error handling for bin/CLI crates
- `clap` 4.5 - CLI argument parsing (derive macros)
- `sha2` 0.10, `hex` 0.4 - Hashing and hex encoding
- `serde_yaml` 0.9 - YAML configuration parsing
- `http` 1.1 - HTTP primitives
- `futures-util` 0.3 - Async utilities
- `async-trait` 0.1 - Async trait support for LLM providers

**Web Frontend:**
- `@tailwindcss/postcss` 4.2.1 - Tailwind CSS integration
- `autoprefixer` 10.4.27 - CSS vendor prefixing
- `eslint-plugin-react-hooks` 7.0.1 - React hooks linting rules
- `eslint-plugin-react-refresh` 0.5.2 - Vite React refresh plugin support

## Configuration

**Environment:**
Environment variables (in `.env.example` and `vel.toml`):
- `VEL_BIND_ADDR` - Server bind address (default: 127.0.0.1:4130)
- `VEL_BASE_URL` - Base URL for external references (default: http://127.0.0.1:4130)
- `VEL_DB_PATH` - SQLite database path (default: var/data/vel.sqlite)
- `VEL_ARTIFACT_ROOT` - Artifact storage directory (default: var/artifacts)
- `VEL_LOG_LEVEL` - Log level: info, debug, warn, error (default: info)
- `VEL_AGENT_SPEC_PATH` - Path to agent specifications YAML (default: config/agent-specs.yaml)
- `VEL_POLICIES_PATH` - Path to policies YAML (default: config/policies.yaml)
- `VEL_OPERATOR_API_TOKEN` - Optional auth token for operator routes (env var only)
- `VEL_WORKER_API_TOKEN` - Optional auth token for worker routes (env var only)

LLM Model configuration:
- `VEL_LLM_MODEL_PATH` - Path to main LLM weights (e.g., qwen3-coder-30b GGUF)
- `VEL_LLM_FAST_MODEL_PATH` - Path to fast LLM weights (e.g., qwen2.5-1.5b GGUF)
- Model profiles via `configs/models/model-profiles.yaml` - Provider routing and capabilities

Integration snapshot paths (configured in `vel.toml`):
- `calendar_ics_path` - Local .ics file path (default: var/integrations/calendar/local.ics)
- `calendar_ics_url` - Optional calendar .ics URL for pull-based sync
- `todoist_snapshot_path` - Local Todoist JSON snapshot (default: var/integrations/todoist/snapshot.json)
- `activity_snapshot_path` - Activity tracking snapshot (default: var/integrations/activity/snapshot.json)
- `health_snapshot_path` - Health metrics snapshot (default: var/integrations/health/snapshot.json)
- `git_snapshot_path` - Git activity snapshot (default: var/integrations/git/snapshot.json)
- `messaging_snapshot_path` - Messaging activity snapshot (default: var/integrations/messaging/snapshot.json)
- `reminders_snapshot_path` - Reminders snapshot (default: var/integrations/reminders/snapshot.json)
- `notes_path` - Notes directory for markdown/plaintext sync (default: var/integrations/notes)
- `transcript_snapshot_path` - Chat/transcript snapshot (default: var/integrations/transcripts/snapshot.json)

**Build:**
- `Cargo.toml` - Rust workspace manifest with shared dependencies (crates/*/Cargo.toml reference parent)
- `clients/web/package.json` - Web frontend npm dependencies
- `rust-toolchain.toml` - Pinned Rust toolchain (1.86.0) with clippy, rustfmt

## Platform Requirements

**Development:**
- Nix (toolchain via `shell.nix`) - Reproducible development environment
- Rust 1.86.0 with clippy, rustfmt - Backend compilation and linting
- Node.js (npm) - Web frontend development
- SQLite 3 - Database file format
- Unix/Linux shell - Makefile-based build workflow (BSD make compatible)

**Production:**
- Debian bookworm-slim - Container base (Dockerfile)
- Rust 1.86.0 - Compiled to native x86_64 ELF binary
- SQLite 3 - File-based persistence (var/data/vel.sqlite)
- Optional: llama-server 0.x - External inference backend for local LLM (via llama.cpp OpenAI-compatible API)
- Optional: openai-oauth proxy - OAuth bridge for OpenAI-compatible providers

## Architecture Notes

**Crate Layering (layering rules enforced):**
- `vel-core` - Domain types, no transport, no storage dependencies
- `vel-storage` - SQLite repositories via sqlx (must NOT depend on vel-api-types)
- `vel-api-types` - HTTP transport DTOs only
- `vel-config` - TOML/YAML config loading, paths, defaults
- `vel-llm` - Provider-agnostic LLM abstraction (llama.cpp, OpenAI-compatible)
- `veld` - HTTP daemon: routes → services → storage
- `vel-cli` - CLI binary (vel command)

**Web Frontend Architecture:**
- `clients/web/src/api/` - API client methods
- `clients/web/src/components/` - React components
- `clients/web/src/data/` - State management and queries
- `clients/web/src/realtime/` - WebSocket sync
- `clients/web/src/types.ts` - Comprehensive TypeScript types

---

*Stack analysis: 2026-03-17*
