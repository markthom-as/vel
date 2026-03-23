# Phase 71 Browser Proof — No Silent Fallback

## Command

`npm run proof:phase71:system-no-fallback`

## What Was Tested

Forced the canonical inspect route to fail in a real browser and verified that System surfaces an explicit error state without falling back to guessed or stale structural content.

## Expected Canonical Behavior

Canonical route failure should block the affected surface, render explicit error UI, and avoid stale or inferred fallback controls.

## Observed Result

The browser rendered `API 500: canonical inspect failed`, and no System integration controls or structural cards silently rendered underneath that error state.

## Deviation

None.
