---
title: Vel Documentation Reconciliation and Canonical Status Spec
status: proposed
owner: docs / platform
created: 2026-03-16
updated: 2026-03-16
---

# Purpose

This spec defines the canonical ownership model for Vel documentation, resolves the specific mismatches identified in the repo scan, and establishes implementation tickets that make documentation track code rather than aspiration.

The immediate goals are to:

1. answer the open repo questions decisively,
2. reconcile known mismatches between docs and implementation,
3. define a single source of truth for implementation status,
4. define a clean API documentation structure for both `/v1` and `/api`, and
5. add guardrails so status drift does not quietly reappear.

# Problem statement

The repository currently has several documentation surfaces that overlap in authority:

- `docs/status.md`
- `docs/vel-documentation-index-and-implementation-status.md`
- focused subsystem docs such as `docs/chat-interface-status-and-outstanding.md`
- top-level `README.md`
- `docs/api.md`

These documents currently disagree on shipped behavior, route availability, payload shapes, and maturity labels. Some docs also describe planned behavior as if it were live. That creates operator confusion, client integration errors, and reviewer distrust.

# Decisions

## Decision 1: canonical implementation ledger

`docs/status.md` is the canonical repository-wide implementation ledger.

It owns the repo-wide answer to:

- what is implemented,
- what is partial,
- what is deferred,
- what is planned,
- what is experimental, and
- what the current truth is for each major subsystem.

It must be concise, current, and written from observed implementation truth.

## Decision 2: documentation index is not a second ledger

`docs/vel-documentation-index-and-implementation-status.md` becomes a navigation and coverage index, not a competing implementation ledger.

It may summarize status at a very high level, but it must defer to `docs/status.md` for canonical repo-wide implementation truth.

Allowed content:
- what docs exist,
- what area each doc covers,
- whether coverage is complete/incomplete,
- direct links to canonical status sections.

Disallowed content:
- detailed per-feature rollout truth that can drift independently from `docs/status.md`.

## Decision 3: focused subsystem docs own detail, not repo-wide truth

Focused docs such as `docs/chat-interface-status-and-outstanding.md` should describe subsystem-specific behavior, payloads, known limitations, and remaining work.

They may contain subsystem status detail, but they must not contradict `docs/status.md`. If a subsystem doc includes a status section, it should explicitly state that repo-wide rollout status is canonically tracked in `docs/status.md`.

## Decision 4: Apple client status vocabulary

Apple clients should be labeled:

- `bootstrap implemented`
- `product maturity deferred`

Meaning:
- the codebase contains a working bootstrap client or scaffold,
- the area is not yet considered product-complete or first-class.

The docs must stop using a binary implemented/deferred framing when the reality is staged maturity.

## Decision 5: one API documentation entrypoint for all externally relevant routes

The canonical API documentation must include both:

- `/v1` product/runtime API
- `/api` chat API

These can be separated into sections or child docs, but they must be discoverable from one canonical API entrypoint. No hidden second API universe.

Recommended structure:
- `docs/api/README.md` — canonical entrypoint and taxonomy
- `docs/api/runtime.md` — `/v1`
- `docs/api/chat.md` — `/api`

`docs/api.md` may either redirect to this structure or be replaced by the entrypoint.

## Decision 6: docs describe current behavior first

Any document that describes routes, handlers, or payloads must reflect current code behavior first.

Future behavior belongs in a clearly separated section such as:
- `Planned changes`
- `Open gaps`
- `Not yet implemented`

Aspirational behavior must never appear in the same tone or table as implemented behavior.

# Resolution of reported findings

## Risk API mismatch

Finding: docs say `GET /v1/risk` recomputes, but implementation is read-only and evaluation requires `POST /v1/evaluate` first.

Resolution:
- docs must be updated to describe the current contract,
- if recompute-on-GET is still desired, it should be treated as future work and explicitly labeled as not implemented.

Canonical current truth:
- `GET /v1/risk` reads current risk state
- `POST /v1/evaluate` triggers evaluation / recomputation workflow

## Nudges PATCH mismatch

Finding: docs claim `GET/PATCH /v1/nudges/:id`, but implementation only exposes read plus action routes such as done and snooze.

Resolution:
- remove or relabel `PATCH /v1/nudges/:id` from implementation docs unless actually added,
- action routes should be documented as the canonical mutation interface.

Canonical current truth:
- `GET /v1/nudges/:id` exists
- mutation occurs via action routes, not generic patch, unless code changes later.

## Chat status drift

Finding: status docs disagree on whether chat work through tickets 034 and 035 is done.

Resolution:
- `docs/status.md` must reflect current code-backed state,
- documentation index must defer to that status,
- subsystem chat doc should retain detailed implementation notes and outstanding gaps.

## Apple status ambiguity

Finding: mobile clients are described as deferred while Apple bootstrap is present.

Resolution:
- introduce staged maturity language,
- document Apple as bootstrap implemented / product deferred.

## Apple daemon URL mismatch

Finding: Apple README says `http://localhost:4242`, while actual defaults are `127.0.0.1:4130`.

Resolution:
- Apple README must be corrected to actual current defaults,
- if multiple environments are supported, document each explicitly.

## API entrypoint confusion

Finding: API docs do not present a single clear entrypoint for both `/v1` and `/api`.

Resolution:
- create `docs/api/README.md` with links to runtime and chat API docs,
- ensure `docs/api.md` either redirects or is refactored to map to this structure.

## Guardrails

To prevent re-drift:

- add explicit ownership notes to `docs/status.md` and the doc index,
- add a short “scope and authority” section to subsystem docs,
- consider periodic drift checks driven by the self-knowledge system.

