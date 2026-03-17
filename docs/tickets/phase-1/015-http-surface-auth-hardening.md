---
title: Auth-By-Default HTTP Surfaces & Deny-By-Default Routing
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 011-documentation-truth-repair
labels:
  - veld
  - api
  - security
  - phase-1
---

# Context & Objectives

Vel currently has a growing HTTP surface, but the queue does not yet treat route exposure and auth posture as a first-class architecture concern.

This ticket formalizes route classes, auth defaults, and deny-by-default behavior so new surfaces do not accidentally ship with broader access than intended.

# Impacted Files & Symbols

- **File**: `crates/veld/src/app.rs`
  - **Symbols**: `build_app_with_state`
- **Directory**: `crates/veld/src/routes/`
  - **Symbols**: auth extractors, route handlers, fallback handlers
- **File**: `docs/api/runtime.md`
  - **Symbols**: route documentation and surface classification
- **File**: `docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md`
  - **Symbols**: trust domains, capability boundaries

# Technical Requirements

- **Route Classification**: Define explicit route classes such as `local_public`, `operator_authenticated`, `worker_authenticated`, and `future_external`.
- **Default Posture**: New routes should require auth or explicit classification by default.
- **Deny By Default**: Unsupported routes and unsupported action patterns should reject safely instead of flowing through accidental fallbacks.
- **Auth Boundary**: Centralize auth extraction and route policy mapping instead of duplicating ad hoc checks in handlers.
- **Docs Sync**: Document which surfaces are intentionally public and why.

# Implementation Steps (The "How")

1. **Inventory**: Enumerate current routes and classify their intended exposure.
2. **Boundary Design**: Introduce explicit auth or policy layers for route groups.
3. **Fallback Hardening**: Ensure undefined or unsupported routes fail closed.
4. **Docs**: Update API and architecture docs to match the implemented exposure model.

# Acceptance Criteria

1. [ ] Every mounted HTTP route belongs to an explicit exposure class.
2. [ ] New route additions require an explicit auth/exposure choice in code review.
3. [ ] Undefined or unsupported routes/actions reject safely by default.
4. [ ] API docs identify intentional public surfaces and authenticated surfaces.

# Verification & Regression

- **Unit Test**: auth extractor and route policy tests under `crates/veld/src/routes/`
- **Integration Test**: request matrix covering public, authenticated, and denied routes
- **Smoke Check**: `curl` requests for allowed and denied paths
- **Invariants**: no sensitive route becomes public without an explicit code and docs change

# Agent Guardrails

- **No Blanket Publicing**: Do not mark routes public to avoid plumbing auth.
- **Docs Matter**: Route exposure changes must update nearby docs in the same patch.
- **Fail Closed**: If route intent is unclear, choose the safer default and document the ambiguity.
