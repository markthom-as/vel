# Vel Deployment And Setup

This guide covers the two practical ways to run Vel today:

- Nix for a reproducible local or server-side runtime environment
- Docker/Podman for a persistent NAS-style daemon deployment

Use Nix when you want a host-managed setup with the toolchain available directly on the machine.

Use Docker or Podman when you want a simple always-on daemon with one persistent volume and minimal host setup.

## What you are deploying

The shipped runtime is centered on:

- `veld` as the daemon and HTTP API
- `vel` as the operator CLI

The web UI and Apple clients are separate clients of the same daemon. The deployment goal is therefore to get a stable `veld` endpoint running first.

## Option 1: Nix

The repo ships a `shell.nix` that now includes the Rust toolchain, Node/npm, OpenSSL build inputs, Swift tooling, and the extra local-operator utilities already used by the project.

### Enter the environment

From the repo root:

```bash
nix-shell
```

Or verify the toolchain without opening an interactive shell:

```bash
make nix-shell-info
```

### Run the daemon with Nix

From the repo root:

```bash
make nix-dev-api
```

That is equivalent to:

```bash
nix-shell --run 'cargo run -p veld'
```

By default the daemon listens on:

- `http://127.0.0.1:4130`

### Build with Nix

```bash
make nix-build
```

That runs the repo build inside the Nix shell and is the simplest reproducible host-side build path currently shipped.

### Persisting data in a Nix-hosted run

By default Vel uses:

- database: `var/data/vel.sqlite`
- artifacts: `var/artifacts`
- logs: `var/logs`

If you want the daemon to use a NAS-backed or host-specific persistent path, override these:

```bash
VEL_DB_PATH=/srv/vel/db/vel.sqlite \
VEL_ARTIFACT_ROOT=/srv/vel/artifacts \
VEL_BIND_ADDR=0.0.0.0:4130 \
VEL_BASE_URL=http://<host>:4130 \
nix-shell --run 'cargo run -p veld'
```

This is the preferred non-container path if you want full host visibility and direct filesystem control.

## Option 2: Docker Or Podman

The repo ships:

- `Dockerfile`
- `docker-compose.yml`

This path is intended for NAS or always-on daemon use.

The compose workflow is runtime-agnostic through the repo helper scripts:

- `make container-build`
- `make container-up`
- `make container-down`
- `make container-config`

They auto-detect `docker`, then `podman`, then `podman-compose`. You can also force a runtime:

```bash
CONTAINER_RUNTIME=docker make container-up
CONTAINER_RUNTIME=podman make container-up
```

### Start it

From the repo root:

```bash
mkdir -p var/docker/vel
make container-up
```

Then verify:

```bash
make container-config
curl http://127.0.0.1:4130/v1/health
```

If checking from another machine, replace `127.0.0.1` with the NAS hostname or LAN IP.

### Persistent storage

The default compose file maps:

- `./var/docker/vel:/data`

Inside the container, Vel stores:

- database: `/data/db/vel.sqlite`
- artifacts: `/data/artifacts`
- logs: `/data/logs`

For Synology, QNAP, or TrueNAS, replace the host side of that bind mount with your shared storage path, for example:

```yaml
volumes:
  - /volume1/docker/vel:/data
```

### Container runtime values

The container sets:

- `VEL_BIND_ADDR=0.0.0.0:4130`
- `VEL_DB_PATH=/data/db/vel.sqlite`
- `VEL_ARTIFACT_ROOT=/data/artifacts`

That makes the daemon reachable on the LAN while preserving SQLite and artifact durability on the mounted volume.

## Client configuration

Once the daemon is up, clients should target:

- `http://<host>:4130`

Examples:

- CLI from another machine: `VEL_BASE_URL=http://<host>:4130 cargo run -p vel-cli -- health`
- web dev client: `VITE_API_URL=http://<host>:4130 npm run dev`
- Apple clients: set `vel_tailscale_url` (preferred) or `vel_base_url`/`vel_lan_base_url` to the same host

## Optional local-source inputs

The daemon can run without any extra local-source files.

If you want the deployed daemon to ingest local snapshots or exported files, configure one or more of:

- `VEL_CALENDAR_ICS_PATH`
- `VEL_TODOIST_SNAPSHOT_PATH`
- `VEL_ACTIVITY_SNAPSHOT_PATH`
- `VEL_HEALTH_SNAPSHOT_PATH`
- `VEL_GIT_SNAPSHOT_PATH`
- `VEL_MESSAGING_SNAPSHOT_PATH`
- `VEL_NOTES_PATH`
- `VEL_TRANSCRIPT_SNAPSHOT_PATH`

These can point to host paths in a Nix-hosted run, or mounted container paths in Docker/Podman.

## Which path to choose

- Choose Nix if you want direct host execution, a reproducible shell, and easier debugging.
- Choose Docker/Podman if you want the cleanest NAS deployment with one persistent bind mount and automatic restart behavior.

## Updating

For Nix-hosted runs:

```bash
nix-shell --run 'cargo run -p veld'
```

after pulling the latest code.

For Docker/Podman:

```bash
make container-up
```

To stop the container deployment:

```bash
make container-down
```
