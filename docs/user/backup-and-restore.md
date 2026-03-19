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

At the time this contract was published, `vel backup` is still a guidance surface rather than a full export engine. Use it as a reminder of the local database and artifact roots that matter, and use this document as the trust model for any later snapshot/export implementation.

### Backup Inspect And Verify

- `backup inspect`: read the manifest, coverage, and omission lists before trusting the pack.
- `backup verify`: confirm the checksum and verification summary before treating the pack as usable.
- `manual restore`: stop the daemon, restore the snapshot and artifacts, then re-check the pack before daily use.

## Restore Caution

Do not assume that an unverified copy is a safe restore target.

Before trusting a restored environment, confirm:

- the database snapshot opens cleanly
- the artifact root exists and matches the manifest
- omitted secret material is rehydrated from the correct local source
- the operator can explain any missing path before the backup is considered usable

If you need automatic restore behavior later, treat that as a separate phase and a separate contract.
