# Vel Troubleshooting

This guide covers the most likely real failures when running Vel locally.

For implementation truth about what is and is not shipped, see [status.md](../status.md).

## First checks

When something feels wrong, run these first:

```bash
cargo run -p vel-cli -- health
cargo run -p vel-cli -- doctor
cargo run -p vel-cli -- config show
cargo run -p vel-cli -- runs --today
```

These usually tell you whether the daemon is up, whether the database and artifact paths are valid, and whether recent runs or evaluations are actually happening.

## The daemon is not reachable

Symptoms:

- `vel health` fails
- the web app says the API is unavailable
- Apple clients say no reachable endpoint

What to do:

1. start `veld` with `make dev-api` or `cargo run -p veld`
2. confirm the daemon is listening on `http://127.0.0.1:4130`
3. rerun `cargo run -p vel-cli -- health`
4. if using Apple clients, verify `vel_base_url`, `vel_tailscale_url`, or `vel_lan_base_url`

If you are using the reproducible Nix path instead of a host-installed toolchain, use:

```bash
make nix-dev-api
```

If you are using the Docker/Podman NAS path, use:

```bash
./scripts/container-compose.sh ps
./scripts/container-compose.sh logs veld
```

If the daemon still does not start cleanly, use `vel doctor` and inspect the configured `db_path` and `artifact_root`.

## Current context is empty or stale

Symptoms:

- `vel context` or the `Now` view has little or no useful state
- the web `Now` surface says no current context yet
- new source data is not reflected

What to do:

1. sync the relevant source
2. run `cargo run -p vel-cli -- evaluate`
3. recheck `cargo run -p vel-cli -- context`

Useful commands:

```bash
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- sync notes
cargo run -p vel-cli -- evaluate
```

If the source itself is stale or missing, evaluate will not invent data. Fix the source path or export first.

## A local integration is configured but not producing data

Symptoms:

- sync succeeds with little or no effect
- the Settings page shows missing or stale guidance
- expected summaries do not appear in current context

What to check:

1. run `cargo run -p vel-cli -- config show`
2. verify the relevant source path actually exists
3. verify the file or directory contains current data in the expected format
4. rerun the specific `vel sync ...` command

Common causes:

- the path points to an old export
- the file is empty
- the wrong source path is configured
- the source was synced but `evaluate` was not rerun yet

## macOS local auto-discovery is not working

Symptoms:

- `veld` does not pick up Apple-local sources automatically
- startup does not seem to ingest activity, health, messages, notes, git, or transcripts

What to check:

1. verify the files exist under `~/Library/Application Support/Vel/`
2. verify the expected subpaths exist exactly
3. restart `veld`
4. inspect `Now` or rerun a manual sync if you want an immediate refresh

Expected current locations:

- `activity/snapshot.json`
- `health/snapshot.json`
- `git/snapshot.json`
- `messages/snapshot.json`
- `notes/`
- `transcripts/snapshot.json`

If these files do not exist, auto-discovery has nothing to ingest.

## VelMac is running but messages or health still do not appear

Symptoms:

- VelMac launches but does not produce useful local Apple data
- `health` or `messaging` remains missing or degraded

What to check:

1. confirm `VelMac` has permission to access the needed data
2. for health, confirm HealthKit is available and authorization was granted
3. for messages, confirm the host allows reading `~/Library/Messages/chat.db`
4. on macOS, grant Full Disk Access if required by host policy

Important current reality:

- Apple-linked ingestion is snapshot-based
- if the snapshot is not written locally, `veld` cannot ingest it

## Google Calendar or Todoist is stale or disconnected

Symptoms:

- the `Now` view shows stale calendar or Todoist data
- the Settings page shows disconnected or failed sync state

What to do:

1. open the Settings page
2. inspect the integration card guidance
3. reconnect or save credentials if needed
4. use `Sync now`
5. rerun `evaluate` if current context still looks stale

Current shipped behavior:

- Google and Todoist have dedicated settings and sync flows
- local-source integrations use source paths instead

## Notes, transcripts, activity, git, or messaging shows guidance about source path

That usually means the local source path is not configured or not resolvable.

What to do:

1. open Settings
2. save the correct local source path for that integration
3. run `Sync now`
4. rerun `evaluate` if needed

On macOS, if the source is one of the auto-discovered local sources, you can also place the file in the Application Support location instead of manually configuring a path.

## The `Now` view says data is degraded

The current web `Now` surface exposes source freshness and recovery actions.

Interpretation:

- `aging` means the data may still be usable, but recent changes may be missing
- `stale` means you should not trust it without refresh
- `error` means the last sync failed
- `disconnected` means the source is not connected or configured

What to do:

- use the provided `Sync` action for retryable sources
- use `Re-run evaluate` when the context itself is stale
- open Settings when the UI tells you the problem is configuration or credentials

## I need to inspect why Vel said something

Use the inspect and explain surfaces instead of guessing:

```bash
cargo run -p vel-cli -- runs --today
cargo run -p vel-cli -- run inspect <id>
cargo run -p vel-cli -- inspect artifact <id>
```

In the web client, use:

- the Context panel
- the Now source freshness details
- the Suggestions evidence/detail view

## Last resort

If the state still looks wrong:

1. verify the daemon is healthy
2. verify the source files are current
3. sync the source explicitly
4. rerun `evaluate`
5. inspect the latest runs and artifacts

That sequence usually distinguishes:

- no source data,
- bad source configuration,
- stale inference state,
- or a genuine runtime bug.
