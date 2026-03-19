# Todoist Integration

This guide covers the current shipped credential-backed Todoist path.

## What is implemented now

Todoist is handled through the web Settings integrations surface with:

- local save of a Todoist API token,
- `Sync now`,
- `Disconnect`,
- sync history and guidance,
- allowed Todoist write actions for create, update, complete, and reopen through the operator-authenticated runtime.

For shipped-truth authority, see [MASTER_PLAN.md](../../MASTER_PLAN.md).

## Setup flow

In the web Settings `Integrations` tab:

1. Paste your Todoist API token.
2. Click `Save token`.
3. Click `Sync now`.

When connected, the integration card shows status and last sync metadata.

## What "working" looks like

Usually healthy state means:

- integration badge is connected,
- `last_sync_status` is successful,
- `last_sync_at` updates after sync,
- `last_item_count` reflects synced tasks,
- Todoist-backed commitments appear in `Now` and commitment flows.

## Troubleshooting

If Todoist is stale, disconnected, or failing:

1. check guidance and status in Settings
2. save token again if needed
3. retry `Sync now`
4. rerun evaluate if context still looks stale

CLI check path:

```bash
cargo run -p vel-cli -- sync todoist
cargo run -p vel-cli -- evaluate
```

## Data handling notes

The Todoist API token is stored in dedicated local secret settings records rather than being exposed in public integration payloads.

Operationally:

- treat the local machine and database as sensitive,
- verify sync freshness from status and history,
- do not assume saved token means current data without running sync.

## Write-back and conflict review

The allowed Todoist write surface is intentionally small:

- create a new Todoist task from typed local fields,
- update an existing Todoist-backed commitment,
- complete an existing Todoist-backed commitment,
- reopen an existing Todoist-backed commitment.

These writes do not silently force last-write-wins behavior. Before `todoist_update_task`, `todoist_complete_task`, or `todoist_reopen_task`, Vel fetches the latest upstream task and compares it with the last synced snapshot stored in local upstream refs.

If the upstream task drifted, Vel opens a conflict review item instead of overwriting it:

- `stale_write` when the upstream `updated_at` no longer matches the last synced snapshot,
- `upstream_vs_local` when the upstream task payload no longer matches the stored local view.

Todoist labels remain compatibility-only metadata at the adapter boundary. The durable Vel-side contract is the typed field set `project_id`, `scheduled_for`, `priority`, `waiting_on`, and `review_state`.

The read path stays separate from the write path:

- `POST /v1/sync/todoist` remains the read/sync surface,
- write-back uses the bounded operator routes under `/api/integrations/todoist/*`.
