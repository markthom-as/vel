# Phase 89 Browser Proof — Now Degraded

## Command

`npm --prefix clients/web run proof:phase89:ui-proof`

## What Was Tested

Loaded degraded trust state on `Now` and verified the bounded trust intervention path.

## Expected Canonical Behavior

`Now` should surface trust only when degraded and offer escalation into `System` without widening the page.

## Observed Result

The browser rendered the degraded trust card with `Open system detail` while keeping the rest of `Now` bounded.

## Deviation

None.
