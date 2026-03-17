# Google Calendar Integration

This guide covers the current shipped credential-backed Google Calendar path.

## What is implemented now

Google Calendar is handled through the web Settings integrations surface with:

- local save of Google client credentials,
- a `Connect Google` OAuth flow,
- `Sync now`,
- `Disconnect`,
- sync history and guidance,
- calendar selection controls.

For shipped-truth authority, see [status.md](../../status.md).

## Setup flow

In the web Settings `Integrations` tab:

1. Enter `Client ID` and `Client secret`.
2. Click `Save credentials`.
3. Click `Connect Google`.
4. Complete the OAuth flow.
5. Click `Sync now`.

When connected, the integration card shows status and last sync metadata.

## Calendar selection behavior

Current behavior in Settings:

- `Sync all calendars by default` is enabled by default.
- If you disable that toggle, you can choose individual calendars.
- If no calendars are loaded yet, Settings will tell you to connect Google and run sync.

This is a sync-scoping control, not a separate integration mode.

## What "working" looks like

Usually healthy state means:

- integration badge is connected,
- `last_sync_status` is successful,
- `last_sync_at` updates after sync,
- `last_item_count` is non-zero when events exist,
- calendar-derived schedule data appears in `Now` and context surfaces.

## Troubleshooting

If Google Calendar is stale, disconnected, or failing:

1. check guidance and status in Settings
2. retry `Sync now`
3. reconnect if disconnected
4. rerun evaluate if context still looks stale

CLI check path:

```bash
cargo run -p vel-cli -- sync calendar
cargo run -p vel-cli -- evaluate
```

## Data handling notes

Sensitive Google fields are stored in dedicated local secret settings records instead of public integration payloads.

Operationally:

- treat the local machine and database as sensitive,
- use Settings status/history to verify connection state,
- do not assume connected means fresh without checking last sync status/time.
