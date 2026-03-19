# Phase 14: Product discovery, operator modes, and milestone shaping - Research

**Researched:** 2026-03-19
**Domain:** Operator product-shaping, surface classification, trust/onboarding ergonomics, and roadmap restructuring
**Confidence:** HIGH

<user_constraints>
## User Constraints

### Locked Decisions
- Architecture first, then product discovery, then incremental migration, then logic-first implementation, then UI embodiment.
- Phase 14 discovery/planning should begin in parallel with Phase 13 implementation.
- Discovery is expected to take time and may produce multiple new future phases.
- Align with current codebase reality; do not assume greenfield rewrites.
- Focus especially on the default operator experience versus advanced/dev/admin surfaces.
- Decide what should move behind menus or advanced mode.
- Improve onboarding and trust ergonomics without widening product sprawl.
- Evaluate whether current UI surfaces expose too much internal state.

### Claude's Discretion
- Exact operator-mode taxonomy and names.
- Whether advanced/runtime/developer concerns should be split across one or more future phases.
- Which existing surfaces should remain primary versus become secondary or deferred.
- Whether milestone reshaping should insert a new future phase or only re-scope Phases 15 and 16.

### Deferred Ideas (OUT OF SCOPE)
- Greenfield shell redesigns detached from current web/Apple/CLI reality.
- Full desktop shell implementation.
- Full Apple FFI migration.
- Broad provider/platform expansion unrelated to operator product shape.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PROD-01 | Define the default operator product shape for repeated daily use. | Current code already frames `Now`, `Inbox`, and `Projects` as primary; research recommends ratifying that as the default shell. |
| MODE-01 | Classify surfaces into default, advanced operator, and internal/developer buckets. | Current `Settings` mixes onboarding, trust, runtime controls, agent grounding, and execution review; Phase 14 should split those concerns intentionally. |
| UX-CORE-01 | Decide which surface and language patterns best support daily use without product sprawl. | `NowView`, `Sidebar`, `daily-use.md`, and `vel` CLI all reinforce daily-loop-first behavior; that should remain the center of gravity. |
| TRUST-UX-01 | Set trust and review ergonomics so operators can understand safety state without reading raw internal diagnostics. | Existing backup trust, writeback mode, agent grounding, and review-status projections provide a backend-owned base that should be summarized before raw detail. |
| ONBOARD-02 | Define onboarding and recovery flows that route operators to the next safe step rather than internal state spelunking. | `SettingsPage` already contains an onboarding guide driven from typed payloads; Phase 14 should make that pattern canonical. |
| ROADMAP-01 | Shape milestone structure so migration, logic, and shell embodiment do not collapse into one mixed phase. | Research recommends keeping 15 and 16 but adding a new post-16 shell embodiment phase. |
</phase_requirements>

## Summary

Vel already has a product center of gravity: `Now` for daily orientation, `Inbox` for triage, `Projects` for workspace detail, and `Settings` as the recovery/trust/config surface. The issue is not that the product lacks structure. The issue is that `Settings` currently carries too many responsibilities at once: onboarding guidance, cross-client routing, writeback mode, backup trust, agent grounding, runtime diagnostics, component control, and execution handoff review. That makes the default operator story weaker and risks teaching users internal implementation categories instead of product ones.

The current repo also already contains the raw ingredients for a clean mode model. The sidebar marks `Now`, `Inbox`, and `Projects` as primary. `daily-use.md` tells operators to treat `Now` as the main daily surface and `Settings` as the setup/trust path. `SettingsPage.tsx` has a meaningful split between `general`, `integrations`, and `runtime`, and its own copy says the runtime tab is for operator actions while Stats is for passive observability. That is strong evidence that Vel is ready for progressive disclosure, but the policy is still implicit and unevenly expressed.

Phase 14 should therefore be a discovery-and-decision phase, not a thin UX polish phase. It should publish one canonical operator-surface taxonomy, one default-versus-advanced mode policy, one onboarding/trust journey model, and one milestone reshaping recommendation. The correct milestone outcome is not to push everything into Phase 16. Keep Phase 15 for seam migration, keep Phase 16 for canonical backend logic, and add a new follow-on shell embodiment phase so UI restructuring happens after product policy and backend logic are stable.

