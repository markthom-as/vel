---
title: Vel repo audit hardening
status: active
owner: agent
class: convergence
authority: execution
status_model:
  - todo
  - in_progress
  - done
  - deferred
created: 2026-03-17
updated: 2026-03-17
---

# Vel Repo Audit Hardening

This pack turns the current repo audit into an execution sequence.

Repo-wide shipped behavior still resolves through [docs/status.md](../../status.md). This pack owns convergence work, not implementation truth.

Primary spec:

- [docs/specs/vel-repo-audit-hardening-spec.md](../../specs/vel-repo-audit-hardening-spec.md)

## Why this pack exists

Vel now has enough code, docs, and planning surface area that repo hygiene has become first-order product work.

The main goals are:

1. make top-level docs strictly trustworthy,
2. make tests and loops reliable from a clean checkout,
3. normalize pack maturity and authority,
4. create a big-picture architecture map before major decomposition.

Current map outputs:

- [docs/architecture-inventory.md](../../architecture-inventory.md): current-state architecture inventory and drift audit
- [docs/future-architecture-map.md](../../future-architecture-map.md): future-facing synthesis that active decomposition work should converge toward

## Pack schema

This pack uses the following pack-level metadata:

- `class: convergence`
- `authority: execution`
- `status_model: todo | in_progress | done | deferred`
- `source_of_truth: docs/status.md`

Use this pack when the task is about repo truth, architecture cleanup, test hermeticity, or pack normalization.

Do not use this pack alone to answer "what is shipped now".

## Current status

- `001` docs truth repair and entrypoint alignment: done
- `002` hermetic local-integration tests and loop behavior: in_progress
- `003` pack schema and maturity normalization: done
- `004` architecture map and module-boundary audit: done
- `005` chat service-boundary extraction plan: done
- `006` storage-layer modularization plan: in_progress
- `007` frontend surface decomposition plan: done
- `008` doc drift guardrails expansion: done
- `009` active-work surfacing and overlap protocol: done

## Current focus

- `002` finish hermeticity verification once the unrelated in-flight `veld` integration compile work settles
- `006` continue internal `vel-storage` modularization with stable `Storage` facade boundaries

## Execution order

1. `001-docs-truth-repair-and-entrypoint-alignment.md`
2. `002-hermetic-local-integration-tests-and-loop-behavior.md`
3. `003-ticket-pack-schema-and-maturity-normalization.md`
4. `004-architecture-map-and-module-boundary-audit.md`
5. `005-chat-service-boundary-extraction-plan.md`
6. `006-storage-layer-modularization-plan.md`
7. `007-frontend-surface-decomposition-plan.md`
8. `008-doc-drift-guardrails-expansion.md`
9. `009-active-work-surfacing-and-agent-overlap-protocol.md`

## Exit criteria

- top-level operational docs are strict and current,
- test runs are hermetic with respect to optional local snapshot inputs,
- ticket packs have a documented schema and maturity framing,
- the repo has a big-picture architecture map that can guide future extraction work.
