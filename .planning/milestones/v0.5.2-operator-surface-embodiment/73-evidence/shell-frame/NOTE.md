# Phase 73 Browser Proof — Shared Shell Frame

## Command

`npm run proof:phase73:shell-frame`

## What Was Tested

Loaded the app in a real browser, verified the three-surface nav frame, confirmed the global info rail toggle is absent, and navigated across Threads and System using the shared shell.

## Expected Canonical Behavior

The shell should expose only Now, Threads, and System, keep icon-plus-label nav, and remove the old global info rail from the shared frame.

## Observed Result

The browser rendered the three-surface nav, exposed no global info toggle, and preserved the shared frame while navigating into Threads and System.

## Deviation

None.
