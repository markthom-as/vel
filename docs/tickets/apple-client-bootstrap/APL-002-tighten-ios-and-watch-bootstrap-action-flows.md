---
id: APL-002
title: Tighten iOS and watch bootstrap and action flows
status: todo
priority: P1
write_scope:
  - clients/apple/Apps/VeliOS/VelApp.swift
  - clients/apple/Apps/VeliOS/ContentView.swift
  - clients/apple/Apps/VelWatch/VelWatchApp.swift
  - clients/apple/Apps/VelWatch/ContentView.swift
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Make iOS and watch surfaces cleaner consumers of shared bootstrap/offline/action semantics.

# Acceptance criteria

1. bootstrap and queued-action flows are explicit
2. stale/disconnected behavior is honest
3. app code does not duplicate shared client logic
