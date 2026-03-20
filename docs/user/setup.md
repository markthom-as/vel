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
- `reminders_snapshot_path`
- `notes_path`
- `transcript_snapshot_path`

The checked-in repo config already points these at local seed files under `var/integrations/`.

## LLM routing and remote OpenAI auth

Vel's assistant path uses the configured model routing under `configs/models/`.

- local models use `provider = "llama_cpp"`
- localhost OAuth-backed OpenAI-compatible proxies use `provider = "openai_oauth"`
- `openai_oauth` profiles are only enabled when `VEL_ENABLE_OPENAI_OAUTH=1`
- for safety, `openai_oauth` base URLs must point to `localhost` or `127.0.0.1`

The assistant chat path can now use a bounded read-only Vel tool surface on top of the configured LLM profile. That means the model can answer from `Now`, projects, people, commitments, local semantic recall, active daily-loop state, and filtered threads instead of only seeing bare conversation history.

Current recall limit:

- this is still bounded local recall over persisted Vel data, not broad hosted memory
- answers are only as good as the captures, notes, projects, people, threads, and transcripts already stored locally
- the backend now returns typed assistant-context packs with summary, source breakdown, scores, and provenance so shells do not have to invent their own memory state

## Local integrations

Vel currently ingests several sources from local files or snapshots.

Current local source inputs:

- calendar from `.ics`
- Todoist from snapshot JSON
- activity from snapshot JSON
- health from snapshot JSON
- git from snapshot JSON
- messaging from snapshot JSON
- reminders from snapshot JSON
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
cargo run -p vel-cli -- sync reminders
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
- `reminders/snapshot.json`
- `notes/`
- `transcripts/snapshot.json`

When these files exist, `veld` can bootstrap them on startup so they reach current context without manual path entry.

## VelMac exporter behavior

`VelMac` can write local snapshots into the same Application Support tree.

Current snapshot-backed exports:

- activity heartbeat
- health summary snapshot
- recent messages thread snapshot
- reminders snapshot

Important caveats:

- this is still snapshot-based ingestion, not full direct system integration inside Rust
- health export depends on HealthKit availability and authorization
- health snapshot can include step count, active energy, sleep hours, heart rate, stand hours, and blood pressure
- messages export reads the local Messages database and may require Full Disk Access on macOS
- reminders export depends on EventKit reminders authorization
- if the daemon is unreachable, snapshots can still be written locally but daemon sync happens later

For the Apple client details, see [clients/apple/README.md](../../clients/apple/README.md).

## Apple client endpoint resolution

Apple clients currently try endpoints in this order:

1. `vel_tailscale_url`
2. `vel_base_url`
3. `vel_lan_base_url`
4. `http://127.0.0.1:4130`
5. `http://localhost:4130`

Use:

- `vel_tailscale_url` for normal multi-device tailnet use
- `vel_base_url` as a fallback or temporary direct host target
- `vel_lan_base_url` as a local-network fallback

The web Settings page now also exposes the daemon-side sync metadata that cluster/bootstrap surfaces use:

- `node_display_name`
- `tailscale_base_url`
- `lan_base_url`

Set `tailscale_base_url` to the stable tailnet address for the daemon you want every client to prefer. Apple clients should use the same URL for `vel_tailscale_url`.

When Settings shows linking or Apple recovery friction, follow this order:

1. confirm `veld` is reachable and cluster bootstrap has loaded
2. confirm the Apple client endpoint order still resolves to the same daemon
3. confirm local snapshot or notes paths exist on the daemon host
4. only then retry pairing, sync, or evaluate

This keeps setup grounded in the shipped trust model: endpoint resolution first, local path discovery second, then sync.

## Which guide to open from the shell

When the web shell or Settings tells you something needs setup, use this path:

- `Now` freshness or stale connector warning:
  open the matching integration guide under `docs/user/integrations/` and then rerun the relevant sync.
- `Now` check-in card:
  answer it inline if the suggested action is correct; if the question needs more context, continue in Threads and then return to `Now`.
- `Now` reflow suggestion:
  treat it as a day-plan recalculation prompt. Confirm it when the current schedule is stale, a key event was missed, or today no longer fits the remaining time.
  `Now` now shows the compact same-day recovery proposal directly; use `Threads` only when the result needs longer shaping or disagreement.
  `Now` may also show compact same-day schedule continuity after a proposal is staged or applied; treat that as backend-owned review/apply status, not as a second planner UI.
  if Settings or `Now` says planning is using inferred fallback, that means durable routine blocks have not been configured yet and the backend is still shaping the day from transient context only.
- `Settings` routine/planning profile card:
  use it when you want to save or remove durable routine blocks or bounded planning constraints.
  those edits now go through the typed backend planning-profile seam; they are not just generic local settings text fields.
  CLI and Apple summary surfaces read that same profile, so if those shells disagree with web `Settings`, treat that as a bug rather than separate configuration.
  the same backend response now also carries compact proposal continuity, so summary surfaces can show pending review or the last applied/failed edit without becoming planners.
  if assistant entry or Apple voice staged a planning-profile edit instead of applying it, continue in `Threads` to inspect the proposed change and its continuity metadata; conversational shells are not allowed to silently rewrite the saved profile.
- linking or paired-node trust question:
  use the linking routes in Settings first, then `docs/api/runtime.md` for `/v1/linking/*` behavior and `docs/user/troubleshooting.md` for recovery.
- Todoist token or sync question:
  open `docs/user/integrations/todoist.md`.
- Apple endpoint, `vel_tailscale_url`, or local-source path question:
  stay in this setup guide first, then continue to `docs/user/integrations/apple-macos.md` or `docs/user/integrations/local-sources.md`.

## Summary-first setup and trust rule

Vel should normally route setup, trust, and recovery work in this order:

1. short summary in `Now` or Settings
2. one suggested next action
3. optional deeper inspection
4. only then raw troubleshooting or implementation-aware recovery

Examples:

- onboarding blocker
- stale source input
- writeback trust issue
- linking problem
- Apple local-source discovery failure
- reflow/recovery posture when the current day plan is no longer trustworthy
- durable routine-backed planning posture when `Now` or Settings shows inferred fallback instead of operator-managed routines
- routine/planning profile management when you need to replace inferred fallback with saved durable routine blocks

If you land directly in diagnostics, step back to the matching summary surface first unless the product is already in a clear recovery-only state.

Phase 12 decision note:

- the current shell/help path reuses existing typed runtime routes and user docs
- there is not yet a dedicated backend `help` payload
- if a later shell slice needs one, it must land as a typed contract before UI-only behavior depends on it

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
