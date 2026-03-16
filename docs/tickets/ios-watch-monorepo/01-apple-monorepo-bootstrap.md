---
id: APPLE-001
title: Create Apple subtree and workspace inside monorepo
status: proposed
owner: agent
priority: p0
area: clients/apple
depends_on: [APPLE-EPIC-000]
---

# Goal

Create the baseline Apple structure within the Vel monorepo and make it runnable by an agent with one or two obvious commands.

# Requirements

- Add `clients/apple/VelApple.xcworkspace`
- Add repo-level `clients/apple/README.md`
- Add `clients/apple/Package.swift`
- Add directories:
  - `Apps/VeliOS`
  - `Apps/VelWatch`
  - `Apps/VelWidgets`
  - `Packages/VelAppleCore`
  - `Packages/VelAppleUI`
  - `Packages/VelAppleSync`
  - `Packages/VelAppleNotifications`
  - `Packages/VelAppleVoice`
  - `Tests/ContractTests`
- Add a local `AGENTS.md` describing:
  - architectural boundaries
  - how to run builds/tests
  - no direct ad hoc model drift from backend contracts

# Implementation notes

Use a **workspace + Swift package hybrid**:

- app targets live under `Apps/*`
- most actual logic lives in local Swift packages
- platform-specific code stays thin
- shared models never get redefined independently in each target

# Suggested commands

Provide deterministic commands in repo scripts, for example:

```bash
just apple-build
just apple-test
just apple-open
```

or equivalent `make` / `task` targets already used by Vel.

# Acceptance criteria

- Fresh clone agent can discover Apple subtree without guesswork
- `README.md` explains exact build/test entrypoints
- workspace opens and builds on a configured macOS host
- packages are referenced via local paths, not manual Xcode spaghetti
