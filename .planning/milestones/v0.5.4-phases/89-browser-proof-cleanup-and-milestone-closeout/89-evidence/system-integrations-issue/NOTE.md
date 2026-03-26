# Phase 89 Browser Proof — System Integrations Issue

## Command

`npm --prefix clients/web run proof:phase89:ui-proof`

## What Was Tested

Escalated from degraded `Now` trust state into `System > Integrations` with a stale provider token.

## Expected Canonical Behavior

`System` should show a browse/detail integration issue view where provider identity stays subordinate to degraded trust state.

## Observed Result

The browser opened `System > Providers` with a degraded Google Calendar detail pane, named actions, and browse/detail structure.

## Deviation

None.
