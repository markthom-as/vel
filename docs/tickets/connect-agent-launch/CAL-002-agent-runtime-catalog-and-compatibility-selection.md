---
id: CAL-002
title: Agent runtime catalog and compatibility selection
status: todo
priority: P0
dependencies:
  - CAL-001
---

# Goal

Model external agent runtimes as a catalog with capability metadata and provide compatibility filtering between runtime requirements and Connect-instance manifests.

# Scope

- add runtime catalog types and loader
- support runtimes such as `codex`, `copilot_agent`, `cursor_agent`, `claude_code`, `opencode`, `gemini_cli`
- define runtime capability fields
- implement selection/filtering logic for compatible instance/runtime pairs

# Deliverables

- runtime catalog model
- compatibility matcher/scorer
- API surface for listing runtimes or compatibility options
- fixtures covering mixed support across multiple instances

# Acceptance criteria

- Runtime availability is driven by catalog + instance capability data rather than brand-specific UI conditionals.
- Vel can answer "which runtimes can this instance launch?" and "which instances can run this runtime?"
- The catalog is extensible without schema churn.

# Notes

This ticket is the bridge between raw capability manifests and launch UX.
