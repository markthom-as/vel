---
id: APL-001
title: Split VelAPI shared contracts, client, and store
status: todo
priority: P0
write_scope:
  - clients/apple/VelAPI/Sources/VelAPI/Models.swift
  - clients/apple/VelAPI/Sources/VelAPI/VelClient.swift
  - clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift
  - clients/apple/VelAPI/Sources/VelAPI/VelLocalSourceExporter.swift
  - clients/apple/VelAPI/Sources/VelAPI/VelDocumentation.swift
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Turn `VelAPI` into a family-oriented shared client contract instead of a small cluster of multi-purpose files.

# Acceptance criteria

1. bootstrap, sync, models, offline state, and local export have clear ownership
2. Apple shared models stay aligned with the same daemon/client contract family consumed by web
