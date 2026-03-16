---
id: TKT-001
status: proposed
title: Bootstrap Vel Apple platform workspace and module boundaries
priority: P0
estimate: 2-3 days
depends_on: []
owner: agent
---

## Goal

Create the initial Apple workspace with clean package boundaries so iOS/watchOS do not congeal into a single sticky blob of SwiftUI and side effects.

## Why

Without hard boundaries now, the app will drift into “UI as architecture,” which is a polite way of saying spaghetti with SF Symbols on top.

## Scope

- Create a new repo/workspace: `vel-apple`
- Add iOS app target and watchOS companion target
- Add Swift Packages:
  - `VelAppCore`
  - `VelSharedModels`
  - `VelSync`
  - `VelNotifications`
  - `VelWidgets`
- Establish environment/config management for dev/staging/prod
- Add CI for:
  - build
  - tests
  - SwiftLint or equivalent formatter/lint pass
- Define minimum OS versions
- Add dependency injection container pattern for clients/services

## Implementation notes

- Use SwiftUI + Observation / Swift Concurrency
- Keep reducers/view models in `VelAppCore`, not in view files
- Treat `VelSharedModels` as generated-or-mirrored DTOs from the Vel API schema
- Prefer protocol-driven service boundaries with concrete implementations injected at composition root
- Keep watch app intentionally thin; it should not grow a second brain

## Deliverables

- Compiling workspace
- README with local dev steps
- Basic CI pipeline
- Module dependency graph documented in `/docs/architecture/apple-modules.md`

## Acceptance criteria

- `xcodebuild` succeeds for iOS and watchOS targets
- Package boundaries prevent UI targets from importing networking details directly except through app core APIs
- A new engineer can run the app locally using documented steps
