# 39 Research

## Goal

Use `~/Downloads/Vel.csv` as a regression guardrail and closeout matrix for the Phase 34-38 usability arc without letting it replace the product rules already locked through operator interview and prior specs.

## Inputs

- [39-CONTEXT.md](/home/jove/code/vel/.planning/phases/39-vel-csv-regression-sweep-and-daily-use-closeout/39-CONTEXT.md)
- operator interview outcomes captured during the Phase 34-39 arc definition
- shipped Phase 34-38 changes across web, Apple, and current-day/runtime behavior
- `Vel.csv` in `~/Downloads` as acceptance pressure and regression evidence

## Key Findings

- the remaining risk is not missing raw capability; it is regression, duplication, and uneven daily-use quality across web and Apple
- `Vel.csv` is most valuable when converted into a structured matrix tied to already-locked product rules
- the acceptance spine is the operator’s “good Vel day,” so verification should follow wake-up through closeout rather than isolated feature checks
- any remaining rough edges after this phase must be explicitly deferred or fixed; silent UX drift is the failure mode

## Risks

- letting `Vel.csv` act as a second product authority will re-open decisions that were already settled
- closing only web or only Apple gaps will leave cross-surface parity claims untrustworthy
- trying to solve every planning/product ambition in this phase will dilute the closeout into another open-ended roadmap lane

## Recommended Shape

1. turn `Vel.csv` into a structured regression matrix mapped to the Phase 34-38 acceptance rules
2. fix remaining daily-use friction revealed by that matrix
3. verify the “good Vel day” flow across web and Apple
4. record milestone closeout truth plus explicit deferred items
