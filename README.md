# Vel

Vel is a local-first personal executive system for capture, recall, and daily orientation.

This repository currently contains the bootstrap runtime and CLI:

- `veld`: the daemon and HTTP API
- `vel-cli`: the operator shell

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

## Bootstrap Surface

The current scaffold implements:

- `GET /v1/health`
- `POST /v1/captures`
- `GET /v1/search`
- `GET /v1/context/today`
- `GET /v1/context/morning`
- `vel health`
- `vel capture <text>`
- `vel search <query>`
- `vel today`
- `vel morning`
- `vel config show`
