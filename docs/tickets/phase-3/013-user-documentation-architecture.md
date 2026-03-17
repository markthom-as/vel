---
title: Comprehensive User Documentation & Support Wiki
status: in-progress
owner: staff-eng
type: documentation
priority: low
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 019-operator-accessibility-config-clarity
labels:
  - docs
  - user-experience
  - support
  - phase-3
---

# Context & Objectives

User docs under `docs/user/` already provide substantial setup, usage, surface, and troubleshooting guidance. Remaining work is about architecture/operations closure rather than first-time documentation creation.

This ticket closes the remaining gaps for support workflow, versioning discipline, and cross-surface guidance alignment.

# Current Baseline (Already Present)

- User doc entrypoint, quickstart, setup, daily-use, and troubleshooting guides are shipped.
- Integration docs exist for local sources, Google Calendar, Todoist, and Apple/macOS paths.
- Maturity and reality guidance distinguishes shipped vs planned behavior.

# Remaining Work Focus

- Define support/wiki ownership and update workflow.
- Tighten versioned doc-to-release parity checks.
- Ensure CLI/web/Apple operator language maps directly to user docs.
- Add explicit escalation and recovery decision trees for common failure modes.

# Impacted Files & Symbols

- **Directory**: `docs/user/`
  - **Symbols**: setup, troubleshooting, surfaces, maturity docs
- **Directory**: `docs/api/`
  - **Symbols**: operator-facing endpoint behavior references
- **Directory**: `clients/web/src/`, `clients/apple/`, `crates/vel-cli/src/`
  - **Symbols**: copy and labels referenced by user guides

# Technical Requirements

- **Support Architecture**: define where canonical user guidance lives and how updates are reviewed.
- **Version Parity**: docs must clearly track shipped behavior for current release phase.
- **Recovery Guidance**: troubleshooting must map symptoms to concrete operator actions.
- **Accessibility Language**: user docs keep clear reading order and operator-first wording.

# Cross-Cutting Trait Impact

- **Modularity**: affected — docs should map to real subsystem seams without duplication drift.
- **Accessibility**: required — primary user-facing readability and structure concern.
- **Configurability**: required — effective config defaults/inspection paths are documented.
- **Data Logging**: affected — docs explain where to inspect runs/events/diagnostics.
- **Rewind/Replay**: affected — recovery paths should include retry/replay guidance where relevant.
- **Composability**: required — language and support flows remain consistent across surfaces.

# Implementation Steps (The How)

1. **Gap audit**: map current user docs to core operator journeys and support incidents.
2. **Ownership model**: define update ownership/review and lightweight support wiki process.
3. **Parity pass**: align doc wording with shipped CLI/web/Apple surfaces.
4. **Verification hooks**: add checks or review gates that prevent stale behavior claims.

# Acceptance Criteria

1. [ ] User docs cover core setup/operate/recover flows with current shipped behavior.
2. [ ] Support/update ownership is documented and actionable.
3. [ ] Troubleshooting paths map directly to concrete operator commands/surfaces.
4. [ ] Docs clearly separate implemented behavior from planned behavior.

# Verification & Regression

- **Doc Check**: run repo-truth and documentation consistency checks.
- **Smoke Check**: execute documented operator commands against local runtime.
- **Surface Check**: verify wording parity with CLI/web/Apple labels for key states.
- **Invariants**: no user doc asserts shipped behavior that contradicts `MASTER_PLAN.md`.
