# Phase 71 Browser Proof — Controlled Degraded State

## Command

`npm run proof:phase71:system-degraded`

## What Was Tested

Injected a controlled degraded canonical inspect response in a real browser and verified that System renders an explicit degraded state rather than silently using stale structural data.

## Expected Canonical Behavior

A degraded canonical response should fail loudly in development-mode proof runs and render explicit degraded UI instead of stale canonical content.

## Observed Result

The browser rendered the explicit degraded response message for `/v1/agent/inspect`, and canonical detail content such as `Avery` did not render.

## Deviation

None.
