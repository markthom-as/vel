# Phase 71 Browser Proof — Now Canonical Read

## Command

`npm run proof:phase71:now-read`

## What Was Tested

Loaded the shipped app in a real browser, stayed on the default Now surface, and verified the canonical task and calendar sections render without reviving Inbox-era affordances.

## Expected Canonical Behavior

Now should render adjacent canonical Tasks and Calendar sections, show current task/event truth, and avoid synthetic Inbox or local reschedule controls.

## Observed Result

The browser rendered `Jove's Now`, separate `Tasks` and `Calendar` headings, the active task `Write weekly review`, and the upcoming event `Design review`. No Inbox or reschedule affordances rendered.

## Deviation

None.