**Primary recommendation:** Treat Phase 14 as a product-contract phase that produces a canonical surface taxonomy and explicitly inserts a new post-16 shell embodiment phase.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-cli` | workspace `0.1.0` | Canonical operator shell and command vocabulary | The CLI is already the clearest product authority for repeated daily use and trust checks. |
| `veld` | workspace `0.1.0` | Backend-owned trust, onboarding, review, and runtime projections | Product-mode decisions should map to backend-owned contracts, not component-local heuristics. |
| `clients/web` React | `19.2.4` | Current shell embodiment and navigation evidence | Existing web surfaces are where current product sprawl and good seams are both visible. |
| `clients/web` TypeScript | `5.9.3` | Typed view-model and contract decoding | Surface classification should stay explicit and typed once implemented. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Vitest | `2.1.9` installed (`package.json` declares `^2.1.8`) | Component and projection verification | Use for surface-taxonomy, navigation, and progressive-disclosure tests. |
| React Testing Library | `16.1.0` | Shell interaction tests | Use when Phase 14 outputs become concrete view contracts. |
| Existing docs under `docs/user/` | current repo docs | Operator guidance authority | Use to keep onboarding/trust language aligned with product decisions. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Progressive disclosure over existing shell surfaces | Leave all current tabs and cards equally exposed | Faster short-term, but preserves product sprawl and teaches internal categories by accident. |
| Backend-owned mode policy plus shell embodiment later | UI-only cleanup in current React components | Easier to ship visually, but would repeat the current problem: shell-first product definition. |
| One new post-16 UI embodiment phase | Stuff UI simplification into Phase 16 | Blurs logic closure with shell redesign and weakens reviewability. |

**Installation:**
```bash
# No new packages recommended for Phase 14 research/planning.
# Use existing workspace + web test stack.
```

**Version verification:** Verified from checked-in manifests on 2026-03-19: [Cargo.toml](/home/jove/code/vel/Cargo.toml), [clients/web/package.json](/home/jove/code/vel/clients/web/package.json).

## Architecture Patterns

### Recommended Project Structure
```text
docs/
├── product/
│   ├── operator-surface-taxonomy.md
│   ├── onboarding-and-trust-journeys.md
│   └── milestone-reshaping.md
clients/web/src/
├── components/
│   ├── shell/              # default shell and advanced-entry components
│   ├── runtime/            # advanced operator/runtime surfaces
│   └── trust/              # review/trust summaries and drills
└── data/
    └── operator-mode.ts    # typed surface classification + labels
