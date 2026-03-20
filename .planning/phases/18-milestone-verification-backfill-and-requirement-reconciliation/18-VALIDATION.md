# Phase 18 Validation

## Objective

Validate that milestone-closeout truth is restorable from repo artifacts, not chat memory.

## Required Outcomes

- A verification artifact exists for every milestone phase that still needs one for archival truth
- `REQUIREMENTS.md` reflects reconciled requirement status based on explicit evidence
- Historical baseline nuance remains preserved instead of being flattened into false completion
- Phase 19 can consume Phase 18 outputs without needing to rediscover milestone truth

## Validation Checks

- [ ] Every Phase 18 plan has explicit command-backed verification or deterministic artifact checks
- [ ] New or repaired `VERIFICATION.md` artifacts are present for the targeted phases
- [ ] `REQUIREMENTS.md` checkbox state and traceability rows are updated consistently
- [ ] No new product/runtime behavior is introduced
- [ ] Phase 19 inputs (`ROADMAP.md`, `REQUIREMENTS.md`, milestone audit references) are clearer after Phase 18 than before it

## Failure Conditions

- Verification artifacts are still missing for targeted milestone phases
- Requirements are marked complete without explicit supporting evidence
- Historical re-scoped work is incorrectly marked as shipped/validated
- The phase expands into archive/tag work that belongs to Phase 19

