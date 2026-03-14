# Vel

Vel is a **local-first cognition runtime** for capture, recall, and daily orientation.

- **veld** — daemon and HTTP API  
- **vel-cli** — operator CLI

## Status

**Implemented:** capture storage, lexical search, artifacts API, run/event schema and inspection, doctor diagnostics, context endpoints (today/morning/end-of-day), full CLI surface. Domain types live in `vel-core`; storage does not depend on API types.

**Planned next:** run-backed context generation (today/morning as runs with artifacts and provenance), typed JSON payloads, structured doctor output, doc hierarchy.

See [docs/status.md](docs/status.md) for details. Canonical runtime concepts: [docs/vel-runtime-concepts.md](docs/vel-runtime-concepts.md).

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
- `vel capture <text>` — create capture
- `vel search <query>` — lexical search
- `vel today` / `vel morning` / `vel end-of-day` — context
- `vel inspect capture <id>` — inspect a capture
- `vel runs` — list runs
- `vel run inspect <id>` — run detail
- `vel config show`