```

### Pattern 1: Default Shell Is Daily-Use First
**What:** Keep the default operator path centered on `Now`, `Inbox`, `Projects`, and the daily-loop entry points.
**When to use:** Any navigation, onboarding, or information-architecture decision.
**Example:**
```tsx
// Source: /home/jove/code/vel/clients/web/src/components/Sidebar.tsx
const primaryItems = ['now', 'inbox', 'projects'];
```

### Pattern 2: Progressive Disclosure For Trust And Runtime
**What:** Show summary-level trust state in the default path; put diagnostics, controls, and internal runtime detail behind an advanced operator entry.
**When to use:** Backup trust, agent grounding, execution review, node diagnostics, and component controls.
**Example:**
```tsx
// Source: /home/jove/code/vel/clients/web/src/components/SettingsPage.tsx
activeTab === 'runtime' ? <RuntimeControls /> : null
```

### Pattern 3: Onboarding Is Guided Next-Step Routing, Not Raw Diagnostics
**What:** The product should route the operator to the next unfinished safe step, then let them drill into raw detail only when needed.
**When to use:** Linking, connector setup, recovery, and daemon-path issues.
**Example:**
```tsx
// Source: /home/jove/code/vel/clients/web/src/components/SettingsPage.tsx
onboardingGuide.nextAction
```

### Pattern 4: Internal Or Developer Surfaces Are Not Default Navigation
**What:** Runtime internals, component restarts, and low-level execution queues should not sit at the same conceptual level as daily-use product surfaces.
**When to use:** Stats, runtime controls, developer/admin affordances, or future debug views.
**Example:** Keep direct links or advanced-mode entry points, but remove them from the default product story.

### Anti-Patterns to Avoid
- **Settings as the junk drawer:** Do not keep onboarding, trust, review, runtime control, and developer detail as one undifferentiated destination.
- **Mode by copy only:** Do not rely on headings like “Use Stats for passive observability” without an explicit product mode model behind them.
- **Surface parity without policy:** Do not make Apple/web/CLI expose the same raw detail just because the transport exists.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Product boundary decisions | Ad hoc tab choices in each shell | One canonical surface taxonomy artifact from Phase 14 | Prevents shell-led drift. |
| Advanced mode | Per-component booleans and one-off hiding rules | Typed mode classification shared across shells and docs | Keeps policy inspectable and portable. |
| Onboarding logic | Raw diagnostics checklists in every view | Existing backend-owned onboarding guide pattern | It already routes from typed state to next safe action. |
| Trust presentation | Dumping backup, review, and capability details directly into the default shell | Summary-first trust/readiness projections with drill-down | Operators need confidence first, internals second. |
| Milestone shaping | Folding migration, logic, and UI cleanup into one phase | Separate phases for migration, logic closure, and embodiment | Improves reviewability and reduces accidental rewrites. |

**Key insight:** Phase 14 should not invent new infrastructure. It should classify and simplify the infrastructure and surfaces Vel already has.

## Common Pitfalls

### Pitfall 1: Default Shell Still Teaches Internal Runtime Categories
**What goes wrong:** Operators learn “runtime queue”, “capability groups”, and “diagnostics” before they learn the daily-use loop.
**Why it happens:** Current `Settings` mixes product trust summaries with internal control surfaces.
**How to avoid:** Define summary-level trust cards for the default shell and move deep runtime inspection behind advanced mode.
**Warning signs:** Default flows require opening `Settings > runtime` to understand normal product state.

### Pitfall 2: Settings Becomes The Permanent Escape Hatch
**What goes wrong:** Every hard-to-place feature lands in Settings and no longer has a product owner.
**Why it happens:** Settings already contains valid but unrelated slices: onboarding, sync routing, trust, runtime controls, and local path management.
**How to avoid:** Phase 14 should classify each section as core settings, advanced operator, or developer/internal.
**Warning signs:** A single page is responsible for setup, recovery, diagnostics, execution review, and component restarts.

### Pitfall 3: Discovery Bleeds Into Refactor Or UI Work
**What goes wrong:** The roadmap starts migrating components or rewriting layouts before the product-mode contract exists.
**Why it happens:** Phase 14 sits between architecture and migration and can easily absorb implementation work.
**How to avoid:** Keep Phase 14 outputs doc-first and contract-first: taxonomy, journeys, scope moves, and milestone structure.
**Warning signs:** New navigation work begins before surface classification and milestone reshaping are written down.

### Pitfall 4: Phase 16 Quietly Becomes Both Logic Closure And UI Rebuild
**What goes wrong:** Backend logic and shell redesign compete for the same phase budget.
**Why it happens:** Current roadmap has Phase 16 for logic closure but no explicit UI embodiment phase after it.
**How to avoid:** Insert a post-16 shell embodiment phase now.
**Warning signs:** Phase 16 plans start discussing navigation, menus, card hierarchy, and platform-specific interaction polish.

## Code Examples

Verified patterns from current repo sources:

### Primary Versus Support Surfaces
```tsx
// Source: /home/jove/code/vel/clients/web/src/components/Sidebar.tsx
const primaryItems = [
  { view: 'now', label: 'Now' },
  { view: 'inbox', label: 'Inbox' },
  { view: 'projects', label: 'Projects' },
];
```

### Summary-First Onboarding
```tsx
// Source: /home/jove/code/vel/clients/web/src/components/SettingsPage.tsx
<p>{onboardingGuide.nextAction}</p>
{onboardingGuide.steps.map(renderStep)}
```

### Daily-Loop-First Product Framing
```tsx
// Source: /home/jove/code/vel/clients/web/src/components/NowView.tsx
<DailyLoopPanel ... />
<Panel title="Action stack" ... />
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Shells discover product shape implicitly from UI growth | Roadmap explicitly sequences architecture → discovery → migration → logic | 2026-03-19 | Creates room to ratify product shape before more implementation widens. |
| Settings as a broad catch-all | `general` / `integrations` / `runtime` tabs plus Stats distinction | 2026-03-19 Phase 12 state | Shows progressive disclosure intent exists, but it is not yet a formal mode policy. |
| Daily loop as one view among many | `daily-use.md` and `Now` position the daily loop as primary authority | 2026-03-19 | Supports making daily use the default mode instead of a feature among equals. |

