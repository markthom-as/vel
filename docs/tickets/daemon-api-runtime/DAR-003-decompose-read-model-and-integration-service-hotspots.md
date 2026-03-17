---
id: DAR-003
title: Decompose read-model and integration service hotspots
status: todo
priority: P1
write_scope:
  - crates/veld/src/services/inference.rs
  - crates/veld/src/services/now.rs
  - crates/veld/src/services/explain.rs
  - crates/veld/src/services/risk.rs
  - crates/veld/src/services/nudge_engine.rs
  - crates/veld/src/services/integrations.rs
  - crates/veld/src/services/integrations_google.rs
  - crates/veld/src/services/integrations_todoist.rs
  - crates/veld/src/services/integrations_host.rs
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Split major service hotspots so shared read-model logic and integration runtime logic stop colliding.

# Acceptance criteria

1. shared read-model shaping is explicit
2. integration runtime ownership is explicit
3. repeated shaping work moves toward one daemon-side home
