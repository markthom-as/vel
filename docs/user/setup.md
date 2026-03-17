# Vel Setup

This guide covers the practical configuration and local data paths needed to run Vel.

## Runtime shape

Vel is primarily a local daemon plus local operator surfaces:

- `veld` is the daemon and HTTP API
- `vel` is the CLI
- the web client and Apple clients are clients of the same daemon

The default local daemon address is:

- `http://127.0.0.1:4130`

For Docker/NAS deployment, the containerized daemon binds:

- `0.0.0.0:4130` inside the container

For the full Nix and Docker deployment paths, see [Deployment And Setup](deployment.md).

## Local storage

The default repo-local development layout is:

- database: `var/data/vel.sqlite`
- artifacts: `var/artifacts`
- logs: `var/logs`

You can inspect the effective values with:

```bash
cargo run -p vel-cli -- config show
```

## Main config keys

The most important local config keys are:

- `db_path`
- `artifact_root`
- `agent_spec_path`
- `llm_model_path`
- `llm_fast_model_path`
- `calendar_ics_path`
- `todoist_snapshot_path`
- `activity_snapshot_path`
- `health_snapshot_path`
- `git_snapshot_path`
- `messaging_snapshot_path`
- `notes_path`
- `transcript_snapshot_path`

The checked-in repo config already points these at local seed files under `var/integrations/`.

## Local integrations

Vel currently ingests several sources from local files or snapshots.

Current local source inputs:

- calendar from `.ics`
- Todoist from snapshot JSON
- activity from snapshot JSON
- health from snapshot JSON
- git from snapshot JSON
- messaging from snapshot JSON
- notes from a file or directory
- transcripts from snapshot JSON

Typical manual sync commands:

```bash
cargo run -p vel-cli -- sync calendar
cargo run -p vel-cli -- sync todoist
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync health
cargo run -p vel-cli -- sync git
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- sync notes
cargo run -p vel-cli -- sync transcripts
```

After syncing, run:

```bash
cargo run -p vel-cli -- evaluate
```

That forces inference and nudge generation from the latest signals.

## macOS local-source auto-discovery

On macOS, `veld` will auto-discover certain local sources when files exist under:

- `~/Library/Application Support/Vel/`

Current auto-discovered locations:

- `activity/snapshot.json`
- `health/snapshot.json`
- `git/snapshot.json`
- `messages/snapshot.json`
- `notes/`
- `transcripts/snapshot.json`

When these files exist, `veld` can bootstrap them on startup so they reach current context without manual path entry.

## VelMac exporter behavior

`VelMac` can write local snapshots into the same Application Support tree.

Current snapshot-backed exports:

- activity heartbeat
- health summary snapshot
- recent messages thread snapshot

Important caveats:

- this is still snapshot-based ingestion, not full direct system integration inside Rust
- health export depends on HealthKit availability and authorization
- messages export reads the local Messages database and may require Full Disk Access on macOS
- if the daemon is unreachable, snapshots can still be written locally but daemon sync happens later

For the Apple client details, see [clients/apple/README.md](../../clients/apple/README.md).

## Apple client endpoint resolution

Apple clients currently try endpoints in this order:

1. `vel_base_url`
2. `vel_tailscale_url`
3. `vel_lan_base_url`
4. `http://127.0.0.1:4130`
5. `http://localhost:4130`

Use:

- `vel_tailscale_url` for normal multi-device tailnet use
- `vel_base_url` for explicit override
- `vel_lan_base_url` as a local-network fallback

The web Settings page now also exposes the daemon-side sync metadata that cluster/bootstrap surfaces use:

- `node_display_name`
- `tailscale_base_url`
- `lan_base_url`

Set `tailscale_base_url` to the stable tailnet address for the daemon you want every client to prefer. Apple clients should use the same URL for `vel_tailscale_url`.

## Obsidian vault setup

If you use Obsidian Sync for notes across devices, point Vel's `notes` integration at the local vault root on the machine running `veld`.

- Obsidian Sync remains responsible for replicating vault files.
- Vel reads the synced markdown/plaintext files from disk and ingests them through `sync notes`.
- The web Settings page labels this adapter as `Obsidian Vault` so the path you save there should be the local vault directory, not a separate export.

## Trust and local ownership

Vel is designed to be local-first.

Operationally, that means:

- the main database and artifacts are local files,
- local integrations can run entirely from user-controlled files and snapshots,
- secret settings are stored locally rather than exposed in public integration payloads,
- exported local source snapshots are inspectable files.

## Common setup checks

Use these commands when setup feels wrong:

```bash
cargo run -p vel-cli -- health
cargo run -p vel-cli -- doctor
cargo run -p vel-cli -- config show
cargo run -p vel-cli -- runs --today
```

If current context looks stale after a source update, sync the relevant source and run `evaluate` again.