**Deprecated/outdated:**
- Treating all currently visible web surfaces as equally “core” product surfaces.
- Assuming runtime diagnostics belong in the same conceptual bucket as onboarding and everyday use.

## Milestone Reshaping Recommendation

**Recommendation:** Yes. Phase 14 should explicitly emit at least one additional future phase.

### Keep
- **Phase 15** as incremental core migration and seam sharpening.
- **Phase 16** as logic-first product closure on canonical Rust surfaces.

### Add
- **Phase 17: Shell embodiment, operator-mode application, and surface simplification**

### Why add Phase 17
- The thread decisions already require UI embodiment to come after logic-first implementation.
- The current roadmap stops at logic closure and has no dedicated phase for applying the approved mode model across web, Apple, and CLI shells.
- Without a dedicated embodiment phase, Phase 16 will absorb navigation, settings decomposition, advanced-mode entry, and trust-surface restructuring, which weakens both logic and UX work.

### What Phase 17 should own
- Apply the Phase 14 taxonomy to web, Apple, and CLI shells.
- Move advanced/runtime/developer concerns behind explicit entry points.
- Reframe trust and onboarding as summary-first flows in default mode.
- Simplify or relocate surfaces that currently expose too much internal state.

### What Phase 17 should not own
- New product semantics that belong in backend logic.
- Broad crate or service migration work.
- New desktop implementation beyond shell-ready contracts.

## Open Questions

1. **Should Stats remain a passive observability surface or absorb parts of runtime detail currently sitting in Settings?**
   - What we know: `SettingsPage` already tells operators to use Stats for passive observability and runtime for actions.
   - What's unclear: whether Stats should become the advanced read-only destination, leaving runtime as action-only.
   - Recommendation: Resolve this in Phase 14 and make the answer part of the surface taxonomy.

2. **Where should agent grounding live in the product hierarchy?**
   - What we know: it is currently presented inside Settings as a trust surface.
   - What's unclear: whether it belongs in default trust summaries, advanced trust inspection, or a dedicated supervised-execution area.
   - Recommendation: Default shell should show summarized readiness only; full grounding should be advanced operator detail.

3. **How much runtime/config detail should Apple clients ever expose directly?**
   - What we know: Apple is a shell over the same daemon and should not own product logic.
   - What's unclear: whether advanced operator mode is web/CLI-only or should have a bounded Apple form.
   - Recommendation: Decide that Apple gets summary-first trust/setup cues, with deep runtime control remaining web/CLI-first unless a concrete mobile use case exists.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Vitest `2.1.9` (installed) + React Testing Library; Rust `cargo test` workspace |
