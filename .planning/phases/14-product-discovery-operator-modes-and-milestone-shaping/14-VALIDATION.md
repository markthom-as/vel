---
phase: 14
slug: product-discovery-operator-modes-and-milestone-shaping
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 14 — Validation Strategy

> Per-phase validation contract for product-surface classification, operator modes, onboarding/trust journeys, and milestone reshaping.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `vitest` + React Testing Library + roadmap/doc truth checks |
| **Config file** | `clients/web/vitest.config.ts` |
| **Quick run command** | `npm --prefix clients/web test -- src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/NowView.test.tsx src/components/SettingsPage.test.tsx src/data/operator.test.ts` |
| **Full suite command** | `npm --prefix clients/web test && rg -n "Phase 17|advanced|developer|operator mode|surface taxonomy" .planning/ROADMAP.md .planning/phases/14-product-discovery-operator-modes-and-milestone-shaping docs` |
| **Estimated runtime** | ~30-60s for targeted checks plus manual doc review |

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 14-01-01 | 01 | 0 | PROD-01, MODE-01 | docs/truth | `rg -n "default|advanced|developer|internal|surface taxonomy" .planning/phases/14-product-discovery-operator-modes-and-milestone-shaping docs` | ❌ W0 | ⬜ pending |
| 14-02-01 | 02 | 1 | TRUST-UX-01, ONBOARD-02 | web evidence | `npm --prefix clients/web test -- src/components/SettingsPage.test.tsx src/data/operator.test.ts` | ✅ baseline | ⬜ pending |
| 14-03-01 | 03 | 2 | MODE-01, UX-CORE-01 | web evidence | `npm --prefix clients/web test -- src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/NowView.test.tsx` | ✅ baseline | ⬜ pending |
| 14-04-01 | 04 | 3 | ROADMAP-01 | roadmap/doc | `rg -n "Phase 17|shell embodiment|surface simplification" .planning/ROADMAP.md .planning/phases/14-product-discovery-operator-modes-and-milestone-shaping` | ❌ W0 | ⬜ pending |

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| The default operator story remains daily-use-first rather than runtime-first | PROD-01, UX-CORE-01 | This is a product-classification judgment | Read the final taxonomy and journey docs and verify they center `Now`, `Inbox`, `Projects`, and the daily loop. |
| Advanced/runtime/developer concerns are classified explicitly instead of hidden by copy only | MODE-01 | Classification quality is semantic | Review the final mode policy and verify it assigns current Settings/Stats/runtime surfaces to explicit buckets. |
| Milestone reshaping is durable and execution-usable | ROADMAP-01 | Roadmap clarity matters more than testability here | Confirm the roadmap includes any approved new phase and that later phases have distinct responsibilities. |

---

## Validation Sign-Off

- [x] All tasks have automated or explicit manual verification
- [x] Wave 0 captures planning artifacts and roadmap reshaping checks
- [x] No watch-mode flags
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
