# Phase 89 Browser Proof — Threads Focused Block

## Command

`npm --prefix clients/web run proof:phase89:ui-proof`

## What Was Tested

Opened the focused provenance block from the thread continuity stream.

## Expected Canonical Behavior

Focused review should expand as a bounded detail surface without turning the thread into a raw trace dump.

## Observed Result

The browser opened the `Provenance` drawer from `Show why`, exposing message evidence and structured summaries while keeping the thread intact underneath.

## Deviation

None.
