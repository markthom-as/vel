# Vel Privacy And Data Ownership

Vel is designed to be local-first.

This guide explains what that means in practical user terms today.

## Core model

Vel’s default shape is:

- local daemon
- local database
- local artifacts
- local file or snapshot based integrations

That is a trust advantage only if the user can actually inspect and control those files, so the current runtime keeps those storage points visible.

## Where your local data lives

In the default repo-local setup, Vel stores data under:

- `var/data/vel.sqlite`
- `var/artifacts`
- `var/logs`

Use:

```bash
cargo run -p vel-cli -- config show
```

to confirm the effective paths on your machine.

## What is stored locally

Current local storage can include:

- captures
- commitments
- nudges
- runs
- artifacts
- inferred current context
- source sync state and integration metadata
- local secret settings for supported integrations

## Secrets

Sensitive Google and Todoist credentials are stored in dedicated local secret settings records rather than being exposed through the public integrations payloads.

That does not make them magical. It means:

- they are persisted locally,
- they are not supposed to appear in normal operator settings responses,
- you should still treat the machine and database as sensitive.

## Local source snapshots

Many integrations currently work from local files and snapshots.

That includes inputs such as:

- calendar `.ics`
- Todoist snapshots
- activity snapshots
- health snapshots
- messaging snapshots
- notes directories
- transcript snapshots

These files are inspectable and user-controlled. That is a feature, not a temporary accident.

## macOS local exports

On macOS, local Apple-linked data may be exported into:

- `~/Library/Application Support/Vel/`

Current snapshot-backed examples include:

- activity
- health
- messages

These are local files. If you want to know what Vel can ingest, inspect those files directly.

## What leaves the machine

Current answer:

- the local-first runtime is designed so core operation can happen on your machine
- some optional integrations involve external services by definition, such as Google Calendar or Todoist
- Apple clients and the web client talk to `veld` over HTTP

You should not assume “local-first” means “nothing ever crosses a process or network boundary.” It means local ownership and local inspectability are the default design center.

## How to inspect what Vel knows

Use:

```bash
cargo run -p vel-cli -- config show
cargo run -p vel-cli -- runs --today
cargo run -p vel-cli -- run inspect <id>
cargo run -p vel-cli -- inspect artifact <id>
```

This is the practical inspectability path for:

- where the runtime points,
- what ran,
- what was generated,
- what persisted.

## How to remove or replace local source data

Because many sources are file-backed or snapshot-backed, removal is straightforward:

1. remove or replace the source file
2. clear or change the configured source path if needed
3. rerun sync and evaluate

This is simpler and more trustworthy than opaque vendor-side sync state.

## What local-first does not guarantee

Local-first does not automatically mean:

- encrypted at rest everywhere
- zero-copy handling across all surfaces
- zero external dependencies
- full privacy review completed for every path

It means the architecture prefers local control, inspectability, and user-owned storage over opaque remote defaults.

## Current trust rule

If trust matters:

1. inspect the configured paths
2. inspect the local snapshot or source file
3. inspect the generated run or artifact
4. check [MASTER_PLAN.md](../MASTER_PLAN.md) for the current shipped truth

That is the right trust posture for Vel today.
