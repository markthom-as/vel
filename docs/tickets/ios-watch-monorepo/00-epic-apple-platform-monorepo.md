---
id: APPLE-EPIC-000
title: Apple platform surfaces inside Vel monorepo
status: proposed
owner: agent
priority: p0
area: clients/apple
depends_on: []
---

# Summary

Implement iOS, watchOS, widgets, and notification surfaces **inside the existing Vel monorepo** under `clients/apple`, using shared contracts from core/domain and core/sync rather than a separate repository.

# Why

Monorepo better fits Vel's actual dependency graph:

- reminder engine, risk engine, sync, voice, and UI surfaces are one system
- agentic refactors are much easier when server, schemas, docs, tests, and Apple clients co-reside
- architectural boundaries can be enforced through packages and dependency rules instead of repo splits

# Scope

Includes:

- Apple subtree layout
- shared Swift package(s)
- iOS app shell
- watch app shell
- widgets / complications
- notifications and quick actions
- sync/event-log integration
- voice capture entrypoint
- contract/integration testing
- repo tasks and docs

Excludes for now:

- App Store release automation
- HealthKit / clinical integrations
- SiriKit/App Intents beyond minimal capture hooks
- fancy polish beyond functional MVP

# Target layout

```text
vel/
  clients/
    apple/
      VelApple.xcworkspace
      Package.swift
      Apps/
        VeliOS/
        VelWatch/
        VelWidgets/
      Packages/
        VelAppleCore/
        VelAppleUI/
        VelAppleSync/
        VelAppleNotifications/
        VelAppleVoice/
      Tests/
        ContractTests/
        SnapshotTests/
```

# Deliverable definition

Done when:

- code compiles for iOS + watchOS
- shared package compiles independently
- at least one end-to-end flow works: reminder due -> notification -> quick action -> event sync -> reflected in app/watch state
- tickets below are completed
