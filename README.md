# Vel

Vel is a **local-first cognition runtime** for capture, recall, and daily orientation.

- **veld** — daemon and HTTP API  
- **vel-cli** — operator CLI

## Status

**Implemented:** capture storage, lexical search, artifacts API, run/event schema and inspection, doctor diagnostics, **run-backed context** (today/morning/end-of-day create runs, artifacts, and provenance), full CLI surface including `vel recent`, `vel inspect artifact`, capture with `--type`/`--source`/`--stdin`. Domain types live in `vel-core`; storage does not depend on API types.

**Planned next:** recent/review flows, synthesis workflows, import/ingestion, usability improvements for daily operation. See [docs/status.md](docs/status.md) for the dogfooding roadmap.

Context endpoints (today/morning/end-of-day) are **run-backed**: each request creates a run, writes a managed JSON artifact, and links run → artifact; inspect with `vel run inspect <id>`.

See [docs/status.md](docs/status.md) for details. Start doc navigation from [docs/README.md](docs/README.md). Canonical runtime concepts: [docs/runtime-concepts.md](docs/runtime-concepts.md).

## Build and run (dev)

From the repo root:

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
| `make install-web` | Install web dependencies in `clients/web` (`npm ci`). |
| `make bootstrap-demo-data` | Populate a local API with starter captures/commitments (`scripts/bootstrap-demo-data.sh`). |

veld runs migrations on startup. The web client uses `VITE_API_URL` (default `http://localhost:4130`) to talk to veld.

## Local Development (data and CLI)

Vel stores local development data under `var/` by default:

- database: `var/data/vel.sqlite`
- artifacts: `var/artifacts`
- logs: `var/logs`
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

## Local integration defaults

The repo-local `vel.toml` points the current workspace at local integration seed files:

- calendar ICS: `var/integrations/calendar/local.ics`
- Todoist snapshot: `var/integrations/todoist/snapshot.json`

These are local file-based inputs for `vel sync calendar` and `vel sync todoist`. Replace them with your real exported ICS feed and Todoist snapshot when you are ready, keeping the same config keys:

```toml
calendar_ics_path = "var/integrations/calendar/local.ics"
todoist_snapshot_path = "var/integrations/todoist/snapshot.json"
```

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
