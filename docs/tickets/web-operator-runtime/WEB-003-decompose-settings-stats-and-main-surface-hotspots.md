---
id: WEB-003
title: Decompose settings, stats, and main surface hotspots
status: todo
priority: P1
write_scope:
  - clients/web/src/App.tsx
  - clients/web/src/components/AppShell.tsx
  - clients/web/src/components/MainPanel.tsx
  - clients/web/src/components/Sidebar.tsx
  - clients/web/src/components/ContextPanel.tsx
  - clients/web/src/components/SettingsPage.tsx
  - clients/web/src/components/StatsView.tsx
  - clients/web/src/components/SurfaceState.tsx
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Reduce the largest web component hotspots while preserving shared UX semantics for freshness, degraded state, and operator controls.

# Acceptance criteria

1. settings/stats/shell ownership is clearer
2. sync-health and degraded-state semantics have reusable UI homes
3. unrelated surface work stops colliding in the same files
