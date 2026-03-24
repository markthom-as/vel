# Phase 96 Context

## Goal

Run browser proof and a direct acceptance audit against `TODO.md`, then close the milestone honestly only if the implemented line actually meets the remaining bar.

## Preconditions now satisfied

- API/persistence seams from Phase 93 are in place
- accepted runtime behavior from Phase 94 is wired truthfully
- the major remaining shell/surface polish implementation from Phase 95 is on disk
- local browser-proof tooling exists in `clients/web/scripts/proof/`

## Main implementation targets

- capture current browser evidence for `Now`, `Threads`, and `System`
- audit the milestone directly against `TODO.md`
- distinguish completed, partial, and deferred items honestly
- close the milestone only if the evidence supports it

## Risks

- a proof run may surface acceptance gaps that still require another implementation pass
- milestone closeout must not happen just because builds/tests are green
