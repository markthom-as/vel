# Vel

Vel is a **local-first personal executive system** and **autonomous cognition runtime** designed for capture, recall, and distributed agent orchestration.

- **veld** — the core daemon and "Authority" node
- **vel-cli** — the primary operator shell
- **clients/apple** — iOS/watchOS/macOS "Limb" clients
- **clients/web** — The unified operator dashboard

---

## 🚀 The Master Plan

Vel is currently transitioning from a prototype to a bulletproof distributed swarm. Our development is guided by a **four-phase Master Plan** that consolidates all architectural and feature requirements:

1.  **Phase 1: Foundations** — Monolith decomposition, modular storage, and typed context. **[COMPLETE]**
2.  **Phase 2: Swarm & Sync** — Distributed state, deterministic sync ordering, and Agent Connect. **[PARTIAL]**
3.  **Phase 3: Verification** — Deterministic simulation, reasoning evals, and tracing. **[PARTIAL]**
4.  **Phase 4: Autonomy** — Semantic Graph RAG, WASM sandboxing, and P2P sync. **[PLANNED]**

**Full Roadmap & Status**: [docs/MASTER_PLAN.md](docs/MASTER_PLAN.md)  
**Active Tickets**: [docs/tickets/README.md](docs/tickets/README.md)

---

## Current Status (MVP)

The **Vel v0 (MVP)** is 75% complete and currently blocked on remaining hardening/contract closure outside the shipped core.

- **Implemented**: Capture storage (CLI/Web), Lexical search (FTS5), Commitment CRUD, SQLite migration engine, Context runs (Today/Morning/End-of-Day), and Cluster worker heartbeats.
- **Active Development**: Connect agent launch protocol, distributed sync foundations, and operator-surface accessibility clarity.

---

## Getting Started

### 1. Developer Onboarding
If you are contributing to the codebase, please review the following:
- **Docs Guide**: [docs/README.md](docs/README.md)
- **Canonical Architecture**: [docs/MASTER_PLAN.md](docs/MASTER_PLAN.md)
- **Ticket Queue**: [docs/tickets/README.md](docs/tickets/README.md)
- **Architecture-First Queue**: [docs/tickets/architecture-first-parallel-queue.md](docs/tickets/architecture-first-parallel-queue.md)
- **Concept Spec**: [docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md](docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md)
- **Cross-Cutting Traits**: [docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md](docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md)
- **Schemas & Contracts**: [docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md](docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md)
- **Config Assets**: [config/README.md](config/README.md)
- **Repo Hardening**: [docs/tickets/phase-1/001-storage-modularization.md](docs/tickets/phase-1/001-storage-modularization.md)
- **Agent SDK**: [docs/tickets/phase-4/010-wasm-agent-sandboxing.md](docs/tickets/phase-4/010-wasm-agent-sandboxing.md)

### 2. Operator Quickstart
If you are running Vel as a user, start here:
- [docs/user/README.md](docs/user/README.md)
- [docs/user/setup.md](docs/user/setup.md)

---

## Build and Run (Dev)

Vel uses Nix for a reproducible toolchain. Ensure you are in the `nix-shell` before running the following:

| Command | Description |
|--------|-------------|
| `make build` | Build veld and the web client. |
| `make dev` | Start **veld** and the **web dev server** (Vite UI at http://localhost:5173). |
| `make dev-api` | Start only `veld` for runtime and API work. |
| `make dev-web` | Start only the web dev server against an existing daemon. |
| `make verify` | Run Rust fmt/clippy and full test suite. |
| `make ci` | Run local CI verification (install, check, test, build). |
| `make seed` | Populate local API with sample captures and commitments. |
| `make smoke` | Run the daemon/API/CLI smoke path. |
| `make bootstrap-demo-data` | Load demo-oriented local data and snapshots. |

### Example CLI Commands
```bash
vel health                  # Check daemon status
vel capture "Lidar budget"  # Quick capture
vel today                   # Generate morning briefing
vel runs                    # View active worker runs
vel config show             # Inspect local node configuration
```

---

## Persistence & Data
By default, Vel stores all data under `var/` (Git-ignored):
- **Database**: `var/data/vel.sqlite`
- **Artifacts**: `var/artifacts/`
- **Logs**: `var/logs/`

For detailed configuration of local integrations (Calendar, Todoist, Git, etc.), see the `Local Development` section in [docs/MASTER_PLAN.md](docs/MASTER_PLAN.md).
