# Phase 18 Research

## Conclusion

Phase 18 should be planned as a repo-truth and milestone-closeout phase, not a feature phase. No new ecosystem research is needed. The work is primarily:

- harvesting command-backed verification evidence already present in shipped summaries
- producing missing `VERIFICATION.md` artifacts in a consistent format
- reconciling `REQUIREMENTS.md` against those artifacts and against the historical baseline decisions already documented in [PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md)

## Key Findings

### Existing closeout inputs are uneven

- There are `76` `*-SUMMARY.md` files under `.planning/phases/`
- There is only `1` `*-VERIFICATION.md` file: [1.1-VERIFICATION.md](/home/jove/code/vel/.planning/phases/1.1-preflight-pre-phase-2-hardening/1.1-VERIFICATION.md)
- [v0.1-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.1-MILESTONE-AUDIT.md) already identifies the exact blockers, so Phase 18 should consume that audit rather than rediscovering it

### Summary files already contain most of the raw evidence

Many later summaries include:

- verification command lists
- notes about focused test coverage
- `requirements-completed` frontmatter

That means the backfill work is mostly formalization and reconciliation, not fresh implementation.

### Requirements truth needs explicit reconciliation rules

[REQUIREMENTS.md](/home/jove/code/vel/.planning/REQUIREMENTS.md) is currently stale:

- only `OPS-01` and `OPS-02` are checked
- later milestone requirements remain unchecked even where summaries claim completion
- historical Phases 2 and 4 were explicitly re-scoped, so requirement closure cannot simply mirror summary frontmatter without preserving that nuance

Phase 18 therefore needs a truth policy:

- mark complete only where verification evidence is explicit and milestone scope still claims closure
- preserve unfinished or re-scoped historical items as unresolved/deferred rather than rewriting history

### Useful reusable patterns already exist

- [1.1-VERIFICATION.md](/home/jove/code/vel/.planning/phases/1.1-preflight-pre-phase-2-hardening/1.1-VERIFICATION.md) provides the only existing concrete verification artifact pattern
- `roadmap analyze` is already a closeout consistency check for roadmap/state drift
- prior summaries sometimes record follow-up repairs when earlier evidence was incomplete; Phase 18 can use the same “repair repo truth” mindset at milestone scale

## Planning Implications

Phase 18 should be split into a few reviewable slices:

1. define the reconciliation rules and verification inventory
2. backfill historical verification artifacts for the earlier architecture/baseline phases
3. backfill verification artifacts for the later shipped product phases
4. reconcile `REQUIREMENTS.md` to the new verification truth and milestone audit inputs

Phase 19 should be left to consume these outputs for:

- roadmap/archive metadata cleanup
- rerunning the milestone audit
- final milestone closeout

## Risks

- It is easy to over-claim requirement completion if summary frontmatter is treated as authoritative by itself
- It is easy to over-correct and erase historical baseline nuance from Phases 2 and 4
- Bulk edits across many planning artifacts can become hard to review unless Phase 18 stays slice-based

## Recommendation

Plan Phase 18 as a 4-slice closeout phase centered on verification inventory, backfill, and requirements truth. Do not include archive generation, git tagging, or final re-audit in this phase.
