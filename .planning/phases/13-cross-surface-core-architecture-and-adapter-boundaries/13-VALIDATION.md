---
phase: 13
slug: cross-surface-core-architecture-and-adapter-boundaries
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 13 — Validation Strategy

> Per-phase validation contract for cross-surface architecture, adapter-boundary ownership, and future migration planning.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + `vitest` + doc/contract truth checks |
| **Config file** | Workspace `Cargo.toml`; `clients/web/vitest.config.ts` |
| **Quick run command** | `rg -n "cross-surface|adapter|ffi|embedded|daemon|server|tauri" docs clients/apple crates .planning/phases/13-cross-surface-core-architecture-and-adapter-boundaries` |
| **Full suite command** | `cargo test -p vel-api-types -- --nocapture && cargo test -p veld daily_loop_morning -- --nocapture && cargo test -p veld agent_grounding_inspect -- --nocapture && npm --prefix clients/web test -- --run src/types.test.ts src/data/agent-grounding.test.ts src/components/NowView.test.tsx src/components/SettingsPage.test.tsx` |
| **Estimated runtime** | ~90 seconds for targeted checks plus manual doc review |

---

## Sampling Rate

- **After every task commit:** Run the narrowest truth check for the touched contract/doc/code seam.
- **After every plan wave:** Re-run the focused commands for any proof flow named by the architecture docs.
- **Before phase close:** Manual read-through must confirm current-state-to-target-state mappings and future-phase notes still match the live repo.
- **Max feedback latency:** 90 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 13-01-01 | 01 | 0 | ARCH-XS-01, ARCH-XS-02 | doc/truth | `rg -n "embedded|daemon|server|current-state|target-state|vel-core|vel-api-types|VelAPI" docs .planning/phases/13-cross-surface-core-architecture-and-adapter-boundaries` | ❌ W0 | ⬜ pending |
| 13-01-02 | 01 | 0 | ARCH-XS-02 | repo-map/manual | `find crates -maxdepth 2 -mindepth 1 -type d | sort && find clients/apple -maxdepth 3 -mindepth 1 -type d | sort | sed -n '1,80p'` | ✅ baseline | ⬜ pending |
| 13-02-01 | 02 | 1 | ADAPT-01, API-ARCH-01 | DTO/contract | `cargo test -p vel-api-types -- --nocapture` | ✅ baseline | ⬜ pending |
| 13-02-02 | 02 | 1 | ADAPT-01, ADAPT-02 | web contract parity | `npm --prefix clients/web test -- --run src/types.test.ts src/data/agent-grounding.test.ts src/components/NowView.test.tsx src/components/SettingsPage.test.tsx` | ✅ baseline | ⬜ pending |
| 13-03-01 | 03 | 2 | APPLE-ARCH-01, ADAPT-01 | doc/apple boundary | `rg -n "HTTP|VelAPI|FFI|embedded|daemon|server" clients/apple/README.md clients/apple/VelAPI/Sources/VelAPI/VelClient.swift docs .planning/phases/13-cross-surface-core-architecture-and-adapter-boundaries` | ✅ baseline | ⬜ pending |
| 13-03-02 | 03 | 2 | ADAPT-01 | future desktop path | `rg -n "tauri|desktop|sidecar|daemon" docs .planning/ROADMAP.md .planning/phases/13-cross-surface-core-architecture-and-adapter-boundaries` | ❌ W0 | ⬜ pending |
| 13-04-01 | 04 | 3 | ADAPT-02, API-ARCH-01 | proof flow / Rust | `cargo test -p veld daily_loop_morning -- --nocapture && cargo test -p veld agent_grounding_inspect -- --nocapture` | ✅ baseline | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `13-CONTEXT.md` — user decisions and scope are preserved in repo, not just chat
- [ ] `13-RESEARCH.md` — current-state architecture findings and recommendations
- [ ] at least one architecture authority doc path identified for Phase 13 deliverables
- [ ] roadmap entries for Phases 13-16 reflect the agreed sequencing from this thread

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Phase 13 architecture matches the current repo instead of an aspirational rewrite | ARCH-XS-01, ARCH-XS-02 | This is primarily an alignment check across docs and code reality | Compare the final Phase 13 architecture docs against the live crate list, Apple README, runtime API docs, and active client boundaries. |
| Future Apple FFI migration is documented without pretending it is already current truth | APPLE-ARCH-01 | The main failure mode is architectural confusion, not code breakage | Read the final Apple-path docs and confirm they clearly distinguish current HTTP-first behavior from future embedded options. |
| Phase 14-16 descriptions retain the sequencing rationale from this thread | ARCH-XS-02 | Roadmap intent can be lost even if Phase 13 is well planned | Read `.planning/ROADMAP.md` and confirm the architecture → discovery → migration → logic-first ordering remains explicit. |

---

## Validation Sign-Off

- [x] All tasks have automated or explicit manual verification
- [x] Wave 0 captures planning artifacts and roadmap sequencing
- [x] No watch-mode flags
- [x] Feedback latency < 90s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
