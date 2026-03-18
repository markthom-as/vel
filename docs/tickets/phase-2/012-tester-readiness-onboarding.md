---
title: Tester-Readiness Onboarding & Node Discovery
status: in-progress
owner: staff-eng
type: onboarding
priority: medium
created: 2026-03-17
updated: 2026-03-17
labels:
  - onboarding
  - distributed
  - phase-2
---

# Context & Objectives

Parts of onboarding are already present (local source discovery paths and cluster/node visibility), but first-run linking and guided setup are incomplete.

This ticket is now treated as **partial implementation closure**: preserve shipped pieces and close the remaining pairing and guided onboarding gaps.

# Execution Slices

- **Slice A (Already Partial)**: source discovery suggestions, cluster/node freshness visibility.
- **Slice B (Remaining)**: `vel node link` pairing flow, short-lived scoped pairing tokens, guided web/CLI linking wizard.

# Impacted Files & Symbols

- **Crate**: `vel-cli`
  - **Symbols**: node link/list command set
- **Directory**: `clients/web/src/`
  - **Symbols**: onboarding and diagnostics surfaces
- **Directory**: `clients/apple/`
  - **Symbols**: linked-node diagnostics and setup affordances
- **File**: `crates/veld/src/routes/cluster.rs`
  - **Symbols**: node visibility/status surfaces

# Technical Requirements

- **Scoped Pairing**: pairing tokens are short-lived, revocable, and purpose scoped.
- **Fallback Paths**: onboarding remains operable through web or CLI if one surface is unavailable.
- **Trust Visibility**: link flow states what a linked device can read/write/execute.
- **Config Visibility**: linked-node config and pairing state remain inspectable post-setup.

# Cross-Cutting Trait Impact

- **Modularity**: affected — onboarding should consume existing cluster/sync services.
- **Accessibility**: required — keyboard/readability baseline across web and CLI.
- **Configurability**: required — effective pairing and endpoint config must be visible.
- **Data Logging**: affected — onboarding failures should be diagnosable.
- **Rewind/Replay**: affected — queued sync state should explain replay behavior.
- **Composability**: required — onboarding terminology aligns with operator/config docs.

# Implementation Steps (The How)

1. **Command surface**: add `vel node link` and supporting link status commands.
2. **Token flow**: implement short-lived pairing token issue/redeem path.
3. **Wizard pass**: add minimal guided setup flow in CLI/web.
4. **Trust text**: expose capability scope and linked-node permissions in setup/status outputs.

# Acceptance Criteria

**Slice A (Already Partial — source discovery & freshness visibility):**

2. [ ] Local data source suggestions are surfaced during onboarding.
3. [ ] Linked nodes and freshness state are inspectable in CLI/web diagnostics.

**Slice B (Remaining — pairing flow & guided wizard):**

1. [ ] Non-technical tester can complete node linking in a short guided flow.
4. [ ] Pairing does not grant broad long-lived credentials by default.

# Verification & Regression

- **Unit Test**: token lifetime/scope enforcement.
- **Integration Test**: pairing issue/redeem and linked-node visibility.
- **Smoke Check**: CLI-driven linking from a clean local setup.
- **Invariants**: pairing flows fail closed when token is expired, malformed, or out-of-scope.
