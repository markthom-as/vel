---
phase: 12-operator-shell-onboarding-and-connector-ergonomics
plan: 04
subsystem: web-docs
tags: [phase-12, onboarding, linking, apple, local-sources, docs]
requires:
  - phase: 12-operator-shell-onboarding-and-connector-ergonomics
    plan: 03
    provides: clearer project/settings trust boundaries and doc framing
provides:
  - an onboarding-and-recovery checklist in Settings derived from existing bootstrap, linking, and integration payloads
  - local-source cards that point directly to the matching Apple, local-source, and troubleshooting docs
  - setup and troubleshooting docs aligned to the shipped Apple/macOS endpoint and local-source recovery flow
affects: [phase-12, settings, linking, onboarding, apple, docs]
tech-stack:
  added: []
  patterns: [typed onboarding summary, operator-first recovery guidance, doc-aligned setup flow]
key-files:
  created:
    - .planning/phases/12-operator-shell-onboarding-and-connector-ergonomics/12-04-SUMMARY.md
  modified:
    - clients/web/src/components/SettingsPage.tsx
    - clients/web/src/components/SettingsPage.test.tsx
    - clients/web/src/data/operator.ts
    - clients/web/src/data/operator.test.ts
    - docs/user/setup.md
    - docs/user/troubleshooting.md
    - docs/user/integrations/local-sources.md
    - docs/user/integrations/apple-macos.md
    - clients/apple/README.md
key-decisions:
  - "Onboarding guidance should be synthesized from existing typed operator payloads rather than introducing parallel client-owned setup state."
  - "Apple and local-source recovery should route operators into explicit setup and troubleshooting docs from the same Settings surfaces where the issue appears."
patterns-established:
  - "When setup friction spans linking, endpoint resolution, and path discovery, use one linear next-step summary over current runtime data and keep the actual recovery work on the existing backend or docs surfaces."
requirements-completed: [ONBOARD-01, INTEGR-UX-01, DOCS-01]
duration: 17m
completed: 2026-03-19
---

# Phase 12-04 Summary

**Phase 12 closes with a practical onboarding loop: Settings now tells the operator what to do next for daemon reachability, linking, local-source paths, and Apple/macOS recovery, and the linked docs now describe the same flow without repo-only assumptions.**

## Accomplishments

- Added [operator.ts](/home/jove/code/vel/clients/web/src/data/operator.ts) onboarding helpers that derive a next-step checklist from the existing cluster bootstrap, linking status, worker presence, and integration payloads.
- Updated [SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) to render a new `Onboarding and recovery` card on the general tab and to attach direct local-source, Apple/macOS, and troubleshooting doc pointers to the operator path-selection area in Integrations.
- Added regression coverage in [SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx) and [operator.test.ts](/home/jove/code/vel/clients/web/src/data/operator.test.ts) for the new onboarding summary and contextual help paths.
- Updated [setup.md](/home/jove/code/vel/docs/user/setup.md), [troubleshooting.md](/home/jove/code/vel/docs/user/troubleshooting.md), [local-sources.md](/home/jove/code/vel/docs/user/integrations/local-sources.md), [apple-macos.md](/home/jove/code/vel/docs/user/integrations/apple-macos.md), and [README.md](/home/jove/code/vel/clients/apple/README.md) so endpoint resolution, Apple snapshot/export behavior, local-source path selection, and recovery order match the shipped Settings flow.

## Verification

- `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx src/data/operator.test.ts`
- `rg -n "vel_tailscale_url|vel_base_url|local-source|auto-discovery|pairing token|linking" docs/user/setup.md docs/user/troubleshooting.md docs/user/integrations/local-sources.md docs/user/integrations/apple-macos.md clients/apple/README.md`

Passed.

## Phase Closure

- Phase 12 is now complete at `4/4`.
- No manual browser pass was performed in this slice.
