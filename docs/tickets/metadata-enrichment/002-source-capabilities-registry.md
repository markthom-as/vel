---
id: VEL-META-002
title: Source capabilities registry for metadata read/write/revert support
status: proposed
priority: P0
estimate: 1-2 days
dependencies: [VEL-META-001]
---

# Goal

Prevent invalid enrichment actions by making every integration declare field-level capabilities.

# Scope

- Create a capability registry interface.
- Implement capability declarations for initial sources:
  - Todoist
  - Google Calendar
  - Gmail
  - Drive/docs/files as read-only if needed initially
- Support per-field flags:
  - readable
  - writable
  - bulk writable
  - revertible
  - approval hints

# Deliverables

- `vel-integrations/src/capabilities/mod.rs`
- source-specific capability declarations
- registry lookup helpers
- tests that reject unsupported actions

# Acceptance criteria

- Every candidate/application path must consult registry support.
- Unsupported write attempts fail deterministically.
- Capability data is available to API/UI.

# Notes

This is anti-delusion infrastructure. Keep it crisp.
