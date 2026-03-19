---
phase: 12-operator-shell-onboarding-and-connector-ergonomics
plan: 02
subsystem: web
tags: [phase-12, shell, navigation, threads, freshness, todoist]
requires:
  - phase: 12-operator-shell-onboarding-and-connector-ergonomics
    plan: 01
    provides: docs-first decision to reuse existing typed shell/runtime seams
provides:
  - icon-driven primary/support shell grouping without a routing rewrite
  - latest-thread fallback when the Threads surface opens without an explicit conversation selection
  - calmer freshness and connector wording that preserves the same backend-owned actions
affects: [phase-12, web-shell, threads, now, freshness]
tech-stack:
  added: []
  patterns: [thin-shell navigation polish, latest-record fallback from existing typed data, calmer trust messaging without policy drift]
key-files:
  created:
    - .planning/phases/12-operator-shell-onboarding-and-connector-ergonomics/12-02-SUMMARY.md
  modified:
    - clients/web/src/components/Sidebar.tsx
    - clients/web/src/components/Sidebar.test.tsx
    - clients/web/src/components/ThreadView.tsx
    - clients/web/src/components/ThreadView.test.tsx
    - clients/web/src/components/NowView.tsx
    - clients/web/src/components/NowView.test.tsx
key-decisions:
  - "Thread recovery should use the latest updated existing conversation from the typed conversation list instead of leaving the Threads surface empty."
  - "Shell navigation can become clearer through primary/support grouping and icons without introducing a full routing layer."
  - "Freshness warnings should reduce repeated alarm language while preserving the same sync/evaluate/open-settings actions."
patterns-established:
  - "When shell ergonomics need better defaults, prefer deriving them from existing typed query data rather than introducing new local persistence or hidden policy."
requirements-completed: [SHELL-01, SHELL-02, INTEGR-UX-01]
duration: 16m
completed: 2026-03-19
---

# Phase 12-02 Summary

**The web shell now recovers more gracefully: navigation is grouped around primary daily surfaces, Threads falls back to the latest conversation when no thread is explicitly selected, and freshness language is calmer without changing backend authority.**

## Accomplishments

- Updated [Sidebar.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.tsx) to distinguish primary daily surfaces from support surfaces, add lightweight icon-driven affordances, and keep thread history visible by default when the operator actually opens Threads.
- Updated [ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) so opening the Threads surface without a selected conversation automatically resolves to the latest updated conversation from the existing typed conversation list.
- Updated [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) to soften degraded freshness and connector wording, reduce repeated guidance in the banner, and keep the same backend-owned actions for sync, evaluate, or settings navigation.
- Added focused regression coverage in [Sidebar.test.tsx](/home/jove/code/vel/clients/web/src/components/Sidebar.test.tsx), [ThreadView.test.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx), and [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx).

## Verification

- `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/ThreadView.test.tsx src/components/NowView.test.tsx`

Passed.

## Next Phase Readiness

- `12-03` can focus on project detail/edit and Settings clarity work without revisiting shell-level defaults.
- No manual browser pass was performed in this slice.
