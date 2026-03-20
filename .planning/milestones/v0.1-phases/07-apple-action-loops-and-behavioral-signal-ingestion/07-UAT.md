---
status: complete
phase: 07-apple-action-loops-and-behavioral-signal-ingestion
source:
  - 07-01-SUMMARY.md
  - 07-02-SUMMARY.md
  - 07-03-SUMMARY.md
  - 07-04-SUMMARY.md
started: 2026-03-19T07:38:17Z
updated: 2026-03-19T07:39:20Z
---

## Current Test

[testing complete]

## Tests

### 1. Cold Start Smoke Test
expected: Start the current app/client flow from a fresh state for this phase. The backend should boot cleanly, Apple client transport should connect without schema/auth surprises, and a basic read such as the current schedule or health summary should load from live backend data rather than a Swift-local synthesized answer.
result: skipped
reason: user skipped all manual UAT tests

### 2. iPhone Voice Turn Uses Backend Truth
expected: On iPhone, submitting a supported voice request should persist the transcript, return a typed backend response, and answer from backend state rather than from a local Swift-composed reply.
result: skipped
reason: user skipped all manual UAT tests

### 3. Schedule Retrieval Uses /v1/now
expected: Current schedule shown on Apple surfaces should match backend Now data. It should come from the typed /v1/now path or Apple backend payloads, not from older local query synthesis.
result: skipped
reason: user skipped all manual UAT tests

### 4. Nudge Response Stays on Safe Queue Path
expected: Completing or snoozing a nudge from Apple should either apply immediately through the backend or queue a safe offline action for later replay. It should not invent a separate Apple-only mutation path.
result: skipped
reason: user skipped all manual UAT tests

### 5. Apple Behavior Summary Is Bounded and Explainable
expected: The Apple behavior view should show only steps, stand, and exercise-style summaries, with backend-provided freshness or reasons. It should not widen into unrelated health interpretation.
result: skipped
reason: user skipped all manual UAT tests

### 6. Watch Quick Loop Renders Backend-Owned State
expected: Watch quick-loop UI should render cached or live backend-owned schedule, nudge, and behavior summary data, while offline fallback stays limited to cached rendering and queued safe actions.
result: skipped
reason: user skipped all manual UAT tests

## Summary

total: 6
passed: 0
issues: 0
pending: 0
skipped: 6

## Gaps

[]
