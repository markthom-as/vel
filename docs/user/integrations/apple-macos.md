# Apple And macOS Local Sources

This guide explains the current shipped Apple-linked path in Vel.

Current reality:

- Apple integration is bootstrap-level
- the Rust runtime does not directly own HealthKit or Messages framework access
- the current path is local snapshot export plus ingestion by `veld`

That boundary is intentional and matches the repo’s Rust/Swift split.

## What currently exists

Today, `VelMac` can write local snapshots for:

- activity
- health
- messaging
- reminders

`veld` on macOS can auto-discover local files for:

- activity
- health
- git
- messaging
- reminders
- notes
- transcripts

## Auto-discovery path

The current macOS auto-discovery root is:

- `~/Library/Application Support/Vel/`

Expected current locations:

- `activity/snapshot.json`
- `health/snapshot.json`
- `git/snapshot.json`
- `messages/snapshot.json`
- `reminders/snapshot.json`
- `notes/`
- `transcripts/snapshot.json`

When these files exist, `veld` can bootstrap them on startup so they influence current context immediately.

## What VelMac exports

Current exported behavior:

- activity heartbeat from the running app
- health summary snapshot when HealthKit is available and authorized
- recent message-thread snapshot from the local Messages database
- reminder snapshot from EventKit when Reminders access is granted

If the daemon is reachable, `VelMac` can also ask `veld` to sync those sources after export.

## Required host permissions

This is the main operational caveat.

### Health

Health export depends on:

- HealthKit availability on the host
- authorization being granted

Current exported metrics include:

- step count
- active energy burned
- sleep hours
- heart rate
- stand hours
- blood pressure (systolic/diastolic)

If authorization is denied, the health snapshot may not be written.

### Messages

Messages export depends on:

- local access to `~/Library/Messages/chat.db`

Depending on the machine and policy, this may require:

- Full Disk Access

If the process cannot read the Messages database, the messaging snapshot may not be written.

### Reminders

Reminders export depends on:

- EventKit availability on the host
- Reminders authorization being granted

## How to verify it is working

1. confirm the snapshot files exist in `~/Library/Application Support/Vel/`
2. restart `veld` or run manual sync commands
3. inspect `Now`, current context, or integration freshness state

Useful commands:

```bash
cargo run -p vel-cli -- sync activity
cargo run -p vel-cli -- sync health
cargo run -p vel-cli -- sync messaging
cargo run -p vel-cli -- sync reminders
cargo run -p vel-cli -- evaluate
```

## What this is not

This is not yet:

- a full background Apple sync system
- a complete Apple product surface
- direct framework ingestion inside the Rust daemon

It is a practical local-first bridge that gets Apple-local data into Vel now without breaking the current architecture.
