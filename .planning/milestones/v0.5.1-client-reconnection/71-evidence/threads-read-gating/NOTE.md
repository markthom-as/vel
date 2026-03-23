# Phase 71 Browser Proof — Threads Canonical Read And Invocation Gating

## Command

`npm run proof:phase71:threads-read-gating`

## What Was Tested

Loaded the shipped Threads surface in a real browser and verified that continuation context renders while workflow invocation stays gated when no bound canonical object is available.

## Expected Canonical Behavior

Threads should show canonical conversation truth, surface the attach-or-create guidance, and refuse to invent floating workflow execution controls.

## Observed Result

The browser rendered `Proposal thread`, displayed the explicit attach/create-object guidance, and exposed no workflow execution controls.

## Deviation

None.
