---
id: APPLE-012
title: Add repo tasks CI hooks and local agent instructions for Apple subtree
status: proposed
owner: agent
priority: p1
area: tooling
depends_on: [APPLE-001, APPLE-011]
---

# Goal

Make the Apple subtree legible to future agents and brutalize the amount of tacit setup knowledge required.

# Requirements

Add:

- `clients/apple/AGENTS.md`
- build/test/lint commands in repo task runner
- CI job definitions or placeholders
- docs for fixture mode and simulator test paths
- architectural dependency rules

# AGENTS.md should include

- directory map
- module responsibilities
- allowed dependency directions
- how to add a new feature without model drift
- how to update contracts when backend changes
- where to put fixtures
- what not to do:
  - no direct network code in widgets unless approved
  - no duplicate domain enums in UI layers
  - no business policy in SwiftUI views

# Acceptance criteria

- a fresh coding agent can discover commands and structure quickly
- CI can at least validate packages/tests on configured runners
- docs reduce the chance of feral Xcode edits
