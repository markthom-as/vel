---
title: Comprehensive User Documentation & Support Wiki
status: planned
owner: staff-eng
type: documentation
priority: low
created: 2026-03-17
labels:
  - docs
  - user-experience
  - support
---

Design and implement a structured "User Handbook" and Support Wiki to support the end-user launch of Vel v0 and v1.

## Technical Details
- **User Handbook**: A clear, instructional guide covering:
  - **Capture Strategies**: Voice, CLI, and Web capture.
  - **Goal Alignment**: Using commitments and project containers.
  - **Daily Review**: How to use the Morning Brief and End-of-Day summary.
- **Accessibility**: Docs should use direct language, clear reading order, and support operator tasks without assuming visual UI discovery only.
- **Configurability**: Docs should explain defaults, effective-config inspection, and where behavior changes by platform or integration state.
- **Replayability**: Troubleshooting guidance should include inspect, retry, and replay-friendly recovery paths where applicable.
- **Support Wiki**: A community-editable repository for:
  - **Troubleshooting**: Common sync/LLM-inference errors.
  - **Integrations**: Setting up Todoist, Google Calendar, and Custom Adapters.
- **Support-Level Guidance**: In-app/CLI guidance that points to the relevant handbook section.

## Acceptance Criteria
- A non-technical user can successfully set up and use Vel's core "Critical Five" features based on the handbook.
- Common troubleshooting scenarios (e.g., "Why didn't my Git signal sync?") are covered in the wiki.
- The documentation is versioned and matches the shipped v0.x code.
- User docs explain accessibility, configuration, and inspection/recovery paths where they materially affect operation.
