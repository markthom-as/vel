---
phase: 64-google-calendar-multi-account-adapter-and-canonical-calendar-cut-in
plan: 01
work_id: 0.5.64.1
title: Google Calendar multi-account linking and bounded canonical import
status: completed
completed_at: 2026-03-23
---

# 64-01 Summary

## What Landed

- Added multi-account Google Calendar account linking in `crates/vel-adapters-google-calendar/src/account_linking.rs`
- Added deterministic Google provider/account/calendar/event/linkage identifiers in `crates/vel-adapters-google-calendar/src/google_ids.rs`
- Added bounded-window canonical import in `crates/vel-adapters-google-calendar/src/windowed_import.rs`
- Added multi-account and window/idempotence proof coverage in `crates/veld/tests/phase64_gcal_accounts.rs`
- Wired the new adapter surfaces through `crates/vel-adapters-google-calendar/src/lib.rs`

## Proof Coverage

- Google Calendar account identity now enters through canonical `IntegrationAccount`, not a bespoke adapter account path
- bounded import defaults to 90 past / 365 future days and preserves explicit window expansion posture
- canonical `calendar` and `event` objects plus `SyncLink` records are created through the same substrate used by the Todoist adapter
- one account with multiple calendars, multiple accounts with overlapping remote ids, bounded-window skipping, and re-import idempotence are all proved

## Verification

- `rg -n "IntegrationAccount|account|link|remote_id|provider|SyncLink|90|365|window|bounded" crates/vel-adapters-google-calendar/src/account_linking.rs crates/vel-adapters-google-calendar/src/google_ids.rs crates/vel-adapters-google-calendar/src/windowed_import.rs crates/veld/tests/phase64_gcal_accounts.rs`
- `cargo test -p vel-adapters-google-calendar --lib`
- `cargo test -p veld --test phase64_gcal_accounts`
- `cargo check -p vel-adapters-google-calendar && cargo check -p veld`

## Outcome

Phase 64 now starts from the correct constitutional lane: Google Calendar proves multi-account identity and bounded canonical import over `IntegrationAccount` and `SyncLink` law instead of introducing a provider-specific account or sync system.
