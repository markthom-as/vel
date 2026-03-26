# Phase 51: Cross-platform verification and closeout - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 51 is the milestone-wide verification and reconciliation lane for `v0.3`.

This phase owns:

- cross-platform evidence that web, iPhone/iPad, Mac, and reduced watch now follow the same canonical compact `Now` contract closely enough to claim parity
- verification that client mesh, sync/offline posture, and thread continuity remain Rust-owned and cross-surface instead of shell-local
- final milestone reconciliation so `v0.3` can close without hidden shell-specific carryover

This phase should prefer evidence-backed truth over new implementation. If new code is needed, it should only be the smallest slice required to close a concrete verification gap.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** The milestone closes only on execution-backed or source-backed evidence, not on planning intent.
- **D-02:** Known Apple toolchain limits must remain explicit if full app-target validation is unavailable in this environment.
- **D-03:** Verification should trace the same contract across surfaces: compact `Now`, thread continuity, compact mesh posture, and reduced-watch handoff.
- **D-04:** If a parity claim is only partially evidenced, document it as a limit instead of silently treating it as complete.
- **D-05:** Closeout docs should distinguish implemented milestone truth from post-milestone or still-limited follow-up.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- Web verification artifacts from Phase 49 already prove the compact canonical `Now` embodiment in the browser shell.
- Apple parity artifacts from Phase 50 already prove the iPhone/iPad, Mac, and reduced watch shell shifts at the source/doc level.
- Phase 46-50 docs now define the full v0.3 contract and implementation trail.

### Integration Points
- `/home/jove/code/vel/.planning/milestones/v0.3-phases/49-web-canonical-now-embodiment/*`
- `/home/jove/code/vel/.planning/milestones/v0.3-phases/50-apple-parity-and-reduced-watch-embodiment/*`
- `docs/user/daily-use.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- milestone closeout artifacts to be created during this phase

</code_context>

<specifics>
## Specific Ideas

- Gather one milestone-level verification packet that points back to the strongest per-phase evidence instead of rewriting every local proof.
- Preserve the Apple environment gap explicitly rather than pretending a source-only parity sweep equals a full Xcode validation run.
- Use this phase to decide whether the milestone is honestly closable; if so, it should flow directly into milestone audit/archive/cleanup.

</specifics>

<deferred>
## Deferred Ideas

- new cross-platform feature work
- deeper Apple validation beyond what the current environment can execute
- post-milestone UI refinements or contract expansion

</deferred>

---

*Phase: 51-cross-platform-verification-and-closeout*
*Context gathered: 2026-03-21*
