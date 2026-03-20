---
phase: 12-operator-shell-onboarding-and-connector-ergonomics
plan: 03
subsystem: web
tags: [phase-12, projects, settings, integrations, docs]
requires:
  - phase: 12-operator-shell-onboarding-and-connector-ergonomics
    plan: 02
    provides: shell defaults and calmer trust framing over existing typed surfaces
provides:
  - clearer project detail state with bounded create-from-existing handoff instead of invented inline editing
  - settings documentation cues that separate operator help from implementation authority
  - explicit demotion of internal/default connector paths relative to operator-selectable source paths
affects: [phase-12, projects, settings, integrations, docs]
tech-stack:
  added: []
  patterns: [typed-project reuse, bounded draft handoff, operator-first connector framing, shipped-doc alignment]
key-files:
  created:
    - .planning/phases/12-operator-shell-onboarding-and-connector-ergonomics/12-03-SUMMARY.md
  modified:
    - clients/web/src/components/ProjectsView.tsx
    - clients/web/src/components/ProjectsView.test.tsx
    - clients/web/src/components/SettingsPage.tsx
    - clients/web/src/components/SettingsPage.test.tsx
    - docs/user/surfaces.md
key-decisions:
  - "Because there is no shipped project update route, project 'edit ergonomics' should stay within the existing typed record seam by pre-filling the create draft from a selected project."
  - "Settings should foreground operator-usable source paths and documentation, while internal/default paths remain visible only as read-only diagnostic context."
patterns-established:
  - "When a surface needs a clearer edit path but the backend contract does not support mutation, improve the handoff and bounded reuse flow instead of inventing local shadow state."
requirements-completed: [PROJ-UX-01, SHELL-02, DOCS-01, INTEGR-UX-01]
duration: 13m
completed: 2026-03-19
---

# Phase 12-03 Summary

**Projects and Settings now read more like operator surfaces than raw scaffolding: selected projects expose clearer typed detail, Settings distinguishes operator help from implementation authority, and integration cards emphasize selectable source paths over internal defaults.**

## Accomplishments

- Updated [ProjectsView.tsx](/home/jove/code/vel/clients/web/src/components/ProjectsView.tsx) so a selected project shows family, status, extra-root coverage, and provisioning intent, then offers a bounded "Use project as draft" handoff that pre-fills the existing create form without inventing an unsupported inline-edit API.
- Added focused regression coverage in [ProjectsView.test.tsx](/home/jove/code/vel/clients/web/src/components/ProjectsView.test.tsx) for the richer detail state and the draft-prefill flow.
- Updated [SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) to frame documentation as an operator help surface, add clearer hints for core versus user docs, and explicitly separate operator-selectable connector paths from read-only internal/default paths.
- Extended [SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx) to lock the new documentation and connector-path framing in place.
- Updated [surfaces.md](/home/jove/code/vel/docs/user/surfaces.md) so the shipped user guide reflects the bounded project-detail handoff and the revised Settings/path-selection behavior.

## Verification

- `npm --prefix clients/web test -- --run src/components/ProjectsView.test.tsx src/components/SettingsPage.test.tsx src/types.test.ts`

Passed.

## Next Phase Readiness

- `12-04` can now focus on guided onboarding, linking, and Apple/local-source path discovery without reopening project or Settings trust boundaries.
- No manual browser pass was performed in this slice.
