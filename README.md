# Vel

Vel is a **local-first cognition runtime** for capture, recall, and daily orientation.

- **veld** — daemon and HTTP API  
- **vel-cli** — operator CLI

## Status

**Implemented:** capture storage, lexical search, artifacts API, run/event schema and inspection, doctor diagnostics, **run-backed context** (today/morning/end-of-day create runs, artifacts, and provenance), recent/review flows, synthesis workflows, import/ingestion, web/chat operator surfaces, and the current Apple bootstrap clients. Domain types live in `vel-core`; storage does not depend on API types.

**Current work:** repo audit and hardening follow-through live in [docs/tickets/repo-audit-hardening/README.md](docs/tickets/repo-audit-hardening/README.md). Repo-wide truth remains [docs/status.md](docs/status.md).

Context endpoints (today/morning/end-of-day) are **run-backed**: each request creates a run, writes a managed JSON artifact, and links run → artifact; inspect with `vel run inspect <id>`.

See [docs/status.md](docs/status.md) for details. Start doc navigation from [docs/README.md](docs/README.md). Canonical runtime concepts: [docs/runtime-concepts.md](docs/runtime-concepts.md).
Canonical terminology appendix: [docs/vocabulary.md](docs/vocabulary.md).

## User Docs

If you are running Vel as an operator (not working on internals), start here:

- [docs/user/README.md](docs/user/README.md)
- [docs/user/quickstart.md](docs/user/quickstart.md)
- [docs/user/setup.md](docs/user/setup.md)

## Build and run (dev)

From the repo root:

On Nix hosts, Vel expects the shell environment to provide:

- `obsidian` for the local notes/vault workflow
- `obsidian-export` for Obsidian vault CLI export tooling
- `swift` and `swiftpm` for Apple-client checks

The checked-in [shell.nix](shell.nix) includes these requirements.

| Command | Description |
|--------|-------------|
| `make build` | Build veld and the web client (release-style: `cargo build -p veld`, `npm run build` in clients/web). |
| `make dev` | Start **veld** and the **web dev server** together (one terminal). veld runs in the background; the UI is at the Vite dev URL (e.g. http://localhost:5173). Ctrl+C stops both. |
| `make dev-api` | Run only veld (API at **http://127.0.0.1:4130** by default). Use in one terminal. |
| `make dev-web` | Run only the web dev server. Use in a second terminal if you started veld with `make dev-api`. |
| `make seed` | Seed sample chat data (requires veld running). |
| `make verify` | Run repository truth check, Rust fmt/clippy checks, and Rust/web test/lint checks. |
| `make ci` | Run the same command set as CI for local verification (`install-web`, checks, tests, and build). |
| `make smoke` | Run a daemon/API/CLI smoke check (healthy startup, capture/search/recent today-flow). |
| `make check-apple-swift` | Verify the shared Apple `VelAPI` package builds inside the repo Nix shell. |
| `make install-web` | Install web dependencies in `clients/web` (`npm ci`). |
| `make bootstrap-demo-data` | Populate a local API with starter captures/commitments (`scripts/bootstrap-demo-data.sh`). |

veld runs migrations on startup. The web client uses `VITE_API_URL` (default `http://localhost:4130`) to talk to veld.

## Local Development (data and CLI)

Vel stores local development data under `var/` by default:

- database: `var/data/vel.sqlite`
- artifacts: `var/artifacts`
- logs: `var/logs`
- integration credentials: persisted in the local SQLite settings store under `var/data/vel.sqlite`; secret values are kept out of the public `/api/integrations` payloads
- Git safety: `var/` is ignored by Git, so saved local credentials survive restarts without being tracked in the repo
- Use `scripts/bootstrap-demo-data.sh` to prefill a local database/API with demo captures and commitments.

Example CLI commands (veld must be running for health/capture/search/context):

```bash
cargo run -p veld
cargo run -p vel-cli -- health
cargo run -p vel-cli -- capture "remember lidar budget"
cargo run -p vel-cli -- search lidar
cargo run -p vel-cli -- today
cargo run -p vel-cli -- morning
cargo run -p vel-cli -- config show
```

## Hugging Face CLI

The repo declares the official Hugging Face CLI (`hf`) as a project-scoped Nix shell dependency in `shell.nix`.

From the repo root:

```bash
nix-shell --run 'hf --help'
```

To verify the local LLM setup that `make dev` expects, run:

```bash
make check-llm-setup
```

To enter a repo shell with both `hf` and the Swift toolchain available:

```bash
nix-shell
```

## Local integration defaults

The repo-local `vel.toml` points the current workspace at local integration seed files:

- calendar ICS: `var/integrations/calendar/local.ics`
- Todoist snapshot: `var/integrations/todoist/snapshot.json`
- activity snapshot: `var/integrations/activity/snapshot.json`
- health snapshot: `var/integrations/health/snapshot.json`
- git snapshot: `var/integrations/git/snapshot.json`
- messaging snapshot: `var/integrations/messaging/snapshot.json`
- notes root: `var/integrations/notes`
- transcript snapshot: `var/integrations/transcripts/snapshot.json`
- agent specs: `config/agent-specs.yaml`
- primary LLM model: `configs/models/weights/qwen3-coder-30b-a3b-instruct-q4_k_m.gguf`
- fast LLM model: `configs/models/weights/qwen2.5-coder-14b-instruct-q4_k_m.gguf`

These are local file-based inputs for sync/bootstrap flows. Replace them with your real exports or local mirrors when you are ready, keeping the same config keys:

```toml
agent_spec_path = "config/agent-specs.yaml"
llm_model_path = "configs/models/weights/qwen3-coder-30b-a3b-instruct-*.gguf"
llm_fast_model_path = "configs/models/weights/qwen2.5-coder-14b-instruct-*.gguf"
calendar_ics_path = "var/integrations/calendar/local.ics"
todoist_snapshot_path = "var/integrations/todoist/snapshot.json"
activity_snapshot_path = "var/integrations/activity/snapshot.json"
health_snapshot_path = "var/integrations/health/snapshot.json"
git_snapshot_path = "var/integrations/git/snapshot.json"
messaging_snapshot_path = "var/integrations/messaging/snapshot.json"
notes_path = "var/integrations/notes"
transcript_snapshot_path = "var/integrations/transcripts/snapshot.json"
```

On macOS, `veld` also auto-discovers local source files for `activity`, `health`, `messaging`, `notes`, `git`, and `transcripts` under `~/Library/Application Support/Vel/...` when present, then performs a startup bootstrap sync so those sources can influence current context without manual path entry.

Cluster bootstrap metadata now also exposes capability hints for richer clients: node capabilities, repo branch-sync support, and validation/build-test profiles grouped by environment (`api`, `web`, `apple`, `repo`, `runtime`). Structured branch-sync and validation requests can be queued through the existing client sync action lane.

## Operator commands

- `vel health` — daemon health
- `vel doctor` — config, DB, schema version, artifact dir
- `vel capture <text>` — create capture (optional: `--type`, `--source`; use `-` or `--stdin` for stdin)
- `vel recent` — recent captures (`--limit`, `--today`, `--json`)
- `vel search <query>` — lexical search
- `vel today` / `vel morning` / `vel end-of-day` — context
- `vel inspect capture <id>` — inspect a capture
- `vel inspect artifact <id>` — inspect an artifact
- `vel runs` — list runs
- `vel run inspect <id>` — run detail
- `vel config show`
