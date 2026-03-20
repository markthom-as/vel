# Phase 18: Milestone verification backfill and requirement reconciliation - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 18 closes the first half of the milestone-audit blockers. It backfills the missing milestone verification evidence across shipped phases and reconciles `REQUIREMENTS.md` against completed summaries and verification outcomes. It does not add new product behavior, reopen completed feature scope, or archive the milestone yet.

</domain>

<decisions>
## Implementation Decisions

### Verification backfill strategy
- This phase should create durable verification artifacts rather than relying on summary prose or memory of earlier runs.
- Verification work should be grouped into a bounded closeout pass, not scattered retroactively as ad hoc edits across unrelated feature files.
- The goal is milestone auditability, not reimplementation. Reuse existing tests, command-backed evidence, and shipped summaries wherever they remain truthful.

### Requirements reconciliation rules
- `REQUIREMENTS.md` must become a truthful ledger of shipped versus unshipped milestone scope before archival.
- Summary frontmatter alone is insufficient; requirement status should be backed by explicit verification evidence or be left unresolved.
- Historical baseline phases with re-scoped work must remain distinguished from fully closed later phases. The requirements ledger should not flatten that nuance away.

### Scope discipline for closeout work
- Phase 18 is bookkeeping-heavy but still product-critical; it should not widen into new feature delivery.
- If a requirement cannot be marked complete honestly from existing shipped evidence, the phase should record that as unresolved or deferred instead of inventing optimistic status.
- Cross-phase integration and milestone archive/tag steps belong in Phase 19, not here.

### Claude's Discretion
- Exact grouping of the verification/reconciliation work into plan slices.
- Whether verification backfill is organized by historical phase groups, by requirement families, or by artifact type, provided the result is auditable and easy to review.
- How much automation to add for future milestone-closeout hygiene, as long as it does not become a large new subsystem.

</decisions>

<specifics>
## Specific Ideas

- The milestone audit already identified the three closeout blockers clearly: missing `VERIFICATION.md` coverage, stale requirement reconciliation, and inconsistent roadmap/archive inputs.
- The strongest repo signal is that shipped summaries often already contain command-backed verification evidence and `requirements-completed` frontmatter; the missing step is formalizing that into milestone-closeout artifacts.
- The phase should preserve the distinction already documented in [PROJECT.md](../../PROJECT.md) between shipped historical baselines and unfinished scope moved forward into later phases.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone closeout authority
- `.planning/v1.0-MILESTONE-AUDIT.md` — authoritative list of the closeout blockers this phase must address
- `.planning/ROADMAP.md` — Phase 18 goal, dependency, and closeout scope boundary
- `.planning/REQUIREMENTS.md` — current stale ledger that must be reconciled truthfully
- `.planning/STATE.md` — current workflow position and closeout history

### Product and historical truth
- `.planning/PROJECT.md` — authoritative distinction between validated work, historical baselines, and active milestone direction
- `docs/MASTER_PLAN.md` — implementation truth baseline for shipped architecture status

### Existing milestone evidence patterns
- `.planning/phases/1.1-preflight-pre-phase-2-hardening/1.1-VERIFICATION.md` — only current phase verification artifact; useful as the concrete format baseline
- `.planning/phases/*/*-SUMMARY.md` — shipped execution evidence, verification commands, and requirement frontmatter to reconcile into formal closeout state

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `*-SUMMARY.md` files already contain verification commands and requirement-frontmatter in many later phases; these are the primary raw materials for closeout reconciliation.
- `roadmap analyze` from `gsd-tools` already exposes where roadmap/state metadata is inconsistent; that can be reused as a closeout truth check.
- The milestone audit artifact now exists and should remain the source of truth for gap-driven planning.

### Established Patterns
- Earlier phases used explicit `VERIFICATION.md` artifacts when closeout rigor mattered; Phase 18 should return to that pattern rather than inventing a different audit format.
- Summary files are treated as repo truth and are repaired when later evidence closes an earlier gap; this phase can apply the same discipline at milestone scale.
- Planning docs in this repo prefer small reviewable slices with command-backed verification, even for documentation/bookkeeping work.

### Integration Points
- Phase 18 should touch `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, milestone audit artifacts, and per-phase verification outputs.
- Phase 19 will consume the repaired outputs from this phase to rerun audit and complete archival.

</code_context>

<deferred>
## Deferred Ideas

- Automating milestone archive generation more deeply than the current GSD workflow
- Broad cleanup of historical planning metadata beyond what is required for a truthful v1.0 closeout
- Any new product, runtime, or shell capability work

</deferred>

---

*Phase: 18-milestone-verification-backfill-and-requirement-reconciliation*
*Context gathered: 2026-03-19*
