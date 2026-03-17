# Todoist Integration

This guide covers the current shipped credential-backed Todoist path.

## What is implemented now

Todoist is handled through the web Settings integrations surface with:

- local save of a Todoist API token,
- `Sync now`,
- `Disconnect`,
- sync history and guidance.

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
