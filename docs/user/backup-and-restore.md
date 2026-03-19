---
title: Backup And Restore
doc_type: user
status: draft
owner: vel-user-docs
created: 2026-03-19
updated: 2026-03-19
keywords:
  - backup
  - restore
  - operator
  - trust
summary: Operator guidance for inspectable Vel backup packs and manual-first recovery posture.
---

# Backup And Restore

Vel treats backup as a trust workflow first and a recovery workflow second.

The shipped posture for Phase 09 is:

- keep the backup pack typed and inspectable
- make omissions explicit
- treat verification as first-class evidence
- keep restore manual-first instead of hiding it behind a broad automatic recovery engine

## What A Backup Pack Must Explain

A trustworthy backup pack should always answer these questions:

1. what is the backup ID
2. when was it created
3. where is the pack rooted on disk
4. which SQLite snapshot is being trusted
5. which artifacts were included
6. which config files were included
7. which items were intentionally omitted
8. whether secret-bearing settings were excluded
9. how verification was performed
10. what checksum was recorded

If the pack does not answer those questions clearly, it should not be treated as trustworthy yet.

## What Is Intentionally Omitted

Vel keeps backup packs small enough to inspect and safe enough to handle.

By design, a backup pack should omit:

- secret settings records
- integration tokens and API credentials
- local private key material
- transient cache files
- temporary files and scratch directories

Those omissions should be explicit in the manifest. Silence is not enough.

## Manual Restore First

Vel does not center backup around automatic restore.

The operator posture is:

1. inspect the manifest
2. confirm the checksum and verification summary
3. review the coverage and omission lists
4. stop the daemon before touching the live database
5. restore the SQLite snapshot and artifact roots manually
6. re-check the restored pack before trusting it for daily use

That manual-first posture is deliberate. It keeps the operator in control and avoids pretending recovery is simpler than it really is.

## Current Operator Guidance

Phase 09 now ships a bounded backup workflow:

- `vel backup --create [--output-root <dir>]`
- `vel backup --inspect <backup_root>`
- `vel backup --verify <backup_root>`
- `vel backup --dry-run-restore <backup_root>`

### Create A Backup Pack

```bash
vel backup --create
vel backup --create --output-root ~/vel-backups
```

The create flow asks the backend to:

1. create a SQLite-safe snapshot
2. copy only durable artifact/config files
3. record explicit omissions
4. write `manifest.json`
5. persist the last-success backup status for doctor/settings/web surfaces

### Backup Inspect And Verify

Use inspect before trusting a pack:

```bash
vel backup --inspect ~/vel-backups/<backup_id>
```

Use verify before trusting a pack for recovery:

```bash
vel backup --verify ~/vel-backups/<backup_id>
```

These commands surface:

- backup ID and created timestamp
- output root and database snapshot path
- artifact/config coverage counts
- omission lists
- verification checksum state

### Manual Restore

Use the dry-run helper first:

```bash
vel backup --dry-run-restore ~/vel-backups/<backup_id>
```

That command is intentionally non-destructive. It prints the manual restore sequence without mutating the live environment.

The real manual restore sequence is:

1. stop `veld`
2. confirm the backup root matches the manifest `output_root`
3. copy `data/vel.sqlite` back to the live database path
4. copy `artifacts/` back to the live artifact root if you want those durable artifacts restored
5. rehydrate omitted secrets from their original local sources
6. run `vel backup --verify <backup_root>` before trusting the restored environment

## Restore Caution

Do not assume that an unverified copy is a safe restore target.

Before trusting a restored environment, confirm:

- the database snapshot opens cleanly
- the artifact root exists and matches the manifest
- omitted secret material is rehydrated from the correct local source
- the operator can explain any missing path before the backup is considered usable

If you need automatic restore behavior later, treat that as a separate phase and a separate contract.
