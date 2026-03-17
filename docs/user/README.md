# Vel User Docs

This is the canonical user-facing entrypoint for running Vel.

Use these docs if your goal is to get Vel working for daily use, not to study the internal architecture.

## What Vel is

Vel is a local-first cognition runtime for:

- capturing thoughts and tasks,
- recalling relevant context,
- tracking commitments and nudges,
- building a usable picture of what matters now.

Today, the strongest shipped paths are:

- local daemon + CLI use,
- current context and `Now`-style operator surfaces,
- local file and snapshot based source ingestion,
- bootstrap Apple clients,
- macOS local-source auto-discovery for several sources.

Vel is not yet a polished consumer product. Some surfaces are bootstrap-level, and some architecture or ticket docs describe planned behavior rather than shipped behavior. For current implementation truth and phase status, see [MASTER_PLAN.md](../MASTER_PLAN.md).

## Start here

1. [Quickstart](quickstart.md) for the shortest path from repo clone to first useful result.
2. [Deployment And Setup](deployment.md) for the main Nix and Docker runtime paths.
3. [Setup](setup.md) for config, storage, integrations, and macOS source setup.
4. [Daily Use](daily-use.md) for the repeated workflow once Vel is running.
5. [Surfaces](surfaces.md) for what the main UI and CLI surfaces mean.
6. [Troubleshooting](troubleshooting.md) when Vel feels stale, degraded, or disconnected.
7. [Integrations](integrations/README.md) for credential-backed (Google/Todoist), local-source, and Apple/macOS setup details.
8. [Privacy](privacy.md) for the current local-first trust model and data locations.
9. [Reality And Maturity](reality-and-maturity.md) for what is shipped, partial, bootstrap, and still planned.

## What works now

- `veld` runs a local daemon and HTTP API.
- `veld` can also run as a persistent Docker container for NAS/local network deployment.
- `vel` provides the main operator CLI.
- captures, commitments, context generation, nudges, recent/review flows, and artifact/run inspection are implemented.
- local integrations can be ingested from files or snapshots.
- on macOS, `veld` auto-discovers local `activity`, `health`, `git`, `messaging`, `notes`, and `transcripts` sources under `~/Library/Application Support/Vel/...` when they exist.

## What is still partial

- Apple apps are bootstrap clients, not full product-complete apps.
- Apple-linked local ingestion is snapshot-based, not a full background sync platform.
- some integrations require local export files or host permissions before they produce data.

## User truth rules

- trust [MASTER_PLAN.md](../MASTER_PLAN.md) for what is actually shipped and what phase the repo is in,
- treat architecture and ticket docs as target design unless the Master Plan or these user docs say the behavior is shipped,
- use these user docs for setup and operation rather than inferring behavior from architecture docs.
