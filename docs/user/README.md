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
5. [Coding Workflows](coding-workflows.md) for the repo-local Phase 08 execution path from context export to handoff review to supervised runtime launch.
6. [Surfaces](surfaces.md) for what the main UI and CLI surfaces mean.
7. [System Surface](system.md) for the operator-facing `System` view and its intended use.
8. [Troubleshooting](troubleshooting.md) when Vel feels stale, degraded, or disconnected.
9. [Integrations](integrations/README.md) for credential-backed (Google/Todoist), local-source, and Apple/macOS setup details.
10. [Backup And Restore](backup-and-restore.md) for the manual-first backup trust model and recovery posture.
11. [Eval Runner](evals.md) for fixture-driven deterministic replay and optional judge scoring.
12. [Privacy](privacy.md) for the current local-first trust model and data locations.
13. [Reality And Maturity](reality-and-maturity.md) for what is shipped, partial, bootstrap, and still planned.

## Support And Update Model

These docs are the operator support surface for shipped behavior.

- Update `docs/user/` when a shipped CLI, web, Apple, or recovery workflow changes.
- Update `docs/api/` in the same slice when an operator-visible endpoint or inspect contract changes.
- Keep `docs/MASTER_PLAN.md` as the release-truth source for shipped vs planned status.
- Prefer repairing the nearest user doc over adding a new shadow note elsewhere in the repo.

For support work, use this order:

1. confirm shipped status in [MASTER_PLAN.md](../MASTER_PLAN.md)
2. check the relevant setup/use/troubleshooting page in `docs/user/`
3. check [docs/api/runtime.md](../api/runtime.md) when the problem involves run inspection, traces, or endpoint behavior
4. inspect the live runtime with CLI or web surfaces before assuming the docs are wrong

## What works now

- `veld` runs a local daemon and HTTP API.
- `veld` can also run as a persistent Docker container for NAS/local network deployment.
- `vel` provides the main operator CLI.
- captures, commitments, context generation, nudges, recent/review flows, and artifact/run inspection are implemented.
- recent run inspection now exposes trace lineage (`trace_id`, optional `parent_run_id`) in both CLI and web runtime surfaces.
- fixture-driven eval runs are available through `veld-evals`, with deterministic replay as the hard gate and optional judge scoring layered on top.
- local integrations can be ingested from files or snapshots.
- on macOS, `veld` auto-discovers local `activity`, `health`, `git`, `messaging`, `reminders`, `notes`, and `transcripts` sources under `~/Library/Application Support/Vel/...` when they exist.

## What is still partial

- Apple apps are bootstrap clients, not full product-complete apps.
- Apple-linked local ingestion is snapshot-based, not a full background sync platform.
- some integrations require local export files or host permissions before they produce data.

## User truth rules

- trust [MASTER_PLAN.md](../MASTER_PLAN.md) for what is actually shipped and what phase the repo is in,
- treat architecture and ticket docs as target design unless the Master Plan or these user docs say the behavior is shipped,
- use these user docs for setup and operation rather than inferring behavior from architecture docs.
