---
phase: 12
slug: operator-shell-onboarding-and-connector-ergonomics
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 12 — Validation Strategy

> Per-phase validation contract for operator-shell polish, contextual help, project/settings ergonomics, and guided connector onboarding.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (Rust unit/integration), `vitest` 2.1.8 (web) |
| **Config file** | Workspace `Cargo.toml`, `clients/web/vitest.config.ts` |
| **Quick run command** | `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/types.test.ts` |
| **Full suite command** | `cargo test -p veld phase12 -- --nocapture && npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/NowView.test.tsx src/components/ProjectsView.test.tsx src/components/SettingsPage.test.tsx src/components/ThreadView.test.tsx src/data/operator.test.ts src/types.test.ts` |
| **Estimated runtime** | ~30-90 seconds for targeted web checks; longer if route/service coverage expands |

---

## Sampling Rate

- **After every task commit:** run the narrowest affected web or Rust command from the table below.
- **After every plan wave:** rerun the shared shell/component suite plus any route/service checks backing the changed UX.
- **Before `$gsd-verify-work`:** run the full targeted web suite, at least one route-level Rust check for any new backend seam, and one manual operator pass across shell, projects, settings, and onboarding help flows.
- **Max feedback latency:** 60 seconds for task-level checks.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 12-01-01 | 01 | 0 | SHELL-01, DOCS-01 | web/types | `npm --prefix clients/web test -- --run src/types.test.ts` | ❌ W0 | ⬜ pending |
| 12-01-02 | 01 | 0 | DOCS-01, INTEGR-UX-01 | docs/contract | `rg -n "phase 12|help|onboarding|connector|template" docs/user docs/api config clients/web/src/types.ts` | ❌ W0 | ⬜ pending |
| 12-02-01 | 02 | 1 | SHELL-01, SHELL-02 | web/component | `npm --prefix clients/web test -- --run src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/ThreadView.test.tsx` | ✅ partial | ⬜ pending |
| 12-02-02 | 02 | 1 | INTEGR-UX-01 | web/component | `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/data/operator.test.ts` | ✅ partial | ⬜ pending |
| 12-03-01 | 03 | 2 | PROJ-UX-01, SHELL-02 | web/component | `npm --prefix clients/web test -- --run src/components/ProjectsView.test.tsx src/components/SettingsPage.test.tsx` | ❌ W1 | ⬜ pending |
| 12-03-02 | 03 | 2 | DOCS-01, INTEGR-UX-01 | web/component | `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx src/types.test.ts` | ✅ partial | ⬜ pending |
| 12-04-01 | 04 | 3 | ONBOARD-01, INTEGR-UX-01 | Rust/integration | `cargo test -p veld linking_status -- --nocapture` | ✅ partial | ⬜ pending |
| 12-04-02 | 04 | 3 | ONBOARD-01, DOCS-01 | web/component | `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx src/data/operator.test.ts` | ✅ partial | ⬜ pending |
| 12-04-03 | 04 | 3 | ONBOARD-01, INTEGR-UX-01 | docs/manual-aid | `rg -n "vel_tailscale_url|local-source|linking|pairing token|auto-discovery" docs/user/setup.md docs/user/troubleshooting.md docs/user/integrations/*.md clients/apple/README.md` | ✅ partial | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/vel-api-types/src/lib.rs` or existing typed web/runtime DTO surface — any new shell/help/setup payloads must be typed first
- [ ] `config/examples/phase12-shell-help.example.json` or equivalent checked-in fixture if new help/navigation contract is added
- [ ] `docs/user/` updates for contextual help/onboarding entrypoints once the boundary is stable
- [ ] `clients/web/src/types.ts` decoder coverage for any new shell/help/setup payload

*If no new shared contract is added, execution must explicitly document that existing typed payloads were sufficient and no Wave 0 files were needed.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| The shell makes the primary daily surfaces easier to find without hiding critical trust/review information | SHELL-01, SHELL-02 | Navigation usefulness and cognitive load are partly experiential | Open the web shell, move through `Now`, `Inbox`, `Projects`, `Threads`, and `Settings`, and confirm the path to the current daily loop, latest thread, and integration help is shorter and clearer than before. |
| Freshness and degraded connector states feel calmer but still actionable | SHELL-02, INTEGR-UX-01 | Tone and urgency balance are hard to prove from snapshots alone | Force or simulate degraded source freshness, then confirm the shell preserves visibility, avoids panic language, and still exposes one obvious next action. |
| Project detail/edit flows expose the right data without turning projects into a secondary dashboard product | PROJ-UX-01 | Scope discipline is architectural and UX-based | Open a project from the shell, edit the supported fields, and confirm the surface remains grounded in the typed project record and related daily-use context. |
| Onboarding and connector help routes land on the correct setup docs for Apple, Todoist, linking, and local sources | ONBOARD-01, DOCS-01 | This spans UI-to-doc routing and human comprehension | From the shell/settings help affordances, open the linked docs and confirm they match the operator's current task instead of dropping them into a generic README. |
| Apple/local-source path discovery remains explicit about bootstrap limits and offline behavior | ONBOARD-01, INTEGR-UX-01 | Trust wording and path expectations need a human read | Walk through the setup/help flow for Apple/macOS local sources and confirm it clearly distinguishes auto-discovery, manual path entry, cached state, and daemon-required actions. |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verification or an explicit human checkpoint
- [x] Sampling continuity: no 3 consecutive tasks without automated verification
- [x] Wave 0 covers shared contract prerequisites if new payloads are introduced
- [x] No watch-mode flags
- [x] Feedback latency < 60s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
