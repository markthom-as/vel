# Vel

Vel is a **local-first cognition runtime** for capture, recall, and daily orientation.

- **veld** — daemon and HTTP API  
- **vel-cli** — operator CLI

## Status

**Implemented:** capture storage, lexical search, artifacts API, run/event schema and inspection, doctor diagnostics, **run-backed context** (today/morning/end-of-day create runs, artifacts, and provenance), full CLI surface including `vel recent`, `vel inspect artifact`, capture with `--type`/`--source`/`--stdin`. Domain types live in `vel-core`; storage does not depend on API types.

**Planned next:** recent/review flows, synthesis workflows, import/ingestion, usability improvements for daily operation. See [docs/status.md](docs/status.md) for the dogfooding roadmap.

Context endpoints (today/morning/end-of-day) are **run-backed**: each request creates a run, writes a managed JSON artifact, and links run → artifact; inspect with `vel run inspect <id>`.

See [docs/status.md](docs/status.md) for details. Canonical runtime concepts: [docs/runtime-concepts.md](docs/runtime-concepts.md).

## Local Development

Vel stores local development data under `var/` by default:

- database: `var/data/vel.sqlite`
- artifacts: `var/artifacts`
- logs: `var/logs`

Example commands:

```bash
cargo run -p veld
cargo run -p vel-cli -- health
cargo run -p vel-cli -- capture "remember lidar budget"
cargo run -p vel-cli -- search lidar
cargo run -p vel-cli -- today
cargo run -p vel-cli -- morning
cargo run -p vel-cli -- config show
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
