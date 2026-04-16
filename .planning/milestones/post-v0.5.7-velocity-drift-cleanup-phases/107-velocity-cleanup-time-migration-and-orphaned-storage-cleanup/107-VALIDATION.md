# Phase 107 Validation

## Required Checks

### Temporal Migration

- `rg -n 'chrono|chrono_tz' crates/veld Cargo.toml`
- targeted recurrence/availability tests after each migration step
- at least one wider compile/test pass after removing the old dependencies
- first slice check: `now.rs` should have no direct `chrono` import or `timezone.tz()` call; local display behavior should be covered by `timezone` tests

### Schema Re-Verification

- prove `storage_targets` is active through code and backup docs
- decide whether `verification_records` supports backup verification/trust direction
- decide whether `vel_self_metrics` supports self-awareness/reflective-tuning direction
- only generate a drop migration for tables that fail both live-code and Master Plan ownership checks
- Phase 107 first slice decision: no drop migration. `storage_targets` is active; `verification_records` and `vel_self_metrics` are planned foundation schema and remain kept.

### Automated

- `cargo test -p veld phase64_recurrence_and_availability -- --nocapture`
- `cargo test -p veld phase62_recurrence_and_attendees -- --nocapture`
- `cargo test -p veld phase62_availability -- --nocapture`
- `cargo test -p veld phase64_gcal_black_box -- --nocapture`
- `cargo test -p veld backup_flow -- --nocapture`
- `cargo check --workspace --all-targets`

### Documentation Truth

- if a flagged table is preserved, add or update the owning planning/doc reference
- if a table is dropped, document why it is off-plan, not just unused today

## Failure Conditions

- removing a schema object that still supports backup-first trust or self-awareness direction
- finishing the migration but leaving mixed time libraries in the targeted `veld` seams
- preserving ambiguous schema without recording who owns it and why it remains
