---
id: NOW-003
status: proposed
title: Fix broken mode fallback and define sane time-of-day policy
owner: backend
priority: P0
---

## Goal

Stop mode from collapsing into `morning_mode` by default.

## Why

Current inference falls through to `morning_mode` in the final branch, which is plainly wrong for large parts of the day.

## Files likely touched

- `crates/veld/src/services/inference.rs`
- tests in `crates/veld/src/app.rs` and/or service tests

## Requirements

1. Replace the current mode logic with an explicit policy.
2. Minimum modes for this ticket:
   - `meeting_mode`
   - `commute_mode`
   - `morning_mode`
   - `day_mode`
3. Suggested selection order:
   - if prep window active -> `meeting_mode`
   - else if commute window active -> `commute_mode`
   - else if local time is morning and morning-start tasks unresolved -> `morning_mode`
   - else -> `day_mode`
4. Do not introduce evening/night unless needed.
5. Keep raw enum values stable if other code depends on them; just make them semantically correct.

## Tests

Cover at least:

- 6 AM local with meds pending -> `morning_mode`
- 6 PM local with no prep/commute window -> not `morning_mode`
- prep window active -> `meeting_mode`
- commute window active -> `commute_mode`

## Acceptance criteria

- The Now page does not show `morning_mode` in the evening absent an explicit rule.
- Tests pin the previous regression.
