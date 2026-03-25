# Phase 107 Verification

## Goal-Backwards Questions

1. Can a reviewer see that the `chrono` to `time` migration is actually complete in the targeted runtime seams, not just partially reduced?
2. Can a reviewer explain why each flagged table was kept or dropped using live code plus Master Plan evidence?
3. Is backup/trust infrastructure more coherent after this phase, not less?
4. Did the phase avoid turning “unused today” into accidental deletion of future-directed substrate?

## Evidence To Collect

- before/after search output for `chrono` and `chrono_tz`
- before/after search output for the flagged table names
- passing command output for the targeted recurrence/availability/backup tests
- a short classification ledger for:
  - `storage_targets`
  - `verification_records`
  - `vel_self_metrics`

## Review Traps

- assuming the audit is authoritative over newer code and planning evidence
- deleting schema because there is no repository today even though the product direction still needs it
- leaving the migration half-finished because one timezone edge case was hard