| Config file | [clients/web/vitest.config.ts](/home/jove/code/vel/clients/web/vitest.config.ts) |
| Quick run command | `npm --prefix clients/web test -- src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/NowView.test.tsx src/components/SettingsPage.test.tsx src/data/operator.test.ts` |
| Full suite command | `npm --prefix clients/web test && cargo test --workspace --all-features` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PROD-01 | Default shell keeps daily-use surfaces primary | component | `npm --prefix clients/web test -- src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx` | ✅ |
| MODE-01 | Advanced/runtime/developer surfaces are explicitly gated from default mode | component | `npm --prefix clients/web test -- src/components/OperatorModeLayout.test.tsx` | ❌ Wave 0 |
| UX-CORE-01 | `Now` remains the daily-use authority and settings navigation is intentional | component | `npm --prefix clients/web test -- src/components/NowView.test.tsx src/components/MainPanel.test.tsx` | ✅ |
| TRUST-UX-01 | Trust state is summarized from backend-owned projections before deep detail | component/unit | `npm --prefix clients/web test -- src/components/SettingsPage.test.tsx src/data/operator.test.ts` | ✅ |
| ONBOARD-02 | Onboarding/recovery shows next-step guidance instead of raw diagnostics first | component | `npm --prefix clients/web test -- src/components/SettingsPage.test.tsx` | ✅ |
| ROADMAP-01 | Milestone reshaping outcome is captured and reviewed before Phase 15 planning | manual-only | `rg -n "Phase 17|shell embodiment" .planning/phases/14-product-discovery-operator-modes-and-milestone-shaping/14-RESEARCH.md .planning/ROADMAP.md` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `npm --prefix clients/web test -- src/components/Sidebar.test.tsx src/components/MainPanel.test.tsx src/components/NowView.test.tsx src/components/SettingsPage.test.tsx src/data/operator.test.ts`
- **Per wave merge:** `npm --prefix clients/web test`
- **Phase gate:** `npm --prefix clients/web test && cargo test --workspace --all-features`

### Wave 0 Gaps
- [ ] `clients/web/src/components/OperatorModeLayout.test.tsx` — covers `MODE-01`
- [ ] A typed surface taxonomy artifact or manifest — required so gating is testable across shells
- [ ] Roadmap verification step that confirms Phase 17 is inserted or equivalent embodiment scope is scheduled before Phase 15 planning closes

## Sources

### Primary (HIGH confidence)
- [`.planning/ROADMAP.md`](/home/jove/code/vel/.planning/ROADMAP.md) - Phase 14 through 16 goals, sequencing, and requirement IDs
- [`.planning/PROJECT.md`](/home/jove/code/vel/.planning/PROJECT.md) - accepted product-direction decisions and current constraints
- [`.planning/STATE.md`](/home/jove/code/vel/.planning/STATE.md) - recent roadmap evolution and Phase 13-16 insertion history
- [`docs/MASTER_PLAN.md`](/home/jove/code/vel/docs/MASTER_PLAN.md) - canonical implementation truth and current phase status
- [`docs/user/daily-use.md`](/home/jove/code/vel/docs/user/daily-use.md) - default operator workflow authority
- [`docs/user/setup.md`](/home/jove/code/vel/docs/user/setup.md) - setup/trust/recovery guidance authority
- [`clients/web/src/components/Sidebar.tsx`](/home/jove/code/vel/clients/web/src/components/Sidebar.tsx) - current primary/support surface split
- [`clients/web/src/components/NowView.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.tsx) - daily-loop-first and trust-summary current shell
- [`clients/web/src/components/SettingsPage.tsx`](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) - current overload and existing progressive-disclosure seams
- [`clients/web/src/App.tsx`](/home/jove/code/vel/clients/web/src/App.tsx) - current top-level navigation model
- [`crates/vel-cli/src/main.rs`](/home/jove/code/vel/crates/vel-cli/src/main.rs) - CLI command vocabulary proving operator-shell authority
- [`clients/apple/README.md`](/home/jove/code/vel/clients/apple/README.md) - Apple shell boundaries and daemon-owned business logic stance

### Secondary (MEDIUM confidence)
- [`clients/web/src/components/SettingsPage.test.tsx`](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx) - existing verification evidence for onboarding, trust, and settings behavior
- [`clients/web/src/components/NowView.test.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) - current verification evidence for default daily-use shell
- [`clients/web/src/data/operator.test.ts`](/home/jove/code/vel/clients/web/src/data/operator.test.ts) - existing review/trust projection evidence

### Tertiary (LOW confidence)
- None.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new external stack is recommended; findings come directly from checked-in manifests and current code.
- Architecture: HIGH - the live shell, docs, and roadmap all agree on daily-use-first plus backend-owned policy.
- Pitfalls: HIGH - directly evidenced by current `Settings` scope, current docs, and current navigation structure.

**Research date:** 2026-03-19
**Valid until:** 2026-04-18
