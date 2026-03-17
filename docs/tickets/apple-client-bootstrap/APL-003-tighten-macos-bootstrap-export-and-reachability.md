---
id: APL-003
title: Tighten macOS bootstrap, export, and reachability
status: todo
priority: P1
write_scope:
  - clients/apple/Apps/VelMac/VelMacApp.swift
  - clients/apple/Apps/VelMac/ContentView.swift
  - clients/apple/README.md
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Make `VelMac` a cleaner bootstrap/export bridge with explicit reachability and local-export behavior.

# Acceptance criteria

1. bootstrap and reachability flow are explicit
2. local export responsibilities are legible
3. the app remains a client/export bridge rather than a second brain
