# Phase 75 Browser Proof — Threads State-First Read

## Command

`npm run proof:phase75:threads-read`

## What Was Tested

Loaded the shipped Threads surface in a real browser and verified that a bound thread now foregrounds canonical object state and compact provenance cues before transcript chronology.

## Expected Canonical Behavior

Threads should present the bound object, object-state context, and invocation gating beside the transcript, while avoiding the older attach/create fallback for already-bound threads.

## Observed Result

The browser rendered `Proposal thread` with `Bound object`, `Object state`, and the `proposal review gated` capability state before the `Conversation` transcript section, and it did not show the attach/create fallback guidance.

## Deviation

None.
