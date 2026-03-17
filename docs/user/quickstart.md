# Vel Quickstart

This is the shortest path from repo checkout to a working local Vel instance.

## Prerequisites

- the repo checked out locally
- Rust toolchain
- Node/npm for the web client if you want the browser UI
- Nix shell if you want the repo-provided environment and Apple Swift checks

## 1. Build or enter the repo environment

If you use Nix:

```bash
nix-shell
```

Otherwise build directly from the repo root:

```bash
make build
```

## 2. Start the daemon

From the repo root:

```bash
make dev-api
```

That starts `veld` locally. By default the API is available at `http://127.0.0.1:4130`.

## 3. Verify health

In another terminal:

```bash
cargo run -p vel-cli -- health
```

You should see a healthy response from the daemon.

## 4. Inspect effective local config

```bash
cargo run -p vel-cli -- config show
```

This is the fastest way to confirm:

- database path,
- artifact root,
- active local integration file paths,
- model paths,
- agent spec path.

## 5. Create your first capture

```bash
cargo run -p vel-cli -- capture "remember to test Vel"
```

For a task-like capture:

```bash
cargo run -p vel-cli -- capture "follow up on invoice" --type todo --source laptop
```

## 6. Generate current orientation

```bash
cargo run -p vel-cli -- morning
```

Useful follow-ups:

```bash
cargo run -p vel-cli -- recent --today
cargo run -p vel-cli -- context
cargo run -p vel-cli -- nudges
```

## 7. Optional: run the web client

```bash
make dev
```

That runs both `veld` and the web dev server together.

## 8. Optional: choose a fuller deployment path

If you want either:

- a reproducible host-side Nix environment, or
- a persistent NAS/container deployment

see [Deployment And Setup](deployment.md).

## 9. Optional: use local source snapshots

The repo-local `vel.toml` already points at local seed inputs under `var/integrations/`.

You can manually sync them with commands such as:

```bash
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync health
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- sync notes
```

On macOS, `veld` can also auto-discover local source files under `~/Library/Application Support/Vel/...` and bootstrap them automatically at startup when present.

## If this did not work

Use:

```bash
cargo run -p vel-cli -- doctor
```

Then check [Setup](setup.md) and [status.md](../status.md).
